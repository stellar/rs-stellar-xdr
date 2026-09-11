//! Const-evaluable forms of the generated XDR types.
//!
//! Every type in the parent module has a name here. A type that owns heap data
//! has a borrowing equivalent of its own, holding `&'static` data so it can be
//! built in a const context; every other type is an alias to the owned type,
//! which const evaluation can already hold. So a value of any type can be
//! written as `const`, and encoded to XDR at compile time with the type's
//! `const_xdr_len` and `const_to_xdr`.
//!
//! The names are those of the parent module, without a suffix: `const::Memo` is
//! the const form of [`super::Memo`]. Types that cannot be built in a const
//! context (`Vec`, `String`, `Box`) are replaced by [`VecM`], [`BytesM`],
//! [`StringM`], and `&'static` references respectively.

use core::{ops::Deref, slice};

use super::{pad_len, Error, ErrorLengthExceedsMax};

#[cfg(feature = "arbitrary")]
use arbitrary::Arbitrary;

/// `padding` returns the zero bytes that pad an XDR value of the given length
/// out to a multiple of 4. The padding is never more than 3 bytes, so it is a
/// prefix of a single static.
const fn padding(len: usize) -> &'static [u8] {
    const PADDING: [u8; 3] = [0; 3];
    PADDING.split_at(pad_len(len)).0
}

/// `ConstWriter` serializes XDR into a fixed byte buffer using only const
/// operations.
///
/// It is the const-evaluable counterpart to [`super::WriteXdr::write_xdr`],
/// producing
/// the same bytes. Unlike the streaming path it enforces no depth or length
/// limits: a const value is fixed at compile time, so there is no untrusted
/// input to bound.
///
/// The writers below are the primitives. Every type defined in the XDR files
/// additionally has a generated `write_type_{type}` method on `ConstWriter`
/// that serializes one value of that type, taking the type as this module
/// defines it; a type that appears wrapped gets `write_type_option_{type}` and
/// `write_type_vec_{type}` alongside. The `type_` distinguishes them from these primitives, so a
/// wrapper over a primitive is named `write_option_u32` rather than
/// `write_type_option_u32`.
///
/// Keeping the encoders on the writer, rather than as inherent methods on each
/// generated type, leaves each type with only a thin `const_xdr_len` and
/// `const_to_xdr` pair that wraps its writer method. Each generated method is
/// emitted into the file of the type it serializes, so the two stay together.
///
/// Serialization is infallible. The only way it can fail is a value whose
/// length does not fit the `u32` XDR length prefix, which panics; in a const
/// context that is a compile-time error.
///
/// Bytes are only stored while the running length is within the buffer; bytes
/// past the end of the buffer are counted but not written. This allows the
/// exact encoded length to be measured by serializing into an empty buffer and
/// reading [`ConstWriter::len`], then serializing again into a buffer of that
/// size.
pub struct ConstWriter<'a> {
    buf: &'a mut [u8],
    len: usize,
}

impl<'a> ConstWriter<'a> {
    /// Constructs a new `ConstWriter` that serializes into `buf`.
    #[must_use]
    pub const fn new(buf: &'a mut [u8]) -> Self {
        ConstWriter { buf, len: 0 }
    }

    /// Returns the number of bytes serialized so far, which equals the total
    /// encoded length once serialization completes (even if it exceeded the
    /// buffer).
    #[must_use]
    pub const fn len(&self) -> usize {
        self.len
    }

    /// Returns true while nothing has been serialized yet.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Writes `data` into the buffer, advancing the length. Bytes beyond the
    /// end of the buffer are counted but not stored.
    const fn write_bytes(&mut self, data: &[u8]) {
        let mut i = 0;
        while i < data.len() {
            if self.len < self.buf.len() {
                self.buf[self.len] = data[i];
            }
            self.len += 1;
            i += 1;
        }
    }

    /// Serializes an `i32`, mirroring `<i32 as WriteXdr>::write_xdr`.
    pub const fn write_i32(&mut self, v: i32) {
        self.write_bytes(&v.to_be_bytes());
    }

    /// Serializes a `u32`, mirroring `<u32 as WriteXdr>::write_xdr`.
    pub const fn write_u32(&mut self, v: u32) {
        self.write_bytes(&v.to_be_bytes());
    }

    /// Serializes an `i64`, mirroring `<i64 as WriteXdr>::write_xdr`.
    pub const fn write_i64(&mut self, v: i64) {
        self.write_bytes(&v.to_be_bytes());
    }

    /// Serializes a `u64`, mirroring `<u64 as WriteXdr>::write_xdr`.
    pub const fn write_u64(&mut self, v: u64) {
        self.write_bytes(&v.to_be_bytes());
    }

    /// Serializes a `bool`, mirroring `<bool as WriteXdr>::write_xdr`.
    pub const fn write_bool(&mut self, v: bool) {
        let i = if v { 1u32 } else { 0u32 };
        self.write_u32(i);
    }

    /// Serializes a fixed-length opaque array with trailing padding, mirroring
    /// `<[u8; N] as WriteXdr>::write_xdr`.
    pub const fn write_fixed_opaque(&mut self, data: &[u8]) {
        self.write_bytes(data);
        self.write_bytes(padding(data.len()));
    }

    /// Serializes a `u32` length prefix from a `usize`, mirroring the
    /// `len.try_into()` and `len.write_xdr(w)` of the variable-length
    /// `WriteXdr` implementations.
    ///
    /// ### Panics
    ///
    /// If `len` does not fit in a `u32`. In a const context that is a
    /// compile-time error.
    #[allow(clippy::cast_possible_truncation)]
    pub const fn write_len(&mut self, len: usize) {
        assert!(len <= u32::MAX as usize, "xdr value max length exceeded");
        self.write_u32(len as u32);
    }

    /// Serializes a variable-length opaque byte sequence: a `u32` length
    /// prefix, the bytes, then trailing padding. Mirrors `<VecM<u8> as
    /// WriteXdr>::write_xdr`, `<BytesM as WriteXdr>::write_xdr`, and `<StringM
    /// as WriteXdr>::write_xdr`, which XDR encodes identically.
    pub const fn write_var_opaque(&mut self, data: &[u8]) {
        let n = data.len();
        self.write_len(n);
        self.write_bytes(data);
        self.write_bytes(padding(n));
    }

    // The `Option` and `VecM` serializers below are the ones whose inner type
    // is a builtin scalar. They are written by hand, beside the scalar
    // serializers they call, because they have no generated type to sit with:
    // the generator emits a wrapper into the file of the type it wraps, and a
    // scalar has no file. Wrappers over `opaque`, `string` and defined types
    // are still generated.

    /// Serializes an optional `i32`, mirroring `<Option<i32> as
    /// WriteXdr>::write_xdr`.
    pub const fn write_option_i32(&mut self, v: &Option<i32>) {
        match v {
            Some(v) => {
                self.write_u32(1);
                self.write_i32(*v);
            }
            None => {
                self.write_u32(0);
            }
        }
    }

    /// Serializes an optional `u32`, mirroring `<Option<u32> as
    /// WriteXdr>::write_xdr`.
    pub const fn write_option_u32(&mut self, v: &Option<u32>) {
        match v {
            Some(v) => {
                self.write_u32(1);
                self.write_u32(*v);
            }
            None => {
                self.write_u32(0);
            }
        }
    }

    /// Serializes an optional `i64`, mirroring `<Option<i64> as
    /// WriteXdr>::write_xdr`.
    pub const fn write_option_i64(&mut self, v: &Option<i64>) {
        match v {
            Some(v) => {
                self.write_u32(1);
                self.write_i64(*v);
            }
            None => {
                self.write_u32(0);
            }
        }
    }

    /// Serializes an optional `u64`, mirroring `<Option<u64> as
    /// WriteXdr>::write_xdr`.
    pub const fn write_option_u64(&mut self, v: &Option<u64>) {
        match v {
            Some(v) => {
                self.write_u32(1);
                self.write_u64(*v);
            }
            None => {
                self.write_u32(0);
            }
        }
    }

    /// Serializes an optional `bool`, mirroring `<Option<bool> as
    /// WriteXdr>::write_xdr`.
    pub const fn write_option_bool(&mut self, v: &Option<bool>) {
        match v {
            Some(v) => {
                self.write_u32(1);
                self.write_bool(*v);
            }
            None => {
                self.write_u32(0);
            }
        }
    }

    /// Serializes a variable-length array of `i32`, mirroring `<VecM<i32, MAX>
    /// as WriteXdr>::write_xdr`.
    pub const fn write_vec_i32<const MAX: u32>(&mut self, v: &VecM<i32, MAX>) {
        let s = v.as_slice();
        let len = s.len();
        self.write_len(len);
        let mut i = 0usize;
        while i < len {
            self.write_i32(s[i]);
            i += 1;
        }
    }

    /// Serializes a variable-length array of `u32`, mirroring `<VecM<u32, MAX>
    /// as WriteXdr>::write_xdr`.
    pub const fn write_vec_u32<const MAX: u32>(&mut self, v: &VecM<u32, MAX>) {
        let s = v.as_slice();
        let len = s.len();
        self.write_len(len);
        let mut i = 0usize;
        while i < len {
            self.write_u32(s[i]);
            i += 1;
        }
    }

    /// Serializes a variable-length array of `i64`, mirroring `<VecM<i64, MAX>
    /// as WriteXdr>::write_xdr`.
    pub const fn write_vec_i64<const MAX: u32>(&mut self, v: &VecM<i64, MAX>) {
        let s = v.as_slice();
        let len = s.len();
        self.write_len(len);
        let mut i = 0usize;
        while i < len {
            self.write_i64(s[i]);
            i += 1;
        }
    }

    /// Serializes a variable-length array of `u64`, mirroring `<VecM<u64, MAX>
    /// as WriteXdr>::write_xdr`.
    pub const fn write_vec_u64<const MAX: u32>(&mut self, v: &VecM<u64, MAX>) {
        let s = v.as_slice();
        let len = s.len();
        self.write_len(len);
        let mut i = 0usize;
        while i < len {
            self.write_u64(s[i]);
            i += 1;
        }
    }

    /// Serializes a variable-length array of `bool`, mirroring `<VecM<bool, MAX>
    /// as WriteXdr>::write_xdr`.
    pub const fn write_vec_bool<const MAX: u32>(&mut self, v: &VecM<bool, MAX>) {
        let s = v.as_slice();
        let len = s.len();
        self.write_len(len);
        let mut i = 0usize;
        while i < len {
            self.write_bool(s[i]);
            i += 1;
        }
    }
}

