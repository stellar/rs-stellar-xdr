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