/// A borrowing equivalent of [`super::VecM`] that wraps a slice instead of owning a
/// `Vec`, enforcing the same maximum length `MAX` at construction.
///
/// Usable in const contexts to build values of this module's types from slices
/// of fixed-size arrays, without heap allocation.
#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct VecM<T: 'static, const MAX: u32 = { u32::MAX }>(&'static [T]);

// Copy and Clone are implemented manually because the derived impls would
// require `T: Copy`/`T: Clone`, and the wrapped `&[T]` is copyable for any
// `T`.
impl<T, const MAX: u32> Copy for VecM<T, MAX> {}

#[allow(clippy::expl_impl_clone_on_copy)]
impl<T, const MAX: u32> Clone for VecM<T, MAX> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T, const MAX: u32> Deref for VecM<T, MAX> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<T, const MAX: u32> Default for VecM<T, MAX> {
    fn default() -> Self {
        Self(&[])
    }
}

impl<T, const MAX: u32> VecM<T, MAX> {
    pub const MAX_LEN: usize = { MAX as usize };

    /// Constructs a `VecM` from the given slice, erroring if the length of
    /// the slice exceeds `MAX`.
    ///
    /// The error is [`ErrorLengthExceedsMax`] rather than [`Error`] so the result
    /// can be matched in a const context; use `?` to convert it to [`Error`].
    ///
    /// ### Errors
    ///
    /// If the length of the slice exceeds `MAX`.
    pub const fn try_from_slice(v: &'static [T]) -> Result<Self, ErrorLengthExceedsMax> {
        if v.len() <= Self::MAX_LEN {
            Ok(Self(v))
        } else {
            Err(ErrorLengthExceedsMax)
        }
    }

    /// Constructs a `VecM` from the given slice, panicking if the length of
    /// the slice exceeds `MAX`.
    ///
    /// Usable in const contexts, where an over-length slice is a compile-time
    /// error. Prefer [`Self::try_from_slice`] where a [`Result`] is wanted.
    ///
    /// ### Panics
    ///
    /// If the length of the slice exceeds `MAX`.
    #[must_use]
    pub const fn try_from_slice_or_panic(v: &'static [T]) -> Self {
        match Self::try_from_slice(v) {
            Ok(r) => r,
            Err(ErrorLengthExceedsMax) => panic!("xdr value max length exceeded"),
        }
    }

    #[must_use]
    #[allow(clippy::unused_self)]
    pub const fn max_len(&self) -> usize {
        Self::MAX_LEN
    }

    #[must_use]
    pub const fn as_slice(&self) -> &'static [T] {
        self.0
    }

    #[must_use]
    pub const fn len(&self) -> usize {
        self.0.len()
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn iter(&self) -> slice::Iter<'static, T> {
        self.0.iter()
    }
}

impl<T, const MAX: u32> core::iter::IntoIterator for &VecM<T, MAX> {
    type Item = &'static T;
    type IntoIter = slice::Iter<'static, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T, const MAX: u32> TryFrom<&'static [T]> for VecM<T, MAX> {
    type Error = Error;

    fn try_from(v: &'static [T]) -> Result<Self, Error> {
        Ok(Self::try_from_slice(v)?)
    }
}

/// A borrowing equivalent of [`super::BytesM`] that wraps a byte slice instead of
/// owning a `Vec`, enforcing the same maximum length `MAX` at construction.
///
/// Usable in const contexts to build values of this module's types from slices
/// of fixed-size arrays, without heap allocation.
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct BytesM<const MAX: u32 = { u32::MAX }>(&'static [u8]);

impl<const MAX: u32> core::fmt::Display for BytesM<MAX> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for b in self.0 {
            write!(f, "{b:02x}")?;
        }
        Ok(())
    }
}

impl<const MAX: u32> core::fmt::Debug for BytesM<MAX> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "BytesM(")?;
        for b in self.0 {
            write!(f, "{b:02x}")?;
        }
        write!(f, ")")?;
        Ok(())
    }
}

impl<const MAX: u32> Deref for BytesM<MAX> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<const MAX: u32> Default for BytesM<MAX> {
    fn default() -> Self {
        Self(&[])
    }
}

impl<const MAX: u32> BytesM<MAX> {
    pub const MAX_LEN: usize = { MAX as usize };

    /// Constructs a `BytesM` from the given slice, erroring if the length
    /// of the slice exceeds `MAX`.
    ///
    /// The error is [`ErrorLengthExceedsMax`] rather than [`Error`] so the result
    /// can be matched in a const context; use `?` to convert it to [`Error`].
    ///
    /// ### Errors
    ///
    /// If the length of the slice exceeds `MAX`.
    pub const fn try_from_slice(v: &'static [u8]) -> Result<Self, ErrorLengthExceedsMax> {
        if v.len() <= Self::MAX_LEN {
            Ok(Self(v))
        } else {
            Err(ErrorLengthExceedsMax)
        }
    }

    /// Constructs a `BytesM` from the given slice, panicking if the length
    /// of the slice exceeds `MAX`.
    ///
    /// Usable in const contexts, where an over-length slice is a compile-time
    /// error. Prefer [`Self::try_from_slice`] where a [`Result`] is wanted.
    ///
    /// ### Panics
    ///
    /// If the length of the slice exceeds `MAX`.
    #[must_use]
    pub const fn try_from_slice_or_panic(v: &'static [u8]) -> Self {
        match Self::try_from_slice(v) {
            Ok(r) => r,
            Err(ErrorLengthExceedsMax) => panic!("xdr value max length exceeded"),
        }
    }

    #[must_use]
    #[allow(clippy::unused_self)]
    pub const fn max_len(&self) -> usize {
        Self::MAX_LEN
    }

    #[must_use]
    pub const fn as_slice(&self) -> &'static [u8] {
        self.0
    }

    #[must_use]
    pub const fn len(&self) -> usize {
        self.0.len()
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<const MAX: u32> TryFrom<&'static [u8]> for BytesM<MAX> {
    type Error = Error;

    fn try_from(v: &'static [u8]) -> Result<Self, Error> {
        Ok(Self::try_from_slice(v)?)
    }
}

/// A borrowing equivalent of [`super::StringM`] that wraps a byte slice instead of
/// owning a `Vec`, enforcing the same maximum length `MAX` at construction.
///
/// Usable in const contexts to build values of this module's types from slices
/// of fixed-size arrays or string literals, without heap allocation.
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct StringM<const MAX: u32 = { u32::MAX }>(&'static [u8]);

impl<const MAX: u32> core::fmt::Display for StringM<MAX> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for b in escape_bytes::Escape::new(self.0) {
            write!(f, "{}", b as char)?;
        }
        Ok(())
    }
}

impl<const MAX: u32> core::fmt::Debug for StringM<MAX> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "StringM(")?;
        for b in escape_bytes::Escape::new(self.0) {
            write!(f, "{}", b as char)?;
        }
        write!(f, ")")?;
        Ok(())
    }
}

impl<const MAX: u32> Deref for StringM<MAX> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<const MAX: u32> Default for StringM<MAX> {
    fn default() -> Self {
        Self(&[])
    }
}

impl<const MAX: u32> StringM<MAX> {
    pub const MAX_LEN: usize = { MAX as usize };

    /// Constructs a `StringM` from the given slice, erroring if the length
    /// of the slice exceeds `MAX`.
    ///
    /// The error is [`ErrorLengthExceedsMax`] rather than [`Error`] so the result
    /// can be matched in a const context; use `?` to convert it to [`Error`].
    ///
    /// ### Errors
    ///
    /// If the length of the slice exceeds `MAX`.
    pub const fn try_from_slice(v: &'static [u8]) -> Result<Self, ErrorLengthExceedsMax> {
        if v.len() <= Self::MAX_LEN {
            Ok(Self(v))
        } else {
            Err(ErrorLengthExceedsMax)
        }
    }

    /// Constructs a `StringM` from the given slice, panicking if the length
    /// of the slice exceeds `MAX`.
    ///
    /// Usable in const contexts, where an over-length slice is a compile-time
    /// error. Prefer [`Self::try_from_slice`] where a [`Result`] is wanted.
    ///
    /// ### Panics
    ///
    /// If the length of the slice exceeds `MAX`.
    #[must_use]
    pub const fn try_from_slice_or_panic(v: &'static [u8]) -> Self {
        match Self::try_from_slice(v) {
            Ok(r) => r,
            Err(ErrorLengthExceedsMax) => panic!("xdr value max length exceeded"),
        }
    }

    /// Constructs a `StringM` from the UTF-8 bytes of the given str,
    /// erroring if the length of the str exceeds `MAX`.
    ///
    /// The error is [`ErrorLengthExceedsMax`] rather than [`Error`] so the result
    /// can be matched in a const context; use `?` to convert it to [`Error`].
    ///
    /// ### Errors
    ///
    /// If the length of the str exceeds `MAX`.
    pub const fn try_from_str(s: &'static str) -> Result<Self, ErrorLengthExceedsMax> {
        Self::try_from_slice(s.as_bytes())
    }

    /// Constructs a `StringM` from the UTF-8 bytes of the given str,
    /// panicking if the length of the str exceeds `MAX`.
    ///
    /// Usable in const contexts, where an over-length str is a compile-time
    /// error. Prefer [`Self::try_from_str`] where a [`Result`] is wanted.
    ///
    /// ### Panics
    ///
    /// If the length of the str exceeds `MAX`.
    #[must_use]
    pub const fn try_from_str_or_panic(s: &'static str) -> Self {
        Self::try_from_slice_or_panic(s.as_bytes())
    }

    #[must_use]
    #[allow(clippy::unused_self)]
    pub const fn max_len(&self) -> usize {
        Self::MAX_LEN
    }

    #[must_use]
    pub const fn as_slice(&self) -> &'static [u8] {
        self.0
    }

    #[must_use]
    pub const fn len(&self) -> usize {
        self.0.len()
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<const MAX: u32> TryFrom<&'static [u8]> for StringM<MAX> {
    type Error = Error;

    fn try_from(v: &'static [u8]) -> Result<Self, Error> {
        Ok(Self::try_from_slice(v)?)
    }
}

impl<const MAX: u32> TryFrom<&'static str> for StringM<MAX> {
    type Error = Error;

    fn try_from(s: &'static str) -> Result<Self, Error> {
        Ok(Self::try_from_str(s)?)
    }
}

/// `Arbitrary` support for the const types, mirroring the owned types byte for
/// byte.
///
/// Each impl below delegates to the `Arbitrary` impl of the owned field the
/// const type replaces — `VecM` to `Vec<T>`, `BytesM` and `StringM` to
/// `Vec<u8>`, a `&'static T` recursive reference to `Box<T>` — and then leaks
/// the result to obtain the `'static` data the const types hold. Consuming the
/// same bytes in the same order as the owned type is what makes a matched pair
/// possible: driving [`arbitrary::Arbitrary`] for an owned type and for its
/// const counterpart from the same input produces the same value in both
/// forms, without any conversion between them.
///
/// Leaking is why this is behind the `arbitrary` feature, which exists for
/// fuzzing and testing: every value built this way holds its memory for the
/// life of the process. A bounded number of values, as in a test or a fuzz
/// target that caps its iterations, is fine; an unbounded fuzzing run grows
/// without limit.
#[cfg(feature = "arbitrary")]
mod arbitrary_impls {
    use super::{BytesM, StringM, VecM};
    use arbitrary::{Arbitrary, Result, Unstructured};

    /// Builds an arbitrary `T` behind a `&'static` reference, mirroring
    /// `Box<T>`, for the recursive references the const types hold in place of
    /// a `Box`.
    ///
    /// Named in the generated types by `#[arbitrary(with = ...)]`.
    ///
    /// ### Errors
    ///
    /// If the underlying `T` cannot be built from the remaining input.
    pub fn arbitrary_ref<'a, T: Arbitrary<'a> + 'static>(
        u: &mut Unstructured<'a>,
    ) -> Result<&'static T> {
        Ok(Box::leak(Box::new(T::arbitrary(u)?)))
    }

    /// The [`arbitrary_ref`] equivalent for an optional recursive reference,
    /// mirroring `Option<Box<T>>`.
    ///
    /// ### Errors
    ///
    /// If the underlying value cannot be built from the remaining input.
    pub fn arbitrary_option_ref<'a, T: Arbitrary<'a> + 'static>(
        u: &mut Unstructured<'a>,
    ) -> Result<Option<&'static T>> {
        Ok(match Option::<T>::arbitrary(u)? {
            Some(v) => Some(Box::leak(Box::new(v))),
            None => None,
        })
    }

    impl<'a, T: Arbitrary<'a> + 'static, const MAX: u32> Arbitrary<'a> for VecM<T, MAX> {
        fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
            Ok(Self(Vec::leak(Vec::<T>::arbitrary(u)?)))
        }
    }

    impl<'a, const MAX: u32> Arbitrary<'a> for BytesM<MAX> {
        fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
            Ok(Self(Vec::leak(Vec::<u8>::arbitrary(u)?)))
        }
    }

    impl<'a, const MAX: u32> Arbitrary<'a> for StringM<MAX> {
        fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
            Ok(Self(Vec::leak(Vec::<u8>::arbitrary(u)?)))
        }
    }
}

#[cfg(feature = "arbitrary")]
pub use arbitrary_impls::{arbitrary_option_ref, arbitrary_ref};

mod value;
#[allow(unused_imports)]
pub use value::*;
mod scp_ballot;
#[allow(unused_imports)]
pub use scp_ballot::*;
mod scp_statement_type;
pub use super::ScpStatementType;
#[allow(unused_imports)]
pub use scp_statement_type::*;
mod scp_nomination;
#[allow(unused_imports)]
pub use scp_nomination::*;
mod scp_statement_prepare;
#[allow(unused_imports)]
pub use scp_statement_prepare::*;
mod scp_statement_confirm;
#[allow(unused_imports)]
pub use scp_statement_confirm::*;
mod scp_statement_externalize;
#[allow(unused_imports)]
pub use scp_statement_externalize::*;
mod scp_statement_pledges;
#[allow(unused_imports)]
pub use scp_statement_pledges::*;
mod scp_statement;
#[allow(unused_imports)]
pub use scp_statement::*;
mod scp_envelope;
#[allow(unused_imports)]
pub use scp_envelope::*;
mod scp_quorum_set;
#[allow(unused_imports)]
pub use scp_quorum_set::*;
mod encoded_ledger_key;
#[allow(unused_imports)]
pub use encoded_ledger_key::*;
mod config_setting_contract_execution_lanes_v0;
pub use super::ConfigSettingContractExecutionLanesV0;
#[allow(unused_imports)]
pub use config_setting_contract_execution_lanes_v0::*;
mod config_setting_contract_compute_v0;
pub use super::ConfigSettingContractComputeV0;
#[allow(unused_imports)]
pub use config_setting_contract_compute_v0::*;
mod config_setting_contract_parallel_compute_v0;
pub use super::ConfigSettingContractParallelComputeV0;
#[allow(unused_imports)]
pub use config_setting_contract_parallel_compute_v0::*;
mod config_setting_contract_ledger_cost_v0;
pub use super::ConfigSettingContractLedgerCostV0;
#[allow(unused_imports)]
pub use config_setting_contract_ledger_cost_v0::*;
mod config_setting_contract_ledger_cost_ext_v0;
pub use super::ConfigSettingContractLedgerCostExtV0;
#[allow(unused_imports)]
pub use config_setting_contract_ledger_cost_ext_v0::*;
mod config_setting_contract_historical_data_v0;
pub use super::ConfigSettingContractHistoricalDataV0;
#[allow(unused_imports)]
pub use config_setting_contract_historical_data_v0::*;
mod config_setting_contract_events_v0;
pub use super::ConfigSettingContractEventsV0;
#[allow(unused_imports)]
pub use config_setting_contract_events_v0::*;
mod config_setting_contract_bandwidth_v0;
pub use super::ConfigSettingContractBandwidthV0;
#[allow(unused_imports)]
pub use config_setting_contract_bandwidth_v0::*;
mod contract_cost_type;
pub use super::ContractCostType;
#[allow(unused_imports)]
pub use contract_cost_type::*;
mod contract_cost_param_entry;
pub use super::ContractCostParamEntry;
#[allow(unused_imports)]
pub use contract_cost_param_entry::*;
mod state_archival_settings;
pub use super::StateArchivalSettings;
#[allow(unused_imports)]
pub use state_archival_settings::*;
mod eviction_iterator;
pub use super::EvictionIterator;
#[allow(unused_imports)]
pub use eviction_iterator::*;
mod config_setting_scp_timing;
pub use super::ConfigSettingScpTiming;
#[allow(unused_imports)]
pub use config_setting_scp_timing::*;
mod frozen_ledger_keys;
#[allow(unused_imports)]
pub use frozen_ledger_keys::*;
mod frozen_ledger_keys_delta;
#[allow(unused_imports)]
pub use frozen_ledger_keys_delta::*;
mod freeze_bypass_txs;
#[allow(unused_imports)]
pub use freeze_bypass_txs::*;
mod freeze_bypass_txs_delta;
pub use super::CONTRACT_COST_COUNT_LIMIT;
#[allow(unused_imports)]
pub use freeze_bypass_txs_delta::*;
mod contract_cost_params;
#[allow(unused_imports)]
pub use contract_cost_params::*;
mod config_setting_id;
pub use super::ConfigSettingId;
#[allow(unused_imports)]
pub use config_setting_id::*;
mod config_setting_entry;
#[allow(unused_imports)]
pub use config_setting_entry::*;
mod sc_env_meta_kind;
pub use super::ScEnvMetaKind;
#[allow(unused_imports)]
pub use sc_env_meta_kind::*;
mod sc_env_meta_entry_interface_version;
pub use super::ScEnvMetaEntryInterfaceVersion;
#[allow(unused_imports)]
pub use sc_env_meta_entry_interface_version::*;
mod sc_env_meta_entry;
pub use super::ScEnvMetaEntry;
#[allow(unused_imports)]
pub use sc_env_meta_entry::*;
mod sc_meta_v0;
#[allow(unused_imports)]
pub use sc_meta_v0::*;
mod sc_meta_kind;
pub use super::ScMetaKind;
#[allow(unused_imports)]
pub use sc_meta_kind::*;
mod sc_meta_entry;
pub use super::SC_SPEC_DOC_LIMIT;
#[allow(unused_imports)]
pub use sc_meta_entry::*;
mod sc_spec_type;
pub use super::ScSpecType;
#[allow(unused_imports)]
pub use sc_spec_type::*;
mod sc_spec_type_option;
#[allow(unused_imports)]
pub use sc_spec_type_option::*;
mod sc_spec_type_result;
#[allow(unused_imports)]
pub use sc_spec_type_result::*;
mod sc_spec_type_vec;
#[allow(unused_imports)]
pub use sc_spec_type_vec::*;
mod sc_spec_type_map;
#[allow(unused_imports)]
pub use sc_spec_type_map::*;
mod sc_spec_type_tuple;
#[allow(unused_imports)]
pub use sc_spec_type_tuple::*;
mod sc_spec_type_bytes_n;
pub use super::ScSpecTypeBytesN;
#[allow(unused_imports)]
pub use sc_spec_type_bytes_n::*;
mod sc_spec_type_udt;
#[allow(unused_imports)]
pub use sc_spec_type_udt::*;
mod sc_spec_type_def;
#[allow(unused_imports)]
pub use sc_spec_type_def::*;
mod sc_spec_udt_struct_field_v0;
#[allow(unused_imports)]
pub use sc_spec_udt_struct_field_v0::*;
mod sc_spec_udt_struct_v0;
#[allow(unused_imports)]
pub use sc_spec_udt_struct_v0::*;
mod sc_spec_udt_union_case_void_v0;
#[allow(unused_imports)]
pub use sc_spec_udt_union_case_void_v0::*;
mod sc_spec_udt_union_case_tuple_v0;
#[allow(unused_imports)]
pub use sc_spec_udt_union_case_tuple_v0::*;
mod sc_spec_udt_union_case_v0_kind;
pub use super::ScSpecUdtUnionCaseV0Kind;
#[allow(unused_imports)]
pub use sc_spec_udt_union_case_v0_kind::*;
mod sc_spec_udt_union_case_v0;
#[allow(unused_imports)]
pub use sc_spec_udt_union_case_v0::*;
mod sc_spec_udt_union_v0;
#[allow(unused_imports)]
pub use sc_spec_udt_union_v0::*;
mod sc_spec_udt_enum_case_v0;
#[allow(unused_imports)]
pub use sc_spec_udt_enum_case_v0::*;
mod sc_spec_udt_enum_v0;
#[allow(unused_imports)]
pub use sc_spec_udt_enum_v0::*;
mod sc_spec_udt_error_enum_case_v0;
#[allow(unused_imports)]
pub use sc_spec_udt_error_enum_case_v0::*;
mod sc_spec_udt_error_enum_v0;
#[allow(unused_imports)]
pub use sc_spec_udt_error_enum_v0::*;
mod sc_spec_function_input_v0;
#[allow(unused_imports)]
pub use sc_spec_function_input_v0::*;
mod sc_spec_function_v0;
#[allow(unused_imports)]
pub use sc_spec_function_v0::*;
mod sc_spec_event_param_location_v0;
pub use super::ScSpecEventParamLocationV0;
#[allow(unused_imports)]
pub use sc_spec_event_param_location_v0::*;
mod sc_spec_event_param_v0;
#[allow(unused_imports)]
pub use sc_spec_event_param_v0::*;
mod sc_spec_event_data_format;
pub use super::ScSpecEventDataFormat;
#[allow(unused_imports)]
pub use sc_spec_event_data_format::*;
mod sc_spec_event_v0;
#[allow(unused_imports)]
pub use sc_spec_event_v0::*;
mod sc_spec_entry_kind;
pub use super::ScSpecEntryKind;
#[allow(unused_imports)]
pub use sc_spec_entry_kind::*;
mod sc_spec_entry;
#[allow(unused_imports)]
pub use sc_spec_entry::*;
mod sc_bytes;
#[allow(unused_imports)]
pub use sc_bytes::*;
mod sc_string;
pub use super::SCSYMBOL_LIMIT;
#[allow(unused_imports)]
pub use sc_string::*;
mod sc_symbol;
#[allow(unused_imports)]
pub use sc_symbol::*;
mod sc_val_type;
pub use super::ScValType;
#[allow(unused_imports)]
pub use sc_val_type::*;
mod sc_error_type;
pub use super::ScErrorType;
#[allow(unused_imports)]
pub use sc_error_type::*;
mod sc_error_code;
pub use super::ScErrorCode;
#[allow(unused_imports)]
pub use sc_error_code::*;
mod sc_error;
pub use super::ScError;
#[allow(unused_imports)]
pub use sc_error::*;
mod u_int128_parts;
pub use super::UInt128Parts;
#[allow(unused_imports)]
pub use u_int128_parts::*;
mod int128_parts;
pub use super::Int128Parts;
#[allow(unused_imports)]
pub use int128_parts::*;
mod u_int256_parts;
pub use super::UInt256Parts;
#[allow(unused_imports)]
pub use u_int256_parts::*;
mod int256_parts;
pub use super::Int256Parts;
#[allow(unused_imports)]
pub use int256_parts::*;
mod contract_executable_type;
pub use super::ContractExecutableType;
#[allow(unused_imports)]
pub use contract_executable_type::*;
mod sc_address_type;
pub use super::ScAddressType;
#[allow(unused_imports)]
pub use sc_address_type::*;
mod muxed_ed25519_account;
pub use super::MuxedEd25519Account;
#[allow(unused_imports)]
pub use muxed_ed25519_account::*;
mod muxed_contract;
#[cfg(feature = "cap_0084_muxed_contract")]
pub use super::MuxedContract;
#[allow(unused_imports)]
pub use muxed_contract::*;
mod sc_address;
pub use super::ScAddress;
#[allow(unused_imports)]
pub use sc_address::*;
mod contract_executable_external_ref;
#[allow(unused_imports)]
pub use contract_executable_external_ref::*;
mod contract_executable;
#[allow(unused_imports)]
pub use contract_executable::*;
mod sc_vec;
#[allow(unused_imports)]
pub use sc_vec::*;
mod sc_map;
#[allow(unused_imports)]
pub use sc_map::*;
mod sc_nonce_key;
pub use super::ScNonceKey;
#[allow(unused_imports)]
pub use sc_nonce_key::*;
mod sc_contract_instance;
#[allow(unused_imports)]
pub use sc_contract_instance::*;
mod sc_val;
#[allow(unused_imports)]
pub use sc_val::*;
mod sc_map_entry;
#[allow(unused_imports)]
pub use sc_map_entry::*;
mod ledger_close_meta_batch;
#[allow(unused_imports)]
pub use ledger_close_meta_batch::*;
mod stored_transaction_set;
#[allow(unused_imports)]
pub use stored_transaction_set::*;
mod stored_debug_transaction_set;
#[allow(unused_imports)]
pub use stored_debug_transaction_set::*;
mod persisted_scp_state_v0;
#[allow(unused_imports)]
pub use persisted_scp_state_v0::*;
mod persisted_scp_state_v1;
#[allow(unused_imports)]
pub use persisted_scp_state_v1::*;
mod persisted_scp_state;
#[allow(unused_imports)]
pub use persisted_scp_state::*;
mod thresholds;
pub use super::Thresholds;
#[allow(unused_imports)]
pub use thresholds::*;
mod string32;
#[allow(unused_imports)]
pub use string32::*;
mod string64;
#[allow(unused_imports)]
pub use string64::*;
mod sequence_number;
pub use super::SequenceNumber;
#[allow(unused_imports)]
pub use sequence_number::*;
mod data_value;
#[allow(unused_imports)]
pub use data_value::*;
mod asset_code4;
pub use super::AssetCode4;
#[allow(unused_imports)]
pub use asset_code4::*;
mod asset_code12;
pub use super::AssetCode12;
#[allow(unused_imports)]
pub use asset_code12::*;
mod asset_type;
pub use super::AssetType;
#[allow(unused_imports)]
pub use asset_type::*;
mod asset_code;
pub use super::AssetCode;
#[allow(unused_imports)]
pub use asset_code::*;
mod alpha_num4;
pub use super::AlphaNum4;
#[allow(unused_imports)]
pub use alpha_num4::*;
mod alpha_num12;
pub use super::AlphaNum12;
#[allow(unused_imports)]
pub use alpha_num12::*;
mod asset;
pub use super::Asset;
#[allow(unused_imports)]
pub use asset::*;
mod price;
pub use super::Price;
#[allow(unused_imports)]
pub use price::*;
mod liabilities;
pub use super::Liabilities;
#[allow(unused_imports)]
pub use liabilities::*;
mod threshold_indexes;
pub use super::ThresholdIndexes;
#[allow(unused_imports)]
pub use threshold_indexes::*;
mod ledger_entry_type;
pub use super::LedgerEntryType;
#[allow(unused_imports)]
pub use ledger_entry_type::*;
mod signer;
#[allow(unused_imports)]
pub use signer::*;
mod account_flags;
pub use super::AccountFlags;
pub use super::MASK_ACCOUNT_FLAGS;
pub use super::MASK_ACCOUNT_FLAGS_V17;
pub use super::MAX_SIGNERS;
#[allow(unused_imports)]
pub use account_flags::*;
mod sponsorship_descriptor;
pub use super::SponsorshipDescriptor;
#[allow(unused_imports)]
pub use sponsorship_descriptor::*;
mod account_entry_extension_v3;
pub use super::AccountEntryExtensionV3;
#[allow(unused_imports)]
pub use account_entry_extension_v3::*;
mod account_entry_extension_v2_ext;
pub use super::AccountEntryExtensionV2Ext;
#[allow(unused_imports)]
pub use account_entry_extension_v2_ext::*;
mod account_entry_extension_v2;
#[allow(unused_imports)]
pub use account_entry_extension_v2::*;
mod account_entry_extension_v1_ext;
#[allow(unused_imports)]
pub use account_entry_extension_v1_ext::*;
mod account_entry_extension_v1;
#[allow(unused_imports)]
pub use account_entry_extension_v1::*;
mod account_entry_ext;
#[allow(unused_imports)]
pub use account_entry_ext::*;
mod account_entry;
#[allow(unused_imports)]
pub use account_entry::*;
mod trust_line_flags;
pub use super::TrustLineFlags;
pub use super::MASK_TRUSTLINE_FLAGS;
pub use super::MASK_TRUSTLINE_FLAGS_V13;
pub use super::MASK_TRUSTLINE_FLAGS_V17;
#[allow(unused_imports)]
pub use trust_line_flags::*;
mod liquidity_pool_type;
pub use super::LiquidityPoolType;
#[allow(unused_imports)]
pub use liquidity_pool_type::*;
mod trust_line_asset;
pub use super::TrustLineAsset;
#[allow(unused_imports)]
pub use trust_line_asset::*;
mod trust_line_entry_extension_v2_ext;
pub use super::TrustLineEntryExtensionV2Ext;
#[allow(unused_imports)]
pub use trust_line_entry_extension_v2_ext::*;
mod trust_line_entry_extension_v2;
pub use super::TrustLineEntryExtensionV2;
#[allow(unused_imports)]
pub use trust_line_entry_extension_v2::*;
mod trust_line_entry_v1_ext;
pub use super::TrustLineEntryV1Ext;
#[allow(unused_imports)]
pub use trust_line_entry_v1_ext::*;
mod trust_line_entry_v1;
pub use super::TrustLineEntryV1;
#[allow(unused_imports)]
pub use trust_line_entry_v1::*;
mod trust_line_entry_ext;
pub use super::TrustLineEntryExt;
#[allow(unused_imports)]
pub use trust_line_entry_ext::*;
mod trust_line_entry;
pub use super::TrustLineEntry;
#[allow(unused_imports)]
pub use trust_line_entry::*;
mod offer_entry_flags;
pub use super::OfferEntryFlags;
pub use super::MASK_OFFERENTRY_FLAGS;
#[allow(unused_imports)]
pub use offer_entry_flags::*;
mod offer_entry_ext;
pub use super::OfferEntryExt;
#[allow(unused_imports)]
pub use offer_entry_ext::*;
mod offer_entry;
pub use super::OfferEntry;
#[allow(unused_imports)]
pub use offer_entry::*;
mod data_entry_ext;
pub use super::DataEntryExt;
#[allow(unused_imports)]
pub use data_entry_ext::*;
mod data_entry;
#[allow(unused_imports)]
pub use data_entry::*;
mod claim_predicate_type;
pub use super::ClaimPredicateType;
#[allow(unused_imports)]
pub use claim_predicate_type::*;
mod claim_predicate;
#[allow(unused_imports)]
pub use claim_predicate::*;
mod claimant_type;
pub use super::ClaimantType;
#[allow(unused_imports)]
pub use claimant_type::*;
mod claimant_v0;
#[allow(unused_imports)]
pub use claimant_v0::*;
mod claimant;
#[allow(unused_imports)]
pub use claimant::*;
mod claimable_balance_flags;
pub use super::ClaimableBalanceFlags;
pub use super::MASK_CLAIMABLE_BALANCE_FLAGS;
#[allow(unused_imports)]
pub use claimable_balance_flags::*;
mod claimable_balance_entry_extension_v1_ext;
pub use super::ClaimableBalanceEntryExtensionV1Ext;
#[allow(unused_imports)]
pub use claimable_balance_entry_extension_v1_ext::*;
mod claimable_balance_entry_extension_v1;
pub use super::ClaimableBalanceEntryExtensionV1;
#[allow(unused_imports)]
pub use claimable_balance_entry_extension_v1::*;
mod claimable_balance_entry_ext;
pub use super::ClaimableBalanceEntryExt;
#[allow(unused_imports)]
pub use claimable_balance_entry_ext::*;
mod claimable_balance_entry;
#[allow(unused_imports)]
pub use claimable_balance_entry::*;
mod liquidity_pool_constant_product_parameters;
pub use super::LiquidityPoolConstantProductParameters;
#[allow(unused_imports)]
pub use liquidity_pool_constant_product_parameters::*;
mod liquidity_pool_entry_constant_product;
pub use super::LiquidityPoolEntryConstantProduct;
#[allow(unused_imports)]
pub use liquidity_pool_entry_constant_product::*;
mod liquidity_pool_entry_body;
pub use super::LiquidityPoolEntryBody;
#[allow(unused_imports)]
pub use liquidity_pool_entry_body::*;
mod liquidity_pool_entry;
pub use super::LiquidityPoolEntry;
#[allow(unused_imports)]
pub use liquidity_pool_entry::*;
mod contract_data_durability;
pub use super::ContractDataDurability;
#[allow(unused_imports)]
pub use contract_data_durability::*;
mod contract_data_entry;
#[allow(unused_imports)]
pub use contract_data_entry::*;
mod contract_code_cost_inputs;
pub use super::ContractCodeCostInputs;
#[allow(unused_imports)]
pub use contract_code_cost_inputs::*;
mod contract_code_entry_v1;
pub use super::ContractCodeEntryV1;
#[allow(unused_imports)]
pub use contract_code_entry_v1::*;
mod contract_code_entry_ext;
pub use super::ContractCodeEntryExt;
#[allow(unused_imports)]
pub use contract_code_entry_ext::*;
mod contract_code_entry;
#[allow(unused_imports)]
pub use contract_code_entry::*;
mod ttl_entry;
pub use super::TtlEntry;
#[allow(unused_imports)]
pub use ttl_entry::*;
mod ledger_entry_extension_v1_ext;
pub use super::LedgerEntryExtensionV1Ext;
#[allow(unused_imports)]
pub use ledger_entry_extension_v1_ext::*;
mod ledger_entry_extension_v1;
pub use super::LedgerEntryExtensionV1;
#[allow(unused_imports)]
pub use ledger_entry_extension_v1::*;
mod ledger_entry_data;
#[allow(unused_imports)]
pub use ledger_entry_data::*;
mod ledger_entry_ext;
pub use super::LedgerEntryExt;
#[allow(unused_imports)]
pub use ledger_entry_ext::*;
mod ledger_entry;
#[allow(unused_imports)]
pub use ledger_entry::*;
mod ledger_key_account;
pub use super::LedgerKeyAccount;
#[allow(unused_imports)]
pub use ledger_key_account::*;
mod ledger_key_trust_line;
pub use super::LedgerKeyTrustLine;
#[allow(unused_imports)]
pub use ledger_key_trust_line::*;
mod ledger_key_offer;
pub use super::LedgerKeyOffer;
#[allow(unused_imports)]
pub use ledger_key_offer::*;
mod ledger_key_data;
#[allow(unused_imports)]
pub use ledger_key_data::*;
mod ledger_key_claimable_balance;
pub use super::LedgerKeyClaimableBalance;
#[allow(unused_imports)]
pub use ledger_key_claimable_balance::*;
mod ledger_key_liquidity_pool;
pub use super::LedgerKeyLiquidityPool;
#[allow(unused_imports)]
pub use ledger_key_liquidity_pool::*;
mod ledger_key_contract_data;
#[allow(unused_imports)]
pub use ledger_key_contract_data::*;
mod ledger_key_contract_code;
pub use super::LedgerKeyContractCode;
#[allow(unused_imports)]
pub use ledger_key_contract_code::*;
mod ledger_key_config_setting;
pub use super::LedgerKeyConfigSetting;
#[allow(unused_imports)]
pub use ledger_key_config_setting::*;
mod ledger_key_ttl;
pub use super::LedgerKeyTtl;
#[allow(unused_imports)]
pub use ledger_key_ttl::*;
mod ledger_key;
#[allow(unused_imports)]
pub use ledger_key::*;
mod envelope_type;
pub use super::EnvelopeType;
#[allow(unused_imports)]
pub use envelope_type::*;
mod bucket_list_type;
pub use super::BucketListType;
#[allow(unused_imports)]
pub use bucket_list_type::*;
mod bucket_entry_type;
pub use super::BucketEntryType;
#[allow(unused_imports)]
pub use bucket_entry_type::*;
mod hot_archive_bucket_entry_type;
pub use super::HotArchiveBucketEntryType;
#[allow(unused_imports)]
pub use hot_archive_bucket_entry_type::*;
mod bucket_metadata_ext;
pub use super::BucketMetadataExt;
#[allow(unused_imports)]
pub use bucket_metadata_ext::*;
mod bucket_metadata;
pub use super::BucketMetadata;
#[allow(unused_imports)]
pub use bucket_metadata::*;
mod bucket_entry;
#[allow(unused_imports)]
pub use bucket_entry::*;
mod hot_archive_bucket_entry;
#[allow(unused_imports)]
pub use hot_archive_bucket_entry::*;
mod upgrade_type;
#[allow(unused_imports)]
pub use upgrade_type::*;
mod stellar_value_type;
pub use super::StellarValueType;
#[allow(unused_imports)]
pub use stellar_value_type::*;
mod ledger_close_value_signature;
#[allow(unused_imports)]
pub use ledger_close_value_signature::*;
mod stellar_value_proposed_value;
#[allow(unused_imports)]
pub use stellar_value_proposed_value::*;
mod stellar_value_ext;
#[allow(unused_imports)]
pub use stellar_value_ext::*;
mod stellar_value;
pub use super::MASK_LEDGER_HEADER_FLAGS;
#[allow(unused_imports)]
pub use stellar_value::*;
mod ledger_header_flags;
pub use super::LedgerHeaderFlags;
#[allow(unused_imports)]
pub use ledger_header_flags::*;
mod ledger_header_extension_v1_ext;
pub use super::LedgerHeaderExtensionV1Ext;
#[allow(unused_imports)]
pub use ledger_header_extension_v1_ext::*;
mod ledger_header_extension_v1;
pub use super::LedgerHeaderExtensionV1;
#[allow(unused_imports)]
pub use ledger_header_extension_v1::*;
mod ledger_header_ext;
pub use super::LedgerHeaderExt;
#[allow(unused_imports)]
pub use ledger_header_ext::*;
mod ledger_header;
#[allow(unused_imports)]
pub use ledger_header::*;
mod ledger_upgrade_type;
pub use super::LedgerUpgradeType;
#[allow(unused_imports)]
pub use ledger_upgrade_type::*;
mod config_upgrade_set_key;
pub use super::ConfigUpgradeSetKey;
#[allow(unused_imports)]
pub use config_upgrade_set_key::*;
mod ledger_upgrade;
pub use super::LedgerUpgrade;
#[allow(unused_imports)]
pub use ledger_upgrade::*;
mod config_upgrade_set;
#[allow(unused_imports)]
pub use config_upgrade_set::*;
mod tx_set_component_type;
pub use super::TxSetComponentType;
#[allow(unused_imports)]
pub use tx_set_component_type::*;
mod dependent_tx_cluster;
#[allow(unused_imports)]
pub use dependent_tx_cluster::*;
mod parallel_tx_execution_stage;
#[allow(unused_imports)]
pub use parallel_tx_execution_stage::*;
mod parallel_txs_component;
#[allow(unused_imports)]
pub use parallel_txs_component::*;
mod tx_set_component_txs_maybe_discounted_fee;
#[allow(unused_imports)]
pub use tx_set_component_txs_maybe_discounted_fee::*;
mod tx_set_component;
#[allow(unused_imports)]
pub use tx_set_component::*;
mod transaction_phase;
#[allow(unused_imports)]
pub use transaction_phase::*;
mod transaction_set;
#[allow(unused_imports)]
pub use transaction_set::*;
mod transaction_set_v1;
#[allow(unused_imports)]
pub use transaction_set_v1::*;
mod generalized_transaction_set;
#[allow(unused_imports)]
pub use generalized_transaction_set::*;
mod transaction_result_pair;
#[allow(unused_imports)]
pub use transaction_result_pair::*;
mod transaction_result_set;
#[allow(unused_imports)]
pub use transaction_result_set::*;
mod transaction_history_entry_ext;
#[allow(unused_imports)]
pub use transaction_history_entry_ext::*;
mod transaction_history_entry;
#[allow(unused_imports)]
pub use transaction_history_entry::*;
mod transaction_history_result_entry_ext;
pub use super::TransactionHistoryResultEntryExt;
#[allow(unused_imports)]
pub use transaction_history_result_entry_ext::*;
mod transaction_history_result_entry;
#[allow(unused_imports)]
pub use transaction_history_result_entry::*;
mod ledger_header_history_entry_ext;
pub use super::LedgerHeaderHistoryEntryExt;
#[allow(unused_imports)]
pub use ledger_header_history_entry_ext::*;
mod ledger_header_history_entry;
#[allow(unused_imports)]
pub use ledger_header_history_entry::*;
mod ledger_scp_messages;
#[allow(unused_imports)]
pub use ledger_scp_messages::*;
mod scp_history_entry_v0;
#[allow(unused_imports)]
pub use scp_history_entry_v0::*;
mod scp_history_entry;
#[allow(unused_imports)]
pub use scp_history_entry::*;
mod ledger_entry_change_type;
pub use super::LedgerEntryChangeType;
#[allow(unused_imports)]
pub use ledger_entry_change_type::*;
mod ledger_entry_change;
#[allow(unused_imports)]
pub use ledger_entry_change::*;
mod ledger_entry_changes;
#[allow(unused_imports)]
pub use ledger_entry_changes::*;
mod operation_meta;
#[allow(unused_imports)]
pub use operation_meta::*;
mod transaction_meta_v1;
#[allow(unused_imports)]
pub use transaction_meta_v1::*;
mod transaction_meta_v2;
#[allow(unused_imports)]
pub use transaction_meta_v2::*;
mod contract_event_type;
pub use super::ContractEventType;
#[allow(unused_imports)]
pub use contract_event_type::*;
mod contract_event_v0;
#[allow(unused_imports)]
pub use contract_event_v0::*;
mod contract_event_body;
#[allow(unused_imports)]
pub use contract_event_body::*;
mod contract_event;
#[allow(unused_imports)]
pub use contract_event::*;
mod diagnostic_event;
#[allow(unused_imports)]
pub use diagnostic_event::*;
mod soroban_transaction_meta_ext_v1;
pub use super::SorobanTransactionMetaExtV1;
#[allow(unused_imports)]
pub use soroban_transaction_meta_ext_v1::*;
mod soroban_transaction_meta_ext;
pub use super::SorobanTransactionMetaExt;
#[allow(unused_imports)]
pub use soroban_transaction_meta_ext::*;
mod soroban_transaction_meta;
#[allow(unused_imports)]
pub use soroban_transaction_meta::*;
mod transaction_meta_v3;
#[allow(unused_imports)]
pub use transaction_meta_v3::*;
mod operation_meta_v2;
#[allow(unused_imports)]
pub use operation_meta_v2::*;
mod soroban_transaction_meta_v2;
#[allow(unused_imports)]
pub use soroban_transaction_meta_v2::*;
mod transaction_event_stage;
pub use super::TransactionEventStage;
#[allow(unused_imports)]
pub use transaction_event_stage::*;
mod transaction_event;
#[allow(unused_imports)]
pub use transaction_event::*;
mod transaction_meta_v4;
#[allow(unused_imports)]
pub use transaction_meta_v4::*;
mod invoke_host_function_success_pre_image;
#[allow(unused_imports)]
pub use invoke_host_function_success_pre_image::*;
mod transaction_meta;
#[allow(unused_imports)]
pub use transaction_meta::*;
mod transaction_result_meta;
#[allow(unused_imports)]
pub use transaction_result_meta::*;
mod transaction_result_meta_v1;
#[allow(unused_imports)]
pub use transaction_result_meta_v1::*;
mod upgrade_entry_meta;
#[allow(unused_imports)]
pub use upgrade_entry_meta::*;
mod ledger_close_meta_v0;
#[allow(unused_imports)]
pub use ledger_close_meta_v0::*;
mod ledger_close_meta_ext_v1;
pub use super::LedgerCloseMetaExtV1;
#[allow(unused_imports)]
pub use ledger_close_meta_ext_v1::*;
mod ledger_close_meta_ext;
pub use super::LedgerCloseMetaExt;
#[allow(unused_imports)]
pub use ledger_close_meta_ext::*;
mod ledger_close_meta_v1;
#[allow(unused_imports)]
pub use ledger_close_meta_v1::*;
mod ledger_close_meta_v2;
#[allow(unused_imports)]
pub use ledger_close_meta_v2::*;
mod ledger_close_meta;
#[allow(unused_imports)]
pub use ledger_close_meta::*;
mod error_code;
pub use super::ErrorCode;
#[allow(unused_imports)]
pub use error_code::*;
mod s_error;
#[allow(unused_imports)]
pub use s_error::*;
mod send_more;
pub use super::SendMore;
#[allow(unused_imports)]
pub use send_more::*;
mod send_more_extended;
pub use super::SendMoreExtended;
#[allow(unused_imports)]
pub use send_more_extended::*;
mod auth_cert;
#[allow(unused_imports)]
pub use auth_cert::*;
mod hello;
pub use super::AUTH_MSG_FLAG_FLOW_CONTROL_BYTES_REQUESTED;
#[allow(unused_imports)]
pub use hello::*;
mod auth;
pub use super::Auth;
#[allow(unused_imports)]
pub use auth::*;
mod ip_addr_type;
pub use super::IpAddrType;
#[allow(unused_imports)]
pub use ip_addr_type::*;
mod peer_address_ip;
pub use super::PeerAddressIp;
#[allow(unused_imports)]
pub use peer_address_ip::*;
mod peer_address;
pub use super::PeerAddress;
#[allow(unused_imports)]
pub use peer_address::*;
mod message_type;
pub use super::MessageType;
#[allow(unused_imports)]
pub use message_type::*;
mod dont_have;
pub use super::DontHave;
#[allow(unused_imports)]
pub use dont_have::*;
mod survey_message_command_type;
pub use super::SurveyMessageCommandType;
#[allow(unused_imports)]
pub use survey_message_command_type::*;
mod survey_message_response_type;
pub use super::SurveyMessageResponseType;
#[allow(unused_imports)]
pub use survey_message_response_type::*;
mod time_sliced_survey_start_collecting_message;
pub use super::TimeSlicedSurveyStartCollectingMessage;
#[allow(unused_imports)]
pub use time_sliced_survey_start_collecting_message::*;
mod signed_time_sliced_survey_start_collecting_message;
#[allow(unused_imports)]
pub use signed_time_sliced_survey_start_collecting_message::*;
mod time_sliced_survey_stop_collecting_message;
pub use super::TimeSlicedSurveyStopCollectingMessage;
#[allow(unused_imports)]
pub use time_sliced_survey_stop_collecting_message::*;
mod signed_time_sliced_survey_stop_collecting_message;
#[allow(unused_imports)]
pub use signed_time_sliced_survey_stop_collecting_message::*;
mod survey_request_message;
pub use super::SurveyRequestMessage;
#[allow(unused_imports)]
pub use survey_request_message::*;
mod time_sliced_survey_request_message;
pub use super::TimeSlicedSurveyRequestMessage;
#[allow(unused_imports)]
pub use time_sliced_survey_request_message::*;
mod signed_time_sliced_survey_request_message;
#[allow(unused_imports)]
pub use signed_time_sliced_survey_request_message::*;
mod encrypted_body;
#[allow(unused_imports)]
pub use encrypted_body::*;
mod survey_response_message;
#[allow(unused_imports)]
pub use survey_response_message::*;
mod time_sliced_survey_response_message;
#[allow(unused_imports)]
pub use time_sliced_survey_response_message::*;
mod signed_time_sliced_survey_response_message;
#[allow(unused_imports)]
pub use signed_time_sliced_survey_response_message::*;
mod peer_stats;
#[allow(unused_imports)]
pub use peer_stats::*;
mod time_sliced_node_data;
pub use super::TimeSlicedNodeData;
#[allow(unused_imports)]
pub use time_sliced_node_data::*;
mod time_sliced_peer_data;
#[allow(unused_imports)]
pub use time_sliced_peer_data::*;
mod time_sliced_peer_data_list;
#[allow(unused_imports)]
pub use time_sliced_peer_data_list::*;
mod topology_response_body_v2;
#[allow(unused_imports)]
pub use topology_response_body_v2::*;
mod survey_response_body;
pub use super::TX_ADVERT_VECTOR_MAX_SIZE;
#[allow(unused_imports)]
pub use survey_response_body::*;
mod tx_advert_vector;
#[allow(unused_imports)]
pub use tx_advert_vector::*;
mod flood_advert;
pub use super::TX_DEMAND_VECTOR_MAX_SIZE;
#[allow(unused_imports)]
pub use flood_advert::*;
mod tx_demand_vector;
#[allow(unused_imports)]
pub use tx_demand_vector::*;
mod flood_demand;
#[allow(unused_imports)]
pub use flood_demand::*;
mod stellar_message;
#[allow(unused_imports)]
pub use stellar_message::*;
mod authenticated_message_v0;
#[allow(unused_imports)]
pub use authenticated_message_v0::*;
mod authenticated_message;
pub use super::MAX_OPS_PER_TX;
#[allow(unused_imports)]
pub use authenticated_message::*;
mod liquidity_pool_parameters;
pub use super::LiquidityPoolParameters;
#[allow(unused_imports)]
pub use liquidity_pool_parameters::*;
mod muxed_account_med25519;
pub use super::MuxedAccountMed25519;
#[allow(unused_imports)]
pub use muxed_account_med25519::*;
mod muxed_account;
pub use super::MuxedAccount;
#[allow(unused_imports)]
pub use muxed_account::*;
mod decorated_signature;
#[allow(unused_imports)]
pub use decorated_signature::*;
mod operation_type;
pub use super::OperationType;
#[allow(unused_imports)]
pub use operation_type::*;
mod create_account_op;
pub use super::CreateAccountOp;
#[allow(unused_imports)]
pub use create_account_op::*;
mod payment_op;
pub use super::PaymentOp;
#[allow(unused_imports)]
pub use payment_op::*;
mod path_payment_strict_receive_op;
#[allow(unused_imports)]
pub use path_payment_strict_receive_op::*;
mod path_payment_strict_send_op;
#[allow(unused_imports)]
pub use path_payment_strict_send_op::*;
mod manage_sell_offer_op;
pub use super::ManageSellOfferOp;
#[allow(unused_imports)]
pub use manage_sell_offer_op::*;
mod manage_buy_offer_op;
pub use super::ManageBuyOfferOp;
#[allow(unused_imports)]
pub use manage_buy_offer_op::*;
mod create_passive_sell_offer_op;
pub use super::CreatePassiveSellOfferOp;
#[allow(unused_imports)]
pub use create_passive_sell_offer_op::*;
mod set_options_op;
#[allow(unused_imports)]
pub use set_options_op::*;
mod change_trust_asset;
pub use super::ChangeTrustAsset;
#[allow(unused_imports)]
pub use change_trust_asset::*;
mod change_trust_op;
pub use super::ChangeTrustOp;
#[allow(unused_imports)]
pub use change_trust_op::*;
mod allow_trust_op;
pub use super::AllowTrustOp;
#[allow(unused_imports)]
pub use allow_trust_op::*;
mod manage_data_op;
#[allow(unused_imports)]
pub use manage_data_op::*;
mod bump_sequence_op;
pub use super::BumpSequenceOp;
#[allow(unused_imports)]
pub use bump_sequence_op::*;
mod create_claimable_balance_op;
#[allow(unused_imports)]
pub use create_claimable_balance_op::*;
mod claim_claimable_balance_op;
pub use super::ClaimClaimableBalanceOp;
#[allow(unused_imports)]
pub use claim_claimable_balance_op::*;
mod begin_sponsoring_future_reserves_op;
pub use super::BeginSponsoringFutureReservesOp;
#[allow(unused_imports)]
pub use begin_sponsoring_future_reserves_op::*;
mod revoke_sponsorship_type;
pub use super::RevokeSponsorshipType;
#[allow(unused_imports)]
pub use revoke_sponsorship_type::*;
mod revoke_sponsorship_op_signer;
#[allow(unused_imports)]
pub use revoke_sponsorship_op_signer::*;
mod revoke_sponsorship_op;
#[allow(unused_imports)]
pub use revoke_sponsorship_op::*;
mod clawback_op;
pub use super::ClawbackOp;
#[allow(unused_imports)]
pub use clawback_op::*;
mod clawback_claimable_balance_op;
pub use super::ClawbackClaimableBalanceOp;
#[allow(unused_imports)]
pub use clawback_claimable_balance_op::*;
mod set_trust_line_flags_op;
pub use super::SetTrustLineFlagsOp;
pub use super::LIQUIDITY_POOL_FEE_V18;
#[allow(unused_imports)]
pub use set_trust_line_flags_op::*;
mod liquidity_pool_deposit_op;
pub use super::LiquidityPoolDepositOp;
#[allow(unused_imports)]
pub use liquidity_pool_deposit_op::*;
mod liquidity_pool_withdraw_op;
pub use super::LiquidityPoolWithdrawOp;
#[allow(unused_imports)]
pub use liquidity_pool_withdraw_op::*;
mod host_function_type;
pub use super::HostFunctionType;
#[allow(unused_imports)]
pub use host_function_type::*;
mod contract_id_preimage_type;
pub use super::ContractIdPreimageType;
#[allow(unused_imports)]
pub use contract_id_preimage_type::*;
mod contract_id_preimage_from_address;
pub use super::ContractIdPreimageFromAddress;
#[allow(unused_imports)]
pub use contract_id_preimage_from_address::*;
mod contract_id_preimage;
pub use super::ContractIdPreimage;
#[allow(unused_imports)]
pub use contract_id_preimage::*;
mod create_contract_args;
#[allow(unused_imports)]
pub use create_contract_args::*;
mod create_contract_args_v2;
#[allow(unused_imports)]
pub use create_contract_args_v2::*;
mod invoke_contract_args;
#[allow(unused_imports)]
pub use invoke_contract_args::*;
mod host_function;
#[allow(unused_imports)]
pub use host_function::*;
mod soroban_authorized_function_type;
pub use super::SorobanAuthorizedFunctionType;
#[allow(unused_imports)]
pub use soroban_authorized_function_type::*;
mod soroban_authorized_function;
#[allow(unused_imports)]
pub use soroban_authorized_function::*;
mod soroban_authorized_invocation;
#[allow(unused_imports)]
pub use soroban_authorized_invocation::*;
mod soroban_address_credentials;
#[allow(unused_imports)]
pub use soroban_address_credentials::*;
mod soroban_delegate_signature;
#[allow(unused_imports)]
pub use soroban_delegate_signature::*;
mod soroban_address_credentials_with_delegates;
#[allow(unused_imports)]
pub use soroban_address_credentials_with_delegates::*;
mod soroban_credentials_type;
pub use super::SorobanCredentialsType;
#[allow(unused_imports)]
pub use soroban_credentials_type::*;
mod soroban_credentials;
#[allow(unused_imports)]
pub use soroban_credentials::*;
mod soroban_authorization_entry;
#[allow(unused_imports)]
pub use soroban_authorization_entry::*;
mod soroban_authorization_entries;
#[allow(unused_imports)]
pub use soroban_authorization_entries::*;
mod invoke_host_function_op;
#[allow(unused_imports)]
pub use invoke_host_function_op::*;
mod extend_footprint_ttl_op;
pub use super::ExtendFootprintTtlOp;
#[allow(unused_imports)]
pub use extend_footprint_ttl_op::*;
mod restore_footprint_op;
pub use super::RestoreFootprintOp;
#[allow(unused_imports)]
pub use restore_footprint_op::*;
mod operation_body;
#[allow(unused_imports)]
pub use operation_body::*;
mod operation;
#[allow(unused_imports)]
pub use operation::*;
mod hash_id_preimage_operation_id;
pub use super::HashIdPreimageOperationId;
#[allow(unused_imports)]
pub use hash_id_preimage_operation_id::*;
mod hash_id_preimage_revoke_id;
pub use super::HashIdPreimageRevokeId;
#[allow(unused_imports)]
pub use hash_id_preimage_revoke_id::*;
mod hash_id_preimage_contract_id;
pub use super::HashIdPreimageContractId;
#[allow(unused_imports)]
pub use hash_id_preimage_contract_id::*;
mod hash_id_preimage_soroban_authorization;
#[allow(unused_imports)]
pub use hash_id_preimage_soroban_authorization::*;
mod hash_id_preimage_soroban_authorization_with_address;
#[allow(unused_imports)]
pub use hash_id_preimage_soroban_authorization_with_address::*;
mod hash_id_preimage;
#[allow(unused_imports)]
pub use hash_id_preimage::*;
mod memo_type;
pub use super::MemoType;
#[allow(unused_imports)]
pub use memo_type::*;
mod memo;
#[allow(unused_imports)]
pub use memo::*;
mod time_bounds;
pub use super::TimeBounds;
#[allow(unused_imports)]
pub use time_bounds::*;
mod ledger_bounds;
pub use super::LedgerBounds;
#[allow(unused_imports)]
pub use ledger_bounds::*;
mod preconditions_v2;
#[allow(unused_imports)]
pub use preconditions_v2::*;
mod precondition_type;
pub use super::PreconditionType;
#[allow(unused_imports)]
pub use precondition_type::*;
mod preconditions;
#[allow(unused_imports)]
pub use preconditions::*;
mod ledger_footprint;
#[allow(unused_imports)]
pub use ledger_footprint::*;
mod soroban_resources;
#[allow(unused_imports)]
pub use soroban_resources::*;
mod soroban_resources_ext_v0;
#[allow(unused_imports)]
pub use soroban_resources_ext_v0::*;
mod soroban_transaction_data_ext;
#[allow(unused_imports)]
pub use soroban_transaction_data_ext::*;
mod soroban_transaction_data;
#[allow(unused_imports)]
pub use soroban_transaction_data::*;
mod transaction_v0_ext;
pub use super::TransactionV0Ext;
#[allow(unused_imports)]
pub use transaction_v0_ext::*;
mod transaction_v0;
#[allow(unused_imports)]
pub use transaction_v0::*;
mod transaction_v0_envelope;
#[allow(unused_imports)]
pub use transaction_v0_envelope::*;
mod transaction_ext;
#[allow(unused_imports)]
pub use transaction_ext::*;
mod transaction;
#[allow(unused_imports)]
pub use transaction::*;
mod transaction_v1_envelope;
#[allow(unused_imports)]
pub use transaction_v1_envelope::*;
mod fee_bump_transaction_inner_tx;
#[allow(unused_imports)]
pub use fee_bump_transaction_inner_tx::*;
mod fee_bump_transaction_ext;
pub use super::FeeBumpTransactionExt;
#[allow(unused_imports)]
pub use fee_bump_transaction_ext::*;
mod fee_bump_transaction;
#[allow(unused_imports)]
pub use fee_bump_transaction::*;
mod fee_bump_transaction_envelope;
#[allow(unused_imports)]
pub use fee_bump_transaction_envelope::*;
mod transaction_envelope;
#[allow(unused_imports)]
pub use transaction_envelope::*;
mod transaction_signature_payload_tagged_transaction;
#[allow(unused_imports)]
pub use transaction_signature_payload_tagged_transaction::*;
mod transaction_signature_payload;
#[allow(unused_imports)]
pub use transaction_signature_payload::*;
mod claim_atom_type;
pub use super::ClaimAtomType;
#[allow(unused_imports)]
pub use claim_atom_type::*;
mod claim_offer_atom_v0;
pub use super::ClaimOfferAtomV0;
#[allow(unused_imports)]
pub use claim_offer_atom_v0::*;
mod claim_offer_atom;
pub use super::ClaimOfferAtom;
#[allow(unused_imports)]
pub use claim_offer_atom::*;
mod claim_liquidity_atom;
pub use super::ClaimLiquidityAtom;
#[allow(unused_imports)]
pub use claim_liquidity_atom::*;
mod claim_atom;
pub use super::ClaimAtom;
#[allow(unused_imports)]
pub use claim_atom::*;
mod create_account_result_code;
pub use super::CreateAccountResultCode;
#[allow(unused_imports)]
pub use create_account_result_code::*;
mod create_account_result;
pub use super::CreateAccountResult;
#[allow(unused_imports)]
pub use create_account_result::*;
mod payment_result_code;
pub use super::PaymentResultCode;
#[allow(unused_imports)]
pub use payment_result_code::*;
mod payment_result;
pub use super::PaymentResult;
#[allow(unused_imports)]
pub use payment_result::*;
mod path_payment_strict_receive_result_code;
pub use super::PathPaymentStrictReceiveResultCode;
#[allow(unused_imports)]
pub use path_payment_strict_receive_result_code::*;
mod simple_payment_result;
pub use super::SimplePaymentResult;
#[allow(unused_imports)]
pub use simple_payment_result::*;
mod path_payment_strict_receive_result_success;
#[allow(unused_imports)]
pub use path_payment_strict_receive_result_success::*;
mod path_payment_strict_receive_result;
#[allow(unused_imports)]
pub use path_payment_strict_receive_result::*;
mod path_payment_strict_send_result_code;
pub use super::PathPaymentStrictSendResultCode;
#[allow(unused_imports)]
pub use path_payment_strict_send_result_code::*;
mod path_payment_strict_send_result_success;
#[allow(unused_imports)]
pub use path_payment_strict_send_result_success::*;
mod path_payment_strict_send_result;
#[allow(unused_imports)]
pub use path_payment_strict_send_result::*;
mod manage_sell_offer_result_code;
pub use super::ManageSellOfferResultCode;
#[allow(unused_imports)]
pub use manage_sell_offer_result_code::*;
mod manage_offer_effect;
pub use super::ManageOfferEffect;
#[allow(unused_imports)]
pub use manage_offer_effect::*;
mod manage_offer_success_result_offer;
pub use super::ManageOfferSuccessResultOffer;
#[allow(unused_imports)]
pub use manage_offer_success_result_offer::*;
mod manage_offer_success_result;
#[allow(unused_imports)]
pub use manage_offer_success_result::*;
mod manage_sell_offer_result;
#[allow(unused_imports)]
pub use manage_sell_offer_result::*;
mod manage_buy_offer_result_code;
pub use super::ManageBuyOfferResultCode;
#[allow(unused_imports)]
pub use manage_buy_offer_result_code::*;
mod manage_buy_offer_result;
#[allow(unused_imports)]
pub use manage_buy_offer_result::*;
mod set_options_result_code;
pub use super::SetOptionsResultCode;
#[allow(unused_imports)]
pub use set_options_result_code::*;
mod set_options_result;
pub use super::SetOptionsResult;
#[allow(unused_imports)]
pub use set_options_result::*;
mod change_trust_result_code;
pub use super::ChangeTrustResultCode;
#[allow(unused_imports)]
pub use change_trust_result_code::*;
mod change_trust_result;
pub use super::ChangeTrustResult;
#[allow(unused_imports)]
pub use change_trust_result::*;
mod allow_trust_result_code;
pub use super::AllowTrustResultCode;
#[allow(unused_imports)]
pub use allow_trust_result_code::*;
mod allow_trust_result;
pub use super::AllowTrustResult;
#[allow(unused_imports)]
pub use allow_trust_result::*;
mod account_merge_result_code;
pub use super::AccountMergeResultCode;
#[allow(unused_imports)]
pub use account_merge_result_code::*;
mod account_merge_result;
pub use super::AccountMergeResult;
#[allow(unused_imports)]
pub use account_merge_result::*;
mod inflation_result_code;
pub use super::InflationResultCode;
#[allow(unused_imports)]
pub use inflation_result_code::*;
mod inflation_payout;
pub use super::InflationPayout;
#[allow(unused_imports)]
pub use inflation_payout::*;
mod inflation_result;
#[allow(unused_imports)]
pub use inflation_result::*;
mod manage_data_result_code;
pub use super::ManageDataResultCode;
#[allow(unused_imports)]
pub use manage_data_result_code::*;
mod manage_data_result;
pub use super::ManageDataResult;
#[allow(unused_imports)]
pub use manage_data_result::*;
mod bump_sequence_result_code;
pub use super::BumpSequenceResultCode;
#[allow(unused_imports)]
pub use bump_sequence_result_code::*;
mod bump_sequence_result;
pub use super::BumpSequenceResult;
#[allow(unused_imports)]
pub use bump_sequence_result::*;
mod create_claimable_balance_result_code;
pub use super::CreateClaimableBalanceResultCode;
#[allow(unused_imports)]
pub use create_claimable_balance_result_code::*;
mod create_claimable_balance_result;
pub use super::CreateClaimableBalanceResult;
#[allow(unused_imports)]
pub use create_claimable_balance_result::*;
mod claim_claimable_balance_result_code;
pub use super::ClaimClaimableBalanceResultCode;
#[allow(unused_imports)]
pub use claim_claimable_balance_result_code::*;
mod claim_claimable_balance_result;
pub use super::ClaimClaimableBalanceResult;
#[allow(unused_imports)]
pub use claim_claimable_balance_result::*;
mod begin_sponsoring_future_reserves_result_code;
pub use super::BeginSponsoringFutureReservesResultCode;
#[allow(unused_imports)]
pub use begin_sponsoring_future_reserves_result_code::*;
mod begin_sponsoring_future_reserves_result;
pub use super::BeginSponsoringFutureReservesResult;
#[allow(unused_imports)]
pub use begin_sponsoring_future_reserves_result::*;
mod end_sponsoring_future_reserves_result_code;
pub use super::EndSponsoringFutureReservesResultCode;
#[allow(unused_imports)]
pub use end_sponsoring_future_reserves_result_code::*;
mod end_sponsoring_future_reserves_result;
pub use super::EndSponsoringFutureReservesResult;
#[allow(unused_imports)]
pub use end_sponsoring_future_reserves_result::*;
mod revoke_sponsorship_result_code;
pub use super::RevokeSponsorshipResultCode;
#[allow(unused_imports)]
pub use revoke_sponsorship_result_code::*;
mod revoke_sponsorship_result;
pub use super::RevokeSponsorshipResult;
#[allow(unused_imports)]
pub use revoke_sponsorship_result::*;
mod clawback_result_code;
pub use super::ClawbackResultCode;
#[allow(unused_imports)]
pub use clawback_result_code::*;
mod clawback_result;
pub use super::ClawbackResult;
#[allow(unused_imports)]
pub use clawback_result::*;
mod clawback_claimable_balance_result_code;
pub use super::ClawbackClaimableBalanceResultCode;
#[allow(unused_imports)]
pub use clawback_claimable_balance_result_code::*;
mod clawback_claimable_balance_result;
pub use super::ClawbackClaimableBalanceResult;
#[allow(unused_imports)]
pub use clawback_claimable_balance_result::*;
mod set_trust_line_flags_result_code;
pub use super::SetTrustLineFlagsResultCode;
#[allow(unused_imports)]
pub use set_trust_line_flags_result_code::*;
mod set_trust_line_flags_result;
pub use super::SetTrustLineFlagsResult;
#[allow(unused_imports)]
pub use set_trust_line_flags_result::*;
mod liquidity_pool_deposit_result_code;
pub use super::LiquidityPoolDepositResultCode;
#[allow(unused_imports)]
pub use liquidity_pool_deposit_result_code::*;
mod liquidity_pool_deposit_result;
pub use super::LiquidityPoolDepositResult;
#[allow(unused_imports)]
pub use liquidity_pool_deposit_result::*;
mod liquidity_pool_withdraw_result_code;
pub use super::LiquidityPoolWithdrawResultCode;
#[allow(unused_imports)]
pub use liquidity_pool_withdraw_result_code::*;
mod liquidity_pool_withdraw_result;
pub use super::LiquidityPoolWithdrawResult;
#[allow(unused_imports)]
pub use liquidity_pool_withdraw_result::*;
mod invoke_host_function_result_code;
pub use super::InvokeHostFunctionResultCode;
#[allow(unused_imports)]
pub use invoke_host_function_result_code::*;
mod invoke_host_function_result;
pub use super::InvokeHostFunctionResult;
#[allow(unused_imports)]
pub use invoke_host_function_result::*;
mod extend_footprint_ttl_result_code;
pub use super::ExtendFootprintTtlResultCode;
#[allow(unused_imports)]
pub use extend_footprint_ttl_result_code::*;
mod extend_footprint_ttl_result;
pub use super::ExtendFootprintTtlResult;
#[allow(unused_imports)]
pub use extend_footprint_ttl_result::*;
mod restore_footprint_result_code;
pub use super::RestoreFootprintResultCode;
#[allow(unused_imports)]
pub use restore_footprint_result_code::*;
mod restore_footprint_result;
pub use super::RestoreFootprintResult;
#[allow(unused_imports)]
pub use restore_footprint_result::*;
mod operation_result_code;
pub use super::OperationResultCode;
#[allow(unused_imports)]
pub use operation_result_code::*;
mod operation_result_tr;
#[allow(unused_imports)]
pub use operation_result_tr::*;
mod operation_result;
#[allow(unused_imports)]
pub use operation_result::*;
mod transaction_result_code;
pub use super::TransactionResultCode;
#[allow(unused_imports)]
pub use transaction_result_code::*;
mod inner_transaction_result_result;
#[allow(unused_imports)]
pub use inner_transaction_result_result::*;
mod inner_transaction_result_ext;
pub use super::InnerTransactionResultExt;
#[allow(unused_imports)]
pub use inner_transaction_result_ext::*;
mod inner_transaction_result;
#[allow(unused_imports)]
pub use inner_transaction_result::*;
mod inner_transaction_result_pair;
#[allow(unused_imports)]
pub use inner_transaction_result_pair::*;
mod transaction_result_result;
#[allow(unused_imports)]
pub use transaction_result_result::*;
mod transaction_result_ext;
pub use super::TransactionResultExt;
#[allow(unused_imports)]
pub use transaction_result_ext::*;
mod transaction_result;
#[allow(unused_imports)]
pub use transaction_result::*;
mod hash;
pub use super::Hash;
#[allow(unused_imports)]
pub use hash::*;
mod uint256;
pub use super::Int32;
pub use super::Int64;
pub use super::Uint256;
pub use super::Uint32;
pub use super::Uint64;
#[allow(unused_imports)]
pub use uint256::*;
mod time_point;
pub use super::TimePoint;
#[allow(unused_imports)]
pub use time_point::*;
mod duration;
pub use super::Duration;
#[allow(unused_imports)]
pub use duration::*;
mod extension_point;
pub use super::ExtensionPoint;
#[allow(unused_imports)]
pub use extension_point::*;
mod crypto_key_type;
pub use super::CryptoKeyType;
#[allow(unused_imports)]
pub use crypto_key_type::*;
mod public_key_type;
pub use super::PublicKeyType;
#[allow(unused_imports)]
pub use public_key_type::*;
mod signer_key_type;
pub use super::SignerKeyType;
#[allow(unused_imports)]
pub use signer_key_type::*;
mod public_key;
pub use super::PublicKey;
#[allow(unused_imports)]
pub use public_key::*;
mod signer_key_ed25519_signed_payload;
#[allow(unused_imports)]
pub use signer_key_ed25519_signed_payload::*;
mod signer_key;
#[allow(unused_imports)]
pub use signer_key::*;
mod signature;
#[allow(unused_imports)]
pub use signature::*;
mod signature_hint;
pub use super::SignatureHint;
#[allow(unused_imports)]
pub use signature_hint::*;
mod node_id;
pub use super::NodeId;
#[allow(unused_imports)]
pub use node_id::*;
mod account_id;
pub use super::AccountId;
#[allow(unused_imports)]
pub use account_id::*;
mod contract_id;
pub use super::ContractId;
#[allow(unused_imports)]
pub use contract_id::*;
mod curve25519_secret;
pub use super::Curve25519Secret;
#[allow(unused_imports)]
pub use curve25519_secret::*;
mod curve25519_public;
pub use super::Curve25519Public;
#[allow(unused_imports)]
pub use curve25519_public::*;
mod hmac_sha256_key;
pub use super::HmacSha256Key;
#[allow(unused_imports)]
pub use hmac_sha256_key::*;
mod hmac_sha256_mac;
pub use super::HmacSha256Mac;
#[allow(unused_imports)]
pub use hmac_sha256_mac::*;
mod short_hash_seed;
pub use super::ShortHashSeed;
#[allow(unused_imports)]
pub use short_hash_seed::*;
mod binary_fuse_filter_type;
pub use super::BinaryFuseFilterType;
#[allow(unused_imports)]
pub use binary_fuse_filter_type::*;
mod serialized_binary_fuse_filter;
#[allow(unused_imports)]
pub use serialized_binary_fuse_filter::*;
mod pool_id;
pub use super::PoolId;
#[allow(unused_imports)]
pub use pool_id::*;
mod claimable_balance_id_type;
pub use super::ClaimableBalanceIdType;
#[allow(unused_imports)]
pub use claimable_balance_id_type::*;
mod claimable_balance_id;
pub use super::ClaimableBalanceId;
#[allow(unused_imports)]
pub use claimable_balance_id::*;
mod test_next_type;
#[cfg(feature = "test_feature")]
pub use super::TestNextType;
#[allow(unused_imports)]
pub use test_next_type::*;
