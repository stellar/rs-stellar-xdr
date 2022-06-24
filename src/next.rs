// Module  is generated from:
//  xdr/next/Stellar-SCP.x
//  xdr/next/Stellar-contract-spec.x
//  xdr/next/Stellar-contract.x
//  xdr/next/Stellar-ledger-entries.x
//  xdr/next/Stellar-ledger.x
//  xdr/next/Stellar-overlay.x
//  xdr/next/Stellar-transaction.x
//  xdr/next/Stellar-types.x

#![allow(clippy::missing_errors_doc, clippy::unreadable_literal)]

/// `XDR_FILES_SHA256` is a list of pairs of source files and their SHA256 hashes.
pub const XDR_FILES_SHA256: [(&str, &str); 8] = [
    (
        "xdr/next/Stellar-SCP.x",
        "8f32b04d008f8bc33b8843d075e69837231a673691ee41d8b821ca229a6e802a",
    ),
    (
        "xdr/next/Stellar-contract-spec.x",
        "8899796c5813e3d4944305baba80f4bc62b176aabb05da2e936942e4babc5985",
    ),
    (
        "xdr/next/Stellar-contract.x",
        "2e83db4641554e8e99f809c4217984bf2266ca6fd41f79f04c57175b320c2ce5",
    ),
    (
        "xdr/next/Stellar-ledger-entries.x",
        "d1c0b58d2134370a6dfa57ef509dccc5de5d1950bcbdad22ccc6c640046f79f2",
    ),
    (
        "xdr/next/Stellar-ledger.x",
        "1573aea71199d7e5c8defb24633608811bf4f4dbf9c15170c9fc954003a1d2f9",
    ),
    (
        "xdr/next/Stellar-overlay.x",
        "8ecbc36d2a43103499a6416572c7cd7b4fd7638d1a2af515b81b463ce6909c51",
    ),
    (
        "xdr/next/Stellar-transaction.x",
        "2423e16aba4d6dfccd75bc2c061e6c2cfb207af56c97b2599c24023ba4ca3c67",
    ),
    (
        "xdr/next/Stellar-types.x",
        "60b7588e573f5e5518766eb5e6b6ea42f0e53144663cbe557e485cceb6306c85",
    ),
];

use core::{fmt, fmt::Debug, ops::Deref};

#[cfg(feature = "std")]
use core::marker::PhantomData;

// When feature alloc is turned off use static lifetime Box and Vec types.
#[cfg(not(feature = "alloc"))]
mod noalloc {
    pub mod boxed {
        pub type Box<T> = &'static T;
    }
    pub mod vec {
        pub type Vec<T> = &'static [T];
    }
}
#[cfg(not(feature = "alloc"))]
use noalloc::{boxed::Box, vec::Vec};

// When feature std is turned off, but feature alloc is turned on import the
// alloc crate and use its Box and Vec types.
#[cfg(all(not(feature = "std"), feature = "alloc"))]
extern crate alloc;
#[cfg(all(not(feature = "std"), feature = "alloc"))]
use alloc::{
    borrow::ToOwned,
    boxed::Box,
    string::{FromUtf8Error, String},
    vec::Vec,
};
#[cfg(all(feature = "std"))]
use std::string::FromUtf8Error;

// TODO: Add support for read/write xdr fns when std not available.

#[cfg(feature = "std")]
use std::{
    error, io,
    io::{BufRead, BufReader, Cursor, Read, Write},
};

#[derive(Debug)]
pub enum Error {
    Invalid,
    LengthExceedsMax,
    LengthMismatch,
    NonZeroPadding,
    Utf8Error(core::str::Utf8Error),
    #[cfg(feature = "std")]
    Io(io::Error),
}

#[cfg(feature = "std")]
impl error::Error for Error {
    #[must_use]
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Invalid => write!(f, "xdr value invalid"),
            Error::LengthExceedsMax => write!(f, "xdr value max length exceeded"),
            Error::LengthMismatch => write!(f, "xdr value length does not match"),
            Error::NonZeroPadding => write!(f, "xdr padding contains non-zero bytes"),
            Error::Utf8Error(e) => write!(f, "{}", e),
            #[cfg(feature = "std")]
            Error::Io(e) => write!(f, "{}", e),
        }
    }
}

impl From<core::str::Utf8Error> for Error {
    #[must_use]
    fn from(e: core::str::Utf8Error) -> Self {
        Error::Utf8Error(e)
    }
}

#[cfg(feature = "alloc")]
impl From<FromUtf8Error> for Error {
    #[must_use]
    fn from(e: FromUtf8Error) -> Self {
        Error::Utf8Error(e.utf8_error())
    }
}

#[cfg(feature = "std")]
impl From<io::Error> for Error {
    #[must_use]
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}

impl From<Error> for () {
    fn from(_: Error) {}
}

#[allow(dead_code)]
type Result<T> = core::result::Result<T, Error>;

#[cfg(feature = "std")]
pub struct ReadXdrIter<'r, R: Read, S: ReadXdr> {
    reader: BufReader<&'r mut R>,
    _s: PhantomData<S>,
}

#[cfg(feature = "std")]
impl<'r, R: Read, S: ReadXdr> ReadXdrIter<'r, R, S> {
    fn new(r: &'r mut R) -> Self {
        Self {
            reader: BufReader::new(r),
            _s: PhantomData,
        }
    }
}

#[cfg(feature = "std")]
impl<'r, R: Read, S: ReadXdr> Iterator for ReadXdrIter<'r, R, S> {
    type Item = Result<S>;

    // Next reads the internal reader and XDR decodes it into the Self type. If
    // the EOF is reached without reading any new bytes `None` is returned. If
    // EOF is reached after reading some bytes a truncated entry is assumed an
    // an `Error::Io` containing an `UnexpectedEof`. If any other IO error
    // occurs it is returned. Iteration of this iterator stops naturally when
    // `None` is returned, but not when a `Some(Err(...))` is returned. The
    // caller is responsible for checking each Result.
    fn next(&mut self) -> Option<Self::Item> {
        // Try to fill the buffer to see if the EOF has been reached or not.
        // This happens to effectively peek to see if the stream has finished
        // and there are no more items.  It is necessary to do this because the
        // xdr types in this crate heavily use the `std::io::Read::read_exact`
        // method that doesn't distinguish between an EOF at the beginning of a
        // read and an EOF after a partial fill of a read_exact.
        match self.reader.fill_buf() {
            // If the reader has no more data and is unable to fill any new data
            // into its internal buf, then the EOF has been reached.
            Ok([]) => return None,
            // If an error occurs filling the buffer, treat that as an error and stop.
            Err(e) => return Some(Err(Error::Io(e))),
            // If there is data in the buf available for reading, continue.
            Ok([..]) => (),
        };
        // Read the buf into the type.
        match S::read_xdr(&mut self.reader) {
            Ok(s) => Some(Ok(s)),
            Err(e) => Some(Err(e)),
        }
    }
}

pub trait ReadXdr
where
    Self: Sized,
{
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self>;

    #[cfg(feature = "std")]
    fn read_xdr_into(&mut self, r: &mut impl Read) -> Result<()> {
        *self = Self::read_xdr(r)?;
        Ok(())
    }

    #[cfg(feature = "std")]
    fn read_xdr_iter<R: Read>(r: &mut R) -> ReadXdrIter<R, Self> {
        ReadXdrIter::new(r)
    }

    #[cfg(feature = "std")]
    fn from_xdr<B: AsRef<[u8]>>(bytes: B) -> Result<Self> {
        let mut cursor = Cursor::new(bytes.as_ref());
        let t = Self::read_xdr(&mut cursor)?;
        Ok(t)
    }

    #[cfg(feature = "std")]
    fn from_xdr_base64(b64: String) -> Result<Self> {
        let mut b64_reader = Cursor::new(b64);
        let mut dec = base64::read::DecoderReader::new(&mut b64_reader, base64::STANDARD);
        let t = Self::read_xdr(&mut dec)?;
        Ok(t)
    }
}

pub trait WriteXdr {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()>;

    #[cfg(feature = "std")]
    fn to_xdr(&self) -> Result<Vec<u8>> {
        let mut cursor = Cursor::new(vec![]);
        self.write_xdr(&mut cursor)?;
        let bytes = cursor.into_inner();
        Ok(bytes)
    }

    #[cfg(feature = "std")]
    fn to_xdr_base64(&self) -> Result<String> {
        let mut enc = base64::write::EncoderStringWriter::new(base64::STANDARD);
        self.write_xdr(&mut enc)?;
        let b64 = enc.into_inner();
        Ok(b64)
    }
}

/// `Pad_len` returns the number of bytes to pad an XDR value of the given
/// length to make the final serialized size a multiple of 4.
#[cfg(feature = "std")]
fn pad_len(len: usize) -> usize {
    (4 - (len % 4)) % 4
}

impl ReadXdr for i32 {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let mut b = [0u8; 4];
        r.read_exact(&mut b)?;
        let i = i32::from_be_bytes(b);
        Ok(i)
    }
}

impl WriteXdr for i32 {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let b: [u8; 4] = self.to_be_bytes();
        w.write_all(&b)?;
        Ok(())
    }
}

impl ReadXdr for u32 {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let mut b = [0u8; 4];
        r.read_exact(&mut b)?;
        let i = u32::from_be_bytes(b);
        Ok(i)
    }
}

impl WriteXdr for u32 {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let b: [u8; 4] = self.to_be_bytes();
        w.write_all(&b)?;
        Ok(())
    }
}

impl ReadXdr for i64 {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let mut b = [0u8; 8];
        r.read_exact(&mut b)?;
        let i = i64::from_be_bytes(b);
        Ok(i)
    }
}

impl WriteXdr for i64 {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let b: [u8; 8] = self.to_be_bytes();
        w.write_all(&b)?;
        Ok(())
    }
}

impl ReadXdr for u64 {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let mut b = [0u8; 8];
        r.read_exact(&mut b)?;
        let i = u64::from_be_bytes(b);
        Ok(i)
    }
}

impl WriteXdr for u64 {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let b: [u8; 8] = self.to_be_bytes();
        w.write_all(&b)?;
        Ok(())
    }
}

impl ReadXdr for f32 {
    #[cfg(feature = "std")]
    fn read_xdr(_r: &mut impl Read) -> Result<Self> {
        todo!()
    }
}

impl WriteXdr for f32 {
    #[cfg(feature = "std")]
    fn write_xdr(&self, _w: &mut impl Write) -> Result<()> {
        todo!()
    }
}

impl ReadXdr for f64 {
    #[cfg(feature = "std")]
    fn read_xdr(_r: &mut impl Read) -> Result<Self> {
        todo!()
    }
}

impl WriteXdr for f64 {
    #[cfg(feature = "std")]
    fn write_xdr(&self, _w: &mut impl Write) -> Result<()> {
        todo!()
    }
}

impl ReadXdr for bool {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let i = u32::read_xdr(r)?;
        let b = i == 1;
        Ok(b)
    }
}

impl WriteXdr for bool {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: u32 = if *self { 1 } else { 0 };
        i.write_xdr(w)?;
        Ok(())
    }
}

impl<T: ReadXdr> ReadXdr for Option<T> {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let i = u32::read_xdr(r)?;
        match i {
            0 => Ok(None),
            1 => {
                let t = T::read_xdr(r)?;
                Ok(Some(t))
            }
            _ => Err(Error::Invalid),
        }
    }
}

impl<T: WriteXdr> WriteXdr for Option<T> {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        if let Some(t) = self {
            1u32.write_xdr(w)?;
            t.write_xdr(w)?;
        } else {
            0u32.write_xdr(w)?;
        }
        Ok(())
    }
}

impl<T: ReadXdr> ReadXdr for Box<T> {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let t = T::read_xdr(r)?;
        Ok(Box::new(t))
    }
}

impl<T: WriteXdr> WriteXdr for Box<T> {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        T::write_xdr(self, w)?;
        Ok(())
    }
}

impl ReadXdr for () {
    #[cfg(feature = "std")]
    fn read_xdr(_r: &mut impl Read) -> Result<Self> {
        Ok(())
    }
}

impl WriteXdr for () {
    #[cfg(feature = "std")]
    fn write_xdr(&self, _w: &mut impl Write) -> Result<()> {
        Ok(())
    }
}

impl<const N: usize> ReadXdr for [u8; N] {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let mut arr = [0u8; N];
        r.read_exact(&mut arr)?;

        let pad = &mut [0u8; 3][..pad_len(N)];
        r.read_exact(pad)?;
        if pad.iter().any(|b| *b != 0) {
            return Err(Error::NonZeroPadding);
        }

        Ok(arr)
    }
}

impl<const N: usize> WriteXdr for [u8; N] {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        w.write_all(self)?;
        w.write_all(&[0u8; 3][..pad_len(N)])?;
        Ok(())
    }
}

impl<T: ReadXdr, const N: usize> ReadXdr for [T; N] {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let mut vec = Vec::with_capacity(N);
        for _ in 0..N {
            let t = T::read_xdr(r)?;
            vec.push(t);
        }
        let arr: [T; N] = vec.try_into().unwrap_or_else(|_: Vec<T>| unreachable!());
        Ok(arr)
    }
}

impl<T: WriteXdr, const N: usize> WriteXdr for [T; N] {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        for t in self {
            t.write_xdr(w)?;
        }
        Ok(())
    }
}

#[cfg(feature = "alloc")]
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct VecM<T, const MAX: u32 = { u32::MAX }>(Vec<T>);

#[cfg(not(feature = "alloc"))]
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct VecM<T, const MAX: u32 = { u32::MAX }>(Vec<T>)
where
    T: 'static;

impl<T, const MAX: u32> Deref for VecM<T, MAX> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, const MAX: u32> VecM<T, MAX> {
    pub const MAX_LEN: usize = { MAX as usize };

    #[must_use]
    #[allow(clippy::unused_self)]
    pub fn max_len(&self) -> usize {
        Self::MAX_LEN
    }

    #[must_use]
    pub fn as_vec(&self) -> &Vec<T> {
        self.as_ref()
    }
}

impl<T: Clone, const MAX: u32> VecM<T, MAX> {
    #[must_use]
    #[cfg(feature = "alloc")]
    pub fn to_vec(&self) -> Vec<T> {
        self.into()
    }

    #[must_use]
    pub fn into_vec(self) -> Vec<T> {
        self.into()
    }
}

impl<const MAX: u32> VecM<u8, MAX> {
    #[cfg(feature = "alloc")]
    pub fn to_string(&self) -> Result<String> {
        self.try_into()
    }

    #[cfg(feature = "alloc")]
    pub fn into_string(self) -> Result<String> {
        self.try_into()
    }
}

impl<T, const MAX: u32> TryFrom<Vec<T>> for VecM<T, MAX> {
    type Error = Error;

    fn try_from(v: Vec<T>) -> Result<Self> {
        let len: u32 = v.len().try_into().map_err(|_| Error::LengthExceedsMax)?;
        if len <= MAX {
            Ok(VecM(v))
        } else {
            Err(Error::LengthExceedsMax)
        }
    }
}

impl<T, const MAX: u32> From<VecM<T, MAX>> for Vec<T> {
    #[must_use]
    fn from(v: VecM<T, MAX>) -> Self {
        v.0
    }
}

#[cfg(feature = "alloc")]
impl<T: Clone, const MAX: u32> From<&VecM<T, MAX>> for Vec<T> {
    #[must_use]
    fn from(v: &VecM<T, MAX>) -> Self {
        v.0.clone()
    }
}

impl<T, const MAX: u32> AsRef<Vec<T>> for VecM<T, MAX> {
    #[must_use]
    fn as_ref(&self) -> &Vec<T> {
        &self.0
    }
}

#[cfg(feature = "alloc")]
impl<T: Clone, const MAX: u32> TryFrom<&[T]> for VecM<T, MAX> {
    type Error = Error;

    fn try_from(v: &[T]) -> Result<Self> {
        let len: u32 = v.len().try_into().map_err(|_| Error::LengthExceedsMax)?;
        if len <= MAX {
            Ok(VecM(v.to_vec()))
        } else {
            Err(Error::LengthExceedsMax)
        }
    }
}

impl<T, const MAX: u32> AsRef<[T]> for VecM<T, MAX> {
    #[cfg(feature = "alloc")]
    #[must_use]
    fn as_ref(&self) -> &[T] {
        self.0.as_ref()
    }
    #[cfg(not(feature = "alloc"))]
    #[must_use]
    fn as_ref(&self) -> &[T] {
        self.0
    }
}

#[cfg(feature = "alloc")]
impl<T: Clone, const N: usize, const MAX: u32> TryFrom<[T; N]> for VecM<T, MAX> {
    type Error = Error;

    fn try_from(v: [T; N]) -> Result<Self> {
        let len: u32 = v.len().try_into().map_err(|_| Error::LengthExceedsMax)?;
        if len <= MAX {
            Ok(VecM(v.to_vec()))
        } else {
            Err(Error::LengthExceedsMax)
        }
    }
}

#[cfg(feature = "alloc")]
impl<T: Clone, const N: usize, const MAX: u32> TryFrom<VecM<T, MAX>> for [T; N] {
    type Error = VecM<T, MAX>;

    fn try_from(v: VecM<T, MAX>) -> core::result::Result<Self, Self::Error> {
        let s: [T; N] = v.0.try_into().map_err(|v: Vec<T>| VecM::<T, MAX>(v))?;
        Ok(s)
    }
}

#[cfg(feature = "alloc")]
impl<T: Clone, const N: usize, const MAX: u32> TryFrom<&[T; N]> for VecM<T, MAX> {
    type Error = Error;

    fn try_from(v: &[T; N]) -> Result<Self> {
        let len: u32 = v.len().try_into().map_err(|_| Error::LengthExceedsMax)?;
        if len <= MAX {
            Ok(VecM(v.to_vec()))
        } else {
            Err(Error::LengthExceedsMax)
        }
    }
}

#[cfg(not(feature = "alloc"))]
impl<T: Clone, const N: usize, const MAX: u32> TryFrom<&'static [T; N]> for VecM<T, MAX>
where
    T: 'static,
{
    type Error = Error;

    fn try_from(v: &'static [T; N]) -> Result<Self> {
        let len: u32 = v.len().try_into().map_err(|_| Error::LengthExceedsMax)?;
        if len <= MAX {
            Ok(VecM(v))
        } else {
            Err(Error::LengthExceedsMax)
        }
    }
}

#[cfg(feature = "alloc")]
impl<const MAX: u32> TryFrom<&String> for VecM<u8, MAX> {
    type Error = Error;

    fn try_from(v: &String) -> Result<Self> {
        let len: u32 = v.len().try_into().map_err(|_| Error::LengthExceedsMax)?;
        if len <= MAX {
            Ok(VecM(v.as_bytes().to_vec()))
        } else {
            Err(Error::LengthExceedsMax)
        }
    }
}

#[cfg(feature = "alloc")]
impl<const MAX: u32> TryFrom<String> for VecM<u8, MAX> {
    type Error = Error;

    fn try_from(v: String) -> Result<Self> {
        let len: u32 = v.len().try_into().map_err(|_| Error::LengthExceedsMax)?;
        if len <= MAX {
            Ok(VecM(v.into()))
        } else {
            Err(Error::LengthExceedsMax)
        }
    }
}

#[cfg(feature = "alloc")]
impl<const MAX: u32> TryFrom<VecM<u8, MAX>> for String {
    type Error = Error;

    fn try_from(v: VecM<u8, MAX>) -> Result<Self> {
        Ok(String::from_utf8(v.0)?)
    }
}

#[cfg(feature = "alloc")]
impl<const MAX: u32> TryFrom<&VecM<u8, MAX>> for String {
    type Error = Error;

    fn try_from(v: &VecM<u8, MAX>) -> Result<Self> {
        Ok(core::str::from_utf8(v.as_ref())?.to_owned())
    }
}

#[cfg(feature = "alloc")]
impl<const MAX: u32> TryFrom<&str> for VecM<u8, MAX> {
    type Error = Error;

    fn try_from(v: &str) -> Result<Self> {
        let len: u32 = v.len().try_into().map_err(|_| Error::LengthExceedsMax)?;
        if len <= MAX {
            Ok(VecM(v.into()))
        } else {
            Err(Error::LengthExceedsMax)
        }
    }
}

#[cfg(not(feature = "alloc"))]
impl<const MAX: u32> TryFrom<&'static str> for VecM<u8, MAX> {
    type Error = Error;

    fn try_from(v: &'static str) -> Result<Self> {
        let len: u32 = v.len().try_into().map_err(|_| Error::LengthExceedsMax)?;
        if len <= MAX {
            Ok(VecM(v.as_bytes()))
        } else {
            Err(Error::LengthExceedsMax)
        }
    }
}

impl<'a, const MAX: u32> TryFrom<&'a VecM<u8, MAX>> for &'a str {
    type Error = Error;

    fn try_from(v: &'a VecM<u8, MAX>) -> Result<Self> {
        Ok(core::str::from_utf8(v.as_ref())?)
    }
}

impl<const MAX: u32> ReadXdr for VecM<u8, MAX> {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let len: u32 = u32::read_xdr(r)?;
        if len > MAX {
            return Err(Error::LengthExceedsMax);
        }

        let mut vec = vec![0u8; len as usize];
        r.read_exact(&mut vec)?;

        let pad = &mut [0u8; 3][..pad_len(len as usize)];
        r.read_exact(pad)?;
        if pad.iter().any(|b| *b != 0) {
            return Err(Error::NonZeroPadding);
        }

        Ok(VecM(vec))
    }
}

impl<const MAX: u32> WriteXdr for VecM<u8, MAX> {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let len: u32 = self.len().try_into().map_err(|_| Error::LengthExceedsMax)?;
        len.write_xdr(w)?;

        w.write_all(&self.0)?;

        w.write_all(&[0u8; 3][..pad_len(len as usize)])?;

        Ok(())
    }
}

impl<T: ReadXdr, const MAX: u32> ReadXdr for VecM<T, MAX> {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let len = u32::read_xdr(r)?;
        if len > MAX {
            return Err(Error::LengthExceedsMax);
        }

        let mut vec = Vec::with_capacity(len as usize);
        for _ in 0..len {
            let t = T::read_xdr(r)?;
            vec.push(t);
        }

        Ok(VecM(vec))
    }
}

impl<T: WriteXdr, const MAX: u32> WriteXdr for VecM<T, MAX> {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let len: u32 = self.len().try_into().map_err(|_| Error::LengthExceedsMax)?;
        len.write_xdr(w)?;

        for t in &self.0 {
            t.write_xdr(w)?;
        }

        Ok(())
    }
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use std::io::Cursor;

    use crate::WriteXdr;

    use super::{Error, ReadXdr, VecM};

    #[test]
    pub fn vec_u8_read_without_padding() {
        let mut buf = Cursor::new(vec![0, 0, 0, 4, 2, 2, 2, 2]);
        let v = VecM::<u8, 8>::read_xdr(&mut buf).unwrap();
        assert_eq!(v.to_vec(), vec![2, 2, 2, 2]);
    }

    #[test]
    pub fn vec_u8_read_with_padding() {
        let mut buf = Cursor::new(vec![0, 0, 0, 1, 2, 0, 0, 0]);
        let v = VecM::<u8, 8>::read_xdr(&mut buf).unwrap();
        assert_eq!(v.to_vec(), vec![2]);
    }

    #[test]
    pub fn vec_u8_read_with_insufficient_padding() {
        let mut buf = Cursor::new(vec![0, 0, 0, 1, 2, 0, 0]);
        let res = VecM::<u8, 8>::read_xdr(&mut buf);
        match res {
            Err(Error::Io(_)) => (),
            _ => panic!("expected IO error got {:?}", res),
        }
    }

    #[test]
    pub fn vec_u8_read_with_non_zero_padding() {
        let mut buf = Cursor::new(vec![0, 0, 0, 1, 2, 3, 0, 0]);
        let res = VecM::<u8, 8>::read_xdr(&mut buf);
        match res {
            Err(Error::NonZeroPadding) => (),
            _ => panic!("expected NonZeroPadding got {:?}", res),
        }
    }

    #[test]
    pub fn vec_u8_write_without_padding() {
        let mut buf = vec![];
        let v: VecM<u8, 8> = vec![2, 2, 2, 2].try_into().unwrap();
        v.write_xdr(&mut Cursor::new(&mut buf)).unwrap();
        assert_eq!(buf, vec![0, 0, 0, 4, 2, 2, 2, 2]);
    }

    #[test]
    pub fn vec_u8_write_with_padding() {
        let mut buf = vec![];
        let v: VecM<u8, 8> = vec![2].try_into().unwrap();
        v.write_xdr(&mut Cursor::new(&mut buf)).unwrap();
        assert_eq!(buf, vec![0, 0, 0, 1, 2, 0, 0, 0]);
    }

    #[test]
    pub fn arr_u8_read_without_padding() {
        let mut buf = Cursor::new(vec![2, 2, 2, 2]);
        let v = <[u8; 4]>::read_xdr(&mut buf).unwrap();
        assert_eq!(v, [2, 2, 2, 2]);
    }

    #[test]
    pub fn arr_u8_read_with_padding() {
        let mut buf = Cursor::new(vec![2, 0, 0, 0]);
        let v = <[u8; 1]>::read_xdr(&mut buf).unwrap();
        assert_eq!(v, [2]);
    }

    #[test]
    pub fn arr_u8_read_with_insufficient_padding() {
        let mut buf = Cursor::new(vec![2, 0, 0]);
        let res = <[u8; 1]>::read_xdr(&mut buf);
        match res {
            Err(Error::Io(_)) => (),
            _ => panic!("expected IO error got {:?}", res),
        }
    }

    #[test]
    pub fn arr_u8_read_with_non_zero_padding() {
        let mut buf = Cursor::new(vec![2, 3, 0, 0]);
        let res = <[u8; 1]>::read_xdr(&mut buf);
        match res {
            Err(Error::NonZeroPadding) => (),
            _ => panic!("expected NonZeroPadding got {:?}", res),
        }
    }

    #[test]
    pub fn arr_u8_write_without_padding() {
        let mut buf = vec![];
        [2u8, 2, 2, 2]
            .write_xdr(&mut Cursor::new(&mut buf))
            .unwrap();
        assert_eq!(buf, vec![2, 2, 2, 2]);
    }

    #[test]
    pub fn arr_u8_write_with_padding() {
        let mut buf = vec![];
        [2u8].write_xdr(&mut Cursor::new(&mut buf)).unwrap();
        assert_eq!(buf, vec![2, 0, 0, 0]);
    }
}

// Value is an XDR Typedef defines as:
//
//   typedef opaque Value<>;
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Value(pub VecM<u8>);

impl From<Value> for VecM<u8> {
    #[must_use]
    fn from(x: Value) -> Self {
        x.0
    }
}

impl From<VecM<u8>> for Value {
    #[must_use]
    fn from(x: VecM<u8>) -> Self {
        Value(x)
    }
}

impl AsRef<VecM<u8>> for Value {
    #[must_use]
    fn as_ref(&self) -> &VecM<u8> {
        &self.0
    }
}

impl ReadXdr for Value {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let i = VecM::<u8>::read_xdr(r)?;
        let v = Value(i);
        Ok(v)
    }
}

impl WriteXdr for Value {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.0.write_xdr(w)
    }
}

impl Deref for Value {
    type Target = VecM<u8>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Value> for Vec<u8> {
    #[must_use]
    fn from(x: Value) -> Self {
        x.0 .0
    }
}

impl TryFrom<Vec<u8>> for Value {
    type Error = Error;
    fn try_from(x: Vec<u8>) -> Result<Self> {
        Ok(Value(x.try_into()?))
    }
}

impl AsRef<Vec<u8>> for Value {
    #[must_use]
    fn as_ref(&self) -> &Vec<u8> {
        &self.0 .0
    }
}

impl AsRef<[u8]> for Value {
    #[cfg(feature = "alloc")]
    #[must_use]
    fn as_ref(&self) -> &[u8] {
        &self.0 .0
    }
    #[cfg(not(feature = "alloc"))]
    #[must_use]
    fn as_ref(&self) -> &[u8] {
        self.0 .0
    }
}

// ScpBallot is an XDR Struct defines as:
//
//   struct SCPBallot
//    {
//        uint32 counter; // n
//        Value value;    // x
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScpBallot {
    pub counter: u32,
    pub value: Value,
}

impl ReadXdr for ScpBallot {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            counter: u32::read_xdr(r)?,
            value: Value::read_xdr(r)?,
        })
    }
}

impl WriteXdr for ScpBallot {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.counter.write_xdr(w)?;
        self.value.write_xdr(w)?;
        Ok(())
    }
}

// ScpStatementType is an XDR Enum defines as:
//
//   enum SCPStatementType
//    {
//        SCP_ST_PREPARE = 0,
//        SCP_ST_CONFIRM = 1,
//        SCP_ST_EXTERNALIZE = 2,
//        SCP_ST_NOMINATE = 3
//    };
//
// enum
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum ScpStatementType {
    Prepare = 0,
    Confirm = 1,
    Externalize = 2,
    Nominate = 3,
}

impl ScpStatementType {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Prepare => "Prepare",
            Self::Confirm => "Confirm",
            Self::Externalize => "Externalize",
            Self::Nominate => "Nominate",
        }
    }
}

impl fmt::Display for ScpStatementType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for ScpStatementType {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self> {
        let e = match i {
            0 => ScpStatementType::Prepare,
            1 => ScpStatementType::Confirm,
            2 => ScpStatementType::Externalize,
            3 => ScpStatementType::Nominate,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<ScpStatementType> for i32 {
    #[must_use]
    fn from(e: ScpStatementType) -> Self {
        e as Self
    }
}

impl ReadXdr for ScpStatementType {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXdr for ScpStatementType {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// ScpNomination is an XDR Struct defines as:
//
//   struct SCPNomination
//    {
//        Hash quorumSetHash; // D
//        Value votes<>;      // X
//        Value accepted<>;   // Y
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScpNomination {
    pub quorum_set_hash: Hash,
    pub votes: VecM<Value>,
    pub accepted: VecM<Value>,
}

impl ReadXdr for ScpNomination {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            quorum_set_hash: Hash::read_xdr(r)?,
            votes: VecM::<Value>::read_xdr(r)?,
            accepted: VecM::<Value>::read_xdr(r)?,
        })
    }
}

impl WriteXdr for ScpNomination {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.quorum_set_hash.write_xdr(w)?;
        self.votes.write_xdr(w)?;
        self.accepted.write_xdr(w)?;
        Ok(())
    }
}

// ScpStatementPrepare is an XDR NestedStruct defines as:
//
//   struct
//            {
//                Hash quorumSetHash;       // D
//                SCPBallot ballot;         // b
//                SCPBallot* prepared;      // p
//                SCPBallot* preparedPrime; // p'
//                uint32 nC;                // c.n
//                uint32 nH;                // h.n
//            }
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScpStatementPrepare {
    pub quorum_set_hash: Hash,
    pub ballot: ScpBallot,
    pub prepared: Option<ScpBallot>,
    pub prepared_prime: Option<ScpBallot>,
    pub n_c: u32,
    pub n_h: u32,
}

impl ReadXdr for ScpStatementPrepare {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            quorum_set_hash: Hash::read_xdr(r)?,
            ballot: ScpBallot::read_xdr(r)?,
            prepared: Option::<ScpBallot>::read_xdr(r)?,
            prepared_prime: Option::<ScpBallot>::read_xdr(r)?,
            n_c: u32::read_xdr(r)?,
            n_h: u32::read_xdr(r)?,
        })
    }
}

impl WriteXdr for ScpStatementPrepare {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.quorum_set_hash.write_xdr(w)?;
        self.ballot.write_xdr(w)?;
        self.prepared.write_xdr(w)?;
        self.prepared_prime.write_xdr(w)?;
        self.n_c.write_xdr(w)?;
        self.n_h.write_xdr(w)?;
        Ok(())
    }
}

// ScpStatementConfirm is an XDR NestedStruct defines as:
//
//   struct
//            {
//                SCPBallot ballot;   // b
//                uint32 nPrepared;   // p.n
//                uint32 nCommit;     // c.n
//                uint32 nH;          // h.n
//                Hash quorumSetHash; // D
//            }
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScpStatementConfirm {
    pub ballot: ScpBallot,
    pub n_prepared: u32,
    pub n_commit: u32,
    pub n_h: u32,
    pub quorum_set_hash: Hash,
}

impl ReadXdr for ScpStatementConfirm {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            ballot: ScpBallot::read_xdr(r)?,
            n_prepared: u32::read_xdr(r)?,
            n_commit: u32::read_xdr(r)?,
            n_h: u32::read_xdr(r)?,
            quorum_set_hash: Hash::read_xdr(r)?,
        })
    }
}

impl WriteXdr for ScpStatementConfirm {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.ballot.write_xdr(w)?;
        self.n_prepared.write_xdr(w)?;
        self.n_commit.write_xdr(w)?;
        self.n_h.write_xdr(w)?;
        self.quorum_set_hash.write_xdr(w)?;
        Ok(())
    }
}

// ScpStatementExternalize is an XDR NestedStruct defines as:
//
//   struct
//            {
//                SCPBallot commit;         // c
//                uint32 nH;                // h.n
//                Hash commitQuorumSetHash; // D used before EXTERNALIZE
//            }
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScpStatementExternalize {
    pub commit: ScpBallot,
    pub n_h: u32,
    pub commit_quorum_set_hash: Hash,
}

impl ReadXdr for ScpStatementExternalize {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            commit: ScpBallot::read_xdr(r)?,
            n_h: u32::read_xdr(r)?,
            commit_quorum_set_hash: Hash::read_xdr(r)?,
        })
    }
}

impl WriteXdr for ScpStatementExternalize {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.commit.write_xdr(w)?;
        self.n_h.write_xdr(w)?;
        self.commit_quorum_set_hash.write_xdr(w)?;
        Ok(())
    }
}

// ScpStatementPledges is an XDR NestedUnion defines as:
//
//   union switch (SCPStatementType type)
//        {
//        case SCP_ST_PREPARE:
//            struct
//            {
//                Hash quorumSetHash;       // D
//                SCPBallot ballot;         // b
//                SCPBallot* prepared;      // p
//                SCPBallot* preparedPrime; // p'
//                uint32 nC;                // c.n
//                uint32 nH;                // h.n
//            } prepare;
//        case SCP_ST_CONFIRM:
//            struct
//            {
//                SCPBallot ballot;   // b
//                uint32 nPrepared;   // p.n
//                uint32 nCommit;     // c.n
//                uint32 nH;          // h.n
//                Hash quorumSetHash; // D
//            } confirm;
//        case SCP_ST_EXTERNALIZE:
//            struct
//            {
//                SCPBallot commit;         // c
//                uint32 nH;                // h.n
//                Hash commitQuorumSetHash; // D used before EXTERNALIZE
//            } externalize;
//        case SCP_ST_NOMINATE:
//            SCPNomination nominate;
//        }
//
// union with discriminant ScpStatementType
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ScpStatementPledges {
    Prepare(ScpStatementPrepare),
    Confirm(ScpStatementConfirm),
    Externalize(ScpStatementExternalize),
    Nominate(ScpNomination),
}

impl ScpStatementPledges {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Prepare(_) => "Prepare",
            Self::Confirm(_) => "Confirm",
            Self::Externalize(_) => "Externalize",
            Self::Nominate(_) => "Nominate",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> ScpStatementType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Prepare(_) => ScpStatementType::Prepare,
            Self::Confirm(_) => ScpStatementType::Confirm,
            Self::Externalize(_) => ScpStatementType::Externalize,
            Self::Nominate(_) => ScpStatementType::Nominate,
        }
    }
}

impl ReadXdr for ScpStatementPledges {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: ScpStatementType = <ScpStatementType as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            ScpStatementType::Prepare => Self::Prepare(ScpStatementPrepare::read_xdr(r)?),
            ScpStatementType::Confirm => Self::Confirm(ScpStatementConfirm::read_xdr(r)?),
            ScpStatementType::Externalize => {
                Self::Externalize(ScpStatementExternalize::read_xdr(r)?)
            }
            ScpStatementType::Nominate => Self::Nominate(ScpNomination::read_xdr(r)?),
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for ScpStatementPledges {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Prepare(v) => v.write_xdr(w)?,
            Self::Confirm(v) => v.write_xdr(w)?,
            Self::Externalize(v) => v.write_xdr(w)?,
            Self::Nominate(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// ScpStatement is an XDR Struct defines as:
//
//   struct SCPStatement
//    {
//        NodeID nodeID;    // v
//        uint64 slotIndex; // i
//
//        union switch (SCPStatementType type)
//        {
//        case SCP_ST_PREPARE:
//            struct
//            {
//                Hash quorumSetHash;       // D
//                SCPBallot ballot;         // b
//                SCPBallot* prepared;      // p
//                SCPBallot* preparedPrime; // p'
//                uint32 nC;                // c.n
//                uint32 nH;                // h.n
//            } prepare;
//        case SCP_ST_CONFIRM:
//            struct
//            {
//                SCPBallot ballot;   // b
//                uint32 nPrepared;   // p.n
//                uint32 nCommit;     // c.n
//                uint32 nH;          // h.n
//                Hash quorumSetHash; // D
//            } confirm;
//        case SCP_ST_EXTERNALIZE:
//            struct
//            {
//                SCPBallot commit;         // c
//                uint32 nH;                // h.n
//                Hash commitQuorumSetHash; // D used before EXTERNALIZE
//            } externalize;
//        case SCP_ST_NOMINATE:
//            SCPNomination nominate;
//        }
//        pledges;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScpStatement {
    pub node_id: NodeId,
    pub slot_index: u64,
    pub pledges: ScpStatementPledges,
}

impl ReadXdr for ScpStatement {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            node_id: NodeId::read_xdr(r)?,
            slot_index: u64::read_xdr(r)?,
            pledges: ScpStatementPledges::read_xdr(r)?,
        })
    }
}

impl WriteXdr for ScpStatement {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.node_id.write_xdr(w)?;
        self.slot_index.write_xdr(w)?;
        self.pledges.write_xdr(w)?;
        Ok(())
    }
}

// ScpEnvelope is an XDR Struct defines as:
//
//   struct SCPEnvelope
//    {
//        SCPStatement statement;
//        Signature signature;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScpEnvelope {
    pub statement: ScpStatement,
    pub signature: Signature,
}

impl ReadXdr for ScpEnvelope {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            statement: ScpStatement::read_xdr(r)?,
            signature: Signature::read_xdr(r)?,
        })
    }
}

impl WriteXdr for ScpEnvelope {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.statement.write_xdr(w)?;
        self.signature.write_xdr(w)?;
        Ok(())
    }
}

// ScpQuorumSet is an XDR Struct defines as:
//
//   struct SCPQuorumSet
//    {
//        uint32 threshold;
//        NodeID validators<>;
//        SCPQuorumSet innerSets<>;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScpQuorumSet {
    pub threshold: u32,
    pub validators: VecM<NodeId>,
    pub inner_sets: VecM<ScpQuorumSet>,
}

impl ReadXdr for ScpQuorumSet {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            threshold: u32::read_xdr(r)?,
            validators: VecM::<NodeId>::read_xdr(r)?,
            inner_sets: VecM::<ScpQuorumSet>::read_xdr(r)?,
        })
    }
}

impl WriteXdr for ScpQuorumSet {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.threshold.write_xdr(w)?;
        self.validators.write_xdr(w)?;
        self.inner_sets.write_xdr(w)?;
        Ok(())
    }
}

// AccountId is an XDR Typedef defines as:
//
//   typedef PublicKey AccountID;
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct AccountId(pub PublicKey);

impl From<AccountId> for PublicKey {
    #[must_use]
    fn from(x: AccountId) -> Self {
        x.0
    }
}

impl From<PublicKey> for AccountId {
    #[must_use]
    fn from(x: PublicKey) -> Self {
        AccountId(x)
    }
}

impl AsRef<PublicKey> for AccountId {
    #[must_use]
    fn as_ref(&self) -> &PublicKey {
        &self.0
    }
}

impl ReadXdr for AccountId {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let i = PublicKey::read_xdr(r)?;
        let v = AccountId(i);
        Ok(v)
    }
}

impl WriteXdr for AccountId {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.0.write_xdr(w)
    }
}

// Thresholds is an XDR Typedef defines as:
//
//   typedef opaque Thresholds[4];
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Thresholds(pub [u8; 4]);

impl From<Thresholds> for [u8; 4] {
    #[must_use]
    fn from(x: Thresholds) -> Self {
        x.0
    }
}

impl From<[u8; 4]> for Thresholds {
    #[must_use]
    fn from(x: [u8; 4]) -> Self {
        Thresholds(x)
    }
}

impl AsRef<[u8; 4]> for Thresholds {
    #[must_use]
    fn as_ref(&self) -> &[u8; 4] {
        &self.0
    }
}

impl ReadXdr for Thresholds {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let i = <[u8; 4]>::read_xdr(r)?;
        let v = Thresholds(i);
        Ok(v)
    }
}

impl WriteXdr for Thresholds {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.0.write_xdr(w)
    }
}

// String32 is an XDR Typedef defines as:
//
//   typedef string string32<32>;
//
pub type String32 = VecM<u8, 32>;

// String64 is an XDR Typedef defines as:
//
//   typedef string string64<64>;
//
pub type String64 = VecM<u8, 64>;

// SequenceNumber is an XDR Typedef defines as:
//
//   typedef int64 SequenceNumber;
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SequenceNumber(pub i64);

impl From<SequenceNumber> for i64 {
    #[must_use]
    fn from(x: SequenceNumber) -> Self {
        x.0
    }
}

impl From<i64> for SequenceNumber {
    #[must_use]
    fn from(x: i64) -> Self {
        SequenceNumber(x)
    }
}

impl AsRef<i64> for SequenceNumber {
    #[must_use]
    fn as_ref(&self) -> &i64 {
        &self.0
    }
}

impl ReadXdr for SequenceNumber {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let i = i64::read_xdr(r)?;
        let v = SequenceNumber(i);
        Ok(v)
    }
}

impl WriteXdr for SequenceNumber {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.0.write_xdr(w)
    }
}

// TimePoint is an XDR Typedef defines as:
//
//   typedef uint64 TimePoint;
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TimePoint(pub u64);

impl From<TimePoint> for u64 {
    #[must_use]
    fn from(x: TimePoint) -> Self {
        x.0
    }
}

impl From<u64> for TimePoint {
    #[must_use]
    fn from(x: u64) -> Self {
        TimePoint(x)
    }
}

impl AsRef<u64> for TimePoint {
    #[must_use]
    fn as_ref(&self) -> &u64 {
        &self.0
    }
}

impl ReadXdr for TimePoint {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let i = u64::read_xdr(r)?;
        let v = TimePoint(i);
        Ok(v)
    }
}

impl WriteXdr for TimePoint {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.0.write_xdr(w)
    }
}

// Duration is an XDR Typedef defines as:
//
//   typedef uint64 Duration;
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Duration(pub u64);

impl From<Duration> for u64 {
    #[must_use]
    fn from(x: Duration) -> Self {
        x.0
    }
}

impl From<u64> for Duration {
    #[must_use]
    fn from(x: u64) -> Self {
        Duration(x)
    }
}

impl AsRef<u64> for Duration {
    #[must_use]
    fn as_ref(&self) -> &u64 {
        &self.0
    }
}

impl ReadXdr for Duration {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let i = u64::read_xdr(r)?;
        let v = Duration(i);
        Ok(v)
    }
}

impl WriteXdr for Duration {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.0.write_xdr(w)
    }
}

// DataValue is an XDR Typedef defines as:
//
//   typedef opaque DataValue<64>;
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct DataValue(pub VecM<u8, 64>);

impl From<DataValue> for VecM<u8, 64> {
    #[must_use]
    fn from(x: DataValue) -> Self {
        x.0
    }
}

impl From<VecM<u8, 64>> for DataValue {
    #[must_use]
    fn from(x: VecM<u8, 64>) -> Self {
        DataValue(x)
    }
}

impl AsRef<VecM<u8, 64>> for DataValue {
    #[must_use]
    fn as_ref(&self) -> &VecM<u8, 64> {
        &self.0
    }
}

impl ReadXdr for DataValue {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let i = VecM::<u8, 64>::read_xdr(r)?;
        let v = DataValue(i);
        Ok(v)
    }
}

impl WriteXdr for DataValue {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.0.write_xdr(w)
    }
}

impl Deref for DataValue {
    type Target = VecM<u8, 64>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<DataValue> for Vec<u8> {
    #[must_use]
    fn from(x: DataValue) -> Self {
        x.0 .0
    }
}

impl TryFrom<Vec<u8>> for DataValue {
    type Error = Error;
    fn try_from(x: Vec<u8>) -> Result<Self> {
        Ok(DataValue(x.try_into()?))
    }
}

impl AsRef<Vec<u8>> for DataValue {
    #[must_use]
    fn as_ref(&self) -> &Vec<u8> {
        &self.0 .0
    }
}

impl AsRef<[u8]> for DataValue {
    #[cfg(feature = "alloc")]
    #[must_use]
    fn as_ref(&self) -> &[u8] {
        &self.0 .0
    }
    #[cfg(not(feature = "alloc"))]
    #[must_use]
    fn as_ref(&self) -> &[u8] {
        self.0 .0
    }
}

// PoolId is an XDR Typedef defines as:
//
//   typedef Hash PoolID;
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct PoolId(pub Hash);

impl From<PoolId> for Hash {
    #[must_use]
    fn from(x: PoolId) -> Self {
        x.0
    }
}

impl From<Hash> for PoolId {
    #[must_use]
    fn from(x: Hash) -> Self {
        PoolId(x)
    }
}

impl AsRef<Hash> for PoolId {
    #[must_use]
    fn as_ref(&self) -> &Hash {
        &self.0
    }
}

impl ReadXdr for PoolId {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let i = Hash::read_xdr(r)?;
        let v = PoolId(i);
        Ok(v)
    }
}

impl WriteXdr for PoolId {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.0.write_xdr(w)
    }
}

// AssetCode4 is an XDR Typedef defines as:
//
//   typedef opaque AssetCode4[4];
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct AssetCode4(pub [u8; 4]);

impl From<AssetCode4> for [u8; 4] {
    #[must_use]
    fn from(x: AssetCode4) -> Self {
        x.0
    }
}

impl From<[u8; 4]> for AssetCode4 {
    #[must_use]
    fn from(x: [u8; 4]) -> Self {
        AssetCode4(x)
    }
}

impl AsRef<[u8; 4]> for AssetCode4 {
    #[must_use]
    fn as_ref(&self) -> &[u8; 4] {
        &self.0
    }
}

impl ReadXdr for AssetCode4 {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let i = <[u8; 4]>::read_xdr(r)?;
        let v = AssetCode4(i);
        Ok(v)
    }
}

impl WriteXdr for AssetCode4 {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.0.write_xdr(w)
    }
}

// AssetCode12 is an XDR Typedef defines as:
//
//   typedef opaque AssetCode12[12];
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct AssetCode12(pub [u8; 12]);

impl From<AssetCode12> for [u8; 12] {
    #[must_use]
    fn from(x: AssetCode12) -> Self {
        x.0
    }
}

impl From<[u8; 12]> for AssetCode12 {
    #[must_use]
    fn from(x: [u8; 12]) -> Self {
        AssetCode12(x)
    }
}

impl AsRef<[u8; 12]> for AssetCode12 {
    #[must_use]
    fn as_ref(&self) -> &[u8; 12] {
        &self.0
    }
}

impl ReadXdr for AssetCode12 {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let i = <[u8; 12]>::read_xdr(r)?;
        let v = AssetCode12(i);
        Ok(v)
    }
}

impl WriteXdr for AssetCode12 {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.0.write_xdr(w)
    }
}

// AssetType is an XDR Enum defines as:
//
//   enum AssetType
//    {
//        ASSET_TYPE_NATIVE = 0,
//        ASSET_TYPE_CREDIT_ALPHANUM4 = 1,
//        ASSET_TYPE_CREDIT_ALPHANUM12 = 2,
//        ASSET_TYPE_POOL_SHARE = 3
//    };
//
// enum
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum AssetType {
    Native = 0,
    CreditAlphanum4 = 1,
    CreditAlphanum12 = 2,
    PoolShare = 3,
}

impl AssetType {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Native => "Native",
            Self::CreditAlphanum4 => "CreditAlphanum4",
            Self::CreditAlphanum12 => "CreditAlphanum12",
            Self::PoolShare => "PoolShare",
        }
    }
}

impl fmt::Display for AssetType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for AssetType {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self> {
        let e = match i {
            0 => AssetType::Native,
            1 => AssetType::CreditAlphanum4,
            2 => AssetType::CreditAlphanum12,
            3 => AssetType::PoolShare,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<AssetType> for i32 {
    #[must_use]
    fn from(e: AssetType) -> Self {
        e as Self
    }
}

impl ReadXdr for AssetType {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXdr for AssetType {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// AssetCode is an XDR Union defines as:
//
//   union AssetCode switch (AssetType type)
//    {
//    case ASSET_TYPE_CREDIT_ALPHANUM4:
//        AssetCode4 assetCode4;
//
//    case ASSET_TYPE_CREDIT_ALPHANUM12:
//        AssetCode12 assetCode12;
//
//        // add other asset types here in the future
//    };
//
// union with discriminant AssetType
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum AssetCode {
    CreditAlphanum4(AssetCode4),
    CreditAlphanum12(AssetCode12),
}

impl AssetCode {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::CreditAlphanum4(_) => "CreditAlphanum4",
            Self::CreditAlphanum12(_) => "CreditAlphanum12",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> AssetType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::CreditAlphanum4(_) => AssetType::CreditAlphanum4,
            Self::CreditAlphanum12(_) => AssetType::CreditAlphanum12,
        }
    }
}

impl ReadXdr for AssetCode {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: AssetType = <AssetType as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            AssetType::CreditAlphanum4 => Self::CreditAlphanum4(AssetCode4::read_xdr(r)?),
            AssetType::CreditAlphanum12 => Self::CreditAlphanum12(AssetCode12::read_xdr(r)?),
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for AssetCode {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::CreditAlphanum4(v) => v.write_xdr(w)?,
            Self::CreditAlphanum12(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// AlphaNum4 is an XDR Struct defines as:
//
//   struct AlphaNum4
//    {
//        AssetCode4 assetCode;
//        AccountID issuer;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct AlphaNum4 {
    pub asset_code: AssetCode4,
    pub issuer: AccountId,
}

impl ReadXdr for AlphaNum4 {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            asset_code: AssetCode4::read_xdr(r)?,
            issuer: AccountId::read_xdr(r)?,
        })
    }
}

impl WriteXdr for AlphaNum4 {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.asset_code.write_xdr(w)?;
        self.issuer.write_xdr(w)?;
        Ok(())
    }
}

// AlphaNum12 is an XDR Struct defines as:
//
//   struct AlphaNum12
//    {
//        AssetCode12 assetCode;
//        AccountID issuer;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct AlphaNum12 {
    pub asset_code: AssetCode12,
    pub issuer: AccountId,
}

impl ReadXdr for AlphaNum12 {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            asset_code: AssetCode12::read_xdr(r)?,
            issuer: AccountId::read_xdr(r)?,
        })
    }
}

impl WriteXdr for AlphaNum12 {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.asset_code.write_xdr(w)?;
        self.issuer.write_xdr(w)?;
        Ok(())
    }
}

// Asset is an XDR Union defines as:
//
//   union Asset switch (AssetType type)
//    {
//    case ASSET_TYPE_NATIVE: // Not credit
//        void;
//
//    case ASSET_TYPE_CREDIT_ALPHANUM4:
//        AlphaNum4 alphaNum4;
//
//    case ASSET_TYPE_CREDIT_ALPHANUM12:
//        AlphaNum12 alphaNum12;
//
//        // add other asset types here in the future
//    };
//
// union with discriminant AssetType
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Asset {
    Native,
    CreditAlphanum4(AlphaNum4),
    CreditAlphanum12(AlphaNum12),
}

impl Asset {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Native => "Native",
            Self::CreditAlphanum4(_) => "CreditAlphanum4",
            Self::CreditAlphanum12(_) => "CreditAlphanum12",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> AssetType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Native => AssetType::Native,
            Self::CreditAlphanum4(_) => AssetType::CreditAlphanum4,
            Self::CreditAlphanum12(_) => AssetType::CreditAlphanum12,
        }
    }
}

impl ReadXdr for Asset {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: AssetType = <AssetType as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            AssetType::Native => Self::Native,
            AssetType::CreditAlphanum4 => Self::CreditAlphanum4(AlphaNum4::read_xdr(r)?),
            AssetType::CreditAlphanum12 => Self::CreditAlphanum12(AlphaNum12::read_xdr(r)?),
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for Asset {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Native => ().write_xdr(w)?,
            Self::CreditAlphanum4(v) => v.write_xdr(w)?,
            Self::CreditAlphanum12(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// Price is an XDR Struct defines as:
//
//   struct Price
//    {
//        int32 n; // numerator
//        int32 d; // denominator
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Price {
    pub n: i32,
    pub d: i32,
}

impl ReadXdr for Price {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            n: i32::read_xdr(r)?,
            d: i32::read_xdr(r)?,
        })
    }
}

impl WriteXdr for Price {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.n.write_xdr(w)?;
        self.d.write_xdr(w)?;
        Ok(())
    }
}

// Liabilities is an XDR Struct defines as:
//
//   struct Liabilities
//    {
//        int64 buying;
//        int64 selling;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Liabilities {
    pub buying: i64,
    pub selling: i64,
}

impl ReadXdr for Liabilities {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            buying: i64::read_xdr(r)?,
            selling: i64::read_xdr(r)?,
        })
    }
}

impl WriteXdr for Liabilities {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.buying.write_xdr(w)?;
        self.selling.write_xdr(w)?;
        Ok(())
    }
}

// ThresholdIndexes is an XDR Enum defines as:
//
//   enum ThresholdIndexes
//    {
//        THRESHOLD_MASTER_WEIGHT = 0,
//        THRESHOLD_LOW = 1,
//        THRESHOLD_MED = 2,
//        THRESHOLD_HIGH = 3
//    };
//
// enum
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum ThresholdIndexes {
    MasterWeight = 0,
    Low = 1,
    Med = 2,
    High = 3,
}

impl ThresholdIndexes {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::MasterWeight => "MasterWeight",
            Self::Low => "Low",
            Self::Med => "Med",
            Self::High => "High",
        }
    }
}

impl fmt::Display for ThresholdIndexes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for ThresholdIndexes {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self> {
        let e = match i {
            0 => ThresholdIndexes::MasterWeight,
            1 => ThresholdIndexes::Low,
            2 => ThresholdIndexes::Med,
            3 => ThresholdIndexes::High,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<ThresholdIndexes> for i32 {
    #[must_use]
    fn from(e: ThresholdIndexes) -> Self {
        e as Self
    }
}

impl ReadXdr for ThresholdIndexes {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXdr for ThresholdIndexes {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// LedgerEntryType is an XDR Enum defines as:
//
//   enum LedgerEntryType
//    {
//        ACCOUNT = 0,
//        TRUSTLINE = 1,
//        OFFER = 2,
//        DATA = 3,
//        CLAIMABLE_BALANCE = 4,
//        LIQUIDITY_POOL = 5,
//        CONTRACT_DATA = 6,
//        CONFIG_SETTING = 7
//    };
//
// enum
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum LedgerEntryType {
    Account = 0,
    Trustline = 1,
    Offer = 2,
    Data = 3,
    ClaimableBalance = 4,
    LiquidityPool = 5,
    ContractData = 6,
    ConfigSetting = 7,
}

impl LedgerEntryType {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Account => "Account",
            Self::Trustline => "Trustline",
            Self::Offer => "Offer",
            Self::Data => "Data",
            Self::ClaimableBalance => "ClaimableBalance",
            Self::LiquidityPool => "LiquidityPool",
            Self::ContractData => "ContractData",
            Self::ConfigSetting => "ConfigSetting",
        }
    }
}

impl fmt::Display for LedgerEntryType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for LedgerEntryType {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self> {
        let e = match i {
            0 => LedgerEntryType::Account,
            1 => LedgerEntryType::Trustline,
            2 => LedgerEntryType::Offer,
            3 => LedgerEntryType::Data,
            4 => LedgerEntryType::ClaimableBalance,
            5 => LedgerEntryType::LiquidityPool,
            6 => LedgerEntryType::ContractData,
            7 => LedgerEntryType::ConfigSetting,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<LedgerEntryType> for i32 {
    #[must_use]
    fn from(e: LedgerEntryType) -> Self {
        e as Self
    }
}

impl ReadXdr for LedgerEntryType {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXdr for LedgerEntryType {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// Signer is an XDR Struct defines as:
//
//   struct Signer
//    {
//        SignerKey key;
//        uint32 weight; // really only need 1 byte
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Signer {
    pub key: SignerKey,
    pub weight: u32,
}

impl ReadXdr for Signer {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            key: SignerKey::read_xdr(r)?,
            weight: u32::read_xdr(r)?,
        })
    }
}

impl WriteXdr for Signer {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.key.write_xdr(w)?;
        self.weight.write_xdr(w)?;
        Ok(())
    }
}

// AccountFlags is an XDR Enum defines as:
//
//   enum AccountFlags
//    { // masks for each flag
//
//        // Flags set on issuer accounts
//        // TrustLines are created with authorized set to "false" requiring
//        // the issuer to set it for each TrustLine
//        AUTH_REQUIRED_FLAG = 0x1,
//        // If set, the authorized flag in TrustLines can be cleared
//        // otherwise, authorization cannot be revoked
//        AUTH_REVOCABLE_FLAG = 0x2,
//        // Once set, causes all AUTH_* flags to be read-only
//        AUTH_IMMUTABLE_FLAG = 0x4,
//        // Trustlines are created with clawback enabled set to "true",
//        // and claimable balances created from those trustlines are created
//        // with clawback enabled set to "true"
//        AUTH_CLAWBACK_ENABLED_FLAG = 0x8
//    };
//
// enum
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum AccountFlags {
    RequiredFlag = 1,
    RevocableFlag = 2,
    ImmutableFlag = 4,
    ClawbackEnabledFlag = 8,
}

impl AccountFlags {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::RequiredFlag => "RequiredFlag",
            Self::RevocableFlag => "RevocableFlag",
            Self::ImmutableFlag => "ImmutableFlag",
            Self::ClawbackEnabledFlag => "ClawbackEnabledFlag",
        }
    }
}

impl fmt::Display for AccountFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for AccountFlags {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self> {
        let e = match i {
            1 => AccountFlags::RequiredFlag,
            2 => AccountFlags::RevocableFlag,
            4 => AccountFlags::ImmutableFlag,
            8 => AccountFlags::ClawbackEnabledFlag,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<AccountFlags> for i32 {
    #[must_use]
    fn from(e: AccountFlags) -> Self {
        e as Self
    }
}

impl ReadXdr for AccountFlags {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXdr for AccountFlags {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// MaskAccountFlags is an XDR Const defines as:
//
//   const MASK_ACCOUNT_FLAGS = 0x7;
//
pub const MASK_ACCOUNT_FLAGS: u64 = 0x7;

// MaskAccountFlagsV17 is an XDR Const defines as:
//
//   const MASK_ACCOUNT_FLAGS_V17 = 0xF;
//
pub const MASK_ACCOUNT_FLAGS_V17: u64 = 0xF;

// MaxSigners is an XDR Const defines as:
//
//   const MAX_SIGNERS = 20;
//
pub const MAX_SIGNERS: u64 = 20;

// SponsorshipDescriptor is an XDR Typedef defines as:
//
//   typedef AccountID* SponsorshipDescriptor;
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SponsorshipDescriptor(pub Option<AccountId>);

impl From<SponsorshipDescriptor> for Option<AccountId> {
    #[must_use]
    fn from(x: SponsorshipDescriptor) -> Self {
        x.0
    }
}

impl From<Option<AccountId>> for SponsorshipDescriptor {
    #[must_use]
    fn from(x: Option<AccountId>) -> Self {
        SponsorshipDescriptor(x)
    }
}

impl AsRef<Option<AccountId>> for SponsorshipDescriptor {
    #[must_use]
    fn as_ref(&self) -> &Option<AccountId> {
        &self.0
    }
}

impl ReadXdr for SponsorshipDescriptor {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let i = Option::<AccountId>::read_xdr(r)?;
        let v = SponsorshipDescriptor(i);
        Ok(v)
    }
}

impl WriteXdr for SponsorshipDescriptor {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.0.write_xdr(w)
    }
}

// AccountEntryExtensionV3 is an XDR Struct defines as:
//
//   struct AccountEntryExtensionV3
//    {
//        // We can use this to add more fields, or because it is first, to
//        // change AccountEntryExtensionV3 into a union.
//        ExtensionPoint ext;
//
//        // Ledger number at which `seqNum` took on its present value.
//        uint32 seqLedger;
//
//        // Time at which `seqNum` took on its present value.
//        TimePoint seqTime;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct AccountEntryExtensionV3 {
    pub ext: ExtensionPoint,
    pub seq_ledger: u32,
    pub seq_time: TimePoint,
}

impl ReadXdr for AccountEntryExtensionV3 {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            ext: ExtensionPoint::read_xdr(r)?,
            seq_ledger: u32::read_xdr(r)?,
            seq_time: TimePoint::read_xdr(r)?,
        })
    }
}

impl WriteXdr for AccountEntryExtensionV3 {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.ext.write_xdr(w)?;
        self.seq_ledger.write_xdr(w)?;
        self.seq_time.write_xdr(w)?;
        Ok(())
    }
}

// AccountEntryExtensionV2Ext is an XDR NestedUnion defines as:
//
//   union switch (int v)
//        {
//        case 0:
//            void;
//        case 3:
//            AccountEntryExtensionV3 v3;
//        }
//
// union with discriminant i32
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum AccountEntryExtensionV2Ext {
    V0,
    V3(AccountEntryExtensionV3),
}

impl AccountEntryExtensionV2Ext {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0 => "V0",
            Self::V3(_) => "V3",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> i32 {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0 => 0,
            Self::V3(_) => 3,
        }
    }
}

impl ReadXdr for AccountEntryExtensionV2Ext {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: i32 = <i32 as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            0 => Self::V0,
            3 => Self::V3(AccountEntryExtensionV3::read_xdr(r)?),
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for AccountEntryExtensionV2Ext {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0 => ().write_xdr(w)?,
            Self::V3(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// AccountEntryExtensionV2 is an XDR Struct defines as:
//
//   struct AccountEntryExtensionV2
//    {
//        uint32 numSponsored;
//        uint32 numSponsoring;
//        SponsorshipDescriptor signerSponsoringIDs<MAX_SIGNERS>;
//
//        union switch (int v)
//        {
//        case 0:
//            void;
//        case 3:
//            AccountEntryExtensionV3 v3;
//        }
//        ext;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct AccountEntryExtensionV2 {
    pub num_sponsored: u32,
    pub num_sponsoring: u32,
    pub signer_sponsoring_i_ds: VecM<SponsorshipDescriptor, 20>,
    pub ext: AccountEntryExtensionV2Ext,
}

impl ReadXdr for AccountEntryExtensionV2 {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            num_sponsored: u32::read_xdr(r)?,
            num_sponsoring: u32::read_xdr(r)?,
            signer_sponsoring_i_ds: VecM::<SponsorshipDescriptor, 20>::read_xdr(r)?,
            ext: AccountEntryExtensionV2Ext::read_xdr(r)?,
        })
    }
}

impl WriteXdr for AccountEntryExtensionV2 {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.num_sponsored.write_xdr(w)?;
        self.num_sponsoring.write_xdr(w)?;
        self.signer_sponsoring_i_ds.write_xdr(w)?;
        self.ext.write_xdr(w)?;
        Ok(())
    }
}

// AccountEntryExtensionV1Ext is an XDR NestedUnion defines as:
//
//   union switch (int v)
//        {
//        case 0:
//            void;
//        case 2:
//            AccountEntryExtensionV2 v2;
//        }
//
// union with discriminant i32
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum AccountEntryExtensionV1Ext {
    V0,
    V2(AccountEntryExtensionV2),
}

impl AccountEntryExtensionV1Ext {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0 => "V0",
            Self::V2(_) => "V2",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> i32 {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0 => 0,
            Self::V2(_) => 2,
        }
    }
}

impl ReadXdr for AccountEntryExtensionV1Ext {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: i32 = <i32 as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            0 => Self::V0,
            2 => Self::V2(AccountEntryExtensionV2::read_xdr(r)?),
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for AccountEntryExtensionV1Ext {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0 => ().write_xdr(w)?,
            Self::V2(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// AccountEntryExtensionV1 is an XDR Struct defines as:
//
//   struct AccountEntryExtensionV1
//    {
//        Liabilities liabilities;
//
//        union switch (int v)
//        {
//        case 0:
//            void;
//        case 2:
//            AccountEntryExtensionV2 v2;
//        }
//        ext;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct AccountEntryExtensionV1 {
    pub liabilities: Liabilities,
    pub ext: AccountEntryExtensionV1Ext,
}

impl ReadXdr for AccountEntryExtensionV1 {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            liabilities: Liabilities::read_xdr(r)?,
            ext: AccountEntryExtensionV1Ext::read_xdr(r)?,
        })
    }
}

impl WriteXdr for AccountEntryExtensionV1 {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.liabilities.write_xdr(w)?;
        self.ext.write_xdr(w)?;
        Ok(())
    }
}

// AccountEntryExt is an XDR NestedUnion defines as:
//
//   union switch (int v)
//        {
//        case 0:
//            void;
//        case 1:
//            AccountEntryExtensionV1 v1;
//        }
//
// union with discriminant i32
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum AccountEntryExt {
    V0,
    V1(AccountEntryExtensionV1),
}

impl AccountEntryExt {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0 => "V0",
            Self::V1(_) => "V1",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> i32 {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0 => 0,
            Self::V1(_) => 1,
        }
    }
}

impl ReadXdr for AccountEntryExt {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: i32 = <i32 as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            0 => Self::V0,
            1 => Self::V1(AccountEntryExtensionV1::read_xdr(r)?),
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for AccountEntryExt {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0 => ().write_xdr(w)?,
            Self::V1(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// AccountEntry is an XDR Struct defines as:
//
//   struct AccountEntry
//    {
//        AccountID accountID;      // master public key for this account
//        int64 balance;            // in stroops
//        SequenceNumber seqNum;    // last sequence number used for this account
//        uint32 numSubEntries;     // number of sub-entries this account has
//                                  // drives the reserve
//        AccountID* inflationDest; // Account to vote for during inflation
//        uint32 flags;             // see AccountFlags
//
//        string32 homeDomain; // can be used for reverse federation and memo lookup
//
//        // fields used for signatures
//        // thresholds stores unsigned bytes: [weight of master|low|medium|high]
//        Thresholds thresholds;
//
//        Signer signers<MAX_SIGNERS>; // possible signers for this account
//
//        // reserved for future use
//        union switch (int v)
//        {
//        case 0:
//            void;
//        case 1:
//            AccountEntryExtensionV1 v1;
//        }
//        ext;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct AccountEntry {
    pub account_id: AccountId,
    pub balance: i64,
    pub seq_num: SequenceNumber,
    pub num_sub_entries: u32,
    pub inflation_dest: Option<AccountId>,
    pub flags: u32,
    pub home_domain: VecM<u8, 32>,
    pub thresholds: Thresholds,
    pub signers: VecM<Signer, 20>,
    pub ext: AccountEntryExt,
}

impl ReadXdr for AccountEntry {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            account_id: AccountId::read_xdr(r)?,
            balance: i64::read_xdr(r)?,
            seq_num: SequenceNumber::read_xdr(r)?,
            num_sub_entries: u32::read_xdr(r)?,
            inflation_dest: Option::<AccountId>::read_xdr(r)?,
            flags: u32::read_xdr(r)?,
            home_domain: VecM::<u8, 32>::read_xdr(r)?,
            thresholds: Thresholds::read_xdr(r)?,
            signers: VecM::<Signer, 20>::read_xdr(r)?,
            ext: AccountEntryExt::read_xdr(r)?,
        })
    }
}

impl WriteXdr for AccountEntry {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.account_id.write_xdr(w)?;
        self.balance.write_xdr(w)?;
        self.seq_num.write_xdr(w)?;
        self.num_sub_entries.write_xdr(w)?;
        self.inflation_dest.write_xdr(w)?;
        self.flags.write_xdr(w)?;
        self.home_domain.write_xdr(w)?;
        self.thresholds.write_xdr(w)?;
        self.signers.write_xdr(w)?;
        self.ext.write_xdr(w)?;
        Ok(())
    }
}

// TrustLineFlags is an XDR Enum defines as:
//
//   enum TrustLineFlags
//    {
//        // issuer has authorized account to perform transactions with its credit
//        AUTHORIZED_FLAG = 1,
//        // issuer has authorized account to maintain and reduce liabilities for its
//        // credit
//        AUTHORIZED_TO_MAINTAIN_LIABILITIES_FLAG = 2,
//        // issuer has specified that it may clawback its credit, and that claimable
//        // balances created with its credit may also be clawed back
//        TRUSTLINE_CLAWBACK_ENABLED_FLAG = 4
//    };
//
// enum
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum TrustLineFlags {
    AuthorizedFlag = 1,
    AuthorizedToMaintainLiabilitiesFlag = 2,
    TrustlineClawbackEnabledFlag = 4,
}

impl TrustLineFlags {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::AuthorizedFlag => "AuthorizedFlag",
            Self::AuthorizedToMaintainLiabilitiesFlag => "AuthorizedToMaintainLiabilitiesFlag",
            Self::TrustlineClawbackEnabledFlag => "TrustlineClawbackEnabledFlag",
        }
    }
}

impl fmt::Display for TrustLineFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for TrustLineFlags {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self> {
        let e = match i {
            1 => TrustLineFlags::AuthorizedFlag,
            2 => TrustLineFlags::AuthorizedToMaintainLiabilitiesFlag,
            4 => TrustLineFlags::TrustlineClawbackEnabledFlag,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<TrustLineFlags> for i32 {
    #[must_use]
    fn from(e: TrustLineFlags) -> Self {
        e as Self
    }
}

impl ReadXdr for TrustLineFlags {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXdr for TrustLineFlags {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// MaskTrustlineFlags is an XDR Const defines as:
//
//   const MASK_TRUSTLINE_FLAGS = 1;
//
pub const MASK_TRUSTLINE_FLAGS: u64 = 1;

// MaskTrustlineFlagsV13 is an XDR Const defines as:
//
//   const MASK_TRUSTLINE_FLAGS_V13 = 3;
//
pub const MASK_TRUSTLINE_FLAGS_V13: u64 = 3;

// MaskTrustlineFlagsV17 is an XDR Const defines as:
//
//   const MASK_TRUSTLINE_FLAGS_V17 = 7;
//
pub const MASK_TRUSTLINE_FLAGS_V17: u64 = 7;

// LiquidityPoolType is an XDR Enum defines as:
//
//   enum LiquidityPoolType
//    {
//        LIQUIDITY_POOL_CONSTANT_PRODUCT = 0
//    };
//
// enum
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum LiquidityPoolType {
    LiquidityPoolConstantProduct = 0,
}

impl LiquidityPoolType {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::LiquidityPoolConstantProduct => "LiquidityPoolConstantProduct",
        }
    }
}

impl fmt::Display for LiquidityPoolType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for LiquidityPoolType {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self> {
        let e = match i {
            0 => LiquidityPoolType::LiquidityPoolConstantProduct,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<LiquidityPoolType> for i32 {
    #[must_use]
    fn from(e: LiquidityPoolType) -> Self {
        e as Self
    }
}

impl ReadXdr for LiquidityPoolType {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXdr for LiquidityPoolType {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// TrustLineAsset is an XDR Union defines as:
//
//   union TrustLineAsset switch (AssetType type)
//    {
//    case ASSET_TYPE_NATIVE: // Not credit
//        void;
//
//    case ASSET_TYPE_CREDIT_ALPHANUM4:
//        AlphaNum4 alphaNum4;
//
//    case ASSET_TYPE_CREDIT_ALPHANUM12:
//        AlphaNum12 alphaNum12;
//
//    case ASSET_TYPE_POOL_SHARE:
//        PoolID liquidityPoolID;
//
//        // add other asset types here in the future
//    };
//
// union with discriminant AssetType
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum TrustLineAsset {
    Native,
    CreditAlphanum4(AlphaNum4),
    CreditAlphanum12(AlphaNum12),
    PoolShare(PoolId),
}

impl TrustLineAsset {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Native => "Native",
            Self::CreditAlphanum4(_) => "CreditAlphanum4",
            Self::CreditAlphanum12(_) => "CreditAlphanum12",
            Self::PoolShare(_) => "PoolShare",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> AssetType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Native => AssetType::Native,
            Self::CreditAlphanum4(_) => AssetType::CreditAlphanum4,
            Self::CreditAlphanum12(_) => AssetType::CreditAlphanum12,
            Self::PoolShare(_) => AssetType::PoolShare,
        }
    }
}

impl ReadXdr for TrustLineAsset {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: AssetType = <AssetType as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            AssetType::Native => Self::Native,
            AssetType::CreditAlphanum4 => Self::CreditAlphanum4(AlphaNum4::read_xdr(r)?),
            AssetType::CreditAlphanum12 => Self::CreditAlphanum12(AlphaNum12::read_xdr(r)?),
            AssetType::PoolShare => Self::PoolShare(PoolId::read_xdr(r)?),
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for TrustLineAsset {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Native => ().write_xdr(w)?,
            Self::CreditAlphanum4(v) => v.write_xdr(w)?,
            Self::CreditAlphanum12(v) => v.write_xdr(w)?,
            Self::PoolShare(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// TrustLineEntryExtensionV2Ext is an XDR NestedUnion defines as:
//
//   union switch (int v)
//        {
//        case 0:
//            void;
//        }
//
// union with discriminant i32
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum TrustLineEntryExtensionV2Ext {
    V0,
}

impl TrustLineEntryExtensionV2Ext {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0 => "V0",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> i32 {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0 => 0,
        }
    }
}

impl ReadXdr for TrustLineEntryExtensionV2Ext {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: i32 = <i32 as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            0 => Self::V0,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for TrustLineEntryExtensionV2Ext {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0 => ().write_xdr(w)?,
        };
        Ok(())
    }
}

// TrustLineEntryExtensionV2 is an XDR Struct defines as:
//
//   struct TrustLineEntryExtensionV2
//    {
//        int32 liquidityPoolUseCount;
//
//        union switch (int v)
//        {
//        case 0:
//            void;
//        }
//        ext;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TrustLineEntryExtensionV2 {
    pub liquidity_pool_use_count: i32,
    pub ext: TrustLineEntryExtensionV2Ext,
}

impl ReadXdr for TrustLineEntryExtensionV2 {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            liquidity_pool_use_count: i32::read_xdr(r)?,
            ext: TrustLineEntryExtensionV2Ext::read_xdr(r)?,
        })
    }
}

impl WriteXdr for TrustLineEntryExtensionV2 {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.liquidity_pool_use_count.write_xdr(w)?;
        self.ext.write_xdr(w)?;
        Ok(())
    }
}

// TrustLineEntryV1Ext is an XDR NestedUnion defines as:
//
//   union switch (int v)
//                {
//                case 0:
//                    void;
//                case 2:
//                    TrustLineEntryExtensionV2 v2;
//                }
//
// union with discriminant i32
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum TrustLineEntryV1Ext {
    V0,
    V2(TrustLineEntryExtensionV2),
}

impl TrustLineEntryV1Ext {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0 => "V0",
            Self::V2(_) => "V2",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> i32 {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0 => 0,
            Self::V2(_) => 2,
        }
    }
}

impl ReadXdr for TrustLineEntryV1Ext {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: i32 = <i32 as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            0 => Self::V0,
            2 => Self::V2(TrustLineEntryExtensionV2::read_xdr(r)?),
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for TrustLineEntryV1Ext {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0 => ().write_xdr(w)?,
            Self::V2(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// TrustLineEntryV1 is an XDR NestedStruct defines as:
//
//   struct
//            {
//                Liabilities liabilities;
//
//                union switch (int v)
//                {
//                case 0:
//                    void;
//                case 2:
//                    TrustLineEntryExtensionV2 v2;
//                }
//                ext;
//            }
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TrustLineEntryV1 {
    pub liabilities: Liabilities,
    pub ext: TrustLineEntryV1Ext,
}

impl ReadXdr for TrustLineEntryV1 {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            liabilities: Liabilities::read_xdr(r)?,
            ext: TrustLineEntryV1Ext::read_xdr(r)?,
        })
    }
}

impl WriteXdr for TrustLineEntryV1 {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.liabilities.write_xdr(w)?;
        self.ext.write_xdr(w)?;
        Ok(())
    }
}

// TrustLineEntryExt is an XDR NestedUnion defines as:
//
//   union switch (int v)
//        {
//        case 0:
//            void;
//        case 1:
//            struct
//            {
//                Liabilities liabilities;
//
//                union switch (int v)
//                {
//                case 0:
//                    void;
//                case 2:
//                    TrustLineEntryExtensionV2 v2;
//                }
//                ext;
//            } v1;
//        }
//
// union with discriminant i32
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum TrustLineEntryExt {
    V0,
    V1(TrustLineEntryV1),
}

impl TrustLineEntryExt {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0 => "V0",
            Self::V1(_) => "V1",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> i32 {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0 => 0,
            Self::V1(_) => 1,
        }
    }
}

impl ReadXdr for TrustLineEntryExt {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: i32 = <i32 as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            0 => Self::V0,
            1 => Self::V1(TrustLineEntryV1::read_xdr(r)?),
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for TrustLineEntryExt {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0 => ().write_xdr(w)?,
            Self::V1(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// TrustLineEntry is an XDR Struct defines as:
//
//   struct TrustLineEntry
//    {
//        AccountID accountID;  // account this trustline belongs to
//        TrustLineAsset asset; // type of asset (with issuer)
//        int64 balance;        // how much of this asset the user has.
//                              // Asset defines the unit for this;
//
//        int64 limit;  // balance cannot be above this
//        uint32 flags; // see TrustLineFlags
//
//        // reserved for future use
//        union switch (int v)
//        {
//        case 0:
//            void;
//        case 1:
//            struct
//            {
//                Liabilities liabilities;
//
//                union switch (int v)
//                {
//                case 0:
//                    void;
//                case 2:
//                    TrustLineEntryExtensionV2 v2;
//                }
//                ext;
//            } v1;
//        }
//        ext;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TrustLineEntry {
    pub account_id: AccountId,
    pub asset: TrustLineAsset,
    pub balance: i64,
    pub limit: i64,
    pub flags: u32,
    pub ext: TrustLineEntryExt,
}

impl ReadXdr for TrustLineEntry {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            account_id: AccountId::read_xdr(r)?,
            asset: TrustLineAsset::read_xdr(r)?,
            balance: i64::read_xdr(r)?,
            limit: i64::read_xdr(r)?,
            flags: u32::read_xdr(r)?,
            ext: TrustLineEntryExt::read_xdr(r)?,
        })
    }
}

impl WriteXdr for TrustLineEntry {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.account_id.write_xdr(w)?;
        self.asset.write_xdr(w)?;
        self.balance.write_xdr(w)?;
        self.limit.write_xdr(w)?;
        self.flags.write_xdr(w)?;
        self.ext.write_xdr(w)?;
        Ok(())
    }
}

// OfferEntryFlags is an XDR Enum defines as:
//
//   enum OfferEntryFlags
//    {
//        // an offer with this flag will not act on and take a reverse offer of equal
//        // price
//        PASSIVE_FLAG = 1
//    };
//
// enum
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum OfferEntryFlags {
    PassiveFlag = 1,
}

impl OfferEntryFlags {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::PassiveFlag => "PassiveFlag",
        }
    }
}

impl fmt::Display for OfferEntryFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for OfferEntryFlags {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self> {
        let e = match i {
            1 => OfferEntryFlags::PassiveFlag,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<OfferEntryFlags> for i32 {
    #[must_use]
    fn from(e: OfferEntryFlags) -> Self {
        e as Self
    }
}

impl ReadXdr for OfferEntryFlags {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXdr for OfferEntryFlags {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// MaskOfferentryFlags is an XDR Const defines as:
//
//   const MASK_OFFERENTRY_FLAGS = 1;
//
pub const MASK_OFFERENTRY_FLAGS: u64 = 1;

// OfferEntryExt is an XDR NestedUnion defines as:
//
//   union switch (int v)
//        {
//        case 0:
//            void;
//        }
//
// union with discriminant i32
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum OfferEntryExt {
    V0,
}

impl OfferEntryExt {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0 => "V0",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> i32 {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0 => 0,
        }
    }
}

impl ReadXdr for OfferEntryExt {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: i32 = <i32 as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            0 => Self::V0,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for OfferEntryExt {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0 => ().write_xdr(w)?,
        };
        Ok(())
    }
}

// OfferEntry is an XDR Struct defines as:
//
//   struct OfferEntry
//    {
//        AccountID sellerID;
//        int64 offerID;
//        Asset selling; // A
//        Asset buying;  // B
//        int64 amount;  // amount of A
//
//        /* price for this offer:
//            price of A in terms of B
//            price=AmountB/AmountA=priceNumerator/priceDenominator
//            price is after fees
//        */
//        Price price;
//        uint32 flags; // see OfferEntryFlags
//
//        // reserved for future use
//        union switch (int v)
//        {
//        case 0:
//            void;
//        }
//        ext;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct OfferEntry {
    pub seller_id: AccountId,
    pub offer_id: i64,
    pub selling: Asset,
    pub buying: Asset,
    pub amount: i64,
    pub price: Price,
    pub flags: u32,
    pub ext: OfferEntryExt,
}

impl ReadXdr for OfferEntry {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            seller_id: AccountId::read_xdr(r)?,
            offer_id: i64::read_xdr(r)?,
            selling: Asset::read_xdr(r)?,
            buying: Asset::read_xdr(r)?,
            amount: i64::read_xdr(r)?,
            price: Price::read_xdr(r)?,
            flags: u32::read_xdr(r)?,
            ext: OfferEntryExt::read_xdr(r)?,
        })
    }
}

impl WriteXdr for OfferEntry {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.seller_id.write_xdr(w)?;
        self.offer_id.write_xdr(w)?;
        self.selling.write_xdr(w)?;
        self.buying.write_xdr(w)?;
        self.amount.write_xdr(w)?;
        self.price.write_xdr(w)?;
        self.flags.write_xdr(w)?;
        self.ext.write_xdr(w)?;
        Ok(())
    }
}

// DataEntryExt is an XDR NestedUnion defines as:
//
//   union switch (int v)
//        {
//        case 0:
//            void;
//        }
//
// union with discriminant i32
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum DataEntryExt {
    V0,
}

impl DataEntryExt {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0 => "V0",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> i32 {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0 => 0,
        }
    }
}

impl ReadXdr for DataEntryExt {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: i32 = <i32 as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            0 => Self::V0,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for DataEntryExt {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0 => ().write_xdr(w)?,
        };
        Ok(())
    }
}

// DataEntry is an XDR Struct defines as:
//
//   struct DataEntry
//    {
//        AccountID accountID; // account this data belongs to
//        string64 dataName;
//        DataValue dataValue;
//
//        // reserved for future use
//        union switch (int v)
//        {
//        case 0:
//            void;
//        }
//        ext;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct DataEntry {
    pub account_id: AccountId,
    pub data_name: VecM<u8, 64>,
    pub data_value: DataValue,
    pub ext: DataEntryExt,
}

impl ReadXdr for DataEntry {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            account_id: AccountId::read_xdr(r)?,
            data_name: VecM::<u8, 64>::read_xdr(r)?,
            data_value: DataValue::read_xdr(r)?,
            ext: DataEntryExt::read_xdr(r)?,
        })
    }
}

impl WriteXdr for DataEntry {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.account_id.write_xdr(w)?;
        self.data_name.write_xdr(w)?;
        self.data_value.write_xdr(w)?;
        self.ext.write_xdr(w)?;
        Ok(())
    }
}

// ClaimPredicateType is an XDR Enum defines as:
//
//   enum ClaimPredicateType
//    {
//        CLAIM_PREDICATE_UNCONDITIONAL = 0,
//        CLAIM_PREDICATE_AND = 1,
//        CLAIM_PREDICATE_OR = 2,
//        CLAIM_PREDICATE_NOT = 3,
//        CLAIM_PREDICATE_BEFORE_ABSOLUTE_TIME = 4,
//        CLAIM_PREDICATE_BEFORE_RELATIVE_TIME = 5
//    };
//
// enum
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum ClaimPredicateType {
    Unconditional = 0,
    And = 1,
    Or = 2,
    Not = 3,
    BeforeAbsoluteTime = 4,
    BeforeRelativeTime = 5,
}

impl ClaimPredicateType {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Unconditional => "Unconditional",
            Self::And => "And",
            Self::Or => "Or",
            Self::Not => "Not",
            Self::BeforeAbsoluteTime => "BeforeAbsoluteTime",
            Self::BeforeRelativeTime => "BeforeRelativeTime",
        }
    }
}

impl fmt::Display for ClaimPredicateType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for ClaimPredicateType {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self> {
        let e = match i {
            0 => ClaimPredicateType::Unconditional,
            1 => ClaimPredicateType::And,
            2 => ClaimPredicateType::Or,
            3 => ClaimPredicateType::Not,
            4 => ClaimPredicateType::BeforeAbsoluteTime,
            5 => ClaimPredicateType::BeforeRelativeTime,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<ClaimPredicateType> for i32 {
    #[must_use]
    fn from(e: ClaimPredicateType) -> Self {
        e as Self
    }
}

impl ReadXdr for ClaimPredicateType {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXdr for ClaimPredicateType {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// ClaimPredicate is an XDR Union defines as:
//
//   union ClaimPredicate switch (ClaimPredicateType type)
//    {
//    case CLAIM_PREDICATE_UNCONDITIONAL:
//        void;
//    case CLAIM_PREDICATE_AND:
//        ClaimPredicate andPredicates<2>;
//    case CLAIM_PREDICATE_OR:
//        ClaimPredicate orPredicates<2>;
//    case CLAIM_PREDICATE_NOT:
//        ClaimPredicate* notPredicate;
//    case CLAIM_PREDICATE_BEFORE_ABSOLUTE_TIME:
//        int64 absBefore; // Predicate will be true if closeTime < absBefore
//    case CLAIM_PREDICATE_BEFORE_RELATIVE_TIME:
//        int64 relBefore; // Seconds since closeTime of the ledger in which the
//                         // ClaimableBalanceEntry was created
//    };
//
// union with discriminant ClaimPredicateType
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ClaimPredicate {
    Unconditional,
    And(VecM<ClaimPredicate, 2>),
    Or(VecM<ClaimPredicate, 2>),
    Not(Option<Box<ClaimPredicate>>),
    BeforeAbsoluteTime(i64),
    BeforeRelativeTime(i64),
}

impl ClaimPredicate {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Unconditional => "Unconditional",
            Self::And(_) => "And",
            Self::Or(_) => "Or",
            Self::Not(_) => "Not",
            Self::BeforeAbsoluteTime(_) => "BeforeAbsoluteTime",
            Self::BeforeRelativeTime(_) => "BeforeRelativeTime",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> ClaimPredicateType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Unconditional => ClaimPredicateType::Unconditional,
            Self::And(_) => ClaimPredicateType::And,
            Self::Or(_) => ClaimPredicateType::Or,
            Self::Not(_) => ClaimPredicateType::Not,
            Self::BeforeAbsoluteTime(_) => ClaimPredicateType::BeforeAbsoluteTime,
            Self::BeforeRelativeTime(_) => ClaimPredicateType::BeforeRelativeTime,
        }
    }
}

impl ReadXdr for ClaimPredicate {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: ClaimPredicateType = <ClaimPredicateType as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            ClaimPredicateType::Unconditional => Self::Unconditional,
            ClaimPredicateType::And => Self::And(VecM::<ClaimPredicate, 2>::read_xdr(r)?),
            ClaimPredicateType::Or => Self::Or(VecM::<ClaimPredicate, 2>::read_xdr(r)?),
            ClaimPredicateType::Not => Self::Not(Option::<Box<ClaimPredicate>>::read_xdr(r)?),
            ClaimPredicateType::BeforeAbsoluteTime => Self::BeforeAbsoluteTime(i64::read_xdr(r)?),
            ClaimPredicateType::BeforeRelativeTime => Self::BeforeRelativeTime(i64::read_xdr(r)?),
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for ClaimPredicate {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Unconditional => ().write_xdr(w)?,
            Self::And(v) => v.write_xdr(w)?,
            Self::Or(v) => v.write_xdr(w)?,
            Self::Not(v) => v.write_xdr(w)?,
            Self::BeforeAbsoluteTime(v) => v.write_xdr(w)?,
            Self::BeforeRelativeTime(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// ClaimantType is an XDR Enum defines as:
//
//   enum ClaimantType
//    {
//        CLAIMANT_TYPE_V0 = 0
//    };
//
// enum
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum ClaimantType {
    ClaimantTypeV0 = 0,
}

impl ClaimantType {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::ClaimantTypeV0 => "ClaimantTypeV0",
        }
    }
}

impl fmt::Display for ClaimantType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for ClaimantType {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self> {
        let e = match i {
            0 => ClaimantType::ClaimantTypeV0,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<ClaimantType> for i32 {
    #[must_use]
    fn from(e: ClaimantType) -> Self {
        e as Self
    }
}

impl ReadXdr for ClaimantType {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXdr for ClaimantType {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// ClaimantV0 is an XDR NestedStruct defines as:
//
//   struct
//        {
//            AccountID destination;    // The account that can use this condition
//            ClaimPredicate predicate; // Claimable if predicate is true
//        }
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ClaimantV0 {
    pub destination: AccountId,
    pub predicate: ClaimPredicate,
}

impl ReadXdr for ClaimantV0 {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            destination: AccountId::read_xdr(r)?,
            predicate: ClaimPredicate::read_xdr(r)?,
        })
    }
}

impl WriteXdr for ClaimantV0 {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.destination.write_xdr(w)?;
        self.predicate.write_xdr(w)?;
        Ok(())
    }
}

// Claimant is an XDR Union defines as:
//
//   union Claimant switch (ClaimantType type)
//    {
//    case CLAIMANT_TYPE_V0:
//        struct
//        {
//            AccountID destination;    // The account that can use this condition
//            ClaimPredicate predicate; // Claimable if predicate is true
//        } v0;
//    };
//
// union with discriminant ClaimantType
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Claimant {
    ClaimantTypeV0(ClaimantV0),
}

impl Claimant {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::ClaimantTypeV0(_) => "ClaimantTypeV0",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> ClaimantType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::ClaimantTypeV0(_) => ClaimantType::ClaimantTypeV0,
        }
    }
}

impl ReadXdr for Claimant {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: ClaimantType = <ClaimantType as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            ClaimantType::ClaimantTypeV0 => Self::ClaimantTypeV0(ClaimantV0::read_xdr(r)?),
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for Claimant {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::ClaimantTypeV0(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// ClaimableBalanceIdType is an XDR Enum defines as:
//
//   enum ClaimableBalanceIDType
//    {
//        CLAIMABLE_BALANCE_ID_TYPE_V0 = 0
//    };
//
// enum
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum ClaimableBalanceIdType {
    ClaimableBalanceIdTypeV0 = 0,
}

impl ClaimableBalanceIdType {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::ClaimableBalanceIdTypeV0 => "ClaimableBalanceIdTypeV0",
        }
    }
}

impl fmt::Display for ClaimableBalanceIdType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for ClaimableBalanceIdType {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self> {
        let e = match i {
            0 => ClaimableBalanceIdType::ClaimableBalanceIdTypeV0,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<ClaimableBalanceIdType> for i32 {
    #[must_use]
    fn from(e: ClaimableBalanceIdType) -> Self {
        e as Self
    }
}

impl ReadXdr for ClaimableBalanceIdType {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXdr for ClaimableBalanceIdType {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// ClaimableBalanceId is an XDR Union defines as:
//
//   union ClaimableBalanceID switch (ClaimableBalanceIDType type)
//    {
//    case CLAIMABLE_BALANCE_ID_TYPE_V0:
//        Hash v0;
//    };
//
// union with discriminant ClaimableBalanceIdType
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ClaimableBalanceId {
    ClaimableBalanceIdTypeV0(Hash),
}

impl ClaimableBalanceId {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::ClaimableBalanceIdTypeV0(_) => "ClaimableBalanceIdTypeV0",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> ClaimableBalanceIdType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::ClaimableBalanceIdTypeV0(_) => ClaimableBalanceIdType::ClaimableBalanceIdTypeV0,
        }
    }
}

impl ReadXdr for ClaimableBalanceId {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: ClaimableBalanceIdType = <ClaimableBalanceIdType as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            ClaimableBalanceIdType::ClaimableBalanceIdTypeV0 => {
                Self::ClaimableBalanceIdTypeV0(Hash::read_xdr(r)?)
            }
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for ClaimableBalanceId {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::ClaimableBalanceIdTypeV0(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// ClaimableBalanceFlags is an XDR Enum defines as:
//
//   enum ClaimableBalanceFlags
//    {
//        // If set, the issuer account of the asset held by the claimable balance may
//        // clawback the claimable balance
//        CLAIMABLE_BALANCE_CLAWBACK_ENABLED_FLAG = 0x1
//    };
//
// enum
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum ClaimableBalanceFlags {
    ClaimableBalanceClawbackEnabledFlag = 1,
}

impl ClaimableBalanceFlags {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::ClaimableBalanceClawbackEnabledFlag => "ClaimableBalanceClawbackEnabledFlag",
        }
    }
}

impl fmt::Display for ClaimableBalanceFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for ClaimableBalanceFlags {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self> {
        let e = match i {
            1 => ClaimableBalanceFlags::ClaimableBalanceClawbackEnabledFlag,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<ClaimableBalanceFlags> for i32 {
    #[must_use]
    fn from(e: ClaimableBalanceFlags) -> Self {
        e as Self
    }
}

impl ReadXdr for ClaimableBalanceFlags {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXdr for ClaimableBalanceFlags {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// MaskClaimableBalanceFlags is an XDR Const defines as:
//
//   const MASK_CLAIMABLE_BALANCE_FLAGS = 0x1;
//
pub const MASK_CLAIMABLE_BALANCE_FLAGS: u64 = 0x1;

// ClaimableBalanceEntryExtensionV1Ext is an XDR NestedUnion defines as:
//
//   union switch (int v)
//        {
//        case 0:
//            void;
//        }
//
// union with discriminant i32
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ClaimableBalanceEntryExtensionV1Ext {
    V0,
}

impl ClaimableBalanceEntryExtensionV1Ext {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0 => "V0",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> i32 {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0 => 0,
        }
    }
}

impl ReadXdr for ClaimableBalanceEntryExtensionV1Ext {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: i32 = <i32 as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            0 => Self::V0,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for ClaimableBalanceEntryExtensionV1Ext {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0 => ().write_xdr(w)?,
        };
        Ok(())
    }
}

// ClaimableBalanceEntryExtensionV1 is an XDR Struct defines as:
//
//   struct ClaimableBalanceEntryExtensionV1
//    {
//        union switch (int v)
//        {
//        case 0:
//            void;
//        }
//        ext;
//
//        uint32 flags; // see ClaimableBalanceFlags
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ClaimableBalanceEntryExtensionV1 {
    pub ext: ClaimableBalanceEntryExtensionV1Ext,
    pub flags: u32,
}

impl ReadXdr for ClaimableBalanceEntryExtensionV1 {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            ext: ClaimableBalanceEntryExtensionV1Ext::read_xdr(r)?,
            flags: u32::read_xdr(r)?,
        })
    }
}

impl WriteXdr for ClaimableBalanceEntryExtensionV1 {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.ext.write_xdr(w)?;
        self.flags.write_xdr(w)?;
        Ok(())
    }
}

// ClaimableBalanceEntryExt is an XDR NestedUnion defines as:
//
//   union switch (int v)
//        {
//        case 0:
//            void;
//        case 1:
//            ClaimableBalanceEntryExtensionV1 v1;
//        }
//
// union with discriminant i32
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ClaimableBalanceEntryExt {
    V0,
    V1(ClaimableBalanceEntryExtensionV1),
}

impl ClaimableBalanceEntryExt {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0 => "V0",
            Self::V1(_) => "V1",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> i32 {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0 => 0,
            Self::V1(_) => 1,
        }
    }
}

impl ReadXdr for ClaimableBalanceEntryExt {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: i32 = <i32 as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            0 => Self::V0,
            1 => Self::V1(ClaimableBalanceEntryExtensionV1::read_xdr(r)?),
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for ClaimableBalanceEntryExt {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0 => ().write_xdr(w)?,
            Self::V1(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// ClaimableBalanceEntry is an XDR Struct defines as:
//
//   struct ClaimableBalanceEntry
//    {
//        // Unique identifier for this ClaimableBalanceEntry
//        ClaimableBalanceID balanceID;
//
//        // List of claimants with associated predicate
//        Claimant claimants<10>;
//
//        // Any asset including native
//        Asset asset;
//
//        // Amount of asset
//        int64 amount;
//
//        // reserved for future use
//        union switch (int v)
//        {
//        case 0:
//            void;
//        case 1:
//            ClaimableBalanceEntryExtensionV1 v1;
//        }
//        ext;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ClaimableBalanceEntry {
    pub balance_id: ClaimableBalanceId,
    pub claimants: VecM<Claimant, 10>,
    pub asset: Asset,
    pub amount: i64,
    pub ext: ClaimableBalanceEntryExt,
}

impl ReadXdr for ClaimableBalanceEntry {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            balance_id: ClaimableBalanceId::read_xdr(r)?,
            claimants: VecM::<Claimant, 10>::read_xdr(r)?,
            asset: Asset::read_xdr(r)?,
            amount: i64::read_xdr(r)?,
            ext: ClaimableBalanceEntryExt::read_xdr(r)?,
        })
    }
}

impl WriteXdr for ClaimableBalanceEntry {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.balance_id.write_xdr(w)?;
        self.claimants.write_xdr(w)?;
        self.asset.write_xdr(w)?;
        self.amount.write_xdr(w)?;
        self.ext.write_xdr(w)?;
        Ok(())
    }
}

// LiquidityPoolConstantProductParameters is an XDR Struct defines as:
//
//   struct LiquidityPoolConstantProductParameters
//    {
//        Asset assetA; // assetA < assetB
//        Asset assetB;
//        int32 fee; // Fee is in basis points, so the actual rate is (fee/100)%
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct LiquidityPoolConstantProductParameters {
    pub asset_a: Asset,
    pub asset_b: Asset,
    pub fee: i32,
}

impl ReadXdr for LiquidityPoolConstantProductParameters {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            asset_a: Asset::read_xdr(r)?,
            asset_b: Asset::read_xdr(r)?,
            fee: i32::read_xdr(r)?,
        })
    }
}

impl WriteXdr for LiquidityPoolConstantProductParameters {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.asset_a.write_xdr(w)?;
        self.asset_b.write_xdr(w)?;
        self.fee.write_xdr(w)?;
        Ok(())
    }
}

// LiquidityPoolEntryConstantProduct is an XDR NestedStruct defines as:
//
//   struct
//            {
//                LiquidityPoolConstantProductParameters params;
//
//                int64 reserveA;        // amount of A in the pool
//                int64 reserveB;        // amount of B in the pool
//                int64 totalPoolShares; // total number of pool shares issued
//                int64 poolSharesTrustLineCount; // number of trust lines for the
//                                                // associated pool shares
//            }
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct LiquidityPoolEntryConstantProduct {
    pub params: LiquidityPoolConstantProductParameters,
    pub reserve_a: i64,
    pub reserve_b: i64,
    pub total_pool_shares: i64,
    pub pool_shares_trust_line_count: i64,
}

impl ReadXdr for LiquidityPoolEntryConstantProduct {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            params: LiquidityPoolConstantProductParameters::read_xdr(r)?,
            reserve_a: i64::read_xdr(r)?,
            reserve_b: i64::read_xdr(r)?,
            total_pool_shares: i64::read_xdr(r)?,
            pool_shares_trust_line_count: i64::read_xdr(r)?,
        })
    }
}

impl WriteXdr for LiquidityPoolEntryConstantProduct {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.params.write_xdr(w)?;
        self.reserve_a.write_xdr(w)?;
        self.reserve_b.write_xdr(w)?;
        self.total_pool_shares.write_xdr(w)?;
        self.pool_shares_trust_line_count.write_xdr(w)?;
        Ok(())
    }
}

// LiquidityPoolEntryBody is an XDR NestedUnion defines as:
//
//   union switch (LiquidityPoolType type)
//        {
//        case LIQUIDITY_POOL_CONSTANT_PRODUCT:
//            struct
//            {
//                LiquidityPoolConstantProductParameters params;
//
//                int64 reserveA;        // amount of A in the pool
//                int64 reserveB;        // amount of B in the pool
//                int64 totalPoolShares; // total number of pool shares issued
//                int64 poolSharesTrustLineCount; // number of trust lines for the
//                                                // associated pool shares
//            } constantProduct;
//        }
//
// union with discriminant LiquidityPoolType
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum LiquidityPoolEntryBody {
    LiquidityPoolConstantProduct(LiquidityPoolEntryConstantProduct),
}

impl LiquidityPoolEntryBody {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::LiquidityPoolConstantProduct(_) => "LiquidityPoolConstantProduct",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> LiquidityPoolType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::LiquidityPoolConstantProduct(_) => {
                LiquidityPoolType::LiquidityPoolConstantProduct
            }
        }
    }
}

impl ReadXdr for LiquidityPoolEntryBody {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: LiquidityPoolType = <LiquidityPoolType as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            LiquidityPoolType::LiquidityPoolConstantProduct => {
                Self::LiquidityPoolConstantProduct(LiquidityPoolEntryConstantProduct::read_xdr(r)?)
            }
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for LiquidityPoolEntryBody {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::LiquidityPoolConstantProduct(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// LiquidityPoolEntry is an XDR Struct defines as:
//
//   struct LiquidityPoolEntry
//    {
//        PoolID liquidityPoolID;
//
//        union switch (LiquidityPoolType type)
//        {
//        case LIQUIDITY_POOL_CONSTANT_PRODUCT:
//            struct
//            {
//                LiquidityPoolConstantProductParameters params;
//
//                int64 reserveA;        // amount of A in the pool
//                int64 reserveB;        // amount of B in the pool
//                int64 totalPoolShares; // total number of pool shares issued
//                int64 poolSharesTrustLineCount; // number of trust lines for the
//                                                // associated pool shares
//            } constantProduct;
//        }
//        body;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct LiquidityPoolEntry {
    pub liquidity_pool_id: PoolId,
    pub body: LiquidityPoolEntryBody,
}

impl ReadXdr for LiquidityPoolEntry {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            liquidity_pool_id: PoolId::read_xdr(r)?,
            body: LiquidityPoolEntryBody::read_xdr(r)?,
        })
    }
}

impl WriteXdr for LiquidityPoolEntry {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.liquidity_pool_id.write_xdr(w)?;
        self.body.write_xdr(w)?;
        Ok(())
    }
}

// ContractDataEntry is an XDR Struct defines as:
//
//   struct ContractDataEntry {
//        Hash contractID;
//        SCVal key;
//        SCVal val;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ContractDataEntry {
    pub contract_id: Hash,
    pub key: ScVal,
    pub val: ScVal,
}

impl ReadXdr for ContractDataEntry {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            contract_id: Hash::read_xdr(r)?,
            key: ScVal::read_xdr(r)?,
            val: ScVal::read_xdr(r)?,
        })
    }
}

impl WriteXdr for ContractDataEntry {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.contract_id.write_xdr(w)?;
        self.key.write_xdr(w)?;
        self.val.write_xdr(w)?;
        Ok(())
    }
}

// ConfigSettingType is an XDR Enum defines as:
//
//   enum ConfigSettingType
//    {
//        CONFIG_SETTING_TYPE_UINT32 = 0
//    };
//
// enum
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum ConfigSettingType {
    ConfigSettingTypeUint32 = 0,
}

impl ConfigSettingType {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::ConfigSettingTypeUint32 => "ConfigSettingTypeUint32",
        }
    }
}

impl fmt::Display for ConfigSettingType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for ConfigSettingType {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self> {
        let e = match i {
            0 => ConfigSettingType::ConfigSettingTypeUint32,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<ConfigSettingType> for i32 {
    #[must_use]
    fn from(e: ConfigSettingType) -> Self {
        e as Self
    }
}

impl ReadXdr for ConfigSettingType {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXdr for ConfigSettingType {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// ConfigSetting is an XDR Union defines as:
//
//   union ConfigSetting switch (ConfigSettingType type)
//    {
//    case CONFIG_SETTING_TYPE_UINT32:
//        uint32 uint32Val;
//    };
//
// union with discriminant ConfigSettingType
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ConfigSetting {
    ConfigSettingTypeUint32(u32),
}

impl ConfigSetting {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::ConfigSettingTypeUint32(_) => "ConfigSettingTypeUint32",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> ConfigSettingType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::ConfigSettingTypeUint32(_) => ConfigSettingType::ConfigSettingTypeUint32,
        }
    }
}

impl ReadXdr for ConfigSetting {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: ConfigSettingType = <ConfigSettingType as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            ConfigSettingType::ConfigSettingTypeUint32 => {
                Self::ConfigSettingTypeUint32(u32::read_xdr(r)?)
            }
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for ConfigSetting {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::ConfigSettingTypeUint32(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// ConfigSettingId is an XDR Enum defines as:
//
//   enum ConfigSettingID
//    {
//        CONFIG_SETTING_CONTRACT_MAX_SIZE = 0
//    };
//
// enum
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum ConfigSettingId {
    ConfigSettingContractMaxSize = 0,
}

impl ConfigSettingId {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::ConfigSettingContractMaxSize => "ConfigSettingContractMaxSize",
        }
    }
}

impl fmt::Display for ConfigSettingId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for ConfigSettingId {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self> {
        let e = match i {
            0 => ConfigSettingId::ConfigSettingContractMaxSize,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<ConfigSettingId> for i32 {
    #[must_use]
    fn from(e: ConfigSettingId) -> Self {
        e as Self
    }
}

impl ReadXdr for ConfigSettingId {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXdr for ConfigSettingId {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// ConfigSettingEntryExt is an XDR NestedUnion defines as:
//
//   union switch (int v)
//        {
//        case 0:
//            void;
//        }
//
// union with discriminant i32
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ConfigSettingEntryExt {
    V0,
}

impl ConfigSettingEntryExt {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0 => "V0",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> i32 {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0 => 0,
        }
    }
}

impl ReadXdr for ConfigSettingEntryExt {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: i32 = <i32 as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            0 => Self::V0,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for ConfigSettingEntryExt {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0 => ().write_xdr(w)?,
        };
        Ok(())
    }
}

// ConfigSettingEntry is an XDR Struct defines as:
//
//   struct ConfigSettingEntry
//    {
//        union switch (int v)
//        {
//        case 0:
//            void;
//        }
//        ext;
//
//        ConfigSettingID configSettingID;
//        ConfigSetting setting;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ConfigSettingEntry {
    pub ext: ConfigSettingEntryExt,
    pub config_setting_id: ConfigSettingId,
    pub setting: ConfigSetting,
}

impl ReadXdr for ConfigSettingEntry {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            ext: ConfigSettingEntryExt::read_xdr(r)?,
            config_setting_id: ConfigSettingId::read_xdr(r)?,
            setting: ConfigSetting::read_xdr(r)?,
        })
    }
}

impl WriteXdr for ConfigSettingEntry {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.ext.write_xdr(w)?;
        self.config_setting_id.write_xdr(w)?;
        self.setting.write_xdr(w)?;
        Ok(())
    }
}

// LedgerEntryExtensionV1Ext is an XDR NestedUnion defines as:
//
//   union switch (int v)
//        {
//        case 0:
//            void;
//        }
//
// union with discriminant i32
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum LedgerEntryExtensionV1Ext {
    V0,
}

impl LedgerEntryExtensionV1Ext {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0 => "V0",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> i32 {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0 => 0,
        }
    }
}

impl ReadXdr for LedgerEntryExtensionV1Ext {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: i32 = <i32 as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            0 => Self::V0,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for LedgerEntryExtensionV1Ext {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0 => ().write_xdr(w)?,
        };
        Ok(())
    }
}

// LedgerEntryExtensionV1 is an XDR Struct defines as:
//
//   struct LedgerEntryExtensionV1
//    {
//        SponsorshipDescriptor sponsoringID;
//
//        union switch (int v)
//        {
//        case 0:
//            void;
//        }
//        ext;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct LedgerEntryExtensionV1 {
    pub sponsoring_id: SponsorshipDescriptor,
    pub ext: LedgerEntryExtensionV1Ext,
}

impl ReadXdr for LedgerEntryExtensionV1 {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            sponsoring_id: SponsorshipDescriptor::read_xdr(r)?,
            ext: LedgerEntryExtensionV1Ext::read_xdr(r)?,
        })
    }
}

impl WriteXdr for LedgerEntryExtensionV1 {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.sponsoring_id.write_xdr(w)?;
        self.ext.write_xdr(w)?;
        Ok(())
    }
}

// LedgerEntryData is an XDR NestedUnion defines as:
//
//   union switch (LedgerEntryType type)
//        {
//        case ACCOUNT:
//            AccountEntry account;
//        case TRUSTLINE:
//            TrustLineEntry trustLine;
//        case OFFER:
//            OfferEntry offer;
//        case DATA:
//            DataEntry data;
//        case CLAIMABLE_BALANCE:
//            ClaimableBalanceEntry claimableBalance;
//        case LIQUIDITY_POOL:
//            LiquidityPoolEntry liquidityPool;
//        case CONTRACT_DATA:
//            ContractDataEntry contractData;
//        case CONFIG_SETTING:
//            ConfigSettingEntry configSetting;
//        }
//
// union with discriminant LedgerEntryType
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum LedgerEntryData {
    Account(AccountEntry),
    Trustline(TrustLineEntry),
    Offer(OfferEntry),
    Data(DataEntry),
    ClaimableBalance(ClaimableBalanceEntry),
    LiquidityPool(LiquidityPoolEntry),
    ContractData(ContractDataEntry),
    ConfigSetting(ConfigSettingEntry),
}

impl LedgerEntryData {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Account(_) => "Account",
            Self::Trustline(_) => "Trustline",
            Self::Offer(_) => "Offer",
            Self::Data(_) => "Data",
            Self::ClaimableBalance(_) => "ClaimableBalance",
            Self::LiquidityPool(_) => "LiquidityPool",
            Self::ContractData(_) => "ContractData",
            Self::ConfigSetting(_) => "ConfigSetting",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> LedgerEntryType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Account(_) => LedgerEntryType::Account,
            Self::Trustline(_) => LedgerEntryType::Trustline,
            Self::Offer(_) => LedgerEntryType::Offer,
            Self::Data(_) => LedgerEntryType::Data,
            Self::ClaimableBalance(_) => LedgerEntryType::ClaimableBalance,
            Self::LiquidityPool(_) => LedgerEntryType::LiquidityPool,
            Self::ContractData(_) => LedgerEntryType::ContractData,
            Self::ConfigSetting(_) => LedgerEntryType::ConfigSetting,
        }
    }
}

impl ReadXdr for LedgerEntryData {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: LedgerEntryType = <LedgerEntryType as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            LedgerEntryType::Account => Self::Account(AccountEntry::read_xdr(r)?),
            LedgerEntryType::Trustline => Self::Trustline(TrustLineEntry::read_xdr(r)?),
            LedgerEntryType::Offer => Self::Offer(OfferEntry::read_xdr(r)?),
            LedgerEntryType::Data => Self::Data(DataEntry::read_xdr(r)?),
            LedgerEntryType::ClaimableBalance => {
                Self::ClaimableBalance(ClaimableBalanceEntry::read_xdr(r)?)
            }
            LedgerEntryType::LiquidityPool => Self::LiquidityPool(LiquidityPoolEntry::read_xdr(r)?),
            LedgerEntryType::ContractData => Self::ContractData(ContractDataEntry::read_xdr(r)?),
            LedgerEntryType::ConfigSetting => Self::ConfigSetting(ConfigSettingEntry::read_xdr(r)?),
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for LedgerEntryData {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Account(v) => v.write_xdr(w)?,
            Self::Trustline(v) => v.write_xdr(w)?,
            Self::Offer(v) => v.write_xdr(w)?,
            Self::Data(v) => v.write_xdr(w)?,
            Self::ClaimableBalance(v) => v.write_xdr(w)?,
            Self::LiquidityPool(v) => v.write_xdr(w)?,
            Self::ContractData(v) => v.write_xdr(w)?,
            Self::ConfigSetting(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// LedgerEntryExt is an XDR NestedUnion defines as:
//
//   union switch (int v)
//        {
//        case 0:
//            void;
//        case 1:
//            LedgerEntryExtensionV1 v1;
//        }
//
// union with discriminant i32
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum LedgerEntryExt {
    V0,
    V1(LedgerEntryExtensionV1),
}

impl LedgerEntryExt {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0 => "V0",
            Self::V1(_) => "V1",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> i32 {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0 => 0,
            Self::V1(_) => 1,
        }
    }
}

impl ReadXdr for LedgerEntryExt {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: i32 = <i32 as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            0 => Self::V0,
            1 => Self::V1(LedgerEntryExtensionV1::read_xdr(r)?),
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for LedgerEntryExt {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0 => ().write_xdr(w)?,
            Self::V1(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// LedgerEntry is an XDR Struct defines as:
//
//   struct LedgerEntry
//    {
//        uint32 lastModifiedLedgerSeq; // ledger the LedgerEntry was last changed
//
//        union switch (LedgerEntryType type)
//        {
//        case ACCOUNT:
//            AccountEntry account;
//        case TRUSTLINE:
//            TrustLineEntry trustLine;
//        case OFFER:
//            OfferEntry offer;
//        case DATA:
//            DataEntry data;
//        case CLAIMABLE_BALANCE:
//            ClaimableBalanceEntry claimableBalance;
//        case LIQUIDITY_POOL:
//            LiquidityPoolEntry liquidityPool;
//        case CONTRACT_DATA:
//            ContractDataEntry contractData;
//        case CONFIG_SETTING:
//            ConfigSettingEntry configSetting;
//        }
//        data;
//
//        // reserved for future use
//        union switch (int v)
//        {
//        case 0:
//            void;
//        case 1:
//            LedgerEntryExtensionV1 v1;
//        }
//        ext;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct LedgerEntry {
    pub last_modified_ledger_seq: u32,
    pub data: LedgerEntryData,
    pub ext: LedgerEntryExt,
}

impl ReadXdr for LedgerEntry {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            last_modified_ledger_seq: u32::read_xdr(r)?,
            data: LedgerEntryData::read_xdr(r)?,
            ext: LedgerEntryExt::read_xdr(r)?,
        })
    }
}

impl WriteXdr for LedgerEntry {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.last_modified_ledger_seq.write_xdr(w)?;
        self.data.write_xdr(w)?;
        self.ext.write_xdr(w)?;
        Ok(())
    }
}

// LedgerKeyAccount is an XDR NestedStruct defines as:
//
//   struct
//        {
//            AccountID accountID;
//        }
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct LedgerKeyAccount {
    pub account_id: AccountId,
}

impl ReadXdr for LedgerKeyAccount {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            account_id: AccountId::read_xdr(r)?,
        })
    }
}

impl WriteXdr for LedgerKeyAccount {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.account_id.write_xdr(w)?;
        Ok(())
    }
}

// LedgerKeyTrustLine is an XDR NestedStruct defines as:
//
//   struct
//        {
//            AccountID accountID;
//            TrustLineAsset asset;
//        }
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct LedgerKeyTrustLine {
    pub account_id: AccountId,
    pub asset: TrustLineAsset,
}

impl ReadXdr for LedgerKeyTrustLine {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            account_id: AccountId::read_xdr(r)?,
            asset: TrustLineAsset::read_xdr(r)?,
        })
    }
}

impl WriteXdr for LedgerKeyTrustLine {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.account_id.write_xdr(w)?;
        self.asset.write_xdr(w)?;
        Ok(())
    }
}

// LedgerKeyOffer is an XDR NestedStruct defines as:
//
//   struct
//        {
//            AccountID sellerID;
//            int64 offerID;
//        }
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct LedgerKeyOffer {
    pub seller_id: AccountId,
    pub offer_id: i64,
}

impl ReadXdr for LedgerKeyOffer {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            seller_id: AccountId::read_xdr(r)?,
            offer_id: i64::read_xdr(r)?,
        })
    }
}

impl WriteXdr for LedgerKeyOffer {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.seller_id.write_xdr(w)?;
        self.offer_id.write_xdr(w)?;
        Ok(())
    }
}

// LedgerKeyData is an XDR NestedStruct defines as:
//
//   struct
//        {
//            AccountID accountID;
//            string64 dataName;
//        }
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct LedgerKeyData {
    pub account_id: AccountId,
    pub data_name: VecM<u8, 64>,
}

impl ReadXdr for LedgerKeyData {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            account_id: AccountId::read_xdr(r)?,
            data_name: VecM::<u8, 64>::read_xdr(r)?,
        })
    }
}

impl WriteXdr for LedgerKeyData {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.account_id.write_xdr(w)?;
        self.data_name.write_xdr(w)?;
        Ok(())
    }
}

// LedgerKeyClaimableBalance is an XDR NestedStruct defines as:
//
//   struct
//        {
//            ClaimableBalanceID balanceID;
//        }
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct LedgerKeyClaimableBalance {
    pub balance_id: ClaimableBalanceId,
}

impl ReadXdr for LedgerKeyClaimableBalance {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            balance_id: ClaimableBalanceId::read_xdr(r)?,
        })
    }
}

impl WriteXdr for LedgerKeyClaimableBalance {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.balance_id.write_xdr(w)?;
        Ok(())
    }
}

// LedgerKeyLiquidityPool is an XDR NestedStruct defines as:
//
//   struct
//        {
//            PoolID liquidityPoolID;
//        }
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct LedgerKeyLiquidityPool {
    pub liquidity_pool_id: PoolId,
}

impl ReadXdr for LedgerKeyLiquidityPool {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            liquidity_pool_id: PoolId::read_xdr(r)?,
        })
    }
}

impl WriteXdr for LedgerKeyLiquidityPool {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.liquidity_pool_id.write_xdr(w)?;
        Ok(())
    }
}

// LedgerKeyContractData is an XDR NestedStruct defines as:
//
//   struct
//        {
//            Hash contractID;
//            SCVal key;
//        }
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct LedgerKeyContractData {
    pub contract_id: Hash,
    pub key: ScVal,
}

impl ReadXdr for LedgerKeyContractData {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            contract_id: Hash::read_xdr(r)?,
            key: ScVal::read_xdr(r)?,
        })
    }
}

impl WriteXdr for LedgerKeyContractData {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.contract_id.write_xdr(w)?;
        self.key.write_xdr(w)?;
        Ok(())
    }
}

// LedgerKeyConfigSetting is an XDR NestedStruct defines as:
//
//   struct
//        {
//            ConfigSettingID configSettingID;
//        }
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct LedgerKeyConfigSetting {
    pub config_setting_id: ConfigSettingId,
}

impl ReadXdr for LedgerKeyConfigSetting {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            config_setting_id: ConfigSettingId::read_xdr(r)?,
        })
    }
}

impl WriteXdr for LedgerKeyConfigSetting {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.config_setting_id.write_xdr(w)?;
        Ok(())
    }
}

// LedgerKey is an XDR Union defines as:
//
//   union LedgerKey switch (LedgerEntryType type)
//    {
//    case ACCOUNT:
//        struct
//        {
//            AccountID accountID;
//        } account;
//
//    case TRUSTLINE:
//        struct
//        {
//            AccountID accountID;
//            TrustLineAsset asset;
//        } trustLine;
//
//    case OFFER:
//        struct
//        {
//            AccountID sellerID;
//            int64 offerID;
//        } offer;
//
//    case DATA:
//        struct
//        {
//            AccountID accountID;
//            string64 dataName;
//        } data;
//
//    case CLAIMABLE_BALANCE:
//        struct
//        {
//            ClaimableBalanceID balanceID;
//        } claimableBalance;
//
//    case LIQUIDITY_POOL:
//        struct
//        {
//            PoolID liquidityPoolID;
//        } liquidityPool;
//    case CONTRACT_DATA:
//        struct
//        {
//            Hash contractID;
//            SCVal key;
//        } contractData;
//    case CONFIG_SETTING:
//        struct
//        {
//            ConfigSettingID configSettingID;
//        } configSetting;
//    };
//
// union with discriminant LedgerEntryType
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum LedgerKey {
    Account(LedgerKeyAccount),
    Trustline(LedgerKeyTrustLine),
    Offer(LedgerKeyOffer),
    Data(LedgerKeyData),
    ClaimableBalance(LedgerKeyClaimableBalance),
    LiquidityPool(LedgerKeyLiquidityPool),
    ContractData(LedgerKeyContractData),
    ConfigSetting(LedgerKeyConfigSetting),
}

impl LedgerKey {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Account(_) => "Account",
            Self::Trustline(_) => "Trustline",
            Self::Offer(_) => "Offer",
            Self::Data(_) => "Data",
            Self::ClaimableBalance(_) => "ClaimableBalance",
            Self::LiquidityPool(_) => "LiquidityPool",
            Self::ContractData(_) => "ContractData",
            Self::ConfigSetting(_) => "ConfigSetting",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> LedgerEntryType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Account(_) => LedgerEntryType::Account,
            Self::Trustline(_) => LedgerEntryType::Trustline,
            Self::Offer(_) => LedgerEntryType::Offer,
            Self::Data(_) => LedgerEntryType::Data,
            Self::ClaimableBalance(_) => LedgerEntryType::ClaimableBalance,
            Self::LiquidityPool(_) => LedgerEntryType::LiquidityPool,
            Self::ContractData(_) => LedgerEntryType::ContractData,
            Self::ConfigSetting(_) => LedgerEntryType::ConfigSetting,
        }
    }
}

impl ReadXdr for LedgerKey {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: LedgerEntryType = <LedgerEntryType as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            LedgerEntryType::Account => Self::Account(LedgerKeyAccount::read_xdr(r)?),
            LedgerEntryType::Trustline => Self::Trustline(LedgerKeyTrustLine::read_xdr(r)?),
            LedgerEntryType::Offer => Self::Offer(LedgerKeyOffer::read_xdr(r)?),
            LedgerEntryType::Data => Self::Data(LedgerKeyData::read_xdr(r)?),
            LedgerEntryType::ClaimableBalance => {
                Self::ClaimableBalance(LedgerKeyClaimableBalance::read_xdr(r)?)
            }
            LedgerEntryType::LiquidityPool => {
                Self::LiquidityPool(LedgerKeyLiquidityPool::read_xdr(r)?)
            }
            LedgerEntryType::ContractData => {
                Self::ContractData(LedgerKeyContractData::read_xdr(r)?)
            }
            LedgerEntryType::ConfigSetting => {
                Self::ConfigSetting(LedgerKeyConfigSetting::read_xdr(r)?)
            }
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for LedgerKey {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Account(v) => v.write_xdr(w)?,
            Self::Trustline(v) => v.write_xdr(w)?,
            Self::Offer(v) => v.write_xdr(w)?,
            Self::Data(v) => v.write_xdr(w)?,
            Self::ClaimableBalance(v) => v.write_xdr(w)?,
            Self::LiquidityPool(v) => v.write_xdr(w)?,
            Self::ContractData(v) => v.write_xdr(w)?,
            Self::ConfigSetting(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// EnvelopeType is an XDR Enum defines as:
//
//   enum EnvelopeType
//    {
//        ENVELOPE_TYPE_TX_V0 = 0,
//        ENVELOPE_TYPE_SCP = 1,
//        ENVELOPE_TYPE_TX = 2,
//        ENVELOPE_TYPE_AUTH = 3,
//        ENVELOPE_TYPE_SCPVALUE = 4,
//        ENVELOPE_TYPE_TX_FEE_BUMP = 5,
//        ENVELOPE_TYPE_OP_ID = 6,
//        ENVELOPE_TYPE_POOL_REVOKE_OP_ID = 7,
//        ENVELOPE_TYPE_CONTRACT_ID_FROM_ED25519 = 8,
//        ENVELOPE_TYPE_CONTRACT_ID_FROM_CONTRACT = 9
//    };
//
// enum
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum EnvelopeType {
    TxV0 = 0,
    Scp = 1,
    Tx = 2,
    Auth = 3,
    Scpvalue = 4,
    TxFeeBump = 5,
    OpId = 6,
    PoolRevokeOpId = 7,
    ContractIdFromEd25519 = 8,
    ContractIdFromContract = 9,
}

impl EnvelopeType {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::TxV0 => "TxV0",
            Self::Scp => "Scp",
            Self::Tx => "Tx",
            Self::Auth => "Auth",
            Self::Scpvalue => "Scpvalue",
            Self::TxFeeBump => "TxFeeBump",
            Self::OpId => "OpId",
            Self::PoolRevokeOpId => "PoolRevokeOpId",
            Self::ContractIdFromEd25519 => "ContractIdFromEd25519",
            Self::ContractIdFromContract => "ContractIdFromContract",
        }
    }
}

impl fmt::Display for EnvelopeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for EnvelopeType {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self> {
        let e = match i {
            0 => EnvelopeType::TxV0,
            1 => EnvelopeType::Scp,
            2 => EnvelopeType::Tx,
            3 => EnvelopeType::Auth,
            4 => EnvelopeType::Scpvalue,
            5 => EnvelopeType::TxFeeBump,
            6 => EnvelopeType::OpId,
            7 => EnvelopeType::PoolRevokeOpId,
            8 => EnvelopeType::ContractIdFromEd25519,
            9 => EnvelopeType::ContractIdFromContract,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<EnvelopeType> for i32 {
    #[must_use]
    fn from(e: EnvelopeType) -> Self {
        e as Self
    }
}

impl ReadXdr for EnvelopeType {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXdr for EnvelopeType {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// UpgradeType is an XDR Typedef defines as:
//
//   typedef opaque UpgradeType<128>;
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct UpgradeType(pub VecM<u8, 128>);

impl From<UpgradeType> for VecM<u8, 128> {
    #[must_use]
    fn from(x: UpgradeType) -> Self {
        x.0
    }
}

impl From<VecM<u8, 128>> for UpgradeType {
    #[must_use]
    fn from(x: VecM<u8, 128>) -> Self {
        UpgradeType(x)
    }
}

impl AsRef<VecM<u8, 128>> for UpgradeType {
    #[must_use]
    fn as_ref(&self) -> &VecM<u8, 128> {
        &self.0
    }
}

impl ReadXdr for UpgradeType {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let i = VecM::<u8, 128>::read_xdr(r)?;
        let v = UpgradeType(i);
        Ok(v)
    }
}

impl WriteXdr for UpgradeType {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.0.write_xdr(w)
    }
}

impl Deref for UpgradeType {
    type Target = VecM<u8, 128>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<UpgradeType> for Vec<u8> {
    #[must_use]
    fn from(x: UpgradeType) -> Self {
        x.0 .0
    }
}

impl TryFrom<Vec<u8>> for UpgradeType {
    type Error = Error;
    fn try_from(x: Vec<u8>) -> Result<Self> {
        Ok(UpgradeType(x.try_into()?))
    }
}

impl AsRef<Vec<u8>> for UpgradeType {
    #[must_use]
    fn as_ref(&self) -> &Vec<u8> {
        &self.0 .0
    }
}

impl AsRef<[u8]> for UpgradeType {
    #[cfg(feature = "alloc")]
    #[must_use]
    fn as_ref(&self) -> &[u8] {
        &self.0 .0
    }
    #[cfg(not(feature = "alloc"))]
    #[must_use]
    fn as_ref(&self) -> &[u8] {
        self.0 .0
    }
}

// StellarValueType is an XDR Enum defines as:
//
//   enum StellarValueType
//    {
//        STELLAR_VALUE_BASIC = 0,
//        STELLAR_VALUE_SIGNED = 1
//    };
//
// enum
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum StellarValueType {
    Basic = 0,
    Signed = 1,
}

impl StellarValueType {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Basic => "Basic",
            Self::Signed => "Signed",
        }
    }
}

impl fmt::Display for StellarValueType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for StellarValueType {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self> {
        let e = match i {
            0 => StellarValueType::Basic,
            1 => StellarValueType::Signed,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<StellarValueType> for i32 {
    #[must_use]
    fn from(e: StellarValueType) -> Self {
        e as Self
    }
}

impl ReadXdr for StellarValueType {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXdr for StellarValueType {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// LedgerCloseValueSignature is an XDR Struct defines as:
//
//   struct LedgerCloseValueSignature
//    {
//        NodeID nodeID;       // which node introduced the value
//        Signature signature; // nodeID's signature
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct LedgerCloseValueSignature {
    pub node_id: NodeId,
    pub signature: Signature,
}

impl ReadXdr for LedgerCloseValueSignature {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            node_id: NodeId::read_xdr(r)?,
            signature: Signature::read_xdr(r)?,
        })
    }
}

impl WriteXdr for LedgerCloseValueSignature {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.node_id.write_xdr(w)?;
        self.signature.write_xdr(w)?;
        Ok(())
    }
}

// StellarValueExt is an XDR NestedUnion defines as:
//
//   union switch (StellarValueType v)
//        {
//        case STELLAR_VALUE_BASIC:
//            void;
//        case STELLAR_VALUE_SIGNED:
//            LedgerCloseValueSignature lcValueSignature;
//        }
//
// union with discriminant StellarValueType
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum StellarValueExt {
    Basic,
    Signed(LedgerCloseValueSignature),
}

impl StellarValueExt {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Basic => "Basic",
            Self::Signed(_) => "Signed",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> StellarValueType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Basic => StellarValueType::Basic,
            Self::Signed(_) => StellarValueType::Signed,
        }
    }
}

impl ReadXdr for StellarValueExt {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: StellarValueType = <StellarValueType as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            StellarValueType::Basic => Self::Basic,
            StellarValueType::Signed => Self::Signed(LedgerCloseValueSignature::read_xdr(r)?),
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for StellarValueExt {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Basic => ().write_xdr(w)?,
            Self::Signed(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// StellarValue is an XDR Struct defines as:
//
//   struct StellarValue
//    {
//        Hash txSetHash;      // transaction set to apply to previous ledger
//        TimePoint closeTime; // network close time
//
//        // upgrades to apply to the previous ledger (usually empty)
//        // this is a vector of encoded 'LedgerUpgrade' so that nodes can drop
//        // unknown steps during consensus if needed.
//        // see notes below on 'LedgerUpgrade' for more detail
//        // max size is dictated by number of upgrade types (+ room for future)
//        UpgradeType upgrades<6>;
//
//        // reserved for future use
//        union switch (StellarValueType v)
//        {
//        case STELLAR_VALUE_BASIC:
//            void;
//        case STELLAR_VALUE_SIGNED:
//            LedgerCloseValueSignature lcValueSignature;
//        }
//        ext;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct StellarValue {
    pub tx_set_hash: Hash,
    pub close_time: TimePoint,
    pub upgrades: VecM<UpgradeType, 6>,
    pub ext: StellarValueExt,
}

impl ReadXdr for StellarValue {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            tx_set_hash: Hash::read_xdr(r)?,
            close_time: TimePoint::read_xdr(r)?,
            upgrades: VecM::<UpgradeType, 6>::read_xdr(r)?,
            ext: StellarValueExt::read_xdr(r)?,
        })
    }
}

impl WriteXdr for StellarValue {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.tx_set_hash.write_xdr(w)?;
        self.close_time.write_xdr(w)?;
        self.upgrades.write_xdr(w)?;
        self.ext.write_xdr(w)?;
        Ok(())
    }
}

// MaskLedgerHeaderFlags is an XDR Const defines as:
//
//   const MASK_LEDGER_HEADER_FLAGS = 0x7F;
//
pub const MASK_LEDGER_HEADER_FLAGS: u64 = 0x7F;

// LedgerHeaderFlags is an XDR Enum defines as:
//
//   enum LedgerHeaderFlags
//    {
//        DISABLE_LIQUIDITY_POOL_TRADING_FLAG = 0x1,
//        DISABLE_LIQUIDITY_POOL_DEPOSIT_FLAG = 0x2,
//        DISABLE_LIQUIDITY_POOL_WITHDRAWAL_FLAG = 0x4,
//        DISABLE_CONTRACT_CREATE = 0x8,
//        DISABLE_CONTRACT_UPDATE = 0x10,
//        DISABLE_CONTRACT_REMOVE = 0x20,
//        DISABLE_CONTRACT_INVOKE = 0x40
//    };
//
// enum
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum LedgerHeaderFlags {
    LiquidityPoolTradingFlag = 1,
    LiquidityPoolDepositFlag = 2,
    LiquidityPoolWithdrawalFlag = 4,
    ContractCreate = 8,
    ContractUpdate = 16,
    ContractRemove = 32,
    ContractInvoke = 64,
}

impl LedgerHeaderFlags {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::LiquidityPoolTradingFlag => "LiquidityPoolTradingFlag",
            Self::LiquidityPoolDepositFlag => "LiquidityPoolDepositFlag",
            Self::LiquidityPoolWithdrawalFlag => "LiquidityPoolWithdrawalFlag",
            Self::ContractCreate => "ContractCreate",
            Self::ContractUpdate => "ContractUpdate",
            Self::ContractRemove => "ContractRemove",
            Self::ContractInvoke => "ContractInvoke",
        }
    }
}

impl fmt::Display for LedgerHeaderFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for LedgerHeaderFlags {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self> {
        let e = match i {
            1 => LedgerHeaderFlags::LiquidityPoolTradingFlag,
            2 => LedgerHeaderFlags::LiquidityPoolDepositFlag,
            4 => LedgerHeaderFlags::LiquidityPoolWithdrawalFlag,
            8 => LedgerHeaderFlags::ContractCreate,
            16 => LedgerHeaderFlags::ContractUpdate,
            32 => LedgerHeaderFlags::ContractRemove,
            64 => LedgerHeaderFlags::ContractInvoke,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<LedgerHeaderFlags> for i32 {
    #[must_use]
    fn from(e: LedgerHeaderFlags) -> Self {
        e as Self
    }
}

impl ReadXdr for LedgerHeaderFlags {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXdr for LedgerHeaderFlags {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// LedgerHeaderExtensionV1Ext is an XDR NestedUnion defines as:
//
//   union switch (int v)
//        {
//        case 0:
//            void;
//        }
//
// union with discriminant i32
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum LedgerHeaderExtensionV1Ext {
    V0,
}

impl LedgerHeaderExtensionV1Ext {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0 => "V0",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> i32 {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0 => 0,
        }
    }
}

impl ReadXdr for LedgerHeaderExtensionV1Ext {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: i32 = <i32 as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            0 => Self::V0,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for LedgerHeaderExtensionV1Ext {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0 => ().write_xdr(w)?,
        };
        Ok(())
    }
}

// LedgerHeaderExtensionV1 is an XDR Struct defines as:
//
//   struct LedgerHeaderExtensionV1
//    {
//        uint32 flags; // LedgerHeaderFlags
//
//        union switch (int v)
//        {
//        case 0:
//            void;
//        }
//        ext;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct LedgerHeaderExtensionV1 {
    pub flags: u32,
    pub ext: LedgerHeaderExtensionV1Ext,
}

impl ReadXdr for LedgerHeaderExtensionV1 {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            flags: u32::read_xdr(r)?,
            ext: LedgerHeaderExtensionV1Ext::read_xdr(r)?,
        })
    }
}

impl WriteXdr for LedgerHeaderExtensionV1 {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.flags.write_xdr(w)?;
        self.ext.write_xdr(w)?;
        Ok(())
    }
}

// LedgerHeaderExt is an XDR NestedUnion defines as:
//
//   union switch (int v)
//        {
//        case 0:
//            void;
//        case 1:
//            LedgerHeaderExtensionV1 v1;
//        }
//
// union with discriminant i32
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum LedgerHeaderExt {
    V0,
    V1(LedgerHeaderExtensionV1),
}

impl LedgerHeaderExt {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0 => "V0",
            Self::V1(_) => "V1",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> i32 {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0 => 0,
            Self::V1(_) => 1,
        }
    }
}

impl ReadXdr for LedgerHeaderExt {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: i32 = <i32 as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            0 => Self::V0,
            1 => Self::V1(LedgerHeaderExtensionV1::read_xdr(r)?),
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for LedgerHeaderExt {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0 => ().write_xdr(w)?,
            Self::V1(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// LedgerHeader is an XDR Struct defines as:
//
//   struct LedgerHeader
//    {
//        uint32 ledgerVersion;    // the protocol version of the ledger
//        Hash previousLedgerHash; // hash of the previous ledger header
//        StellarValue scpValue;   // what consensus agreed to
//        Hash txSetResultHash;    // the TransactionResultSet that led to this ledger
//        Hash bucketListHash;     // hash of the ledger state
//
//        uint32 ledgerSeq; // sequence number of this ledger
//
//        int64 totalCoins; // total number of stroops in existence.
//                          // 10,000,000 stroops in 1 XLM
//
//        int64 feePool;       // fees burned since last inflation run
//        uint32 inflationSeq; // inflation sequence number
//
//        uint64 idPool; // last used global ID, used for generating objects
//
//        uint32 baseFee;     // base fee per operation in stroops
//        uint32 baseReserve; // account base reserve in stroops
//
//        uint32 maxTxSetSize; // maximum size a transaction set can be
//
//        Hash skipList[4]; // hashes of ledgers in the past. allows you to jump back
//                          // in time without walking the chain back ledger by ledger
//                          // each slot contains the oldest ledger that is mod of
//                          // either 50  5000  50000 or 500000 depending on index
//                          // skipList[0] mod(50), skipList[1] mod(5000), etc
//
//        // reserved for future use
//        union switch (int v)
//        {
//        case 0:
//            void;
//        case 1:
//            LedgerHeaderExtensionV1 v1;
//        }
//        ext;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct LedgerHeader {
    pub ledger_version: u32,
    pub previous_ledger_hash: Hash,
    pub scp_value: StellarValue,
    pub tx_set_result_hash: Hash,
    pub bucket_list_hash: Hash,
    pub ledger_seq: u32,
    pub total_coins: i64,
    pub fee_pool: i64,
    pub inflation_seq: u32,
    pub id_pool: u64,
    pub base_fee: u32,
    pub base_reserve: u32,
    pub max_tx_set_size: u32,
    pub skip_list: [Hash; 4],
    pub ext: LedgerHeaderExt,
}

impl ReadXdr for LedgerHeader {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            ledger_version: u32::read_xdr(r)?,
            previous_ledger_hash: Hash::read_xdr(r)?,
            scp_value: StellarValue::read_xdr(r)?,
            tx_set_result_hash: Hash::read_xdr(r)?,
            bucket_list_hash: Hash::read_xdr(r)?,
            ledger_seq: u32::read_xdr(r)?,
            total_coins: i64::read_xdr(r)?,
            fee_pool: i64::read_xdr(r)?,
            inflation_seq: u32::read_xdr(r)?,
            id_pool: u64::read_xdr(r)?,
            base_fee: u32::read_xdr(r)?,
            base_reserve: u32::read_xdr(r)?,
            max_tx_set_size: u32::read_xdr(r)?,
            skip_list: <[Hash; 4]>::read_xdr(r)?,
            ext: LedgerHeaderExt::read_xdr(r)?,
        })
    }
}

impl WriteXdr for LedgerHeader {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.ledger_version.write_xdr(w)?;
        self.previous_ledger_hash.write_xdr(w)?;
        self.scp_value.write_xdr(w)?;
        self.tx_set_result_hash.write_xdr(w)?;
        self.bucket_list_hash.write_xdr(w)?;
        self.ledger_seq.write_xdr(w)?;
        self.total_coins.write_xdr(w)?;
        self.fee_pool.write_xdr(w)?;
        self.inflation_seq.write_xdr(w)?;
        self.id_pool.write_xdr(w)?;
        self.base_fee.write_xdr(w)?;
        self.base_reserve.write_xdr(w)?;
        self.max_tx_set_size.write_xdr(w)?;
        self.skip_list.write_xdr(w)?;
        self.ext.write_xdr(w)?;
        Ok(())
    }
}

// LedgerUpgradeType is an XDR Enum defines as:
//
//   enum LedgerUpgradeType
//    {
//        LEDGER_UPGRADE_VERSION = 1,
//        LEDGER_UPGRADE_BASE_FEE = 2,
//        LEDGER_UPGRADE_MAX_TX_SET_SIZE = 3,
//        LEDGER_UPGRADE_BASE_RESERVE = 4,
//        LEDGER_UPGRADE_FLAGS = 5,
//        LEDGER_UPGRADE_CONFIG = 6
//    };
//
// enum
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum LedgerUpgradeType {
    Version = 1,
    BaseFee = 2,
    MaxTxSetSize = 3,
    BaseReserve = 4,
    Flags = 5,
    Config = 6,
}

impl LedgerUpgradeType {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Version => "Version",
            Self::BaseFee => "BaseFee",
            Self::MaxTxSetSize => "MaxTxSetSize",
            Self::BaseReserve => "BaseReserve",
            Self::Flags => "Flags",
            Self::Config => "Config",
        }
    }
}

impl fmt::Display for LedgerUpgradeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for LedgerUpgradeType {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self> {
        let e = match i {
            1 => LedgerUpgradeType::Version,
            2 => LedgerUpgradeType::BaseFee,
            3 => LedgerUpgradeType::MaxTxSetSize,
            4 => LedgerUpgradeType::BaseReserve,
            5 => LedgerUpgradeType::Flags,
            6 => LedgerUpgradeType::Config,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<LedgerUpgradeType> for i32 {
    #[must_use]
    fn from(e: LedgerUpgradeType) -> Self {
        e as Self
    }
}

impl ReadXdr for LedgerUpgradeType {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXdr for LedgerUpgradeType {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// LedgerUpgradeConfigSetting is an XDR NestedStruct defines as:
//
//   struct
//        {
//            ConfigSettingID id; // id to update
//            ConfigSetting setting; // new value
//        }
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct LedgerUpgradeConfigSetting {
    pub id: ConfigSettingId,
    pub setting: ConfigSetting,
}

impl ReadXdr for LedgerUpgradeConfigSetting {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            id: ConfigSettingId::read_xdr(r)?,
            setting: ConfigSetting::read_xdr(r)?,
        })
    }
}

impl WriteXdr for LedgerUpgradeConfigSetting {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.id.write_xdr(w)?;
        self.setting.write_xdr(w)?;
        Ok(())
    }
}

// LedgerUpgrade is an XDR Union defines as:
//
//   union LedgerUpgrade switch (LedgerUpgradeType type)
//    {
//    case LEDGER_UPGRADE_VERSION:
//        uint32 newLedgerVersion; // update ledgerVersion
//    case LEDGER_UPGRADE_BASE_FEE:
//        uint32 newBaseFee; // update baseFee
//    case LEDGER_UPGRADE_MAX_TX_SET_SIZE:
//        uint32 newMaxTxSetSize; // update maxTxSetSize
//    case LEDGER_UPGRADE_BASE_RESERVE:
//        uint32 newBaseReserve; // update baseReserve
//    case LEDGER_UPGRADE_FLAGS:
//        uint32 newFlags; // update flags
//    case LEDGER_UPGRADE_CONFIG:
//        struct
//        {
//            ConfigSettingID id; // id to update
//            ConfigSetting setting; // new value
//        } configSetting;
//    };
//
// union with discriminant LedgerUpgradeType
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum LedgerUpgrade {
    Version(u32),
    BaseFee(u32),
    MaxTxSetSize(u32),
    BaseReserve(u32),
    Flags(u32),
    Config(LedgerUpgradeConfigSetting),
}

impl LedgerUpgrade {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Version(_) => "Version",
            Self::BaseFee(_) => "BaseFee",
            Self::MaxTxSetSize(_) => "MaxTxSetSize",
            Self::BaseReserve(_) => "BaseReserve",
            Self::Flags(_) => "Flags",
            Self::Config(_) => "Config",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> LedgerUpgradeType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Version(_) => LedgerUpgradeType::Version,
            Self::BaseFee(_) => LedgerUpgradeType::BaseFee,
            Self::MaxTxSetSize(_) => LedgerUpgradeType::MaxTxSetSize,
            Self::BaseReserve(_) => LedgerUpgradeType::BaseReserve,
            Self::Flags(_) => LedgerUpgradeType::Flags,
            Self::Config(_) => LedgerUpgradeType::Config,
        }
    }
}

impl ReadXdr for LedgerUpgrade {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: LedgerUpgradeType = <LedgerUpgradeType as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            LedgerUpgradeType::Version => Self::Version(u32::read_xdr(r)?),
            LedgerUpgradeType::BaseFee => Self::BaseFee(u32::read_xdr(r)?),
            LedgerUpgradeType::MaxTxSetSize => Self::MaxTxSetSize(u32::read_xdr(r)?),
            LedgerUpgradeType::BaseReserve => Self::BaseReserve(u32::read_xdr(r)?),
            LedgerUpgradeType::Flags => Self::Flags(u32::read_xdr(r)?),
            LedgerUpgradeType::Config => Self::Config(LedgerUpgradeConfigSetting::read_xdr(r)?),
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for LedgerUpgrade {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Version(v) => v.write_xdr(w)?,
            Self::BaseFee(v) => v.write_xdr(w)?,
            Self::MaxTxSetSize(v) => v.write_xdr(w)?,
            Self::BaseReserve(v) => v.write_xdr(w)?,
            Self::Flags(v) => v.write_xdr(w)?,
            Self::Config(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// BucketEntryType is an XDR Enum defines as:
//
//   enum BucketEntryType
//    {
//        METAENTRY =
//            -1, // At-and-after protocol 11: bucket metadata, should come first.
//        LIVEENTRY = 0, // Before protocol 11: created-or-updated;
//                       // At-and-after protocol 11: only updated.
//        DEADENTRY = 1,
//        INITENTRY = 2 // At-and-after protocol 11: only created.
//    };
//
// enum
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum BucketEntryType {
    Metaentry = -1,
    Liveentry = 0,
    Deadentry = 1,
    Initentry = 2,
}

impl BucketEntryType {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Metaentry => "Metaentry",
            Self::Liveentry => "Liveentry",
            Self::Deadentry => "Deadentry",
            Self::Initentry => "Initentry",
        }
    }
}

impl fmt::Display for BucketEntryType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for BucketEntryType {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self> {
        let e = match i {
            -1 => BucketEntryType::Metaentry,
            0 => BucketEntryType::Liveentry,
            1 => BucketEntryType::Deadentry,
            2 => BucketEntryType::Initentry,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<BucketEntryType> for i32 {
    #[must_use]
    fn from(e: BucketEntryType) -> Self {
        e as Self
    }
}

impl ReadXdr for BucketEntryType {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXdr for BucketEntryType {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// BucketMetadataExt is an XDR NestedUnion defines as:
//
//   union switch (int v)
//        {
//        case 0:
//            void;
//        }
//
// union with discriminant i32
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum BucketMetadataExt {
    V0,
}

impl BucketMetadataExt {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0 => "V0",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> i32 {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0 => 0,
        }
    }
}

impl ReadXdr for BucketMetadataExt {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: i32 = <i32 as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            0 => Self::V0,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for BucketMetadataExt {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0 => ().write_xdr(w)?,
        };
        Ok(())
    }
}

// BucketMetadata is an XDR Struct defines as:
//
//   struct BucketMetadata
//    {
//        // Indicates the protocol version used to create / merge this bucket.
//        uint32 ledgerVersion;
//
//        // reserved for future use
//        union switch (int v)
//        {
//        case 0:
//            void;
//        }
//        ext;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct BucketMetadata {
    pub ledger_version: u32,
    pub ext: BucketMetadataExt,
}

impl ReadXdr for BucketMetadata {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            ledger_version: u32::read_xdr(r)?,
            ext: BucketMetadataExt::read_xdr(r)?,
        })
    }
}

impl WriteXdr for BucketMetadata {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.ledger_version.write_xdr(w)?;
        self.ext.write_xdr(w)?;
        Ok(())
    }
}

// BucketEntry is an XDR Union defines as:
//
//   union BucketEntry switch (BucketEntryType type)
//    {
//    case LIVEENTRY:
//    case INITENTRY:
//        LedgerEntry liveEntry;
//
//    case DEADENTRY:
//        LedgerKey deadEntry;
//    case METAENTRY:
//        BucketMetadata metaEntry;
//    };
//
// union with discriminant BucketEntryType
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum BucketEntry {
    Liveentry(LedgerEntry),
    Initentry(LedgerEntry),
    Deadentry(LedgerKey),
    Metaentry(BucketMetadata),
}

impl BucketEntry {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Liveentry(_) => "Liveentry",
            Self::Initentry(_) => "Initentry",
            Self::Deadentry(_) => "Deadentry",
            Self::Metaentry(_) => "Metaentry",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> BucketEntryType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Liveentry(_) => BucketEntryType::Liveentry,
            Self::Initentry(_) => BucketEntryType::Initentry,
            Self::Deadentry(_) => BucketEntryType::Deadentry,
            Self::Metaentry(_) => BucketEntryType::Metaentry,
        }
    }
}

impl ReadXdr for BucketEntry {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: BucketEntryType = <BucketEntryType as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            BucketEntryType::Liveentry => Self::Liveentry(LedgerEntry::read_xdr(r)?),
            BucketEntryType::Initentry => Self::Initentry(LedgerEntry::read_xdr(r)?),
            BucketEntryType::Deadentry => Self::Deadentry(LedgerKey::read_xdr(r)?),
            BucketEntryType::Metaentry => Self::Metaentry(BucketMetadata::read_xdr(r)?),
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for BucketEntry {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Liveentry(v) => v.write_xdr(w)?,
            Self::Initentry(v) => v.write_xdr(w)?,
            Self::Deadentry(v) => v.write_xdr(w)?,
            Self::Metaentry(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// TransactionSet is an XDR Struct defines as:
//
//   struct TransactionSet
//    {
//        Hash previousLedgerHash;
//        TransactionEnvelope txs<>;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TransactionSet {
    pub previous_ledger_hash: Hash,
    pub txs: VecM<TransactionEnvelope>,
}

impl ReadXdr for TransactionSet {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            previous_ledger_hash: Hash::read_xdr(r)?,
            txs: VecM::<TransactionEnvelope>::read_xdr(r)?,
        })
    }
}

impl WriteXdr for TransactionSet {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.previous_ledger_hash.write_xdr(w)?;
        self.txs.write_xdr(w)?;
        Ok(())
    }
}

// TransactionResultPair is an XDR Struct defines as:
//
//   struct TransactionResultPair
//    {
//        Hash transactionHash;
//        TransactionResult result; // result for the transaction
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TransactionResultPair {
    pub transaction_hash: Hash,
    pub result: TransactionResult,
}

impl ReadXdr for TransactionResultPair {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            transaction_hash: Hash::read_xdr(r)?,
            result: TransactionResult::read_xdr(r)?,
        })
    }
}

impl WriteXdr for TransactionResultPair {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.transaction_hash.write_xdr(w)?;
        self.result.write_xdr(w)?;
        Ok(())
    }
}

// TransactionResultSet is an XDR Struct defines as:
//
//   struct TransactionResultSet
//    {
//        TransactionResultPair results<>;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TransactionResultSet {
    pub results: VecM<TransactionResultPair>,
}

impl ReadXdr for TransactionResultSet {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            results: VecM::<TransactionResultPair>::read_xdr(r)?,
        })
    }
}

impl WriteXdr for TransactionResultSet {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.results.write_xdr(w)?;
        Ok(())
    }
}

// TransactionHistoryEntryExt is an XDR NestedUnion defines as:
//
//   union switch (int v)
//        {
//        case 0:
//            void;
//        }
//
// union with discriminant i32
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum TransactionHistoryEntryExt {
    V0,
}

impl TransactionHistoryEntryExt {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0 => "V0",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> i32 {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0 => 0,
        }
    }
}

impl ReadXdr for TransactionHistoryEntryExt {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: i32 = <i32 as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            0 => Self::V0,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for TransactionHistoryEntryExt {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0 => ().write_xdr(w)?,
        };
        Ok(())
    }
}

// TransactionHistoryEntry is an XDR Struct defines as:
//
//   struct TransactionHistoryEntry
//    {
//        uint32 ledgerSeq;
//        TransactionSet txSet;
//
//        // reserved for future use
//        union switch (int v)
//        {
//        case 0:
//            void;
//        }
//        ext;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TransactionHistoryEntry {
    pub ledger_seq: u32,
    pub tx_set: TransactionSet,
    pub ext: TransactionHistoryEntryExt,
}

impl ReadXdr for TransactionHistoryEntry {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            ledger_seq: u32::read_xdr(r)?,
            tx_set: TransactionSet::read_xdr(r)?,
            ext: TransactionHistoryEntryExt::read_xdr(r)?,
        })
    }
}

impl WriteXdr for TransactionHistoryEntry {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.ledger_seq.write_xdr(w)?;
        self.tx_set.write_xdr(w)?;
        self.ext.write_xdr(w)?;
        Ok(())
    }
}

// TransactionHistoryResultEntryExt is an XDR NestedUnion defines as:
//
//   union switch (int v)
//        {
//        case 0:
//            void;
//        }
//
// union with discriminant i32
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum TransactionHistoryResultEntryExt {
    V0,
}

impl TransactionHistoryResultEntryExt {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0 => "V0",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> i32 {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0 => 0,
        }
    }
}

impl ReadXdr for TransactionHistoryResultEntryExt {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: i32 = <i32 as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            0 => Self::V0,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for TransactionHistoryResultEntryExt {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0 => ().write_xdr(w)?,
        };
        Ok(())
    }
}

// TransactionHistoryResultEntry is an XDR Struct defines as:
//
//   struct TransactionHistoryResultEntry
//    {
//        uint32 ledgerSeq;
//        TransactionResultSet txResultSet;
//
//        // reserved for future use
//        union switch (int v)
//        {
//        case 0:
//            void;
//        }
//        ext;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TransactionHistoryResultEntry {
    pub ledger_seq: u32,
    pub tx_result_set: TransactionResultSet,
    pub ext: TransactionHistoryResultEntryExt,
}

impl ReadXdr for TransactionHistoryResultEntry {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            ledger_seq: u32::read_xdr(r)?,
            tx_result_set: TransactionResultSet::read_xdr(r)?,
            ext: TransactionHistoryResultEntryExt::read_xdr(r)?,
        })
    }
}

impl WriteXdr for TransactionHistoryResultEntry {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.ledger_seq.write_xdr(w)?;
        self.tx_result_set.write_xdr(w)?;
        self.ext.write_xdr(w)?;
        Ok(())
    }
}

// LedgerHeaderHistoryEntryExt is an XDR NestedUnion defines as:
//
//   union switch (int v)
//        {
//        case 0:
//            void;
//        }
//
// union with discriminant i32
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum LedgerHeaderHistoryEntryExt {
    V0,
}

impl LedgerHeaderHistoryEntryExt {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0 => "V0",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> i32 {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0 => 0,
        }
    }
}

impl ReadXdr for LedgerHeaderHistoryEntryExt {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: i32 = <i32 as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            0 => Self::V0,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for LedgerHeaderHistoryEntryExt {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0 => ().write_xdr(w)?,
        };
        Ok(())
    }
}

// LedgerHeaderHistoryEntry is an XDR Struct defines as:
//
//   struct LedgerHeaderHistoryEntry
//    {
//        Hash hash;
//        LedgerHeader header;
//
//        // reserved for future use
//        union switch (int v)
//        {
//        case 0:
//            void;
//        }
//        ext;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct LedgerHeaderHistoryEntry {
    pub hash: Hash,
    pub header: LedgerHeader,
    pub ext: LedgerHeaderHistoryEntryExt,
}

impl ReadXdr for LedgerHeaderHistoryEntry {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            hash: Hash::read_xdr(r)?,
            header: LedgerHeader::read_xdr(r)?,
            ext: LedgerHeaderHistoryEntryExt::read_xdr(r)?,
        })
    }
}

impl WriteXdr for LedgerHeaderHistoryEntry {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.hash.write_xdr(w)?;
        self.header.write_xdr(w)?;
        self.ext.write_xdr(w)?;
        Ok(())
    }
}

// LedgerScpMessages is an XDR Struct defines as:
//
//   struct LedgerSCPMessages
//    {
//        uint32 ledgerSeq;
//        SCPEnvelope messages<>;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct LedgerScpMessages {
    pub ledger_seq: u32,
    pub messages: VecM<ScpEnvelope>,
}

impl ReadXdr for LedgerScpMessages {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            ledger_seq: u32::read_xdr(r)?,
            messages: VecM::<ScpEnvelope>::read_xdr(r)?,
        })
    }
}

impl WriteXdr for LedgerScpMessages {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.ledger_seq.write_xdr(w)?;
        self.messages.write_xdr(w)?;
        Ok(())
    }
}

// ScpHistoryEntryV0 is an XDR Struct defines as:
//
//   struct SCPHistoryEntryV0
//    {
//        SCPQuorumSet quorumSets<>; // additional quorum sets used by ledgerMessages
//        LedgerSCPMessages ledgerMessages;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScpHistoryEntryV0 {
    pub quorum_sets: VecM<ScpQuorumSet>,
    pub ledger_messages: LedgerScpMessages,
}

impl ReadXdr for ScpHistoryEntryV0 {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            quorum_sets: VecM::<ScpQuorumSet>::read_xdr(r)?,
            ledger_messages: LedgerScpMessages::read_xdr(r)?,
        })
    }
}

impl WriteXdr for ScpHistoryEntryV0 {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.quorum_sets.write_xdr(w)?;
        self.ledger_messages.write_xdr(w)?;
        Ok(())
    }
}

// ScpHistoryEntry is an XDR Union defines as:
//
//   union SCPHistoryEntry switch (int v)
//    {
//    case 0:
//        SCPHistoryEntryV0 v0;
//    };
//
// union with discriminant i32
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ScpHistoryEntry {
    V0(ScpHistoryEntryV0),
}

impl ScpHistoryEntry {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0(_) => "V0",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> i32 {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0(_) => 0,
        }
    }
}

impl ReadXdr for ScpHistoryEntry {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: i32 = <i32 as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            0 => Self::V0(ScpHistoryEntryV0::read_xdr(r)?),
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for ScpHistoryEntry {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// LedgerEntryChangeType is an XDR Enum defines as:
//
//   enum LedgerEntryChangeType
//    {
//        LEDGER_ENTRY_CREATED = 0, // entry was added to the ledger
//        LEDGER_ENTRY_UPDATED = 1, // entry was modified in the ledger
//        LEDGER_ENTRY_REMOVED = 2, // entry was removed from the ledger
//        LEDGER_ENTRY_STATE = 3    // value of the entry
//    };
//
// enum
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum LedgerEntryChangeType {
    Created = 0,
    Updated = 1,
    Removed = 2,
    State = 3,
}

impl LedgerEntryChangeType {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Created => "Created",
            Self::Updated => "Updated",
            Self::Removed => "Removed",
            Self::State => "State",
        }
    }
}

impl fmt::Display for LedgerEntryChangeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for LedgerEntryChangeType {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self> {
        let e = match i {
            0 => LedgerEntryChangeType::Created,
            1 => LedgerEntryChangeType::Updated,
            2 => LedgerEntryChangeType::Removed,
            3 => LedgerEntryChangeType::State,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<LedgerEntryChangeType> for i32 {
    #[must_use]
    fn from(e: LedgerEntryChangeType) -> Self {
        e as Self
    }
}

impl ReadXdr for LedgerEntryChangeType {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXdr for LedgerEntryChangeType {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// LedgerEntryChange is an XDR Union defines as:
//
//   union LedgerEntryChange switch (LedgerEntryChangeType type)
//    {
//    case LEDGER_ENTRY_CREATED:
//        LedgerEntry created;
//    case LEDGER_ENTRY_UPDATED:
//        LedgerEntry updated;
//    case LEDGER_ENTRY_REMOVED:
//        LedgerKey removed;
//    case LEDGER_ENTRY_STATE:
//        LedgerEntry state;
//    };
//
// union with discriminant LedgerEntryChangeType
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum LedgerEntryChange {
    Created(LedgerEntry),
    Updated(LedgerEntry),
    Removed(LedgerKey),
    State(LedgerEntry),
}

impl LedgerEntryChange {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Created(_) => "Created",
            Self::Updated(_) => "Updated",
            Self::Removed(_) => "Removed",
            Self::State(_) => "State",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> LedgerEntryChangeType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Created(_) => LedgerEntryChangeType::Created,
            Self::Updated(_) => LedgerEntryChangeType::Updated,
            Self::Removed(_) => LedgerEntryChangeType::Removed,
            Self::State(_) => LedgerEntryChangeType::State,
        }
    }
}

impl ReadXdr for LedgerEntryChange {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: LedgerEntryChangeType = <LedgerEntryChangeType as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            LedgerEntryChangeType::Created => Self::Created(LedgerEntry::read_xdr(r)?),
            LedgerEntryChangeType::Updated => Self::Updated(LedgerEntry::read_xdr(r)?),
            LedgerEntryChangeType::Removed => Self::Removed(LedgerKey::read_xdr(r)?),
            LedgerEntryChangeType::State => Self::State(LedgerEntry::read_xdr(r)?),
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for LedgerEntryChange {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Created(v) => v.write_xdr(w)?,
            Self::Updated(v) => v.write_xdr(w)?,
            Self::Removed(v) => v.write_xdr(w)?,
            Self::State(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// LedgerEntryChanges is an XDR Typedef defines as:
//
//   typedef LedgerEntryChange LedgerEntryChanges<>;
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct LedgerEntryChanges(pub VecM<LedgerEntryChange>);

impl From<LedgerEntryChanges> for VecM<LedgerEntryChange> {
    #[must_use]
    fn from(x: LedgerEntryChanges) -> Self {
        x.0
    }
}

impl From<VecM<LedgerEntryChange>> for LedgerEntryChanges {
    #[must_use]
    fn from(x: VecM<LedgerEntryChange>) -> Self {
        LedgerEntryChanges(x)
    }
}

impl AsRef<VecM<LedgerEntryChange>> for LedgerEntryChanges {
    #[must_use]
    fn as_ref(&self) -> &VecM<LedgerEntryChange> {
        &self.0
    }
}

impl ReadXdr for LedgerEntryChanges {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let i = VecM::<LedgerEntryChange>::read_xdr(r)?;
        let v = LedgerEntryChanges(i);
        Ok(v)
    }
}

impl WriteXdr for LedgerEntryChanges {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.0.write_xdr(w)
    }
}

impl Deref for LedgerEntryChanges {
    type Target = VecM<LedgerEntryChange>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<LedgerEntryChanges> for Vec<LedgerEntryChange> {
    #[must_use]
    fn from(x: LedgerEntryChanges) -> Self {
        x.0 .0
    }
}

impl TryFrom<Vec<LedgerEntryChange>> for LedgerEntryChanges {
    type Error = Error;
    fn try_from(x: Vec<LedgerEntryChange>) -> Result<Self> {
        Ok(LedgerEntryChanges(x.try_into()?))
    }
}

impl AsRef<Vec<LedgerEntryChange>> for LedgerEntryChanges {
    #[must_use]
    fn as_ref(&self) -> &Vec<LedgerEntryChange> {
        &self.0 .0
    }
}

impl AsRef<[LedgerEntryChange]> for LedgerEntryChanges {
    #[cfg(feature = "alloc")]
    #[must_use]
    fn as_ref(&self) -> &[LedgerEntryChange] {
        &self.0 .0
    }
    #[cfg(not(feature = "alloc"))]
    #[must_use]
    fn as_ref(&self) -> &[LedgerEntryChange] {
        self.0 .0
    }
}

// OperationMeta is an XDR Struct defines as:
//
//   struct OperationMeta
//    {
//        LedgerEntryChanges changes;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct OperationMeta {
    pub changes: LedgerEntryChanges,
}

impl ReadXdr for OperationMeta {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            changes: LedgerEntryChanges::read_xdr(r)?,
        })
    }
}

impl WriteXdr for OperationMeta {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.changes.write_xdr(w)?;
        Ok(())
    }
}

// TransactionMetaV1 is an XDR Struct defines as:
//
//   struct TransactionMetaV1
//    {
//        LedgerEntryChanges txChanges; // tx level changes if any
//        OperationMeta operations<>;   // meta for each operation
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TransactionMetaV1 {
    pub tx_changes: LedgerEntryChanges,
    pub operations: VecM<OperationMeta>,
}

impl ReadXdr for TransactionMetaV1 {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            tx_changes: LedgerEntryChanges::read_xdr(r)?,
            operations: VecM::<OperationMeta>::read_xdr(r)?,
        })
    }
}

impl WriteXdr for TransactionMetaV1 {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.tx_changes.write_xdr(w)?;
        self.operations.write_xdr(w)?;
        Ok(())
    }
}

// TransactionMetaV2 is an XDR Struct defines as:
//
//   struct TransactionMetaV2
//    {
//        LedgerEntryChanges txChangesBefore; // tx level changes before operations
//                                            // are applied if any
//        OperationMeta operations<>;         // meta for each operation
//        LedgerEntryChanges txChangesAfter;  // tx level changes after operations are
//                                            // applied if any
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TransactionMetaV2 {
    pub tx_changes_before: LedgerEntryChanges,
    pub operations: VecM<OperationMeta>,
    pub tx_changes_after: LedgerEntryChanges,
}

impl ReadXdr for TransactionMetaV2 {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            tx_changes_before: LedgerEntryChanges::read_xdr(r)?,
            operations: VecM::<OperationMeta>::read_xdr(r)?,
            tx_changes_after: LedgerEntryChanges::read_xdr(r)?,
        })
    }
}

impl WriteXdr for TransactionMetaV2 {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.tx_changes_before.write_xdr(w)?;
        self.operations.write_xdr(w)?;
        self.tx_changes_after.write_xdr(w)?;
        Ok(())
    }
}

// TransactionMeta is an XDR Union defines as:
//
//   union TransactionMeta switch (int v)
//    {
//    case 0:
//        OperationMeta operations<>;
//    case 1:
//        TransactionMetaV1 v1;
//    case 2:
//        TransactionMetaV2 v2;
//    };
//
// union with discriminant i32
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum TransactionMeta {
    V0(VecM<OperationMeta>),
    V1(TransactionMetaV1),
    V2(TransactionMetaV2),
}

impl TransactionMeta {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0(_) => "V0",
            Self::V1(_) => "V1",
            Self::V2(_) => "V2",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> i32 {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0(_) => 0,
            Self::V1(_) => 1,
            Self::V2(_) => 2,
        }
    }
}

impl ReadXdr for TransactionMeta {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: i32 = <i32 as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            0 => Self::V0(VecM::<OperationMeta>::read_xdr(r)?),
            1 => Self::V1(TransactionMetaV1::read_xdr(r)?),
            2 => Self::V2(TransactionMetaV2::read_xdr(r)?),
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for TransactionMeta {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0(v) => v.write_xdr(w)?,
            Self::V1(v) => v.write_xdr(w)?,
            Self::V2(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// TransactionResultMeta is an XDR Struct defines as:
//
//   struct TransactionResultMeta
//    {
//        TransactionResultPair result;
//        LedgerEntryChanges feeProcessing;
//        TransactionMeta txApplyProcessing;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TransactionResultMeta {
    pub result: TransactionResultPair,
    pub fee_processing: LedgerEntryChanges,
    pub tx_apply_processing: TransactionMeta,
}

impl ReadXdr for TransactionResultMeta {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            result: TransactionResultPair::read_xdr(r)?,
            fee_processing: LedgerEntryChanges::read_xdr(r)?,
            tx_apply_processing: TransactionMeta::read_xdr(r)?,
        })
    }
}

impl WriteXdr for TransactionResultMeta {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.result.write_xdr(w)?;
        self.fee_processing.write_xdr(w)?;
        self.tx_apply_processing.write_xdr(w)?;
        Ok(())
    }
}

// UpgradeEntryMeta is an XDR Struct defines as:
//
//   struct UpgradeEntryMeta
//    {
//        LedgerUpgrade upgrade;
//        LedgerEntryChanges changes;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct UpgradeEntryMeta {
    pub upgrade: LedgerUpgrade,
    pub changes: LedgerEntryChanges,
}

impl ReadXdr for UpgradeEntryMeta {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            upgrade: LedgerUpgrade::read_xdr(r)?,
            changes: LedgerEntryChanges::read_xdr(r)?,
        })
    }
}

impl WriteXdr for UpgradeEntryMeta {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.upgrade.write_xdr(w)?;
        self.changes.write_xdr(w)?;
        Ok(())
    }
}

// LedgerCloseMetaV0 is an XDR Struct defines as:
//
//   struct LedgerCloseMetaV0
//    {
//        LedgerHeaderHistoryEntry ledgerHeader;
//        // NB: txSet is sorted in "Hash order"
//        TransactionSet txSet;
//
//        // NB: transactions are sorted in apply order here
//        // fees for all transactions are processed first
//        // followed by applying transactions
//        TransactionResultMeta txProcessing<>;
//
//        // upgrades are applied last
//        UpgradeEntryMeta upgradesProcessing<>;
//
//        // other misc information attached to the ledger close
//        SCPHistoryEntry scpInfo<>;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct LedgerCloseMetaV0 {
    pub ledger_header: LedgerHeaderHistoryEntry,
    pub tx_set: TransactionSet,
    pub tx_processing: VecM<TransactionResultMeta>,
    pub upgrades_processing: VecM<UpgradeEntryMeta>,
    pub scp_info: VecM<ScpHistoryEntry>,
}

impl ReadXdr for LedgerCloseMetaV0 {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            ledger_header: LedgerHeaderHistoryEntry::read_xdr(r)?,
            tx_set: TransactionSet::read_xdr(r)?,
            tx_processing: VecM::<TransactionResultMeta>::read_xdr(r)?,
            upgrades_processing: VecM::<UpgradeEntryMeta>::read_xdr(r)?,
            scp_info: VecM::<ScpHistoryEntry>::read_xdr(r)?,
        })
    }
}

impl WriteXdr for LedgerCloseMetaV0 {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.ledger_header.write_xdr(w)?;
        self.tx_set.write_xdr(w)?;
        self.tx_processing.write_xdr(w)?;
        self.upgrades_processing.write_xdr(w)?;
        self.scp_info.write_xdr(w)?;
        Ok(())
    }
}

// LedgerCloseMeta is an XDR Union defines as:
//
//   union LedgerCloseMeta switch (int v)
//    {
//    case 0:
//        LedgerCloseMetaV0 v0;
//    };
//
// union with discriminant i32
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum LedgerCloseMeta {
    V0(LedgerCloseMetaV0),
}

impl LedgerCloseMeta {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0(_) => "V0",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> i32 {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0(_) => 0,
        }
    }
}

impl ReadXdr for LedgerCloseMeta {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: i32 = <i32 as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            0 => Self::V0(LedgerCloseMetaV0::read_xdr(r)?),
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for LedgerCloseMeta {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// ErrorCode is an XDR Enum defines as:
//
//   enum ErrorCode
//    {
//        ERR_MISC = 0, // Unspecific error
//        ERR_DATA = 1, // Malformed data
//        ERR_CONF = 2, // Misconfiguration error
//        ERR_AUTH = 3, // Authentication failure
//        ERR_LOAD = 4  // System overloaded
//    };
//
// enum
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum ErrorCode {
    Misc = 0,
    Data = 1,
    Conf = 2,
    Auth = 3,
    Load = 4,
}

impl ErrorCode {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Misc => "Misc",
            Self::Data => "Data",
            Self::Conf => "Conf",
            Self::Auth => "Auth",
            Self::Load => "Load",
        }
    }
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for ErrorCode {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self> {
        let e = match i {
            0 => ErrorCode::Misc,
            1 => ErrorCode::Data,
            2 => ErrorCode::Conf,
            3 => ErrorCode::Auth,
            4 => ErrorCode::Load,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<ErrorCode> for i32 {
    #[must_use]
    fn from(e: ErrorCode) -> Self {
        e as Self
    }
}

impl ReadXdr for ErrorCode {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXdr for ErrorCode {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// SError is an XDR Struct defines as:
//
//   struct Error
//    {
//        ErrorCode code;
//        string msg<100>;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SError {
    pub code: ErrorCode,
    pub msg: VecM<u8, 100>,
}

impl ReadXdr for SError {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            code: ErrorCode::read_xdr(r)?,
            msg: VecM::<u8, 100>::read_xdr(r)?,
        })
    }
}

impl WriteXdr for SError {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.code.write_xdr(w)?;
        self.msg.write_xdr(w)?;
        Ok(())
    }
}

// SendMore is an XDR Struct defines as:
//
//   struct SendMore
//    {
//        uint32 numMessages;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SendMore {
    pub num_messages: u32,
}

impl ReadXdr for SendMore {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            num_messages: u32::read_xdr(r)?,
        })
    }
}

impl WriteXdr for SendMore {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.num_messages.write_xdr(w)?;
        Ok(())
    }
}

// AuthCert is an XDR Struct defines as:
//
//   struct AuthCert
//    {
//        Curve25519Public pubkey;
//        uint64 expiration;
//        Signature sig;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct AuthCert {
    pub pubkey: Curve25519Public,
    pub expiration: u64,
    pub sig: Signature,
}

impl ReadXdr for AuthCert {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            pubkey: Curve25519Public::read_xdr(r)?,
            expiration: u64::read_xdr(r)?,
            sig: Signature::read_xdr(r)?,
        })
    }
}

impl WriteXdr for AuthCert {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.pubkey.write_xdr(w)?;
        self.expiration.write_xdr(w)?;
        self.sig.write_xdr(w)?;
        Ok(())
    }
}

// Hello is an XDR Struct defines as:
//
//   struct Hello
//    {
//        uint32 ledgerVersion;
//        uint32 overlayVersion;
//        uint32 overlayMinVersion;
//        Hash networkID;
//        string versionStr<100>;
//        int listeningPort;
//        NodeID peerID;
//        AuthCert cert;
//        uint256 nonce;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Hello {
    pub ledger_version: u32,
    pub overlay_version: u32,
    pub overlay_min_version: u32,
    pub network_id: Hash,
    pub version_str: VecM<u8, 100>,
    pub listening_port: i32,
    pub peer_id: NodeId,
    pub cert: AuthCert,
    pub nonce: Uint256,
}

impl ReadXdr for Hello {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            ledger_version: u32::read_xdr(r)?,
            overlay_version: u32::read_xdr(r)?,
            overlay_min_version: u32::read_xdr(r)?,
            network_id: Hash::read_xdr(r)?,
            version_str: VecM::<u8, 100>::read_xdr(r)?,
            listening_port: i32::read_xdr(r)?,
            peer_id: NodeId::read_xdr(r)?,
            cert: AuthCert::read_xdr(r)?,
            nonce: Uint256::read_xdr(r)?,
        })
    }
}

impl WriteXdr for Hello {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.ledger_version.write_xdr(w)?;
        self.overlay_version.write_xdr(w)?;
        self.overlay_min_version.write_xdr(w)?;
        self.network_id.write_xdr(w)?;
        self.version_str.write_xdr(w)?;
        self.listening_port.write_xdr(w)?;
        self.peer_id.write_xdr(w)?;
        self.cert.write_xdr(w)?;
        self.nonce.write_xdr(w)?;
        Ok(())
    }
}

// Auth is an XDR Struct defines as:
//
//   struct Auth
//    {
//        // Empty message, just to confirm
//        // establishment of MAC keys.
//        int unused;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Auth {
    pub unused: i32,
}

impl ReadXdr for Auth {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            unused: i32::read_xdr(r)?,
        })
    }
}

impl WriteXdr for Auth {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.unused.write_xdr(w)?;
        Ok(())
    }
}

// IpAddrType is an XDR Enum defines as:
//
//   enum IPAddrType
//    {
//        IPv4 = 0,
//        IPv6 = 1
//    };
//
// enum
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum IpAddrType {
    IPv4 = 0,
    IPv6 = 1,
}

impl IpAddrType {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::IPv4 => "IPv4",
            Self::IPv6 => "IPv6",
        }
    }
}

impl fmt::Display for IpAddrType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for IpAddrType {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self> {
        let e = match i {
            0 => IpAddrType::IPv4,
            1 => IpAddrType::IPv6,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<IpAddrType> for i32 {
    #[must_use]
    fn from(e: IpAddrType) -> Self {
        e as Self
    }
}

impl ReadXdr for IpAddrType {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXdr for IpAddrType {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// PeerAddressIp is an XDR NestedUnion defines as:
//
//   union switch (IPAddrType type)
//        {
//        case IPv4:
//            opaque ipv4[4];
//        case IPv6:
//            opaque ipv6[16];
//        }
//
// union with discriminant IpAddrType
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum PeerAddressIp {
    IPv4([u8; 4]),
    IPv6([u8; 16]),
}

impl PeerAddressIp {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::IPv4(_) => "IPv4",
            Self::IPv6(_) => "IPv6",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> IpAddrType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::IPv4(_) => IpAddrType::IPv4,
            Self::IPv6(_) => IpAddrType::IPv6,
        }
    }
}

impl ReadXdr for PeerAddressIp {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: IpAddrType = <IpAddrType as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            IpAddrType::IPv4 => Self::IPv4(<[u8; 4]>::read_xdr(r)?),
            IpAddrType::IPv6 => Self::IPv6(<[u8; 16]>::read_xdr(r)?),
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for PeerAddressIp {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::IPv4(v) => v.write_xdr(w)?,
            Self::IPv6(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// PeerAddress is an XDR Struct defines as:
//
//   struct PeerAddress
//    {
//        union switch (IPAddrType type)
//        {
//        case IPv4:
//            opaque ipv4[4];
//        case IPv6:
//            opaque ipv6[16];
//        }
//        ip;
//        uint32 port;
//        uint32 numFailures;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct PeerAddress {
    pub ip: PeerAddressIp,
    pub port: u32,
    pub num_failures: u32,
}

impl ReadXdr for PeerAddress {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            ip: PeerAddressIp::read_xdr(r)?,
            port: u32::read_xdr(r)?,
            num_failures: u32::read_xdr(r)?,
        })
    }
}

impl WriteXdr for PeerAddress {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.ip.write_xdr(w)?;
        self.port.write_xdr(w)?;
        self.num_failures.write_xdr(w)?;
        Ok(())
    }
}

// MessageType is an XDR Enum defines as:
//
//   enum MessageType
//    {
//        ERROR_MSG = 0,
//        AUTH = 2,
//        DONT_HAVE = 3,
//
//        GET_PEERS = 4, // gets a list of peers this guy knows about
//        PEERS = 5,
//
//        GET_TX_SET = 6, // gets a particular txset by hash
//        TX_SET = 7,
//
//        TRANSACTION = 8, // pass on a tx you have heard about
//
//        // SCP
//        GET_SCP_QUORUMSET = 9,
//        SCP_QUORUMSET = 10,
//        SCP_MESSAGE = 11,
//        GET_SCP_STATE = 12,
//
//        // new messages
//        HELLO = 13,
//
//        SURVEY_REQUEST = 14,
//        SURVEY_RESPONSE = 15,
//
//        SEND_MORE = 16
//    };
//
// enum
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum MessageType {
    ErrorMsg = 0,
    Auth = 2,
    DontHave = 3,
    GetPeers = 4,
    Peers = 5,
    GetTxSet = 6,
    TxSet = 7,
    Transaction = 8,
    GetScpQuorumset = 9,
    ScpQuorumset = 10,
    ScpMessage = 11,
    GetScpState = 12,
    Hello = 13,
    SurveyRequest = 14,
    SurveyResponse = 15,
    SendMore = 16,
}

impl MessageType {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::ErrorMsg => "ErrorMsg",
            Self::Auth => "Auth",
            Self::DontHave => "DontHave",
            Self::GetPeers => "GetPeers",
            Self::Peers => "Peers",
            Self::GetTxSet => "GetTxSet",
            Self::TxSet => "TxSet",
            Self::Transaction => "Transaction",
            Self::GetScpQuorumset => "GetScpQuorumset",
            Self::ScpQuorumset => "ScpQuorumset",
            Self::ScpMessage => "ScpMessage",
            Self::GetScpState => "GetScpState",
            Self::Hello => "Hello",
            Self::SurveyRequest => "SurveyRequest",
            Self::SurveyResponse => "SurveyResponse",
            Self::SendMore => "SendMore",
        }
    }
}

impl fmt::Display for MessageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for MessageType {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self> {
        let e = match i {
            0 => MessageType::ErrorMsg,
            2 => MessageType::Auth,
            3 => MessageType::DontHave,
            4 => MessageType::GetPeers,
            5 => MessageType::Peers,
            6 => MessageType::GetTxSet,
            7 => MessageType::TxSet,
            8 => MessageType::Transaction,
            9 => MessageType::GetScpQuorumset,
            10 => MessageType::ScpQuorumset,
            11 => MessageType::ScpMessage,
            12 => MessageType::GetScpState,
            13 => MessageType::Hello,
            14 => MessageType::SurveyRequest,
            15 => MessageType::SurveyResponse,
            16 => MessageType::SendMore,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<MessageType> for i32 {
    #[must_use]
    fn from(e: MessageType) -> Self {
        e as Self
    }
}

impl ReadXdr for MessageType {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXdr for MessageType {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// DontHave is an XDR Struct defines as:
//
//   struct DontHave
//    {
//        MessageType type;
//        uint256 reqHash;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct DontHave {
    pub type_: MessageType,
    pub req_hash: Uint256,
}

impl ReadXdr for DontHave {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            type_: MessageType::read_xdr(r)?,
            req_hash: Uint256::read_xdr(r)?,
        })
    }
}

impl WriteXdr for DontHave {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.type_.write_xdr(w)?;
        self.req_hash.write_xdr(w)?;
        Ok(())
    }
}

// SurveyMessageCommandType is an XDR Enum defines as:
//
//   enum SurveyMessageCommandType
//    {
//        SURVEY_TOPOLOGY = 0
//    };
//
// enum
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum SurveyMessageCommandType {
    SurveyTopology = 0,
}

impl SurveyMessageCommandType {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::SurveyTopology => "SurveyTopology",
        }
    }
}

impl fmt::Display for SurveyMessageCommandType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for SurveyMessageCommandType {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self> {
        let e = match i {
            0 => SurveyMessageCommandType::SurveyTopology,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<SurveyMessageCommandType> for i32 {
    #[must_use]
    fn from(e: SurveyMessageCommandType) -> Self {
        e as Self
    }
}

impl ReadXdr for SurveyMessageCommandType {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXdr for SurveyMessageCommandType {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// SurveyRequestMessage is an XDR Struct defines as:
//
//   struct SurveyRequestMessage
//    {
//        NodeID surveyorPeerID;
//        NodeID surveyedPeerID;
//        uint32 ledgerNum;
//        Curve25519Public encryptionKey;
//        SurveyMessageCommandType commandType;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SurveyRequestMessage {
    pub surveyor_peer_id: NodeId,
    pub surveyed_peer_id: NodeId,
    pub ledger_num: u32,
    pub encryption_key: Curve25519Public,
    pub command_type: SurveyMessageCommandType,
}

impl ReadXdr for SurveyRequestMessage {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            surveyor_peer_id: NodeId::read_xdr(r)?,
            surveyed_peer_id: NodeId::read_xdr(r)?,
            ledger_num: u32::read_xdr(r)?,
            encryption_key: Curve25519Public::read_xdr(r)?,
            command_type: SurveyMessageCommandType::read_xdr(r)?,
        })
    }
}

impl WriteXdr for SurveyRequestMessage {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.surveyor_peer_id.write_xdr(w)?;
        self.surveyed_peer_id.write_xdr(w)?;
        self.ledger_num.write_xdr(w)?;
        self.encryption_key.write_xdr(w)?;
        self.command_type.write_xdr(w)?;
        Ok(())
    }
}

// SignedSurveyRequestMessage is an XDR Struct defines as:
//
//   struct SignedSurveyRequestMessage
//    {
//        Signature requestSignature;
//        SurveyRequestMessage request;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SignedSurveyRequestMessage {
    pub request_signature: Signature,
    pub request: SurveyRequestMessage,
}

impl ReadXdr for SignedSurveyRequestMessage {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            request_signature: Signature::read_xdr(r)?,
            request: SurveyRequestMessage::read_xdr(r)?,
        })
    }
}

impl WriteXdr for SignedSurveyRequestMessage {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.request_signature.write_xdr(w)?;
        self.request.write_xdr(w)?;
        Ok(())
    }
}

// EncryptedBody is an XDR Typedef defines as:
//
//   typedef opaque EncryptedBody<64000>;
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct EncryptedBody(pub VecM<u8, 64000>);

impl From<EncryptedBody> for VecM<u8, 64000> {
    #[must_use]
    fn from(x: EncryptedBody) -> Self {
        x.0
    }
}

impl From<VecM<u8, 64000>> for EncryptedBody {
    #[must_use]
    fn from(x: VecM<u8, 64000>) -> Self {
        EncryptedBody(x)
    }
}

impl AsRef<VecM<u8, 64000>> for EncryptedBody {
    #[must_use]
    fn as_ref(&self) -> &VecM<u8, 64000> {
        &self.0
    }
}

impl ReadXdr for EncryptedBody {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let i = VecM::<u8, 64000>::read_xdr(r)?;
        let v = EncryptedBody(i);
        Ok(v)
    }
}

impl WriteXdr for EncryptedBody {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.0.write_xdr(w)
    }
}

impl Deref for EncryptedBody {
    type Target = VecM<u8, 64000>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<EncryptedBody> for Vec<u8> {
    #[must_use]
    fn from(x: EncryptedBody) -> Self {
        x.0 .0
    }
}

impl TryFrom<Vec<u8>> for EncryptedBody {
    type Error = Error;
    fn try_from(x: Vec<u8>) -> Result<Self> {
        Ok(EncryptedBody(x.try_into()?))
    }
}

impl AsRef<Vec<u8>> for EncryptedBody {
    #[must_use]
    fn as_ref(&self) -> &Vec<u8> {
        &self.0 .0
    }
}

impl AsRef<[u8]> for EncryptedBody {
    #[cfg(feature = "alloc")]
    #[must_use]
    fn as_ref(&self) -> &[u8] {
        &self.0 .0
    }
    #[cfg(not(feature = "alloc"))]
    #[must_use]
    fn as_ref(&self) -> &[u8] {
        self.0 .0
    }
}

// SurveyResponseMessage is an XDR Struct defines as:
//
//   struct SurveyResponseMessage
//    {
//        NodeID surveyorPeerID;
//        NodeID surveyedPeerID;
//        uint32 ledgerNum;
//        SurveyMessageCommandType commandType;
//        EncryptedBody encryptedBody;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SurveyResponseMessage {
    pub surveyor_peer_id: NodeId,
    pub surveyed_peer_id: NodeId,
    pub ledger_num: u32,
    pub command_type: SurveyMessageCommandType,
    pub encrypted_body: EncryptedBody,
}

impl ReadXdr for SurveyResponseMessage {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            surveyor_peer_id: NodeId::read_xdr(r)?,
            surveyed_peer_id: NodeId::read_xdr(r)?,
            ledger_num: u32::read_xdr(r)?,
            command_type: SurveyMessageCommandType::read_xdr(r)?,
            encrypted_body: EncryptedBody::read_xdr(r)?,
        })
    }
}

impl WriteXdr for SurveyResponseMessage {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.surveyor_peer_id.write_xdr(w)?;
        self.surveyed_peer_id.write_xdr(w)?;
        self.ledger_num.write_xdr(w)?;
        self.command_type.write_xdr(w)?;
        self.encrypted_body.write_xdr(w)?;
        Ok(())
    }
}

// SignedSurveyResponseMessage is an XDR Struct defines as:
//
//   struct SignedSurveyResponseMessage
//    {
//        Signature responseSignature;
//        SurveyResponseMessage response;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SignedSurveyResponseMessage {
    pub response_signature: Signature,
    pub response: SurveyResponseMessage,
}

impl ReadXdr for SignedSurveyResponseMessage {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            response_signature: Signature::read_xdr(r)?,
            response: SurveyResponseMessage::read_xdr(r)?,
        })
    }
}

impl WriteXdr for SignedSurveyResponseMessage {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.response_signature.write_xdr(w)?;
        self.response.write_xdr(w)?;
        Ok(())
    }
}

// PeerStats is an XDR Struct defines as:
//
//   struct PeerStats
//    {
//        NodeID id;
//        string versionStr<100>;
//        uint64 messagesRead;
//        uint64 messagesWritten;
//        uint64 bytesRead;
//        uint64 bytesWritten;
//        uint64 secondsConnected;
//
//        uint64 uniqueFloodBytesRecv;
//        uint64 duplicateFloodBytesRecv;
//        uint64 uniqueFetchBytesRecv;
//        uint64 duplicateFetchBytesRecv;
//
//        uint64 uniqueFloodMessageRecv;
//        uint64 duplicateFloodMessageRecv;
//        uint64 uniqueFetchMessageRecv;
//        uint64 duplicateFetchMessageRecv;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct PeerStats {
    pub id: NodeId,
    pub version_str: VecM<u8, 100>,
    pub messages_read: u64,
    pub messages_written: u64,
    pub bytes_read: u64,
    pub bytes_written: u64,
    pub seconds_connected: u64,
    pub unique_flood_bytes_recv: u64,
    pub duplicate_flood_bytes_recv: u64,
    pub unique_fetch_bytes_recv: u64,
    pub duplicate_fetch_bytes_recv: u64,
    pub unique_flood_message_recv: u64,
    pub duplicate_flood_message_recv: u64,
    pub unique_fetch_message_recv: u64,
    pub duplicate_fetch_message_recv: u64,
}

impl ReadXdr for PeerStats {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            id: NodeId::read_xdr(r)?,
            version_str: VecM::<u8, 100>::read_xdr(r)?,
            messages_read: u64::read_xdr(r)?,
            messages_written: u64::read_xdr(r)?,
            bytes_read: u64::read_xdr(r)?,
            bytes_written: u64::read_xdr(r)?,
            seconds_connected: u64::read_xdr(r)?,
            unique_flood_bytes_recv: u64::read_xdr(r)?,
            duplicate_flood_bytes_recv: u64::read_xdr(r)?,
            unique_fetch_bytes_recv: u64::read_xdr(r)?,
            duplicate_fetch_bytes_recv: u64::read_xdr(r)?,
            unique_flood_message_recv: u64::read_xdr(r)?,
            duplicate_flood_message_recv: u64::read_xdr(r)?,
            unique_fetch_message_recv: u64::read_xdr(r)?,
            duplicate_fetch_message_recv: u64::read_xdr(r)?,
        })
    }
}

impl WriteXdr for PeerStats {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.id.write_xdr(w)?;
        self.version_str.write_xdr(w)?;
        self.messages_read.write_xdr(w)?;
        self.messages_written.write_xdr(w)?;
        self.bytes_read.write_xdr(w)?;
        self.bytes_written.write_xdr(w)?;
        self.seconds_connected.write_xdr(w)?;
        self.unique_flood_bytes_recv.write_xdr(w)?;
        self.duplicate_flood_bytes_recv.write_xdr(w)?;
        self.unique_fetch_bytes_recv.write_xdr(w)?;
        self.duplicate_fetch_bytes_recv.write_xdr(w)?;
        self.unique_flood_message_recv.write_xdr(w)?;
        self.duplicate_flood_message_recv.write_xdr(w)?;
        self.unique_fetch_message_recv.write_xdr(w)?;
        self.duplicate_fetch_message_recv.write_xdr(w)?;
        Ok(())
    }
}

// PeerStatList is an XDR Typedef defines as:
//
//   typedef PeerStats PeerStatList<25>;
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct PeerStatList(pub VecM<PeerStats, 25>);

impl From<PeerStatList> for VecM<PeerStats, 25> {
    #[must_use]
    fn from(x: PeerStatList) -> Self {
        x.0
    }
}

impl From<VecM<PeerStats, 25>> for PeerStatList {
    #[must_use]
    fn from(x: VecM<PeerStats, 25>) -> Self {
        PeerStatList(x)
    }
}

impl AsRef<VecM<PeerStats, 25>> for PeerStatList {
    #[must_use]
    fn as_ref(&self) -> &VecM<PeerStats, 25> {
        &self.0
    }
}

impl ReadXdr for PeerStatList {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let i = VecM::<PeerStats, 25>::read_xdr(r)?;
        let v = PeerStatList(i);
        Ok(v)
    }
}

impl WriteXdr for PeerStatList {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.0.write_xdr(w)
    }
}

impl Deref for PeerStatList {
    type Target = VecM<PeerStats, 25>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<PeerStatList> for Vec<PeerStats> {
    #[must_use]
    fn from(x: PeerStatList) -> Self {
        x.0 .0
    }
}

impl TryFrom<Vec<PeerStats>> for PeerStatList {
    type Error = Error;
    fn try_from(x: Vec<PeerStats>) -> Result<Self> {
        Ok(PeerStatList(x.try_into()?))
    }
}

impl AsRef<Vec<PeerStats>> for PeerStatList {
    #[must_use]
    fn as_ref(&self) -> &Vec<PeerStats> {
        &self.0 .0
    }
}

impl AsRef<[PeerStats]> for PeerStatList {
    #[cfg(feature = "alloc")]
    #[must_use]
    fn as_ref(&self) -> &[PeerStats] {
        &self.0 .0
    }
    #[cfg(not(feature = "alloc"))]
    #[must_use]
    fn as_ref(&self) -> &[PeerStats] {
        self.0 .0
    }
}

// TopologyResponseBody is an XDR Struct defines as:
//
//   struct TopologyResponseBody
//    {
//        PeerStatList inboundPeers;
//        PeerStatList outboundPeers;
//
//        uint32 totalInboundPeerCount;
//        uint32 totalOutboundPeerCount;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TopologyResponseBody {
    pub inbound_peers: PeerStatList,
    pub outbound_peers: PeerStatList,
    pub total_inbound_peer_count: u32,
    pub total_outbound_peer_count: u32,
}

impl ReadXdr for TopologyResponseBody {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            inbound_peers: PeerStatList::read_xdr(r)?,
            outbound_peers: PeerStatList::read_xdr(r)?,
            total_inbound_peer_count: u32::read_xdr(r)?,
            total_outbound_peer_count: u32::read_xdr(r)?,
        })
    }
}

impl WriteXdr for TopologyResponseBody {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.inbound_peers.write_xdr(w)?;
        self.outbound_peers.write_xdr(w)?;
        self.total_inbound_peer_count.write_xdr(w)?;
        self.total_outbound_peer_count.write_xdr(w)?;
        Ok(())
    }
}

// SurveyResponseBody is an XDR Union defines as:
//
//   union SurveyResponseBody switch (SurveyMessageCommandType type)
//    {
//    case SURVEY_TOPOLOGY:
//        TopologyResponseBody topologyResponseBody;
//    };
//
// union with discriminant SurveyMessageCommandType
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum SurveyResponseBody {
    SurveyTopology(TopologyResponseBody),
}

impl SurveyResponseBody {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::SurveyTopology(_) => "SurveyTopology",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> SurveyMessageCommandType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::SurveyTopology(_) => SurveyMessageCommandType::SurveyTopology,
        }
    }
}

impl ReadXdr for SurveyResponseBody {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: SurveyMessageCommandType = <SurveyMessageCommandType as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            SurveyMessageCommandType::SurveyTopology => {
                Self::SurveyTopology(TopologyResponseBody::read_xdr(r)?)
            }
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for SurveyResponseBody {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::SurveyTopology(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// StellarMessage is an XDR Union defines as:
//
//   union StellarMessage switch (MessageType type)
//    {
//    case ERROR_MSG:
//        Error error;
//    case HELLO:
//        Hello hello;
//    case AUTH:
//        Auth auth;
//    case DONT_HAVE:
//        DontHave dontHave;
//    case GET_PEERS:
//        void;
//    case PEERS:
//        PeerAddress peers<100>;
//
//    case GET_TX_SET:
//        uint256 txSetHash;
//    case TX_SET:
//        TransactionSet txSet;
//
//    case TRANSACTION:
//        TransactionEnvelope transaction;
//
//    case SURVEY_REQUEST:
//        SignedSurveyRequestMessage signedSurveyRequestMessage;
//
//    case SURVEY_RESPONSE:
//        SignedSurveyResponseMessage signedSurveyResponseMessage;
//
//    // SCP
//    case GET_SCP_QUORUMSET:
//        uint256 qSetHash;
//    case SCP_QUORUMSET:
//        SCPQuorumSet qSet;
//    case SCP_MESSAGE:
//        SCPEnvelope envelope;
//    case GET_SCP_STATE:
//        uint32 getSCPLedgerSeq; // ledger seq requested ; if 0, requests the latest
//    case SEND_MORE:
//        SendMore sendMoreMessage;
//    };
//
// union with discriminant MessageType
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum StellarMessage {
    ErrorMsg(SError),
    Hello(Hello),
    Auth(Auth),
    DontHave(DontHave),
    GetPeers,
    Peers(VecM<PeerAddress, 100>),
    GetTxSet(Uint256),
    TxSet(TransactionSet),
    Transaction(TransactionEnvelope),
    SurveyRequest(SignedSurveyRequestMessage),
    SurveyResponse(SignedSurveyResponseMessage),
    GetScpQuorumset(Uint256),
    ScpQuorumset(ScpQuorumSet),
    ScpMessage(ScpEnvelope),
    GetScpState(u32),
    SendMore(SendMore),
}

impl StellarMessage {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::ErrorMsg(_) => "ErrorMsg",
            Self::Hello(_) => "Hello",
            Self::Auth(_) => "Auth",
            Self::DontHave(_) => "DontHave",
            Self::GetPeers => "GetPeers",
            Self::Peers(_) => "Peers",
            Self::GetTxSet(_) => "GetTxSet",
            Self::TxSet(_) => "TxSet",
            Self::Transaction(_) => "Transaction",
            Self::SurveyRequest(_) => "SurveyRequest",
            Self::SurveyResponse(_) => "SurveyResponse",
            Self::GetScpQuorumset(_) => "GetScpQuorumset",
            Self::ScpQuorumset(_) => "ScpQuorumset",
            Self::ScpMessage(_) => "ScpMessage",
            Self::GetScpState(_) => "GetScpState",
            Self::SendMore(_) => "SendMore",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> MessageType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::ErrorMsg(_) => MessageType::ErrorMsg,
            Self::Hello(_) => MessageType::Hello,
            Self::Auth(_) => MessageType::Auth,
            Self::DontHave(_) => MessageType::DontHave,
            Self::GetPeers => MessageType::GetPeers,
            Self::Peers(_) => MessageType::Peers,
            Self::GetTxSet(_) => MessageType::GetTxSet,
            Self::TxSet(_) => MessageType::TxSet,
            Self::Transaction(_) => MessageType::Transaction,
            Self::SurveyRequest(_) => MessageType::SurveyRequest,
            Self::SurveyResponse(_) => MessageType::SurveyResponse,
            Self::GetScpQuorumset(_) => MessageType::GetScpQuorumset,
            Self::ScpQuorumset(_) => MessageType::ScpQuorumset,
            Self::ScpMessage(_) => MessageType::ScpMessage,
            Self::GetScpState(_) => MessageType::GetScpState,
            Self::SendMore(_) => MessageType::SendMore,
        }
    }
}

impl ReadXdr for StellarMessage {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: MessageType = <MessageType as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            MessageType::ErrorMsg => Self::ErrorMsg(SError::read_xdr(r)?),
            MessageType::Hello => Self::Hello(Hello::read_xdr(r)?),
            MessageType::Auth => Self::Auth(Auth::read_xdr(r)?),
            MessageType::DontHave => Self::DontHave(DontHave::read_xdr(r)?),
            MessageType::GetPeers => Self::GetPeers,
            MessageType::Peers => Self::Peers(VecM::<PeerAddress, 100>::read_xdr(r)?),
            MessageType::GetTxSet => Self::GetTxSet(Uint256::read_xdr(r)?),
            MessageType::TxSet => Self::TxSet(TransactionSet::read_xdr(r)?),
            MessageType::Transaction => Self::Transaction(TransactionEnvelope::read_xdr(r)?),
            MessageType::SurveyRequest => {
                Self::SurveyRequest(SignedSurveyRequestMessage::read_xdr(r)?)
            }
            MessageType::SurveyResponse => {
                Self::SurveyResponse(SignedSurveyResponseMessage::read_xdr(r)?)
            }
            MessageType::GetScpQuorumset => Self::GetScpQuorumset(Uint256::read_xdr(r)?),
            MessageType::ScpQuorumset => Self::ScpQuorumset(ScpQuorumSet::read_xdr(r)?),
            MessageType::ScpMessage => Self::ScpMessage(ScpEnvelope::read_xdr(r)?),
            MessageType::GetScpState => Self::GetScpState(u32::read_xdr(r)?),
            MessageType::SendMore => Self::SendMore(SendMore::read_xdr(r)?),
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for StellarMessage {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::ErrorMsg(v) => v.write_xdr(w)?,
            Self::Hello(v) => v.write_xdr(w)?,
            Self::Auth(v) => v.write_xdr(w)?,
            Self::DontHave(v) => v.write_xdr(w)?,
            Self::GetPeers => ().write_xdr(w)?,
            Self::Peers(v) => v.write_xdr(w)?,
            Self::GetTxSet(v) => v.write_xdr(w)?,
            Self::TxSet(v) => v.write_xdr(w)?,
            Self::Transaction(v) => v.write_xdr(w)?,
            Self::SurveyRequest(v) => v.write_xdr(w)?,
            Self::SurveyResponse(v) => v.write_xdr(w)?,
            Self::GetScpQuorumset(v) => v.write_xdr(w)?,
            Self::ScpQuorumset(v) => v.write_xdr(w)?,
            Self::ScpMessage(v) => v.write_xdr(w)?,
            Self::GetScpState(v) => v.write_xdr(w)?,
            Self::SendMore(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// AuthenticatedMessageV0 is an XDR NestedStruct defines as:
//
//   struct
//        {
//            uint64 sequence;
//            StellarMessage message;
//            HmacSha256Mac mac;
//        }
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct AuthenticatedMessageV0 {
    pub sequence: u64,
    pub message: StellarMessage,
    pub mac: HmacSha256Mac,
}

impl ReadXdr for AuthenticatedMessageV0 {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            sequence: u64::read_xdr(r)?,
            message: StellarMessage::read_xdr(r)?,
            mac: HmacSha256Mac::read_xdr(r)?,
        })
    }
}

impl WriteXdr for AuthenticatedMessageV0 {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.sequence.write_xdr(w)?;
        self.message.write_xdr(w)?;
        self.mac.write_xdr(w)?;
        Ok(())
    }
}

// AuthenticatedMessage is an XDR Union defines as:
//
//   union AuthenticatedMessage switch (uint32 v)
//    {
//    case 0:
//        struct
//        {
//            uint64 sequence;
//            StellarMessage message;
//            HmacSha256Mac mac;
//        } v0;
//    };
//
// union with discriminant u32
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum AuthenticatedMessage {
    V0(AuthenticatedMessageV0),
}

impl AuthenticatedMessage {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0(_) => "V0",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> u32 {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0(_) => 0,
        }
    }
}

impl ReadXdr for AuthenticatedMessage {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: u32 = <u32 as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            0 => Self::V0(AuthenticatedMessageV0::read_xdr(r)?),
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for AuthenticatedMessage {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// LiquidityPoolParameters is an XDR Union defines as:
//
//   union LiquidityPoolParameters switch (LiquidityPoolType type)
//    {
//    case LIQUIDITY_POOL_CONSTANT_PRODUCT:
//        LiquidityPoolConstantProductParameters constantProduct;
//    };
//
// union with discriminant LiquidityPoolType
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum LiquidityPoolParameters {
    LiquidityPoolConstantProduct(LiquidityPoolConstantProductParameters),
}

impl LiquidityPoolParameters {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::LiquidityPoolConstantProduct(_) => "LiquidityPoolConstantProduct",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> LiquidityPoolType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::LiquidityPoolConstantProduct(_) => {
                LiquidityPoolType::LiquidityPoolConstantProduct
            }
        }
    }
}

impl ReadXdr for LiquidityPoolParameters {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: LiquidityPoolType = <LiquidityPoolType as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            LiquidityPoolType::LiquidityPoolConstantProduct => Self::LiquidityPoolConstantProduct(
                LiquidityPoolConstantProductParameters::read_xdr(r)?,
            ),
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for LiquidityPoolParameters {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::LiquidityPoolConstantProduct(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// MuxedAccountMed25519 is an XDR NestedStruct defines as:
//
//   struct
//        {
//            uint64 id;
//            uint256 ed25519;
//        }
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct MuxedAccountMed25519 {
    pub id: u64,
    pub ed25519: Uint256,
}

impl ReadXdr for MuxedAccountMed25519 {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            id: u64::read_xdr(r)?,
            ed25519: Uint256::read_xdr(r)?,
        })
    }
}

impl WriteXdr for MuxedAccountMed25519 {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.id.write_xdr(w)?;
        self.ed25519.write_xdr(w)?;
        Ok(())
    }
}

// MuxedAccount is an XDR Union defines as:
//
//   union MuxedAccount switch (CryptoKeyType type)
//    {
//    case KEY_TYPE_ED25519:
//        uint256 ed25519;
//    case KEY_TYPE_MUXED_ED25519:
//        struct
//        {
//            uint64 id;
//            uint256 ed25519;
//        } med25519;
//    };
//
// union with discriminant CryptoKeyType
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum MuxedAccount {
    Ed25519(Uint256),
    MuxedEd25519(MuxedAccountMed25519),
}

impl MuxedAccount {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Ed25519(_) => "Ed25519",
            Self::MuxedEd25519(_) => "MuxedEd25519",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> CryptoKeyType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Ed25519(_) => CryptoKeyType::Ed25519,
            Self::MuxedEd25519(_) => CryptoKeyType::MuxedEd25519,
        }
    }
}

impl ReadXdr for MuxedAccount {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: CryptoKeyType = <CryptoKeyType as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            CryptoKeyType::Ed25519 => Self::Ed25519(Uint256::read_xdr(r)?),
            CryptoKeyType::MuxedEd25519 => Self::MuxedEd25519(MuxedAccountMed25519::read_xdr(r)?),
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for MuxedAccount {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Ed25519(v) => v.write_xdr(w)?,
            Self::MuxedEd25519(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// DecoratedSignature is an XDR Struct defines as:
//
//   struct DecoratedSignature
//    {
//        SignatureHint hint;  // last 4 bytes of the public key, used as a hint
//        Signature signature; // actual signature
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct DecoratedSignature {
    pub hint: SignatureHint,
    pub signature: Signature,
}

impl ReadXdr for DecoratedSignature {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            hint: SignatureHint::read_xdr(r)?,
            signature: Signature::read_xdr(r)?,
        })
    }
}

impl WriteXdr for DecoratedSignature {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.hint.write_xdr(w)?;
        self.signature.write_xdr(w)?;
        Ok(())
    }
}

// LedgerFootprint is an XDR Struct defines as:
//
//   struct LedgerFootprint
//    {
//        LedgerKey readOnly<>;
//        LedgerKey readWrite<>;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct LedgerFootprint {
    pub read_only: VecM<LedgerKey>,
    pub read_write: VecM<LedgerKey>,
}

impl ReadXdr for LedgerFootprint {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            read_only: VecM::<LedgerKey>::read_xdr(r)?,
            read_write: VecM::<LedgerKey>::read_xdr(r)?,
        })
    }
}

impl WriteXdr for LedgerFootprint {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.read_only.write_xdr(w)?;
        self.read_write.write_xdr(w)?;
        Ok(())
    }
}

// OperationType is an XDR Enum defines as:
//
//   enum OperationType
//    {
//        CREATE_ACCOUNT = 0,
//        PAYMENT = 1,
//        PATH_PAYMENT_STRICT_RECEIVE = 2,
//        MANAGE_SELL_OFFER = 3,
//        CREATE_PASSIVE_SELL_OFFER = 4,
//        SET_OPTIONS = 5,
//        CHANGE_TRUST = 6,
//        ALLOW_TRUST = 7,
//        ACCOUNT_MERGE = 8,
//        INFLATION = 9,
//        MANAGE_DATA = 10,
//        BUMP_SEQUENCE = 11,
//        MANAGE_BUY_OFFER = 12,
//        PATH_PAYMENT_STRICT_SEND = 13,
//        CREATE_CLAIMABLE_BALANCE = 14,
//        CLAIM_CLAIMABLE_BALANCE = 15,
//        BEGIN_SPONSORING_FUTURE_RESERVES = 16,
//        END_SPONSORING_FUTURE_RESERVES = 17,
//        REVOKE_SPONSORSHIP = 18,
//        CLAWBACK = 19,
//        CLAWBACK_CLAIMABLE_BALANCE = 20,
//        SET_TRUST_LINE_FLAGS = 21,
//        LIQUIDITY_POOL_DEPOSIT = 22,
//        LIQUIDITY_POOL_WITHDRAW = 23,
//        INVOKE_HOST_FUNCTION = 24
//    };
//
// enum
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum OperationType {
    CreateAccount = 0,
    Payment = 1,
    PathPaymentStrictReceive = 2,
    ManageSellOffer = 3,
    CreatePassiveSellOffer = 4,
    SetOptions = 5,
    ChangeTrust = 6,
    AllowTrust = 7,
    AccountMerge = 8,
    Inflation = 9,
    ManageData = 10,
    BumpSequence = 11,
    ManageBuyOffer = 12,
    PathPaymentStrictSend = 13,
    CreateClaimableBalance = 14,
    ClaimClaimableBalance = 15,
    BeginSponsoringFutureReserves = 16,
    EndSponsoringFutureReserves = 17,
    RevokeSponsorship = 18,
    Clawback = 19,
    ClawbackClaimableBalance = 20,
    SetTrustLineFlags = 21,
    LiquidityPoolDeposit = 22,
    LiquidityPoolWithdraw = 23,
    InvokeHostFunction = 24,
}

impl OperationType {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::CreateAccount => "CreateAccount",
            Self::Payment => "Payment",
            Self::PathPaymentStrictReceive => "PathPaymentStrictReceive",
            Self::ManageSellOffer => "ManageSellOffer",
            Self::CreatePassiveSellOffer => "CreatePassiveSellOffer",
            Self::SetOptions => "SetOptions",
            Self::ChangeTrust => "ChangeTrust",
            Self::AllowTrust => "AllowTrust",
            Self::AccountMerge => "AccountMerge",
            Self::Inflation => "Inflation",
            Self::ManageData => "ManageData",
            Self::BumpSequence => "BumpSequence",
            Self::ManageBuyOffer => "ManageBuyOffer",
            Self::PathPaymentStrictSend => "PathPaymentStrictSend",
            Self::CreateClaimableBalance => "CreateClaimableBalance",
            Self::ClaimClaimableBalance => "ClaimClaimableBalance",
            Self::BeginSponsoringFutureReserves => "BeginSponsoringFutureReserves",
            Self::EndSponsoringFutureReserves => "EndSponsoringFutureReserves",
            Self::RevokeSponsorship => "RevokeSponsorship",
            Self::Clawback => "Clawback",
            Self::ClawbackClaimableBalance => "ClawbackClaimableBalance",
            Self::SetTrustLineFlags => "SetTrustLineFlags",
            Self::LiquidityPoolDeposit => "LiquidityPoolDeposit",
            Self::LiquidityPoolWithdraw => "LiquidityPoolWithdraw",
            Self::InvokeHostFunction => "InvokeHostFunction",
        }
    }
}

impl fmt::Display for OperationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for OperationType {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self> {
        let e = match i {
            0 => OperationType::CreateAccount,
            1 => OperationType::Payment,
            2 => OperationType::PathPaymentStrictReceive,
            3 => OperationType::ManageSellOffer,
            4 => OperationType::CreatePassiveSellOffer,
            5 => OperationType::SetOptions,
            6 => OperationType::ChangeTrust,
            7 => OperationType::AllowTrust,
            8 => OperationType::AccountMerge,
            9 => OperationType::Inflation,
            10 => OperationType::ManageData,
            11 => OperationType::BumpSequence,
            12 => OperationType::ManageBuyOffer,
            13 => OperationType::PathPaymentStrictSend,
            14 => OperationType::CreateClaimableBalance,
            15 => OperationType::ClaimClaimableBalance,
            16 => OperationType::BeginSponsoringFutureReserves,
            17 => OperationType::EndSponsoringFutureReserves,
            18 => OperationType::RevokeSponsorship,
            19 => OperationType::Clawback,
            20 => OperationType::ClawbackClaimableBalance,
            21 => OperationType::SetTrustLineFlags,
            22 => OperationType::LiquidityPoolDeposit,
            23 => OperationType::LiquidityPoolWithdraw,
            24 => OperationType::InvokeHostFunction,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<OperationType> for i32 {
    #[must_use]
    fn from(e: OperationType) -> Self {
        e as Self
    }
}

impl ReadXdr for OperationType {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXdr for OperationType {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// CreateAccountOp is an XDR Struct defines as:
//
//   struct CreateAccountOp
//    {
//        AccountID destination; // account to create
//        int64 startingBalance; // amount they end up with
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct CreateAccountOp {
    pub destination: AccountId,
    pub starting_balance: i64,
}

impl ReadXdr for CreateAccountOp {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            destination: AccountId::read_xdr(r)?,
            starting_balance: i64::read_xdr(r)?,
        })
    }
}

impl WriteXdr for CreateAccountOp {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.destination.write_xdr(w)?;
        self.starting_balance.write_xdr(w)?;
        Ok(())
    }
}

// PaymentOp is an XDR Struct defines as:
//
//   struct PaymentOp
//    {
//        MuxedAccount destination; // recipient of the payment
//        Asset asset;              // what they end up with
//        int64 amount;             // amount they end up with
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct PaymentOp {
    pub destination: MuxedAccount,
    pub asset: Asset,
    pub amount: i64,
}

impl ReadXdr for PaymentOp {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            destination: MuxedAccount::read_xdr(r)?,
            asset: Asset::read_xdr(r)?,
            amount: i64::read_xdr(r)?,
        })
    }
}

impl WriteXdr for PaymentOp {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.destination.write_xdr(w)?;
        self.asset.write_xdr(w)?;
        self.amount.write_xdr(w)?;
        Ok(())
    }
}

// PathPaymentStrictReceiveOp is an XDR Struct defines as:
//
//   struct PathPaymentStrictReceiveOp
//    {
//        Asset sendAsset; // asset we pay with
//        int64 sendMax;   // the maximum amount of sendAsset to
//                         // send (excluding fees).
//                         // The operation will fail if can't be met
//
//        MuxedAccount destination; // recipient of the payment
//        Asset destAsset;          // what they end up with
//        int64 destAmount;         // amount they end up with
//
//        Asset path<5>; // additional hops it must go through to get there
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct PathPaymentStrictReceiveOp {
    pub send_asset: Asset,
    pub send_max: i64,
    pub destination: MuxedAccount,
    pub dest_asset: Asset,
    pub dest_amount: i64,
    pub path: VecM<Asset, 5>,
}

impl ReadXdr for PathPaymentStrictReceiveOp {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            send_asset: Asset::read_xdr(r)?,
            send_max: i64::read_xdr(r)?,
            destination: MuxedAccount::read_xdr(r)?,
            dest_asset: Asset::read_xdr(r)?,
            dest_amount: i64::read_xdr(r)?,
            path: VecM::<Asset, 5>::read_xdr(r)?,
        })
    }
}

impl WriteXdr for PathPaymentStrictReceiveOp {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.send_asset.write_xdr(w)?;
        self.send_max.write_xdr(w)?;
        self.destination.write_xdr(w)?;
        self.dest_asset.write_xdr(w)?;
        self.dest_amount.write_xdr(w)?;
        self.path.write_xdr(w)?;
        Ok(())
    }
}

// PathPaymentStrictSendOp is an XDR Struct defines as:
//
//   struct PathPaymentStrictSendOp
//    {
//        Asset sendAsset;  // asset we pay with
//        int64 sendAmount; // amount of sendAsset to send (excluding fees)
//
//        MuxedAccount destination; // recipient of the payment
//        Asset destAsset;          // what they end up with
//        int64 destMin;            // the minimum amount of dest asset to
//                                  // be received
//                                  // The operation will fail if it can't be met
//
//        Asset path<5>; // additional hops it must go through to get there
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct PathPaymentStrictSendOp {
    pub send_asset: Asset,
    pub send_amount: i64,
    pub destination: MuxedAccount,
    pub dest_asset: Asset,
    pub dest_min: i64,
    pub path: VecM<Asset, 5>,
}

impl ReadXdr for PathPaymentStrictSendOp {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            send_asset: Asset::read_xdr(r)?,
            send_amount: i64::read_xdr(r)?,
            destination: MuxedAccount::read_xdr(r)?,
            dest_asset: Asset::read_xdr(r)?,
            dest_min: i64::read_xdr(r)?,
            path: VecM::<Asset, 5>::read_xdr(r)?,
        })
    }
}

impl WriteXdr for PathPaymentStrictSendOp {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.send_asset.write_xdr(w)?;
        self.send_amount.write_xdr(w)?;
        self.destination.write_xdr(w)?;
        self.dest_asset.write_xdr(w)?;
        self.dest_min.write_xdr(w)?;
        self.path.write_xdr(w)?;
        Ok(())
    }
}

// ManageSellOfferOp is an XDR Struct defines as:
//
//   struct ManageSellOfferOp
//    {
//        Asset selling;
//        Asset buying;
//        int64 amount; // amount being sold. if set to 0, delete the offer
//        Price price;  // price of thing being sold in terms of what you are buying
//
//        // 0=create a new offer, otherwise edit an existing offer
//        int64 offerID;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ManageSellOfferOp {
    pub selling: Asset,
    pub buying: Asset,
    pub amount: i64,
    pub price: Price,
    pub offer_id: i64,
}

impl ReadXdr for ManageSellOfferOp {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            selling: Asset::read_xdr(r)?,
            buying: Asset::read_xdr(r)?,
            amount: i64::read_xdr(r)?,
            price: Price::read_xdr(r)?,
            offer_id: i64::read_xdr(r)?,
        })
    }
}

impl WriteXdr for ManageSellOfferOp {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.selling.write_xdr(w)?;
        self.buying.write_xdr(w)?;
        self.amount.write_xdr(w)?;
        self.price.write_xdr(w)?;
        self.offer_id.write_xdr(w)?;
        Ok(())
    }
}

// ManageBuyOfferOp is an XDR Struct defines as:
//
//   struct ManageBuyOfferOp
//    {
//        Asset selling;
//        Asset buying;
//        int64 buyAmount; // amount being bought. if set to 0, delete the offer
//        Price price;     // price of thing being bought in terms of what you are
//                         // selling
//
//        // 0=create a new offer, otherwise edit an existing offer
//        int64 offerID;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ManageBuyOfferOp {
    pub selling: Asset,
    pub buying: Asset,
    pub buy_amount: i64,
    pub price: Price,
    pub offer_id: i64,
}

impl ReadXdr for ManageBuyOfferOp {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            selling: Asset::read_xdr(r)?,
            buying: Asset::read_xdr(r)?,
            buy_amount: i64::read_xdr(r)?,
            price: Price::read_xdr(r)?,
            offer_id: i64::read_xdr(r)?,
        })
    }
}

impl WriteXdr for ManageBuyOfferOp {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.selling.write_xdr(w)?;
        self.buying.write_xdr(w)?;
        self.buy_amount.write_xdr(w)?;
        self.price.write_xdr(w)?;
        self.offer_id.write_xdr(w)?;
        Ok(())
    }
}

// CreatePassiveSellOfferOp is an XDR Struct defines as:
//
//   struct CreatePassiveSellOfferOp
//    {
//        Asset selling; // A
//        Asset buying;  // B
//        int64 amount;  // amount taker gets
//        Price price;   // cost of A in terms of B
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct CreatePassiveSellOfferOp {
    pub selling: Asset,
    pub buying: Asset,
    pub amount: i64,
    pub price: Price,
}

impl ReadXdr for CreatePassiveSellOfferOp {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            selling: Asset::read_xdr(r)?,
            buying: Asset::read_xdr(r)?,
            amount: i64::read_xdr(r)?,
            price: Price::read_xdr(r)?,
        })
    }
}

impl WriteXdr for CreatePassiveSellOfferOp {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.selling.write_xdr(w)?;
        self.buying.write_xdr(w)?;
        self.amount.write_xdr(w)?;
        self.price.write_xdr(w)?;
        Ok(())
    }
}

// SetOptionsOp is an XDR Struct defines as:
//
//   struct SetOptionsOp
//    {
//        AccountID* inflationDest; // sets the inflation destination
//
//        uint32* clearFlags; // which flags to clear
//        uint32* setFlags;   // which flags to set
//
//        // account threshold manipulation
//        uint32* masterWeight; // weight of the master account
//        uint32* lowThreshold;
//        uint32* medThreshold;
//        uint32* highThreshold;
//
//        string32* homeDomain; // sets the home domain
//
//        // Add, update or remove a signer for the account
//        // signer is deleted if the weight is 0
//        Signer* signer;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SetOptionsOp {
    pub inflation_dest: Option<AccountId>,
    pub clear_flags: Option<u32>,
    pub set_flags: Option<u32>,
    pub master_weight: Option<u32>,
    pub low_threshold: Option<u32>,
    pub med_threshold: Option<u32>,
    pub high_threshold: Option<u32>,
    pub home_domain: Option<VecM<u8, 32>>,
    pub signer: Option<Signer>,
}

impl ReadXdr for SetOptionsOp {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            inflation_dest: Option::<AccountId>::read_xdr(r)?,
            clear_flags: Option::<u32>::read_xdr(r)?,
            set_flags: Option::<u32>::read_xdr(r)?,
            master_weight: Option::<u32>::read_xdr(r)?,
            low_threshold: Option::<u32>::read_xdr(r)?,
            med_threshold: Option::<u32>::read_xdr(r)?,
            high_threshold: Option::<u32>::read_xdr(r)?,
            home_domain: Option::<VecM<u8, 32>>::read_xdr(r)?,
            signer: Option::<Signer>::read_xdr(r)?,
        })
    }
}

impl WriteXdr for SetOptionsOp {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.inflation_dest.write_xdr(w)?;
        self.clear_flags.write_xdr(w)?;
        self.set_flags.write_xdr(w)?;
        self.master_weight.write_xdr(w)?;
        self.low_threshold.write_xdr(w)?;
        self.med_threshold.write_xdr(w)?;
        self.high_threshold.write_xdr(w)?;
        self.home_domain.write_xdr(w)?;
        self.signer.write_xdr(w)?;
        Ok(())
    }
}

// ChangeTrustAsset is an XDR Union defines as:
//
//   union ChangeTrustAsset switch (AssetType type)
//    {
//    case ASSET_TYPE_NATIVE: // Not credit
//        void;
//
//    case ASSET_TYPE_CREDIT_ALPHANUM4:
//        AlphaNum4 alphaNum4;
//
//    case ASSET_TYPE_CREDIT_ALPHANUM12:
//        AlphaNum12 alphaNum12;
//
//    case ASSET_TYPE_POOL_SHARE:
//        LiquidityPoolParameters liquidityPool;
//
//        // add other asset types here in the future
//    };
//
// union with discriminant AssetType
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ChangeTrustAsset {
    Native,
    CreditAlphanum4(AlphaNum4),
    CreditAlphanum12(AlphaNum12),
    PoolShare(LiquidityPoolParameters),
}

impl ChangeTrustAsset {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Native => "Native",
            Self::CreditAlphanum4(_) => "CreditAlphanum4",
            Self::CreditAlphanum12(_) => "CreditAlphanum12",
            Self::PoolShare(_) => "PoolShare",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> AssetType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Native => AssetType::Native,
            Self::CreditAlphanum4(_) => AssetType::CreditAlphanum4,
            Self::CreditAlphanum12(_) => AssetType::CreditAlphanum12,
            Self::PoolShare(_) => AssetType::PoolShare,
        }
    }
}

impl ReadXdr for ChangeTrustAsset {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: AssetType = <AssetType as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            AssetType::Native => Self::Native,
            AssetType::CreditAlphanum4 => Self::CreditAlphanum4(AlphaNum4::read_xdr(r)?),
            AssetType::CreditAlphanum12 => Self::CreditAlphanum12(AlphaNum12::read_xdr(r)?),
            AssetType::PoolShare => Self::PoolShare(LiquidityPoolParameters::read_xdr(r)?),
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for ChangeTrustAsset {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Native => ().write_xdr(w)?,
            Self::CreditAlphanum4(v) => v.write_xdr(w)?,
            Self::CreditAlphanum12(v) => v.write_xdr(w)?,
            Self::PoolShare(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// ChangeTrustOp is an XDR Struct defines as:
//
//   struct ChangeTrustOp
//    {
//        ChangeTrustAsset line;
//
//        // if limit is set to 0, deletes the trust line
//        int64 limit;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ChangeTrustOp {
    pub line: ChangeTrustAsset,
    pub limit: i64,
}

impl ReadXdr for ChangeTrustOp {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            line: ChangeTrustAsset::read_xdr(r)?,
            limit: i64::read_xdr(r)?,
        })
    }
}

impl WriteXdr for ChangeTrustOp {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.line.write_xdr(w)?;
        self.limit.write_xdr(w)?;
        Ok(())
    }
}

// AllowTrustOp is an XDR Struct defines as:
//
//   struct AllowTrustOp
//    {
//        AccountID trustor;
//        AssetCode asset;
//
//        // One of 0, AUTHORIZED_FLAG, or AUTHORIZED_TO_MAINTAIN_LIABILITIES_FLAG
//        uint32 authorize;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct AllowTrustOp {
    pub trustor: AccountId,
    pub asset: AssetCode,
    pub authorize: u32,
}

impl ReadXdr for AllowTrustOp {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            trustor: AccountId::read_xdr(r)?,
            asset: AssetCode::read_xdr(r)?,
            authorize: u32::read_xdr(r)?,
        })
    }
}

impl WriteXdr for AllowTrustOp {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.trustor.write_xdr(w)?;
        self.asset.write_xdr(w)?;
        self.authorize.write_xdr(w)?;
        Ok(())
    }
}

// ManageDataOp is an XDR Struct defines as:
//
//   struct ManageDataOp
//    {
//        string64 dataName;
//        DataValue* dataValue; // set to null to clear
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ManageDataOp {
    pub data_name: VecM<u8, 64>,
    pub data_value: Option<DataValue>,
}

impl ReadXdr for ManageDataOp {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            data_name: VecM::<u8, 64>::read_xdr(r)?,
            data_value: Option::<DataValue>::read_xdr(r)?,
        })
    }
}

impl WriteXdr for ManageDataOp {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.data_name.write_xdr(w)?;
        self.data_value.write_xdr(w)?;
        Ok(())
    }
}

// BumpSequenceOp is an XDR Struct defines as:
//
//   struct BumpSequenceOp
//    {
//        SequenceNumber bumpTo;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct BumpSequenceOp {
    pub bump_to: SequenceNumber,
}

impl ReadXdr for BumpSequenceOp {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            bump_to: SequenceNumber::read_xdr(r)?,
        })
    }
}

impl WriteXdr for BumpSequenceOp {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.bump_to.write_xdr(w)?;
        Ok(())
    }
}

// CreateClaimableBalanceOp is an XDR Struct defines as:
//
//   struct CreateClaimableBalanceOp
//    {
//        Asset asset;
//        int64 amount;
//        Claimant claimants<10>;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct CreateClaimableBalanceOp {
    pub asset: Asset,
    pub amount: i64,
    pub claimants: VecM<Claimant, 10>,
}

impl ReadXdr for CreateClaimableBalanceOp {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            asset: Asset::read_xdr(r)?,
            amount: i64::read_xdr(r)?,
            claimants: VecM::<Claimant, 10>::read_xdr(r)?,
        })
    }
}

impl WriteXdr for CreateClaimableBalanceOp {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.asset.write_xdr(w)?;
        self.amount.write_xdr(w)?;
        self.claimants.write_xdr(w)?;
        Ok(())
    }
}

// ClaimClaimableBalanceOp is an XDR Struct defines as:
//
//   struct ClaimClaimableBalanceOp
//    {
//        ClaimableBalanceID balanceID;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ClaimClaimableBalanceOp {
    pub balance_id: ClaimableBalanceId,
}

impl ReadXdr for ClaimClaimableBalanceOp {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            balance_id: ClaimableBalanceId::read_xdr(r)?,
        })
    }
}

impl WriteXdr for ClaimClaimableBalanceOp {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.balance_id.write_xdr(w)?;
        Ok(())
    }
}

// BeginSponsoringFutureReservesOp is an XDR Struct defines as:
//
//   struct BeginSponsoringFutureReservesOp
//    {
//        AccountID sponsoredID;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct BeginSponsoringFutureReservesOp {
    pub sponsored_id: AccountId,
}

impl ReadXdr for BeginSponsoringFutureReservesOp {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            sponsored_id: AccountId::read_xdr(r)?,
        })
    }
}

impl WriteXdr for BeginSponsoringFutureReservesOp {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.sponsored_id.write_xdr(w)?;
        Ok(())
    }
}

// RevokeSponsorshipType is an XDR Enum defines as:
//
//   enum RevokeSponsorshipType
//    {
//        REVOKE_SPONSORSHIP_LEDGER_ENTRY = 0,
//        REVOKE_SPONSORSHIP_SIGNER = 1
//    };
//
// enum
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum RevokeSponsorshipType {
    LedgerEntry = 0,
    Signer = 1,
}

impl RevokeSponsorshipType {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::LedgerEntry => "LedgerEntry",
            Self::Signer => "Signer",
        }
    }
}

impl fmt::Display for RevokeSponsorshipType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for RevokeSponsorshipType {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self> {
        let e = match i {
            0 => RevokeSponsorshipType::LedgerEntry,
            1 => RevokeSponsorshipType::Signer,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<RevokeSponsorshipType> for i32 {
    #[must_use]
    fn from(e: RevokeSponsorshipType) -> Self {
        e as Self
    }
}

impl ReadXdr for RevokeSponsorshipType {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXdr for RevokeSponsorshipType {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// RevokeSponsorshipOpSigner is an XDR NestedStruct defines as:
//
//   struct
//        {
//            AccountID accountID;
//            SignerKey signerKey;
//        }
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct RevokeSponsorshipOpSigner {
    pub account_id: AccountId,
    pub signer_key: SignerKey,
}

impl ReadXdr for RevokeSponsorshipOpSigner {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            account_id: AccountId::read_xdr(r)?,
            signer_key: SignerKey::read_xdr(r)?,
        })
    }
}

impl WriteXdr for RevokeSponsorshipOpSigner {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.account_id.write_xdr(w)?;
        self.signer_key.write_xdr(w)?;
        Ok(())
    }
}

// RevokeSponsorshipOp is an XDR Union defines as:
//
//   union RevokeSponsorshipOp switch (RevokeSponsorshipType type)
//    {
//    case REVOKE_SPONSORSHIP_LEDGER_ENTRY:
//        LedgerKey ledgerKey;
//    case REVOKE_SPONSORSHIP_SIGNER:
//        struct
//        {
//            AccountID accountID;
//            SignerKey signerKey;
//        } signer;
//    };
//
// union with discriminant RevokeSponsorshipType
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum RevokeSponsorshipOp {
    LedgerEntry(LedgerKey),
    Signer(RevokeSponsorshipOpSigner),
}

impl RevokeSponsorshipOp {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::LedgerEntry(_) => "LedgerEntry",
            Self::Signer(_) => "Signer",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> RevokeSponsorshipType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::LedgerEntry(_) => RevokeSponsorshipType::LedgerEntry,
            Self::Signer(_) => RevokeSponsorshipType::Signer,
        }
    }
}

impl ReadXdr for RevokeSponsorshipOp {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: RevokeSponsorshipType = <RevokeSponsorshipType as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            RevokeSponsorshipType::LedgerEntry => Self::LedgerEntry(LedgerKey::read_xdr(r)?),
            RevokeSponsorshipType::Signer => Self::Signer(RevokeSponsorshipOpSigner::read_xdr(r)?),
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for RevokeSponsorshipOp {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::LedgerEntry(v) => v.write_xdr(w)?,
            Self::Signer(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// ClawbackOp is an XDR Struct defines as:
//
//   struct ClawbackOp
//    {
//        Asset asset;
//        MuxedAccount from;
//        int64 amount;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ClawbackOp {
    pub asset: Asset,
    pub from: MuxedAccount,
    pub amount: i64,
}

impl ReadXdr for ClawbackOp {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            asset: Asset::read_xdr(r)?,
            from: MuxedAccount::read_xdr(r)?,
            amount: i64::read_xdr(r)?,
        })
    }
}

impl WriteXdr for ClawbackOp {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.asset.write_xdr(w)?;
        self.from.write_xdr(w)?;
        self.amount.write_xdr(w)?;
        Ok(())
    }
}

// ClawbackClaimableBalanceOp is an XDR Struct defines as:
//
//   struct ClawbackClaimableBalanceOp
//    {
//        ClaimableBalanceID balanceID;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ClawbackClaimableBalanceOp {
    pub balance_id: ClaimableBalanceId,
}

impl ReadXdr for ClawbackClaimableBalanceOp {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            balance_id: ClaimableBalanceId::read_xdr(r)?,
        })
    }
}

impl WriteXdr for ClawbackClaimableBalanceOp {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.balance_id.write_xdr(w)?;
        Ok(())
    }
}

// SetTrustLineFlagsOp is an XDR Struct defines as:
//
//   struct SetTrustLineFlagsOp
//    {
//        AccountID trustor;
//        Asset asset;
//
//        uint32 clearFlags; // which flags to clear
//        uint32 setFlags;   // which flags to set
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SetTrustLineFlagsOp {
    pub trustor: AccountId,
    pub asset: Asset,
    pub clear_flags: u32,
    pub set_flags: u32,
}

impl ReadXdr for SetTrustLineFlagsOp {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            trustor: AccountId::read_xdr(r)?,
            asset: Asset::read_xdr(r)?,
            clear_flags: u32::read_xdr(r)?,
            set_flags: u32::read_xdr(r)?,
        })
    }
}

impl WriteXdr for SetTrustLineFlagsOp {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.trustor.write_xdr(w)?;
        self.asset.write_xdr(w)?;
        self.clear_flags.write_xdr(w)?;
        self.set_flags.write_xdr(w)?;
        Ok(())
    }
}

// LiquidityPoolFeeV18 is an XDR Const defines as:
//
//   const LIQUIDITY_POOL_FEE_V18 = 30;
//
pub const LIQUIDITY_POOL_FEE_V18: u64 = 30;

// LiquidityPoolDepositOp is an XDR Struct defines as:
//
//   struct LiquidityPoolDepositOp
//    {
//        PoolID liquidityPoolID;
//        int64 maxAmountA; // maximum amount of first asset to deposit
//        int64 maxAmountB; // maximum amount of second asset to deposit
//        Price minPrice;   // minimum depositA/depositB
//        Price maxPrice;   // maximum depositA/depositB
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct LiquidityPoolDepositOp {
    pub liquidity_pool_id: PoolId,
    pub max_amount_a: i64,
    pub max_amount_b: i64,
    pub min_price: Price,
    pub max_price: Price,
}

impl ReadXdr for LiquidityPoolDepositOp {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            liquidity_pool_id: PoolId::read_xdr(r)?,
            max_amount_a: i64::read_xdr(r)?,
            max_amount_b: i64::read_xdr(r)?,
            min_price: Price::read_xdr(r)?,
            max_price: Price::read_xdr(r)?,
        })
    }
}

impl WriteXdr for LiquidityPoolDepositOp {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.liquidity_pool_id.write_xdr(w)?;
        self.max_amount_a.write_xdr(w)?;
        self.max_amount_b.write_xdr(w)?;
        self.min_price.write_xdr(w)?;
        self.max_price.write_xdr(w)?;
        Ok(())
    }
}

// LiquidityPoolWithdrawOp is an XDR Struct defines as:
//
//   struct LiquidityPoolWithdrawOp
//    {
//        PoolID liquidityPoolID;
//        int64 amount;     // amount of pool shares to withdraw
//        int64 minAmountA; // minimum amount of first asset to withdraw
//        int64 minAmountB; // minimum amount of second asset to withdraw
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct LiquidityPoolWithdrawOp {
    pub liquidity_pool_id: PoolId,
    pub amount: i64,
    pub min_amount_a: i64,
    pub min_amount_b: i64,
}

impl ReadXdr for LiquidityPoolWithdrawOp {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            liquidity_pool_id: PoolId::read_xdr(r)?,
            amount: i64::read_xdr(r)?,
            min_amount_a: i64::read_xdr(r)?,
            min_amount_b: i64::read_xdr(r)?,
        })
    }
}

impl WriteXdr for LiquidityPoolWithdrawOp {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.liquidity_pool_id.write_xdr(w)?;
        self.amount.write_xdr(w)?;
        self.min_amount_a.write_xdr(w)?;
        self.min_amount_b.write_xdr(w)?;
        Ok(())
    }
}

// HostFunction is an XDR Enum defines as:
//
//   enum HostFunction
//    {
//        HOST_FN_CALL = 0,
//        HOST_FN_CREATE_CONTRACT = 1
//    };
//
// enum
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum HostFunction {
    Call = 0,
    CreateContract = 1,
}

impl HostFunction {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Call => "Call",
            Self::CreateContract => "CreateContract",
        }
    }
}

impl fmt::Display for HostFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for HostFunction {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self> {
        let e = match i {
            0 => HostFunction::Call,
            1 => HostFunction::CreateContract,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<HostFunction> for i32 {
    #[must_use]
    fn from(e: HostFunction) -> Self {
        e as Self
    }
}

impl ReadXdr for HostFunction {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXdr for HostFunction {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// InvokeHostFunctionOp is an XDR Struct defines as:
//
//   struct InvokeHostFunctionOp
//    {
//        // The host function to invoke
//        HostFunction function;
//
//        // Parameters to the host function
//        SCVec parameters;
//
//        // The footprint for this invocation
//        LedgerFootprint footprint;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct InvokeHostFunctionOp {
    pub function: HostFunction,
    pub parameters: ScVec,
    pub footprint: LedgerFootprint,
}

impl ReadXdr for InvokeHostFunctionOp {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            function: HostFunction::read_xdr(r)?,
            parameters: ScVec::read_xdr(r)?,
            footprint: LedgerFootprint::read_xdr(r)?,
        })
    }
}

impl WriteXdr for InvokeHostFunctionOp {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.function.write_xdr(w)?;
        self.parameters.write_xdr(w)?;
        self.footprint.write_xdr(w)?;
        Ok(())
    }
}

// OperationBody is an XDR NestedUnion defines as:
//
//   union switch (OperationType type)
//        {
//        case CREATE_ACCOUNT:
//            CreateAccountOp createAccountOp;
//        case PAYMENT:
//            PaymentOp paymentOp;
//        case PATH_PAYMENT_STRICT_RECEIVE:
//            PathPaymentStrictReceiveOp pathPaymentStrictReceiveOp;
//        case MANAGE_SELL_OFFER:
//            ManageSellOfferOp manageSellOfferOp;
//        case CREATE_PASSIVE_SELL_OFFER:
//            CreatePassiveSellOfferOp createPassiveSellOfferOp;
//        case SET_OPTIONS:
//            SetOptionsOp setOptionsOp;
//        case CHANGE_TRUST:
//            ChangeTrustOp changeTrustOp;
//        case ALLOW_TRUST:
//            AllowTrustOp allowTrustOp;
//        case ACCOUNT_MERGE:
//            MuxedAccount destination;
//        case INFLATION:
//            void;
//        case MANAGE_DATA:
//            ManageDataOp manageDataOp;
//        case BUMP_SEQUENCE:
//            BumpSequenceOp bumpSequenceOp;
//        case MANAGE_BUY_OFFER:
//            ManageBuyOfferOp manageBuyOfferOp;
//        case PATH_PAYMENT_STRICT_SEND:
//            PathPaymentStrictSendOp pathPaymentStrictSendOp;
//        case CREATE_CLAIMABLE_BALANCE:
//            CreateClaimableBalanceOp createClaimableBalanceOp;
//        case CLAIM_CLAIMABLE_BALANCE:
//            ClaimClaimableBalanceOp claimClaimableBalanceOp;
//        case BEGIN_SPONSORING_FUTURE_RESERVES:
//            BeginSponsoringFutureReservesOp beginSponsoringFutureReservesOp;
//        case END_SPONSORING_FUTURE_RESERVES:
//            void;
//        case REVOKE_SPONSORSHIP:
//            RevokeSponsorshipOp revokeSponsorshipOp;
//        case CLAWBACK:
//            ClawbackOp clawbackOp;
//        case CLAWBACK_CLAIMABLE_BALANCE:
//            ClawbackClaimableBalanceOp clawbackClaimableBalanceOp;
//        case SET_TRUST_LINE_FLAGS:
//            SetTrustLineFlagsOp setTrustLineFlagsOp;
//        case LIQUIDITY_POOL_DEPOSIT:
//            LiquidityPoolDepositOp liquidityPoolDepositOp;
//        case LIQUIDITY_POOL_WITHDRAW:
//            LiquidityPoolWithdrawOp liquidityPoolWithdrawOp;
//        case INVOKE_HOST_FUNCTION:
//            InvokeHostFunctionOp invokeHostFunctionOp;
//        }
//
// union with discriminant OperationType
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum OperationBody {
    CreateAccount(CreateAccountOp),
    Payment(PaymentOp),
    PathPaymentStrictReceive(PathPaymentStrictReceiveOp),
    ManageSellOffer(ManageSellOfferOp),
    CreatePassiveSellOffer(CreatePassiveSellOfferOp),
    SetOptions(SetOptionsOp),
    ChangeTrust(ChangeTrustOp),
    AllowTrust(AllowTrustOp),
    AccountMerge(MuxedAccount),
    Inflation,
    ManageData(ManageDataOp),
    BumpSequence(BumpSequenceOp),
    ManageBuyOffer(ManageBuyOfferOp),
    PathPaymentStrictSend(PathPaymentStrictSendOp),
    CreateClaimableBalance(CreateClaimableBalanceOp),
    ClaimClaimableBalance(ClaimClaimableBalanceOp),
    BeginSponsoringFutureReserves(BeginSponsoringFutureReservesOp),
    EndSponsoringFutureReserves,
    RevokeSponsorship(RevokeSponsorshipOp),
    Clawback(ClawbackOp),
    ClawbackClaimableBalance(ClawbackClaimableBalanceOp),
    SetTrustLineFlags(SetTrustLineFlagsOp),
    LiquidityPoolDeposit(LiquidityPoolDepositOp),
    LiquidityPoolWithdraw(LiquidityPoolWithdrawOp),
    InvokeHostFunction(InvokeHostFunctionOp),
}

impl OperationBody {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::CreateAccount(_) => "CreateAccount",
            Self::Payment(_) => "Payment",
            Self::PathPaymentStrictReceive(_) => "PathPaymentStrictReceive",
            Self::ManageSellOffer(_) => "ManageSellOffer",
            Self::CreatePassiveSellOffer(_) => "CreatePassiveSellOffer",
            Self::SetOptions(_) => "SetOptions",
            Self::ChangeTrust(_) => "ChangeTrust",
            Self::AllowTrust(_) => "AllowTrust",
            Self::AccountMerge(_) => "AccountMerge",
            Self::Inflation => "Inflation",
            Self::ManageData(_) => "ManageData",
            Self::BumpSequence(_) => "BumpSequence",
            Self::ManageBuyOffer(_) => "ManageBuyOffer",
            Self::PathPaymentStrictSend(_) => "PathPaymentStrictSend",
            Self::CreateClaimableBalance(_) => "CreateClaimableBalance",
            Self::ClaimClaimableBalance(_) => "ClaimClaimableBalance",
            Self::BeginSponsoringFutureReserves(_) => "BeginSponsoringFutureReserves",
            Self::EndSponsoringFutureReserves => "EndSponsoringFutureReserves",
            Self::RevokeSponsorship(_) => "RevokeSponsorship",
            Self::Clawback(_) => "Clawback",
            Self::ClawbackClaimableBalance(_) => "ClawbackClaimableBalance",
            Self::SetTrustLineFlags(_) => "SetTrustLineFlags",
            Self::LiquidityPoolDeposit(_) => "LiquidityPoolDeposit",
            Self::LiquidityPoolWithdraw(_) => "LiquidityPoolWithdraw",
            Self::InvokeHostFunction(_) => "InvokeHostFunction",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> OperationType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::CreateAccount(_) => OperationType::CreateAccount,
            Self::Payment(_) => OperationType::Payment,
            Self::PathPaymentStrictReceive(_) => OperationType::PathPaymentStrictReceive,
            Self::ManageSellOffer(_) => OperationType::ManageSellOffer,
            Self::CreatePassiveSellOffer(_) => OperationType::CreatePassiveSellOffer,
            Self::SetOptions(_) => OperationType::SetOptions,
            Self::ChangeTrust(_) => OperationType::ChangeTrust,
            Self::AllowTrust(_) => OperationType::AllowTrust,
            Self::AccountMerge(_) => OperationType::AccountMerge,
            Self::Inflation => OperationType::Inflation,
            Self::ManageData(_) => OperationType::ManageData,
            Self::BumpSequence(_) => OperationType::BumpSequence,
            Self::ManageBuyOffer(_) => OperationType::ManageBuyOffer,
            Self::PathPaymentStrictSend(_) => OperationType::PathPaymentStrictSend,
            Self::CreateClaimableBalance(_) => OperationType::CreateClaimableBalance,
            Self::ClaimClaimableBalance(_) => OperationType::ClaimClaimableBalance,
            Self::BeginSponsoringFutureReserves(_) => OperationType::BeginSponsoringFutureReserves,
            Self::EndSponsoringFutureReserves => OperationType::EndSponsoringFutureReserves,
            Self::RevokeSponsorship(_) => OperationType::RevokeSponsorship,
            Self::Clawback(_) => OperationType::Clawback,
            Self::ClawbackClaimableBalance(_) => OperationType::ClawbackClaimableBalance,
            Self::SetTrustLineFlags(_) => OperationType::SetTrustLineFlags,
            Self::LiquidityPoolDeposit(_) => OperationType::LiquidityPoolDeposit,
            Self::LiquidityPoolWithdraw(_) => OperationType::LiquidityPoolWithdraw,
            Self::InvokeHostFunction(_) => OperationType::InvokeHostFunction,
        }
    }
}

impl ReadXdr for OperationBody {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: OperationType = <OperationType as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            OperationType::CreateAccount => Self::CreateAccount(CreateAccountOp::read_xdr(r)?),
            OperationType::Payment => Self::Payment(PaymentOp::read_xdr(r)?),
            OperationType::PathPaymentStrictReceive => {
                Self::PathPaymentStrictReceive(PathPaymentStrictReceiveOp::read_xdr(r)?)
            }
            OperationType::ManageSellOffer => {
                Self::ManageSellOffer(ManageSellOfferOp::read_xdr(r)?)
            }
            OperationType::CreatePassiveSellOffer => {
                Self::CreatePassiveSellOffer(CreatePassiveSellOfferOp::read_xdr(r)?)
            }
            OperationType::SetOptions => Self::SetOptions(SetOptionsOp::read_xdr(r)?),
            OperationType::ChangeTrust => Self::ChangeTrust(ChangeTrustOp::read_xdr(r)?),
            OperationType::AllowTrust => Self::AllowTrust(AllowTrustOp::read_xdr(r)?),
            OperationType::AccountMerge => Self::AccountMerge(MuxedAccount::read_xdr(r)?),
            OperationType::Inflation => Self::Inflation,
            OperationType::ManageData => Self::ManageData(ManageDataOp::read_xdr(r)?),
            OperationType::BumpSequence => Self::BumpSequence(BumpSequenceOp::read_xdr(r)?),
            OperationType::ManageBuyOffer => Self::ManageBuyOffer(ManageBuyOfferOp::read_xdr(r)?),
            OperationType::PathPaymentStrictSend => {
                Self::PathPaymentStrictSend(PathPaymentStrictSendOp::read_xdr(r)?)
            }
            OperationType::CreateClaimableBalance => {
                Self::CreateClaimableBalance(CreateClaimableBalanceOp::read_xdr(r)?)
            }
            OperationType::ClaimClaimableBalance => {
                Self::ClaimClaimableBalance(ClaimClaimableBalanceOp::read_xdr(r)?)
            }
            OperationType::BeginSponsoringFutureReserves => {
                Self::BeginSponsoringFutureReserves(BeginSponsoringFutureReservesOp::read_xdr(r)?)
            }
            OperationType::EndSponsoringFutureReserves => Self::EndSponsoringFutureReserves,
            OperationType::RevokeSponsorship => {
                Self::RevokeSponsorship(RevokeSponsorshipOp::read_xdr(r)?)
            }
            OperationType::Clawback => Self::Clawback(ClawbackOp::read_xdr(r)?),
            OperationType::ClawbackClaimableBalance => {
                Self::ClawbackClaimableBalance(ClawbackClaimableBalanceOp::read_xdr(r)?)
            }
            OperationType::SetTrustLineFlags => {
                Self::SetTrustLineFlags(SetTrustLineFlagsOp::read_xdr(r)?)
            }
            OperationType::LiquidityPoolDeposit => {
                Self::LiquidityPoolDeposit(LiquidityPoolDepositOp::read_xdr(r)?)
            }
            OperationType::LiquidityPoolWithdraw => {
                Self::LiquidityPoolWithdraw(LiquidityPoolWithdrawOp::read_xdr(r)?)
            }
            OperationType::InvokeHostFunction => {
                Self::InvokeHostFunction(InvokeHostFunctionOp::read_xdr(r)?)
            }
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for OperationBody {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::CreateAccount(v) => v.write_xdr(w)?,
            Self::Payment(v) => v.write_xdr(w)?,
            Self::PathPaymentStrictReceive(v) => v.write_xdr(w)?,
            Self::ManageSellOffer(v) => v.write_xdr(w)?,
            Self::CreatePassiveSellOffer(v) => v.write_xdr(w)?,
            Self::SetOptions(v) => v.write_xdr(w)?,
            Self::ChangeTrust(v) => v.write_xdr(w)?,
            Self::AllowTrust(v) => v.write_xdr(w)?,
            Self::AccountMerge(v) => v.write_xdr(w)?,
            Self::Inflation => ().write_xdr(w)?,
            Self::ManageData(v) => v.write_xdr(w)?,
            Self::BumpSequence(v) => v.write_xdr(w)?,
            Self::ManageBuyOffer(v) => v.write_xdr(w)?,
            Self::PathPaymentStrictSend(v) => v.write_xdr(w)?,
            Self::CreateClaimableBalance(v) => v.write_xdr(w)?,
            Self::ClaimClaimableBalance(v) => v.write_xdr(w)?,
            Self::BeginSponsoringFutureReserves(v) => v.write_xdr(w)?,
            Self::EndSponsoringFutureReserves => ().write_xdr(w)?,
            Self::RevokeSponsorship(v) => v.write_xdr(w)?,
            Self::Clawback(v) => v.write_xdr(w)?,
            Self::ClawbackClaimableBalance(v) => v.write_xdr(w)?,
            Self::SetTrustLineFlags(v) => v.write_xdr(w)?,
            Self::LiquidityPoolDeposit(v) => v.write_xdr(w)?,
            Self::LiquidityPoolWithdraw(v) => v.write_xdr(w)?,
            Self::InvokeHostFunction(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// Operation is an XDR Struct defines as:
//
//   struct Operation
//    {
//        // sourceAccount is the account used to run the operation
//        // if not set, the runtime defaults to "sourceAccount" specified at
//        // the transaction level
//        MuxedAccount* sourceAccount;
//
//        union switch (OperationType type)
//        {
//        case CREATE_ACCOUNT:
//            CreateAccountOp createAccountOp;
//        case PAYMENT:
//            PaymentOp paymentOp;
//        case PATH_PAYMENT_STRICT_RECEIVE:
//            PathPaymentStrictReceiveOp pathPaymentStrictReceiveOp;
//        case MANAGE_SELL_OFFER:
//            ManageSellOfferOp manageSellOfferOp;
//        case CREATE_PASSIVE_SELL_OFFER:
//            CreatePassiveSellOfferOp createPassiveSellOfferOp;
//        case SET_OPTIONS:
//            SetOptionsOp setOptionsOp;
//        case CHANGE_TRUST:
//            ChangeTrustOp changeTrustOp;
//        case ALLOW_TRUST:
//            AllowTrustOp allowTrustOp;
//        case ACCOUNT_MERGE:
//            MuxedAccount destination;
//        case INFLATION:
//            void;
//        case MANAGE_DATA:
//            ManageDataOp manageDataOp;
//        case BUMP_SEQUENCE:
//            BumpSequenceOp bumpSequenceOp;
//        case MANAGE_BUY_OFFER:
//            ManageBuyOfferOp manageBuyOfferOp;
//        case PATH_PAYMENT_STRICT_SEND:
//            PathPaymentStrictSendOp pathPaymentStrictSendOp;
//        case CREATE_CLAIMABLE_BALANCE:
//            CreateClaimableBalanceOp createClaimableBalanceOp;
//        case CLAIM_CLAIMABLE_BALANCE:
//            ClaimClaimableBalanceOp claimClaimableBalanceOp;
//        case BEGIN_SPONSORING_FUTURE_RESERVES:
//            BeginSponsoringFutureReservesOp beginSponsoringFutureReservesOp;
//        case END_SPONSORING_FUTURE_RESERVES:
//            void;
//        case REVOKE_SPONSORSHIP:
//            RevokeSponsorshipOp revokeSponsorshipOp;
//        case CLAWBACK:
//            ClawbackOp clawbackOp;
//        case CLAWBACK_CLAIMABLE_BALANCE:
//            ClawbackClaimableBalanceOp clawbackClaimableBalanceOp;
//        case SET_TRUST_LINE_FLAGS:
//            SetTrustLineFlagsOp setTrustLineFlagsOp;
//        case LIQUIDITY_POOL_DEPOSIT:
//            LiquidityPoolDepositOp liquidityPoolDepositOp;
//        case LIQUIDITY_POOL_WITHDRAW:
//            LiquidityPoolWithdrawOp liquidityPoolWithdrawOp;
//        case INVOKE_HOST_FUNCTION:
//            InvokeHostFunctionOp invokeHostFunctionOp;
//        }
//        body;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Operation {
    pub source_account: Option<MuxedAccount>,
    pub body: OperationBody,
}

impl ReadXdr for Operation {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            source_account: Option::<MuxedAccount>::read_xdr(r)?,
            body: OperationBody::read_xdr(r)?,
        })
    }
}

impl WriteXdr for Operation {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.source_account.write_xdr(w)?;
        self.body.write_xdr(w)?;
        Ok(())
    }
}

// HashIdPreimageOperationId is an XDR NestedStruct defines as:
//
//   struct
//        {
//            AccountID sourceAccount;
//            SequenceNumber seqNum;
//            uint32 opNum;
//        }
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct HashIdPreimageOperationId {
    pub source_account: AccountId,
    pub seq_num: SequenceNumber,
    pub op_num: u32,
}

impl ReadXdr for HashIdPreimageOperationId {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            source_account: AccountId::read_xdr(r)?,
            seq_num: SequenceNumber::read_xdr(r)?,
            op_num: u32::read_xdr(r)?,
        })
    }
}

impl WriteXdr for HashIdPreimageOperationId {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.source_account.write_xdr(w)?;
        self.seq_num.write_xdr(w)?;
        self.op_num.write_xdr(w)?;
        Ok(())
    }
}

// HashIdPreimageRevokeId is an XDR NestedStruct defines as:
//
//   struct
//        {
//            AccountID sourceAccount;
//            SequenceNumber seqNum;
//            uint32 opNum;
//            PoolID liquidityPoolID;
//            Asset asset;
//        }
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct HashIdPreimageRevokeId {
    pub source_account: AccountId,
    pub seq_num: SequenceNumber,
    pub op_num: u32,
    pub liquidity_pool_id: PoolId,
    pub asset: Asset,
}

impl ReadXdr for HashIdPreimageRevokeId {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            source_account: AccountId::read_xdr(r)?,
            seq_num: SequenceNumber::read_xdr(r)?,
            op_num: u32::read_xdr(r)?,
            liquidity_pool_id: PoolId::read_xdr(r)?,
            asset: Asset::read_xdr(r)?,
        })
    }
}

impl WriteXdr for HashIdPreimageRevokeId {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.source_account.write_xdr(w)?;
        self.seq_num.write_xdr(w)?;
        self.op_num.write_xdr(w)?;
        self.liquidity_pool_id.write_xdr(w)?;
        self.asset.write_xdr(w)?;
        Ok(())
    }
}

// HashIdPreimageContractId is an XDR NestedStruct defines as:
//
//   struct
//        {
//            uint256 ed25519;
//            uint256 salt;
//        }
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct HashIdPreimageContractId {
    pub ed25519: Uint256,
    pub salt: Uint256,
}

impl ReadXdr for HashIdPreimageContractId {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            ed25519: Uint256::read_xdr(r)?,
            salt: Uint256::read_xdr(r)?,
        })
    }
}

impl WriteXdr for HashIdPreimageContractId {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.ed25519.write_xdr(w)?;
        self.salt.write_xdr(w)?;
        Ok(())
    }
}

// HashIdPreimageChildContractId is an XDR NestedStruct defines as:
//
//   struct
//        {
//            Hash contractID; //contractID of parent contract
//            uint256 salt;
//        }
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct HashIdPreimageChildContractId {
    pub contract_id: Hash,
    pub salt: Uint256,
}

impl ReadXdr for HashIdPreimageChildContractId {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            contract_id: Hash::read_xdr(r)?,
            salt: Uint256::read_xdr(r)?,
        })
    }
}

impl WriteXdr for HashIdPreimageChildContractId {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.contract_id.write_xdr(w)?;
        self.salt.write_xdr(w)?;
        Ok(())
    }
}

// HashIdPreimage is an XDR Union defines as:
//
//   union HashIDPreimage switch (EnvelopeType type)
//    {
//    case ENVELOPE_TYPE_OP_ID:
//        struct
//        {
//            AccountID sourceAccount;
//            SequenceNumber seqNum;
//            uint32 opNum;
//        } operationID;
//    case ENVELOPE_TYPE_POOL_REVOKE_OP_ID:
//        struct
//        {
//            AccountID sourceAccount;
//            SequenceNumber seqNum;
//            uint32 opNum;
//            PoolID liquidityPoolID;
//            Asset asset;
//        } revokeID;
//    case ENVELOPE_TYPE_CONTRACT_ID_FROM_ED25519:
//        struct
//        {
//            uint256 ed25519;
//            uint256 salt;
//        } contractID;
//    case ENVELOPE_TYPE_CONTRACT_ID_FROM_CONTRACT:
//        struct
//        {
//            Hash contractID; //contractID of parent contract
//            uint256 salt;
//        } childContractID;
//    };
//
// union with discriminant EnvelopeType
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum HashIdPreimage {
    OpId(HashIdPreimageOperationId),
    PoolRevokeOpId(HashIdPreimageRevokeId),
    ContractIdFromEd25519(HashIdPreimageContractId),
    ContractIdFromContract(HashIdPreimageChildContractId),
}

impl HashIdPreimage {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::OpId(_) => "OpId",
            Self::PoolRevokeOpId(_) => "PoolRevokeOpId",
            Self::ContractIdFromEd25519(_) => "ContractIdFromEd25519",
            Self::ContractIdFromContract(_) => "ContractIdFromContract",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> EnvelopeType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::OpId(_) => EnvelopeType::OpId,
            Self::PoolRevokeOpId(_) => EnvelopeType::PoolRevokeOpId,
            Self::ContractIdFromEd25519(_) => EnvelopeType::ContractIdFromEd25519,
            Self::ContractIdFromContract(_) => EnvelopeType::ContractIdFromContract,
        }
    }
}

impl ReadXdr for HashIdPreimage {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: EnvelopeType = <EnvelopeType as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            EnvelopeType::OpId => Self::OpId(HashIdPreimageOperationId::read_xdr(r)?),
            EnvelopeType::PoolRevokeOpId => {
                Self::PoolRevokeOpId(HashIdPreimageRevokeId::read_xdr(r)?)
            }
            EnvelopeType::ContractIdFromEd25519 => {
                Self::ContractIdFromEd25519(HashIdPreimageContractId::read_xdr(r)?)
            }
            EnvelopeType::ContractIdFromContract => {
                Self::ContractIdFromContract(HashIdPreimageChildContractId::read_xdr(r)?)
            }
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for HashIdPreimage {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::OpId(v) => v.write_xdr(w)?,
            Self::PoolRevokeOpId(v) => v.write_xdr(w)?,
            Self::ContractIdFromEd25519(v) => v.write_xdr(w)?,
            Self::ContractIdFromContract(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// MemoType is an XDR Enum defines as:
//
//   enum MemoType
//    {
//        MEMO_NONE = 0,
//        MEMO_TEXT = 1,
//        MEMO_ID = 2,
//        MEMO_HASH = 3,
//        MEMO_RETURN = 4
//    };
//
// enum
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum MemoType {
    None = 0,
    Text = 1,
    Id = 2,
    Hash = 3,
    Return = 4,
}

impl MemoType {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::None => "None",
            Self::Text => "Text",
            Self::Id => "Id",
            Self::Hash => "Hash",
            Self::Return => "Return",
        }
    }
}

impl fmt::Display for MemoType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for MemoType {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self> {
        let e = match i {
            0 => MemoType::None,
            1 => MemoType::Text,
            2 => MemoType::Id,
            3 => MemoType::Hash,
            4 => MemoType::Return,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<MemoType> for i32 {
    #[must_use]
    fn from(e: MemoType) -> Self {
        e as Self
    }
}

impl ReadXdr for MemoType {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXdr for MemoType {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// Memo is an XDR Union defines as:
//
//   union Memo switch (MemoType type)
//    {
//    case MEMO_NONE:
//        void;
//    case MEMO_TEXT:
//        string text<28>;
//    case MEMO_ID:
//        uint64 id;
//    case MEMO_HASH:
//        Hash hash; // the hash of what to pull from the content server
//    case MEMO_RETURN:
//        Hash retHash; // the hash of the tx you are rejecting
//    };
//
// union with discriminant MemoType
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Memo {
    None,
    Text(VecM<u8, 28>),
    Id(u64),
    Hash(Hash),
    Return(Hash),
}

impl Memo {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::None => "None",
            Self::Text(_) => "Text",
            Self::Id(_) => "Id",
            Self::Hash(_) => "Hash",
            Self::Return(_) => "Return",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> MemoType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::None => MemoType::None,
            Self::Text(_) => MemoType::Text,
            Self::Id(_) => MemoType::Id,
            Self::Hash(_) => MemoType::Hash,
            Self::Return(_) => MemoType::Return,
        }
    }
}

impl ReadXdr for Memo {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: MemoType = <MemoType as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            MemoType::None => Self::None,
            MemoType::Text => Self::Text(VecM::<u8, 28>::read_xdr(r)?),
            MemoType::Id => Self::Id(u64::read_xdr(r)?),
            MemoType::Hash => Self::Hash(Hash::read_xdr(r)?),
            MemoType::Return => Self::Return(Hash::read_xdr(r)?),
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for Memo {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::None => ().write_xdr(w)?,
            Self::Text(v) => v.write_xdr(w)?,
            Self::Id(v) => v.write_xdr(w)?,
            Self::Hash(v) => v.write_xdr(w)?,
            Self::Return(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// TimeBounds is an XDR Struct defines as:
//
//   struct TimeBounds
//    {
//        TimePoint minTime;
//        TimePoint maxTime; // 0 here means no maxTime
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TimeBounds {
    pub min_time: TimePoint,
    pub max_time: TimePoint,
}

impl ReadXdr for TimeBounds {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            min_time: TimePoint::read_xdr(r)?,
            max_time: TimePoint::read_xdr(r)?,
        })
    }
}

impl WriteXdr for TimeBounds {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.min_time.write_xdr(w)?;
        self.max_time.write_xdr(w)?;
        Ok(())
    }
}

// LedgerBounds is an XDR Struct defines as:
//
//   struct LedgerBounds
//    {
//        uint32 minLedger;
//        uint32 maxLedger; // 0 here means no maxLedger
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct LedgerBounds {
    pub min_ledger: u32,
    pub max_ledger: u32,
}

impl ReadXdr for LedgerBounds {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            min_ledger: u32::read_xdr(r)?,
            max_ledger: u32::read_xdr(r)?,
        })
    }
}

impl WriteXdr for LedgerBounds {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.min_ledger.write_xdr(w)?;
        self.max_ledger.write_xdr(w)?;
        Ok(())
    }
}

// PreconditionsV2 is an XDR Struct defines as:
//
//   struct PreconditionsV2
//    {
//        TimeBounds* timeBounds;
//
//        // Transaction only valid for ledger numbers n such that
//        // minLedger <= n < maxLedger (if maxLedger == 0, then
//        // only minLedger is checked)
//        LedgerBounds* ledgerBounds;
//
//        // If NULL, only valid when sourceAccount's sequence number
//        // is seqNum - 1.  Otherwise, valid when sourceAccount's
//        // sequence number n satisfies minSeqNum <= n < tx.seqNum.
//        // Note that after execution the account's sequence number
//        // is always raised to tx.seqNum, and a transaction is not
//        // valid if tx.seqNum is too high to ensure replay protection.
//        SequenceNumber* minSeqNum;
//
//        // For the transaction to be valid, the current ledger time must
//        // be at least minSeqAge greater than sourceAccount's seqTime.
//        Duration minSeqAge;
//
//        // For the transaction to be valid, the current ledger number
//        // must be at least minSeqLedgerGap greater than sourceAccount's
//        // seqLedger.
//        uint32 minSeqLedgerGap;
//
//        // For the transaction to be valid, there must be a signature
//        // corresponding to every Signer in this array, even if the
//        // signature is not otherwise required by the sourceAccount or
//        // operations.
//        SignerKey extraSigners<2>;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct PreconditionsV2 {
    pub time_bounds: Option<TimeBounds>,
    pub ledger_bounds: Option<LedgerBounds>,
    pub min_seq_num: Option<SequenceNumber>,
    pub min_seq_age: Duration,
    pub min_seq_ledger_gap: u32,
    pub extra_signers: VecM<SignerKey, 2>,
}

impl ReadXdr for PreconditionsV2 {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            time_bounds: Option::<TimeBounds>::read_xdr(r)?,
            ledger_bounds: Option::<LedgerBounds>::read_xdr(r)?,
            min_seq_num: Option::<SequenceNumber>::read_xdr(r)?,
            min_seq_age: Duration::read_xdr(r)?,
            min_seq_ledger_gap: u32::read_xdr(r)?,
            extra_signers: VecM::<SignerKey, 2>::read_xdr(r)?,
        })
    }
}

impl WriteXdr for PreconditionsV2 {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.time_bounds.write_xdr(w)?;
        self.ledger_bounds.write_xdr(w)?;
        self.min_seq_num.write_xdr(w)?;
        self.min_seq_age.write_xdr(w)?;
        self.min_seq_ledger_gap.write_xdr(w)?;
        self.extra_signers.write_xdr(w)?;
        Ok(())
    }
}

// PreconditionType is an XDR Enum defines as:
//
//   enum PreconditionType
//    {
//        PRECOND_NONE = 0,
//        PRECOND_TIME = 1,
//        PRECOND_V2 = 2
//    };
//
// enum
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum PreconditionType {
    None = 0,
    Time = 1,
    V2 = 2,
}

impl PreconditionType {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::None => "None",
            Self::Time => "Time",
            Self::V2 => "V2",
        }
    }
}

impl fmt::Display for PreconditionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for PreconditionType {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self> {
        let e = match i {
            0 => PreconditionType::None,
            1 => PreconditionType::Time,
            2 => PreconditionType::V2,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<PreconditionType> for i32 {
    #[must_use]
    fn from(e: PreconditionType) -> Self {
        e as Self
    }
}

impl ReadXdr for PreconditionType {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXdr for PreconditionType {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// Preconditions is an XDR Union defines as:
//
//   union Preconditions switch (PreconditionType type)
//    {
//    case PRECOND_NONE:
//        void;
//    case PRECOND_TIME:
//        TimeBounds timeBounds;
//    case PRECOND_V2:
//        PreconditionsV2 v2;
//    };
//
// union with discriminant PreconditionType
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Preconditions {
    None,
    Time(TimeBounds),
    V2(PreconditionsV2),
}

impl Preconditions {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::None => "None",
            Self::Time(_) => "Time",
            Self::V2(_) => "V2",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> PreconditionType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::None => PreconditionType::None,
            Self::Time(_) => PreconditionType::Time,
            Self::V2(_) => PreconditionType::V2,
        }
    }
}

impl ReadXdr for Preconditions {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: PreconditionType = <PreconditionType as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            PreconditionType::None => Self::None,
            PreconditionType::Time => Self::Time(TimeBounds::read_xdr(r)?),
            PreconditionType::V2 => Self::V2(PreconditionsV2::read_xdr(r)?),
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for Preconditions {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::None => ().write_xdr(w)?,
            Self::Time(v) => v.write_xdr(w)?,
            Self::V2(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// MaxOpsPerTx is an XDR Const defines as:
//
//   const MAX_OPS_PER_TX = 100;
//
pub const MAX_OPS_PER_TX: u64 = 100;

// TransactionV0Ext is an XDR NestedUnion defines as:
//
//   union switch (int v)
//        {
//        case 0:
//            void;
//        }
//
// union with discriminant i32
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum TransactionV0Ext {
    V0,
}

impl TransactionV0Ext {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0 => "V0",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> i32 {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0 => 0,
        }
    }
}

impl ReadXdr for TransactionV0Ext {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: i32 = <i32 as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            0 => Self::V0,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for TransactionV0Ext {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0 => ().write_xdr(w)?,
        };
        Ok(())
    }
}

// TransactionV0 is an XDR Struct defines as:
//
//   struct TransactionV0
//    {
//        uint256 sourceAccountEd25519;
//        uint32 fee;
//        SequenceNumber seqNum;
//        TimeBounds* timeBounds;
//        Memo memo;
//        Operation operations<MAX_OPS_PER_TX>;
//        union switch (int v)
//        {
//        case 0:
//            void;
//        }
//        ext;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TransactionV0 {
    pub source_account_ed25519: Uint256,
    pub fee: u32,
    pub seq_num: SequenceNumber,
    pub time_bounds: Option<TimeBounds>,
    pub memo: Memo,
    pub operations: VecM<Operation, 100>,
    pub ext: TransactionV0Ext,
}

impl ReadXdr for TransactionV0 {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            source_account_ed25519: Uint256::read_xdr(r)?,
            fee: u32::read_xdr(r)?,
            seq_num: SequenceNumber::read_xdr(r)?,
            time_bounds: Option::<TimeBounds>::read_xdr(r)?,
            memo: Memo::read_xdr(r)?,
            operations: VecM::<Operation, 100>::read_xdr(r)?,
            ext: TransactionV0Ext::read_xdr(r)?,
        })
    }
}

impl WriteXdr for TransactionV0 {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.source_account_ed25519.write_xdr(w)?;
        self.fee.write_xdr(w)?;
        self.seq_num.write_xdr(w)?;
        self.time_bounds.write_xdr(w)?;
        self.memo.write_xdr(w)?;
        self.operations.write_xdr(w)?;
        self.ext.write_xdr(w)?;
        Ok(())
    }
}

// TransactionV0Envelope is an XDR Struct defines as:
//
//   struct TransactionV0Envelope
//    {
//        TransactionV0 tx;
//        /* Each decorated signature is a signature over the SHA256 hash of
//         * a TransactionSignaturePayload */
//        DecoratedSignature signatures<20>;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TransactionV0Envelope {
    pub tx: TransactionV0,
    pub signatures: VecM<DecoratedSignature, 20>,
}

impl ReadXdr for TransactionV0Envelope {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            tx: TransactionV0::read_xdr(r)?,
            signatures: VecM::<DecoratedSignature, 20>::read_xdr(r)?,
        })
    }
}

impl WriteXdr for TransactionV0Envelope {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.tx.write_xdr(w)?;
        self.signatures.write_xdr(w)?;
        Ok(())
    }
}

// TransactionExt is an XDR NestedUnion defines as:
//
//   union switch (int v)
//        {
//        case 0:
//            void;
//        }
//
// union with discriminant i32
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum TransactionExt {
    V0,
}

impl TransactionExt {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0 => "V0",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> i32 {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0 => 0,
        }
    }
}

impl ReadXdr for TransactionExt {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: i32 = <i32 as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            0 => Self::V0,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for TransactionExt {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0 => ().write_xdr(w)?,
        };
        Ok(())
    }
}

// Transaction is an XDR Struct defines as:
//
//   struct Transaction
//    {
//        // account used to run the transaction
//        MuxedAccount sourceAccount;
//
//        // the fee the sourceAccount will pay
//        uint32 fee;
//
//        // sequence number to consume in the account
//        SequenceNumber seqNum;
//
//        // validity conditions
//        Preconditions cond;
//
//        Memo memo;
//
//        Operation operations<MAX_OPS_PER_TX>;
//
//        // reserved for future use
//        union switch (int v)
//        {
//        case 0:
//            void;
//        }
//        ext;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Transaction {
    pub source_account: MuxedAccount,
    pub fee: u32,
    pub seq_num: SequenceNumber,
    pub cond: Preconditions,
    pub memo: Memo,
    pub operations: VecM<Operation, 100>,
    pub ext: TransactionExt,
}

impl ReadXdr for Transaction {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            source_account: MuxedAccount::read_xdr(r)?,
            fee: u32::read_xdr(r)?,
            seq_num: SequenceNumber::read_xdr(r)?,
            cond: Preconditions::read_xdr(r)?,
            memo: Memo::read_xdr(r)?,
            operations: VecM::<Operation, 100>::read_xdr(r)?,
            ext: TransactionExt::read_xdr(r)?,
        })
    }
}

impl WriteXdr for Transaction {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.source_account.write_xdr(w)?;
        self.fee.write_xdr(w)?;
        self.seq_num.write_xdr(w)?;
        self.cond.write_xdr(w)?;
        self.memo.write_xdr(w)?;
        self.operations.write_xdr(w)?;
        self.ext.write_xdr(w)?;
        Ok(())
    }
}

// TransactionV1Envelope is an XDR Struct defines as:
//
//   struct TransactionV1Envelope
//    {
//        Transaction tx;
//        /* Each decorated signature is a signature over the SHA256 hash of
//         * a TransactionSignaturePayload */
//        DecoratedSignature signatures<20>;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TransactionV1Envelope {
    pub tx: Transaction,
    pub signatures: VecM<DecoratedSignature, 20>,
}

impl ReadXdr for TransactionV1Envelope {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            tx: Transaction::read_xdr(r)?,
            signatures: VecM::<DecoratedSignature, 20>::read_xdr(r)?,
        })
    }
}

impl WriteXdr for TransactionV1Envelope {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.tx.write_xdr(w)?;
        self.signatures.write_xdr(w)?;
        Ok(())
    }
}

// FeeBumpTransactionInnerTx is an XDR NestedUnion defines as:
//
//   union switch (EnvelopeType type)
//        {
//        case ENVELOPE_TYPE_TX:
//            TransactionV1Envelope v1;
//        }
//
// union with discriminant EnvelopeType
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum FeeBumpTransactionInnerTx {
    Tx(TransactionV1Envelope),
}

impl FeeBumpTransactionInnerTx {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Tx(_) => "Tx",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> EnvelopeType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Tx(_) => EnvelopeType::Tx,
        }
    }
}

impl ReadXdr for FeeBumpTransactionInnerTx {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: EnvelopeType = <EnvelopeType as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            EnvelopeType::Tx => Self::Tx(TransactionV1Envelope::read_xdr(r)?),
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for FeeBumpTransactionInnerTx {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Tx(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// FeeBumpTransactionExt is an XDR NestedUnion defines as:
//
//   union switch (int v)
//        {
//        case 0:
//            void;
//        }
//
// union with discriminant i32
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum FeeBumpTransactionExt {
    V0,
}

impl FeeBumpTransactionExt {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0 => "V0",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> i32 {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0 => 0,
        }
    }
}

impl ReadXdr for FeeBumpTransactionExt {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: i32 = <i32 as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            0 => Self::V0,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for FeeBumpTransactionExt {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0 => ().write_xdr(w)?,
        };
        Ok(())
    }
}

// FeeBumpTransaction is an XDR Struct defines as:
//
//   struct FeeBumpTransaction
//    {
//        MuxedAccount feeSource;
//        int64 fee;
//        union switch (EnvelopeType type)
//        {
//        case ENVELOPE_TYPE_TX:
//            TransactionV1Envelope v1;
//        }
//        innerTx;
//        union switch (int v)
//        {
//        case 0:
//            void;
//        }
//        ext;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct FeeBumpTransaction {
    pub fee_source: MuxedAccount,
    pub fee: i64,
    pub inner_tx: FeeBumpTransactionInnerTx,
    pub ext: FeeBumpTransactionExt,
}

impl ReadXdr for FeeBumpTransaction {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            fee_source: MuxedAccount::read_xdr(r)?,
            fee: i64::read_xdr(r)?,
            inner_tx: FeeBumpTransactionInnerTx::read_xdr(r)?,
            ext: FeeBumpTransactionExt::read_xdr(r)?,
        })
    }
}

impl WriteXdr for FeeBumpTransaction {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.fee_source.write_xdr(w)?;
        self.fee.write_xdr(w)?;
        self.inner_tx.write_xdr(w)?;
        self.ext.write_xdr(w)?;
        Ok(())
    }
}

// FeeBumpTransactionEnvelope is an XDR Struct defines as:
//
//   struct FeeBumpTransactionEnvelope
//    {
//        FeeBumpTransaction tx;
//        /* Each decorated signature is a signature over the SHA256 hash of
//         * a TransactionSignaturePayload */
//        DecoratedSignature signatures<20>;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct FeeBumpTransactionEnvelope {
    pub tx: FeeBumpTransaction,
    pub signatures: VecM<DecoratedSignature, 20>,
}

impl ReadXdr for FeeBumpTransactionEnvelope {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            tx: FeeBumpTransaction::read_xdr(r)?,
            signatures: VecM::<DecoratedSignature, 20>::read_xdr(r)?,
        })
    }
}

impl WriteXdr for FeeBumpTransactionEnvelope {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.tx.write_xdr(w)?;
        self.signatures.write_xdr(w)?;
        Ok(())
    }
}

// TransactionEnvelope is an XDR Union defines as:
//
//   union TransactionEnvelope switch (EnvelopeType type)
//    {
//    case ENVELOPE_TYPE_TX_V0:
//        TransactionV0Envelope v0;
//    case ENVELOPE_TYPE_TX:
//        TransactionV1Envelope v1;
//    case ENVELOPE_TYPE_TX_FEE_BUMP:
//        FeeBumpTransactionEnvelope feeBump;
//    };
//
// union with discriminant EnvelopeType
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum TransactionEnvelope {
    TxV0(TransactionV0Envelope),
    Tx(TransactionV1Envelope),
    TxFeeBump(FeeBumpTransactionEnvelope),
}

impl TransactionEnvelope {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::TxV0(_) => "TxV0",
            Self::Tx(_) => "Tx",
            Self::TxFeeBump(_) => "TxFeeBump",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> EnvelopeType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::TxV0(_) => EnvelopeType::TxV0,
            Self::Tx(_) => EnvelopeType::Tx,
            Self::TxFeeBump(_) => EnvelopeType::TxFeeBump,
        }
    }
}

impl ReadXdr for TransactionEnvelope {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: EnvelopeType = <EnvelopeType as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            EnvelopeType::TxV0 => Self::TxV0(TransactionV0Envelope::read_xdr(r)?),
            EnvelopeType::Tx => Self::Tx(TransactionV1Envelope::read_xdr(r)?),
            EnvelopeType::TxFeeBump => Self::TxFeeBump(FeeBumpTransactionEnvelope::read_xdr(r)?),
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for TransactionEnvelope {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::TxV0(v) => v.write_xdr(w)?,
            Self::Tx(v) => v.write_xdr(w)?,
            Self::TxFeeBump(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// TransactionSignaturePayloadTaggedTransaction is an XDR NestedUnion defines as:
//
//   union switch (EnvelopeType type)
//        {
//        // Backwards Compatibility: Use ENVELOPE_TYPE_TX to sign ENVELOPE_TYPE_TX_V0
//        case ENVELOPE_TYPE_TX:
//            Transaction tx;
//        case ENVELOPE_TYPE_TX_FEE_BUMP:
//            FeeBumpTransaction feeBump;
//        }
//
// union with discriminant EnvelopeType
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum TransactionSignaturePayloadTaggedTransaction {
    Tx(Transaction),
    TxFeeBump(FeeBumpTransaction),
}

impl TransactionSignaturePayloadTaggedTransaction {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Tx(_) => "Tx",
            Self::TxFeeBump(_) => "TxFeeBump",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> EnvelopeType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Tx(_) => EnvelopeType::Tx,
            Self::TxFeeBump(_) => EnvelopeType::TxFeeBump,
        }
    }
}

impl ReadXdr for TransactionSignaturePayloadTaggedTransaction {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: EnvelopeType = <EnvelopeType as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            EnvelopeType::Tx => Self::Tx(Transaction::read_xdr(r)?),
            EnvelopeType::TxFeeBump => Self::TxFeeBump(FeeBumpTransaction::read_xdr(r)?),
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for TransactionSignaturePayloadTaggedTransaction {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Tx(v) => v.write_xdr(w)?,
            Self::TxFeeBump(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// TransactionSignaturePayload is an XDR Struct defines as:
//
//   struct TransactionSignaturePayload
//    {
//        Hash networkId;
//        union switch (EnvelopeType type)
//        {
//        // Backwards Compatibility: Use ENVELOPE_TYPE_TX to sign ENVELOPE_TYPE_TX_V0
//        case ENVELOPE_TYPE_TX:
//            Transaction tx;
//        case ENVELOPE_TYPE_TX_FEE_BUMP:
//            FeeBumpTransaction feeBump;
//        }
//        taggedTransaction;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TransactionSignaturePayload {
    pub network_id: Hash,
    pub tagged_transaction: TransactionSignaturePayloadTaggedTransaction,
}

impl ReadXdr for TransactionSignaturePayload {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            network_id: Hash::read_xdr(r)?,
            tagged_transaction: TransactionSignaturePayloadTaggedTransaction::read_xdr(r)?,
        })
    }
}

impl WriteXdr for TransactionSignaturePayload {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.network_id.write_xdr(w)?;
        self.tagged_transaction.write_xdr(w)?;
        Ok(())
    }
}

// ClaimAtomType is an XDR Enum defines as:
//
//   enum ClaimAtomType
//    {
//        CLAIM_ATOM_TYPE_V0 = 0,
//        CLAIM_ATOM_TYPE_ORDER_BOOK = 1,
//        CLAIM_ATOM_TYPE_LIQUIDITY_POOL = 2
//    };
//
// enum
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum ClaimAtomType {
    V0 = 0,
    OrderBook = 1,
    LiquidityPool = 2,
}

impl ClaimAtomType {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0 => "V0",
            Self::OrderBook => "OrderBook",
            Self::LiquidityPool => "LiquidityPool",
        }
    }
}

impl fmt::Display for ClaimAtomType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for ClaimAtomType {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self> {
        let e = match i {
            0 => ClaimAtomType::V0,
            1 => ClaimAtomType::OrderBook,
            2 => ClaimAtomType::LiquidityPool,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<ClaimAtomType> for i32 {
    #[must_use]
    fn from(e: ClaimAtomType) -> Self {
        e as Self
    }
}

impl ReadXdr for ClaimAtomType {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXdr for ClaimAtomType {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// ClaimOfferAtomV0 is an XDR Struct defines as:
//
//   struct ClaimOfferAtomV0
//    {
//        // emitted to identify the offer
//        uint256 sellerEd25519; // Account that owns the offer
//        int64 offerID;
//
//        // amount and asset taken from the owner
//        Asset assetSold;
//        int64 amountSold;
//
//        // amount and asset sent to the owner
//        Asset assetBought;
//        int64 amountBought;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ClaimOfferAtomV0 {
    pub seller_ed25519: Uint256,
    pub offer_id: i64,
    pub asset_sold: Asset,
    pub amount_sold: i64,
    pub asset_bought: Asset,
    pub amount_bought: i64,
}

impl ReadXdr for ClaimOfferAtomV0 {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            seller_ed25519: Uint256::read_xdr(r)?,
            offer_id: i64::read_xdr(r)?,
            asset_sold: Asset::read_xdr(r)?,
            amount_sold: i64::read_xdr(r)?,
            asset_bought: Asset::read_xdr(r)?,
            amount_bought: i64::read_xdr(r)?,
        })
    }
}

impl WriteXdr for ClaimOfferAtomV0 {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.seller_ed25519.write_xdr(w)?;
        self.offer_id.write_xdr(w)?;
        self.asset_sold.write_xdr(w)?;
        self.amount_sold.write_xdr(w)?;
        self.asset_bought.write_xdr(w)?;
        self.amount_bought.write_xdr(w)?;
        Ok(())
    }
}

// ClaimOfferAtom is an XDR Struct defines as:
//
//   struct ClaimOfferAtom
//    {
//        // emitted to identify the offer
//        AccountID sellerID; // Account that owns the offer
//        int64 offerID;
//
//        // amount and asset taken from the owner
//        Asset assetSold;
//        int64 amountSold;
//
//        // amount and asset sent to the owner
//        Asset assetBought;
//        int64 amountBought;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ClaimOfferAtom {
    pub seller_id: AccountId,
    pub offer_id: i64,
    pub asset_sold: Asset,
    pub amount_sold: i64,
    pub asset_bought: Asset,
    pub amount_bought: i64,
}

impl ReadXdr for ClaimOfferAtom {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            seller_id: AccountId::read_xdr(r)?,
            offer_id: i64::read_xdr(r)?,
            asset_sold: Asset::read_xdr(r)?,
            amount_sold: i64::read_xdr(r)?,
            asset_bought: Asset::read_xdr(r)?,
            amount_bought: i64::read_xdr(r)?,
        })
    }
}

impl WriteXdr for ClaimOfferAtom {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.seller_id.write_xdr(w)?;
        self.offer_id.write_xdr(w)?;
        self.asset_sold.write_xdr(w)?;
        self.amount_sold.write_xdr(w)?;
        self.asset_bought.write_xdr(w)?;
        self.amount_bought.write_xdr(w)?;
        Ok(())
    }
}

// ClaimLiquidityAtom is an XDR Struct defines as:
//
//   struct ClaimLiquidityAtom
//    {
//        PoolID liquidityPoolID;
//
//        // amount and asset taken from the pool
//        Asset assetSold;
//        int64 amountSold;
//
//        // amount and asset sent to the pool
//        Asset assetBought;
//        int64 amountBought;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ClaimLiquidityAtom {
    pub liquidity_pool_id: PoolId,
    pub asset_sold: Asset,
    pub amount_sold: i64,
    pub asset_bought: Asset,
    pub amount_bought: i64,
}

impl ReadXdr for ClaimLiquidityAtom {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            liquidity_pool_id: PoolId::read_xdr(r)?,
            asset_sold: Asset::read_xdr(r)?,
            amount_sold: i64::read_xdr(r)?,
            asset_bought: Asset::read_xdr(r)?,
            amount_bought: i64::read_xdr(r)?,
        })
    }
}

impl WriteXdr for ClaimLiquidityAtom {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.liquidity_pool_id.write_xdr(w)?;
        self.asset_sold.write_xdr(w)?;
        self.amount_sold.write_xdr(w)?;
        self.asset_bought.write_xdr(w)?;
        self.amount_bought.write_xdr(w)?;
        Ok(())
    }
}

// ClaimAtom is an XDR Union defines as:
//
//   union ClaimAtom switch (ClaimAtomType type)
//    {
//    case CLAIM_ATOM_TYPE_V0:
//        ClaimOfferAtomV0 v0;
//    case CLAIM_ATOM_TYPE_ORDER_BOOK:
//        ClaimOfferAtom orderBook;
//    case CLAIM_ATOM_TYPE_LIQUIDITY_POOL:
//        ClaimLiquidityAtom liquidityPool;
//    };
//
// union with discriminant ClaimAtomType
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ClaimAtom {
    V0(ClaimOfferAtomV0),
    OrderBook(ClaimOfferAtom),
    LiquidityPool(ClaimLiquidityAtom),
}

impl ClaimAtom {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0(_) => "V0",
            Self::OrderBook(_) => "OrderBook",
            Self::LiquidityPool(_) => "LiquidityPool",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> ClaimAtomType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0(_) => ClaimAtomType::V0,
            Self::OrderBook(_) => ClaimAtomType::OrderBook,
            Self::LiquidityPool(_) => ClaimAtomType::LiquidityPool,
        }
    }
}

impl ReadXdr for ClaimAtom {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: ClaimAtomType = <ClaimAtomType as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            ClaimAtomType::V0 => Self::V0(ClaimOfferAtomV0::read_xdr(r)?),
            ClaimAtomType::OrderBook => Self::OrderBook(ClaimOfferAtom::read_xdr(r)?),
            ClaimAtomType::LiquidityPool => Self::LiquidityPool(ClaimLiquidityAtom::read_xdr(r)?),
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for ClaimAtom {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0(v) => v.write_xdr(w)?,
            Self::OrderBook(v) => v.write_xdr(w)?,
            Self::LiquidityPool(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// CreateAccountResultCode is an XDR Enum defines as:
//
//   enum CreateAccountResultCode
//    {
//        // codes considered as "success" for the operation
//        CREATE_ACCOUNT_SUCCESS = 0, // account was created
//
//        // codes considered as "failure" for the operation
//        CREATE_ACCOUNT_MALFORMED = -1,   // invalid destination
//        CREATE_ACCOUNT_UNDERFUNDED = -2, // not enough funds in source account
//        CREATE_ACCOUNT_LOW_RESERVE =
//            -3, // would create an account below the min reserve
//        CREATE_ACCOUNT_ALREADY_EXIST = -4 // account already exists
//    };
//
// enum
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum CreateAccountResultCode {
    Success = 0,
    Malformed = -1,
    Underfunded = -2,
    LowReserve = -3,
    AlreadyExist = -4,
}

impl CreateAccountResultCode {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => "Success",
            Self::Malformed => "Malformed",
            Self::Underfunded => "Underfunded",
            Self::LowReserve => "LowReserve",
            Self::AlreadyExist => "AlreadyExist",
        }
    }
}

impl fmt::Display for CreateAccountResultCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for CreateAccountResultCode {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self> {
        let e = match i {
            0 => CreateAccountResultCode::Success,
            -1 => CreateAccountResultCode::Malformed,
            -2 => CreateAccountResultCode::Underfunded,
            -3 => CreateAccountResultCode::LowReserve,
            -4 => CreateAccountResultCode::AlreadyExist,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<CreateAccountResultCode> for i32 {
    #[must_use]
    fn from(e: CreateAccountResultCode) -> Self {
        e as Self
    }
}

impl ReadXdr for CreateAccountResultCode {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXdr for CreateAccountResultCode {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// CreateAccountResult is an XDR Union defines as:
//
//   union CreateAccountResult switch (CreateAccountResultCode code)
//    {
//    case CREATE_ACCOUNT_SUCCESS:
//        void;
//    case CREATE_ACCOUNT_MALFORMED:
//    case CREATE_ACCOUNT_UNDERFUNDED:
//    case CREATE_ACCOUNT_LOW_RESERVE:
//    case CREATE_ACCOUNT_ALREADY_EXIST:
//        void;
//    };
//
// union with discriminant CreateAccountResultCode
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum CreateAccountResult {
    Success,
    Malformed,
    Underfunded,
    LowReserve,
    AlreadyExist,
}

impl CreateAccountResult {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => "Success",
            Self::Malformed => "Malformed",
            Self::Underfunded => "Underfunded",
            Self::LowReserve => "LowReserve",
            Self::AlreadyExist => "AlreadyExist",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> CreateAccountResultCode {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => CreateAccountResultCode::Success,
            Self::Malformed => CreateAccountResultCode::Malformed,
            Self::Underfunded => CreateAccountResultCode::Underfunded,
            Self::LowReserve => CreateAccountResultCode::LowReserve,
            Self::AlreadyExist => CreateAccountResultCode::AlreadyExist,
        }
    }
}

impl ReadXdr for CreateAccountResult {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: CreateAccountResultCode = <CreateAccountResultCode as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            CreateAccountResultCode::Success => Self::Success,
            CreateAccountResultCode::Malformed => Self::Malformed,
            CreateAccountResultCode::Underfunded => Self::Underfunded,
            CreateAccountResultCode::LowReserve => Self::LowReserve,
            CreateAccountResultCode::AlreadyExist => Self::AlreadyExist,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for CreateAccountResult {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => ().write_xdr(w)?,
            Self::Malformed => ().write_xdr(w)?,
            Self::Underfunded => ().write_xdr(w)?,
            Self::LowReserve => ().write_xdr(w)?,
            Self::AlreadyExist => ().write_xdr(w)?,
        };
        Ok(())
    }
}

// PaymentResultCode is an XDR Enum defines as:
//
//   enum PaymentResultCode
//    {
//        // codes considered as "success" for the operation
//        PAYMENT_SUCCESS = 0, // payment successfully completed
//
//        // codes considered as "failure" for the operation
//        PAYMENT_MALFORMED = -1,          // bad input
//        PAYMENT_UNDERFUNDED = -2,        // not enough funds in source account
//        PAYMENT_SRC_NO_TRUST = -3,       // no trust line on source account
//        PAYMENT_SRC_NOT_AUTHORIZED = -4, // source not authorized to transfer
//        PAYMENT_NO_DESTINATION = -5,     // destination account does not exist
//        PAYMENT_NO_TRUST = -6,       // destination missing a trust line for asset
//        PAYMENT_NOT_AUTHORIZED = -7, // destination not authorized to hold asset
//        PAYMENT_LINE_FULL = -8,      // destination would go above their limit
//        PAYMENT_NO_ISSUER = -9       // missing issuer on asset
//    };
//
// enum
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum PaymentResultCode {
    Success = 0,
    Malformed = -1,
    Underfunded = -2,
    SrcNoTrust = -3,
    SrcNotAuthorized = -4,
    NoDestination = -5,
    NoTrust = -6,
    NotAuthorized = -7,
    LineFull = -8,
    NoIssuer = -9,
}

impl PaymentResultCode {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => "Success",
            Self::Malformed => "Malformed",
            Self::Underfunded => "Underfunded",
            Self::SrcNoTrust => "SrcNoTrust",
            Self::SrcNotAuthorized => "SrcNotAuthorized",
            Self::NoDestination => "NoDestination",
            Self::NoTrust => "NoTrust",
            Self::NotAuthorized => "NotAuthorized",
            Self::LineFull => "LineFull",
            Self::NoIssuer => "NoIssuer",
        }
    }
}

impl fmt::Display for PaymentResultCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for PaymentResultCode {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self> {
        let e = match i {
            0 => PaymentResultCode::Success,
            -1 => PaymentResultCode::Malformed,
            -2 => PaymentResultCode::Underfunded,
            -3 => PaymentResultCode::SrcNoTrust,
            -4 => PaymentResultCode::SrcNotAuthorized,
            -5 => PaymentResultCode::NoDestination,
            -6 => PaymentResultCode::NoTrust,
            -7 => PaymentResultCode::NotAuthorized,
            -8 => PaymentResultCode::LineFull,
            -9 => PaymentResultCode::NoIssuer,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<PaymentResultCode> for i32 {
    #[must_use]
    fn from(e: PaymentResultCode) -> Self {
        e as Self
    }
}

impl ReadXdr for PaymentResultCode {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXdr for PaymentResultCode {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// PaymentResult is an XDR Union defines as:
//
//   union PaymentResult switch (PaymentResultCode code)
//    {
//    case PAYMENT_SUCCESS:
//        void;
//    case PAYMENT_MALFORMED:
//    case PAYMENT_UNDERFUNDED:
//    case PAYMENT_SRC_NO_TRUST:
//    case PAYMENT_SRC_NOT_AUTHORIZED:
//    case PAYMENT_NO_DESTINATION:
//    case PAYMENT_NO_TRUST:
//    case PAYMENT_NOT_AUTHORIZED:
//    case PAYMENT_LINE_FULL:
//    case PAYMENT_NO_ISSUER:
//        void;
//    };
//
// union with discriminant PaymentResultCode
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum PaymentResult {
    Success,
    Malformed,
    Underfunded,
    SrcNoTrust,
    SrcNotAuthorized,
    NoDestination,
    NoTrust,
    NotAuthorized,
    LineFull,
    NoIssuer,
}

impl PaymentResult {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => "Success",
            Self::Malformed => "Malformed",
            Self::Underfunded => "Underfunded",
            Self::SrcNoTrust => "SrcNoTrust",
            Self::SrcNotAuthorized => "SrcNotAuthorized",
            Self::NoDestination => "NoDestination",
            Self::NoTrust => "NoTrust",
            Self::NotAuthorized => "NotAuthorized",
            Self::LineFull => "LineFull",
            Self::NoIssuer => "NoIssuer",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> PaymentResultCode {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => PaymentResultCode::Success,
            Self::Malformed => PaymentResultCode::Malformed,
            Self::Underfunded => PaymentResultCode::Underfunded,
            Self::SrcNoTrust => PaymentResultCode::SrcNoTrust,
            Self::SrcNotAuthorized => PaymentResultCode::SrcNotAuthorized,
            Self::NoDestination => PaymentResultCode::NoDestination,
            Self::NoTrust => PaymentResultCode::NoTrust,
            Self::NotAuthorized => PaymentResultCode::NotAuthorized,
            Self::LineFull => PaymentResultCode::LineFull,
            Self::NoIssuer => PaymentResultCode::NoIssuer,
        }
    }
}

impl ReadXdr for PaymentResult {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: PaymentResultCode = <PaymentResultCode as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            PaymentResultCode::Success => Self::Success,
            PaymentResultCode::Malformed => Self::Malformed,
            PaymentResultCode::Underfunded => Self::Underfunded,
            PaymentResultCode::SrcNoTrust => Self::SrcNoTrust,
            PaymentResultCode::SrcNotAuthorized => Self::SrcNotAuthorized,
            PaymentResultCode::NoDestination => Self::NoDestination,
            PaymentResultCode::NoTrust => Self::NoTrust,
            PaymentResultCode::NotAuthorized => Self::NotAuthorized,
            PaymentResultCode::LineFull => Self::LineFull,
            PaymentResultCode::NoIssuer => Self::NoIssuer,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for PaymentResult {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => ().write_xdr(w)?,
            Self::Malformed => ().write_xdr(w)?,
            Self::Underfunded => ().write_xdr(w)?,
            Self::SrcNoTrust => ().write_xdr(w)?,
            Self::SrcNotAuthorized => ().write_xdr(w)?,
            Self::NoDestination => ().write_xdr(w)?,
            Self::NoTrust => ().write_xdr(w)?,
            Self::NotAuthorized => ().write_xdr(w)?,
            Self::LineFull => ().write_xdr(w)?,
            Self::NoIssuer => ().write_xdr(w)?,
        };
        Ok(())
    }
}

// PathPaymentStrictReceiveResultCode is an XDR Enum defines as:
//
//   enum PathPaymentStrictReceiveResultCode
//    {
//        // codes considered as "success" for the operation
//        PATH_PAYMENT_STRICT_RECEIVE_SUCCESS = 0, // success
//
//        // codes considered as "failure" for the operation
//        PATH_PAYMENT_STRICT_RECEIVE_MALFORMED = -1, // bad input
//        PATH_PAYMENT_STRICT_RECEIVE_UNDERFUNDED =
//            -2, // not enough funds in source account
//        PATH_PAYMENT_STRICT_RECEIVE_SRC_NO_TRUST =
//            -3, // no trust line on source account
//        PATH_PAYMENT_STRICT_RECEIVE_SRC_NOT_AUTHORIZED =
//            -4, // source not authorized to transfer
//        PATH_PAYMENT_STRICT_RECEIVE_NO_DESTINATION =
//            -5, // destination account does not exist
//        PATH_PAYMENT_STRICT_RECEIVE_NO_TRUST =
//            -6, // dest missing a trust line for asset
//        PATH_PAYMENT_STRICT_RECEIVE_NOT_AUTHORIZED =
//            -7, // dest not authorized to hold asset
//        PATH_PAYMENT_STRICT_RECEIVE_LINE_FULL =
//            -8, // dest would go above their limit
//        PATH_PAYMENT_STRICT_RECEIVE_NO_ISSUER = -9, // missing issuer on one asset
//        PATH_PAYMENT_STRICT_RECEIVE_TOO_FEW_OFFERS =
//            -10, // not enough offers to satisfy path
//        PATH_PAYMENT_STRICT_RECEIVE_OFFER_CROSS_SELF =
//            -11, // would cross one of its own offers
//        PATH_PAYMENT_STRICT_RECEIVE_OVER_SENDMAX = -12 // could not satisfy sendmax
//    };
//
// enum
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum PathPaymentStrictReceiveResultCode {
    Success = 0,
    Malformed = -1,
    Underfunded = -2,
    SrcNoTrust = -3,
    SrcNotAuthorized = -4,
    NoDestination = -5,
    NoTrust = -6,
    NotAuthorized = -7,
    LineFull = -8,
    NoIssuer = -9,
    TooFewOffers = -10,
    OfferCrossSelf = -11,
    OverSendmax = -12,
}

impl PathPaymentStrictReceiveResultCode {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => "Success",
            Self::Malformed => "Malformed",
            Self::Underfunded => "Underfunded",
            Self::SrcNoTrust => "SrcNoTrust",
            Self::SrcNotAuthorized => "SrcNotAuthorized",
            Self::NoDestination => "NoDestination",
            Self::NoTrust => "NoTrust",
            Self::NotAuthorized => "NotAuthorized",
            Self::LineFull => "LineFull",
            Self::NoIssuer => "NoIssuer",
            Self::TooFewOffers => "TooFewOffers",
            Self::OfferCrossSelf => "OfferCrossSelf",
            Self::OverSendmax => "OverSendmax",
        }
    }
}

impl fmt::Display for PathPaymentStrictReceiveResultCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for PathPaymentStrictReceiveResultCode {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self> {
        let e = match i {
            0 => PathPaymentStrictReceiveResultCode::Success,
            -1 => PathPaymentStrictReceiveResultCode::Malformed,
            -2 => PathPaymentStrictReceiveResultCode::Underfunded,
            -3 => PathPaymentStrictReceiveResultCode::SrcNoTrust,
            -4 => PathPaymentStrictReceiveResultCode::SrcNotAuthorized,
            -5 => PathPaymentStrictReceiveResultCode::NoDestination,
            -6 => PathPaymentStrictReceiveResultCode::NoTrust,
            -7 => PathPaymentStrictReceiveResultCode::NotAuthorized,
            -8 => PathPaymentStrictReceiveResultCode::LineFull,
            -9 => PathPaymentStrictReceiveResultCode::NoIssuer,
            -10 => PathPaymentStrictReceiveResultCode::TooFewOffers,
            -11 => PathPaymentStrictReceiveResultCode::OfferCrossSelf,
            -12 => PathPaymentStrictReceiveResultCode::OverSendmax,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<PathPaymentStrictReceiveResultCode> for i32 {
    #[must_use]
    fn from(e: PathPaymentStrictReceiveResultCode) -> Self {
        e as Self
    }
}

impl ReadXdr for PathPaymentStrictReceiveResultCode {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXdr for PathPaymentStrictReceiveResultCode {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// SimplePaymentResult is an XDR Struct defines as:
//
//   struct SimplePaymentResult
//    {
//        AccountID destination;
//        Asset asset;
//        int64 amount;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SimplePaymentResult {
    pub destination: AccountId,
    pub asset: Asset,
    pub amount: i64,
}

impl ReadXdr for SimplePaymentResult {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            destination: AccountId::read_xdr(r)?,
            asset: Asset::read_xdr(r)?,
            amount: i64::read_xdr(r)?,
        })
    }
}

impl WriteXdr for SimplePaymentResult {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.destination.write_xdr(w)?;
        self.asset.write_xdr(w)?;
        self.amount.write_xdr(w)?;
        Ok(())
    }
}

// PathPaymentStrictReceiveResultSuccess is an XDR NestedStruct defines as:
//
//   struct
//        {
//            ClaimAtom offers<>;
//            SimplePaymentResult last;
//        }
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct PathPaymentStrictReceiveResultSuccess {
    pub offers: VecM<ClaimAtom>,
    pub last: SimplePaymentResult,
}

impl ReadXdr for PathPaymentStrictReceiveResultSuccess {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            offers: VecM::<ClaimAtom>::read_xdr(r)?,
            last: SimplePaymentResult::read_xdr(r)?,
        })
    }
}

impl WriteXdr for PathPaymentStrictReceiveResultSuccess {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.offers.write_xdr(w)?;
        self.last.write_xdr(w)?;
        Ok(())
    }
}

// PathPaymentStrictReceiveResult is an XDR Union defines as:
//
//   union PathPaymentStrictReceiveResult switch (
//        PathPaymentStrictReceiveResultCode code)
//    {
//    case PATH_PAYMENT_STRICT_RECEIVE_SUCCESS:
//        struct
//        {
//            ClaimAtom offers<>;
//            SimplePaymentResult last;
//        } success;
//    case PATH_PAYMENT_STRICT_RECEIVE_MALFORMED:
//    case PATH_PAYMENT_STRICT_RECEIVE_UNDERFUNDED:
//    case PATH_PAYMENT_STRICT_RECEIVE_SRC_NO_TRUST:
//    case PATH_PAYMENT_STRICT_RECEIVE_SRC_NOT_AUTHORIZED:
//    case PATH_PAYMENT_STRICT_RECEIVE_NO_DESTINATION:
//    case PATH_PAYMENT_STRICT_RECEIVE_NO_TRUST:
//    case PATH_PAYMENT_STRICT_RECEIVE_NOT_AUTHORIZED:
//    case PATH_PAYMENT_STRICT_RECEIVE_LINE_FULL:
//        void;
//    case PATH_PAYMENT_STRICT_RECEIVE_NO_ISSUER:
//        Asset noIssuer; // the asset that caused the error
//    case PATH_PAYMENT_STRICT_RECEIVE_TOO_FEW_OFFERS:
//    case PATH_PAYMENT_STRICT_RECEIVE_OFFER_CROSS_SELF:
//    case PATH_PAYMENT_STRICT_RECEIVE_OVER_SENDMAX:
//        void;
//    };
//
// union with discriminant PathPaymentStrictReceiveResultCode
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum PathPaymentStrictReceiveResult {
    Success(PathPaymentStrictReceiveResultSuccess),
    Malformed,
    Underfunded,
    SrcNoTrust,
    SrcNotAuthorized,
    NoDestination,
    NoTrust,
    NotAuthorized,
    LineFull,
    NoIssuer(Asset),
    TooFewOffers,
    OfferCrossSelf,
    OverSendmax,
}

impl PathPaymentStrictReceiveResult {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success(_) => "Success",
            Self::Malformed => "Malformed",
            Self::Underfunded => "Underfunded",
            Self::SrcNoTrust => "SrcNoTrust",
            Self::SrcNotAuthorized => "SrcNotAuthorized",
            Self::NoDestination => "NoDestination",
            Self::NoTrust => "NoTrust",
            Self::NotAuthorized => "NotAuthorized",
            Self::LineFull => "LineFull",
            Self::NoIssuer(_) => "NoIssuer",
            Self::TooFewOffers => "TooFewOffers",
            Self::OfferCrossSelf => "OfferCrossSelf",
            Self::OverSendmax => "OverSendmax",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> PathPaymentStrictReceiveResultCode {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success(_) => PathPaymentStrictReceiveResultCode::Success,
            Self::Malformed => PathPaymentStrictReceiveResultCode::Malformed,
            Self::Underfunded => PathPaymentStrictReceiveResultCode::Underfunded,
            Self::SrcNoTrust => PathPaymentStrictReceiveResultCode::SrcNoTrust,
            Self::SrcNotAuthorized => PathPaymentStrictReceiveResultCode::SrcNotAuthorized,
            Self::NoDestination => PathPaymentStrictReceiveResultCode::NoDestination,
            Self::NoTrust => PathPaymentStrictReceiveResultCode::NoTrust,
            Self::NotAuthorized => PathPaymentStrictReceiveResultCode::NotAuthorized,
            Self::LineFull => PathPaymentStrictReceiveResultCode::LineFull,
            Self::NoIssuer(_) => PathPaymentStrictReceiveResultCode::NoIssuer,
            Self::TooFewOffers => PathPaymentStrictReceiveResultCode::TooFewOffers,
            Self::OfferCrossSelf => PathPaymentStrictReceiveResultCode::OfferCrossSelf,
            Self::OverSendmax => PathPaymentStrictReceiveResultCode::OverSendmax,
        }
    }
}

impl ReadXdr for PathPaymentStrictReceiveResult {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: PathPaymentStrictReceiveResultCode =
            <PathPaymentStrictReceiveResultCode as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            PathPaymentStrictReceiveResultCode::Success => {
                Self::Success(PathPaymentStrictReceiveResultSuccess::read_xdr(r)?)
            }
            PathPaymentStrictReceiveResultCode::Malformed => Self::Malformed,
            PathPaymentStrictReceiveResultCode::Underfunded => Self::Underfunded,
            PathPaymentStrictReceiveResultCode::SrcNoTrust => Self::SrcNoTrust,
            PathPaymentStrictReceiveResultCode::SrcNotAuthorized => Self::SrcNotAuthorized,
            PathPaymentStrictReceiveResultCode::NoDestination => Self::NoDestination,
            PathPaymentStrictReceiveResultCode::NoTrust => Self::NoTrust,
            PathPaymentStrictReceiveResultCode::NotAuthorized => Self::NotAuthorized,
            PathPaymentStrictReceiveResultCode::LineFull => Self::LineFull,
            PathPaymentStrictReceiveResultCode::NoIssuer => Self::NoIssuer(Asset::read_xdr(r)?),
            PathPaymentStrictReceiveResultCode::TooFewOffers => Self::TooFewOffers,
            PathPaymentStrictReceiveResultCode::OfferCrossSelf => Self::OfferCrossSelf,
            PathPaymentStrictReceiveResultCode::OverSendmax => Self::OverSendmax,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for PathPaymentStrictReceiveResult {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success(v) => v.write_xdr(w)?,
            Self::Malformed => ().write_xdr(w)?,
            Self::Underfunded => ().write_xdr(w)?,
            Self::SrcNoTrust => ().write_xdr(w)?,
            Self::SrcNotAuthorized => ().write_xdr(w)?,
            Self::NoDestination => ().write_xdr(w)?,
            Self::NoTrust => ().write_xdr(w)?,
            Self::NotAuthorized => ().write_xdr(w)?,
            Self::LineFull => ().write_xdr(w)?,
            Self::NoIssuer(v) => v.write_xdr(w)?,
            Self::TooFewOffers => ().write_xdr(w)?,
            Self::OfferCrossSelf => ().write_xdr(w)?,
            Self::OverSendmax => ().write_xdr(w)?,
        };
        Ok(())
    }
}

// PathPaymentStrictSendResultCode is an XDR Enum defines as:
//
//   enum PathPaymentStrictSendResultCode
//    {
//        // codes considered as "success" for the operation
//        PATH_PAYMENT_STRICT_SEND_SUCCESS = 0, // success
//
//        // codes considered as "failure" for the operation
//        PATH_PAYMENT_STRICT_SEND_MALFORMED = -1, // bad input
//        PATH_PAYMENT_STRICT_SEND_UNDERFUNDED =
//            -2, // not enough funds in source account
//        PATH_PAYMENT_STRICT_SEND_SRC_NO_TRUST =
//            -3, // no trust line on source account
//        PATH_PAYMENT_STRICT_SEND_SRC_NOT_AUTHORIZED =
//            -4, // source not authorized to transfer
//        PATH_PAYMENT_STRICT_SEND_NO_DESTINATION =
//            -5, // destination account does not exist
//        PATH_PAYMENT_STRICT_SEND_NO_TRUST =
//            -6, // dest missing a trust line for asset
//        PATH_PAYMENT_STRICT_SEND_NOT_AUTHORIZED =
//            -7, // dest not authorized to hold asset
//        PATH_PAYMENT_STRICT_SEND_LINE_FULL = -8, // dest would go above their limit
//        PATH_PAYMENT_STRICT_SEND_NO_ISSUER = -9, // missing issuer on one asset
//        PATH_PAYMENT_STRICT_SEND_TOO_FEW_OFFERS =
//            -10, // not enough offers to satisfy path
//        PATH_PAYMENT_STRICT_SEND_OFFER_CROSS_SELF =
//            -11, // would cross one of its own offers
//        PATH_PAYMENT_STRICT_SEND_UNDER_DESTMIN = -12 // could not satisfy destMin
//    };
//
// enum
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum PathPaymentStrictSendResultCode {
    Success = 0,
    Malformed = -1,
    Underfunded = -2,
    SrcNoTrust = -3,
    SrcNotAuthorized = -4,
    NoDestination = -5,
    NoTrust = -6,
    NotAuthorized = -7,
    LineFull = -8,
    NoIssuer = -9,
    TooFewOffers = -10,
    OfferCrossSelf = -11,
    UnderDestmin = -12,
}

impl PathPaymentStrictSendResultCode {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => "Success",
            Self::Malformed => "Malformed",
            Self::Underfunded => "Underfunded",
            Self::SrcNoTrust => "SrcNoTrust",
            Self::SrcNotAuthorized => "SrcNotAuthorized",
            Self::NoDestination => "NoDestination",
            Self::NoTrust => "NoTrust",
            Self::NotAuthorized => "NotAuthorized",
            Self::LineFull => "LineFull",
            Self::NoIssuer => "NoIssuer",
            Self::TooFewOffers => "TooFewOffers",
            Self::OfferCrossSelf => "OfferCrossSelf",
            Self::UnderDestmin => "UnderDestmin",
        }
    }
}

impl fmt::Display for PathPaymentStrictSendResultCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for PathPaymentStrictSendResultCode {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self> {
        let e = match i {
            0 => PathPaymentStrictSendResultCode::Success,
            -1 => PathPaymentStrictSendResultCode::Malformed,
            -2 => PathPaymentStrictSendResultCode::Underfunded,
            -3 => PathPaymentStrictSendResultCode::SrcNoTrust,
            -4 => PathPaymentStrictSendResultCode::SrcNotAuthorized,
            -5 => PathPaymentStrictSendResultCode::NoDestination,
            -6 => PathPaymentStrictSendResultCode::NoTrust,
            -7 => PathPaymentStrictSendResultCode::NotAuthorized,
            -8 => PathPaymentStrictSendResultCode::LineFull,
            -9 => PathPaymentStrictSendResultCode::NoIssuer,
            -10 => PathPaymentStrictSendResultCode::TooFewOffers,
            -11 => PathPaymentStrictSendResultCode::OfferCrossSelf,
            -12 => PathPaymentStrictSendResultCode::UnderDestmin,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<PathPaymentStrictSendResultCode> for i32 {
    #[must_use]
    fn from(e: PathPaymentStrictSendResultCode) -> Self {
        e as Self
    }
}

impl ReadXdr for PathPaymentStrictSendResultCode {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXdr for PathPaymentStrictSendResultCode {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// PathPaymentStrictSendResultSuccess is an XDR NestedStruct defines as:
//
//   struct
//        {
//            ClaimAtom offers<>;
//            SimplePaymentResult last;
//        }
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct PathPaymentStrictSendResultSuccess {
    pub offers: VecM<ClaimAtom>,
    pub last: SimplePaymentResult,
}

impl ReadXdr for PathPaymentStrictSendResultSuccess {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            offers: VecM::<ClaimAtom>::read_xdr(r)?,
            last: SimplePaymentResult::read_xdr(r)?,
        })
    }
}

impl WriteXdr for PathPaymentStrictSendResultSuccess {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.offers.write_xdr(w)?;
        self.last.write_xdr(w)?;
        Ok(())
    }
}

// PathPaymentStrictSendResult is an XDR Union defines as:
//
//   union PathPaymentStrictSendResult switch (PathPaymentStrictSendResultCode code)
//    {
//    case PATH_PAYMENT_STRICT_SEND_SUCCESS:
//        struct
//        {
//            ClaimAtom offers<>;
//            SimplePaymentResult last;
//        } success;
//    case PATH_PAYMENT_STRICT_SEND_MALFORMED:
//    case PATH_PAYMENT_STRICT_SEND_UNDERFUNDED:
//    case PATH_PAYMENT_STRICT_SEND_SRC_NO_TRUST:
//    case PATH_PAYMENT_STRICT_SEND_SRC_NOT_AUTHORIZED:
//    case PATH_PAYMENT_STRICT_SEND_NO_DESTINATION:
//    case PATH_PAYMENT_STRICT_SEND_NO_TRUST:
//    case PATH_PAYMENT_STRICT_SEND_NOT_AUTHORIZED:
//    case PATH_PAYMENT_STRICT_SEND_LINE_FULL:
//        void;
//    case PATH_PAYMENT_STRICT_SEND_NO_ISSUER:
//        Asset noIssuer; // the asset that caused the error
//    case PATH_PAYMENT_STRICT_SEND_TOO_FEW_OFFERS:
//    case PATH_PAYMENT_STRICT_SEND_OFFER_CROSS_SELF:
//    case PATH_PAYMENT_STRICT_SEND_UNDER_DESTMIN:
//        void;
//    };
//
// union with discriminant PathPaymentStrictSendResultCode
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum PathPaymentStrictSendResult {
    Success(PathPaymentStrictSendResultSuccess),
    Malformed,
    Underfunded,
    SrcNoTrust,
    SrcNotAuthorized,
    NoDestination,
    NoTrust,
    NotAuthorized,
    LineFull,
    NoIssuer(Asset),
    TooFewOffers,
    OfferCrossSelf,
    UnderDestmin,
}

impl PathPaymentStrictSendResult {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success(_) => "Success",
            Self::Malformed => "Malformed",
            Self::Underfunded => "Underfunded",
            Self::SrcNoTrust => "SrcNoTrust",
            Self::SrcNotAuthorized => "SrcNotAuthorized",
            Self::NoDestination => "NoDestination",
            Self::NoTrust => "NoTrust",
            Self::NotAuthorized => "NotAuthorized",
            Self::LineFull => "LineFull",
            Self::NoIssuer(_) => "NoIssuer",
            Self::TooFewOffers => "TooFewOffers",
            Self::OfferCrossSelf => "OfferCrossSelf",
            Self::UnderDestmin => "UnderDestmin",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> PathPaymentStrictSendResultCode {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success(_) => PathPaymentStrictSendResultCode::Success,
            Self::Malformed => PathPaymentStrictSendResultCode::Malformed,
            Self::Underfunded => PathPaymentStrictSendResultCode::Underfunded,
            Self::SrcNoTrust => PathPaymentStrictSendResultCode::SrcNoTrust,
            Self::SrcNotAuthorized => PathPaymentStrictSendResultCode::SrcNotAuthorized,
            Self::NoDestination => PathPaymentStrictSendResultCode::NoDestination,
            Self::NoTrust => PathPaymentStrictSendResultCode::NoTrust,
            Self::NotAuthorized => PathPaymentStrictSendResultCode::NotAuthorized,
            Self::LineFull => PathPaymentStrictSendResultCode::LineFull,
            Self::NoIssuer(_) => PathPaymentStrictSendResultCode::NoIssuer,
            Self::TooFewOffers => PathPaymentStrictSendResultCode::TooFewOffers,
            Self::OfferCrossSelf => PathPaymentStrictSendResultCode::OfferCrossSelf,
            Self::UnderDestmin => PathPaymentStrictSendResultCode::UnderDestmin,
        }
    }
}

impl ReadXdr for PathPaymentStrictSendResult {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: PathPaymentStrictSendResultCode =
            <PathPaymentStrictSendResultCode as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            PathPaymentStrictSendResultCode::Success => {
                Self::Success(PathPaymentStrictSendResultSuccess::read_xdr(r)?)
            }
            PathPaymentStrictSendResultCode::Malformed => Self::Malformed,
            PathPaymentStrictSendResultCode::Underfunded => Self::Underfunded,
            PathPaymentStrictSendResultCode::SrcNoTrust => Self::SrcNoTrust,
            PathPaymentStrictSendResultCode::SrcNotAuthorized => Self::SrcNotAuthorized,
            PathPaymentStrictSendResultCode::NoDestination => Self::NoDestination,
            PathPaymentStrictSendResultCode::NoTrust => Self::NoTrust,
            PathPaymentStrictSendResultCode::NotAuthorized => Self::NotAuthorized,
            PathPaymentStrictSendResultCode::LineFull => Self::LineFull,
            PathPaymentStrictSendResultCode::NoIssuer => Self::NoIssuer(Asset::read_xdr(r)?),
            PathPaymentStrictSendResultCode::TooFewOffers => Self::TooFewOffers,
            PathPaymentStrictSendResultCode::OfferCrossSelf => Self::OfferCrossSelf,
            PathPaymentStrictSendResultCode::UnderDestmin => Self::UnderDestmin,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for PathPaymentStrictSendResult {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success(v) => v.write_xdr(w)?,
            Self::Malformed => ().write_xdr(w)?,
            Self::Underfunded => ().write_xdr(w)?,
            Self::SrcNoTrust => ().write_xdr(w)?,
            Self::SrcNotAuthorized => ().write_xdr(w)?,
            Self::NoDestination => ().write_xdr(w)?,
            Self::NoTrust => ().write_xdr(w)?,
            Self::NotAuthorized => ().write_xdr(w)?,
            Self::LineFull => ().write_xdr(w)?,
            Self::NoIssuer(v) => v.write_xdr(w)?,
            Self::TooFewOffers => ().write_xdr(w)?,
            Self::OfferCrossSelf => ().write_xdr(w)?,
            Self::UnderDestmin => ().write_xdr(w)?,
        };
        Ok(())
    }
}

// ManageSellOfferResultCode is an XDR Enum defines as:
//
//   enum ManageSellOfferResultCode
//    {
//        // codes considered as "success" for the operation
//        MANAGE_SELL_OFFER_SUCCESS = 0,
//
//        // codes considered as "failure" for the operation
//        MANAGE_SELL_OFFER_MALFORMED = -1, // generated offer would be invalid
//        MANAGE_SELL_OFFER_SELL_NO_TRUST =
//            -2,                              // no trust line for what we're selling
//        MANAGE_SELL_OFFER_BUY_NO_TRUST = -3, // no trust line for what we're buying
//        MANAGE_SELL_OFFER_SELL_NOT_AUTHORIZED = -4, // not authorized to sell
//        MANAGE_SELL_OFFER_BUY_NOT_AUTHORIZED = -5,  // not authorized to buy
//        MANAGE_SELL_OFFER_LINE_FULL = -6, // can't receive more of what it's buying
//        MANAGE_SELL_OFFER_UNDERFUNDED = -7, // doesn't hold what it's trying to sell
//        MANAGE_SELL_OFFER_CROSS_SELF =
//            -8, // would cross an offer from the same user
//        MANAGE_SELL_OFFER_SELL_NO_ISSUER = -9, // no issuer for what we're selling
//        MANAGE_SELL_OFFER_BUY_NO_ISSUER = -10, // no issuer for what we're buying
//
//        // update errors
//        MANAGE_SELL_OFFER_NOT_FOUND =
//            -11, // offerID does not match an existing offer
//
//        MANAGE_SELL_OFFER_LOW_RESERVE =
//            -12 // not enough funds to create a new Offer
//    };
//
// enum
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum ManageSellOfferResultCode {
    Success = 0,
    Malformed = -1,
    SellNoTrust = -2,
    BuyNoTrust = -3,
    SellNotAuthorized = -4,
    BuyNotAuthorized = -5,
    LineFull = -6,
    Underfunded = -7,
    CrossSelf = -8,
    SellNoIssuer = -9,
    BuyNoIssuer = -10,
    NotFound = -11,
    LowReserve = -12,
}

impl ManageSellOfferResultCode {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => "Success",
            Self::Malformed => "Malformed",
            Self::SellNoTrust => "SellNoTrust",
            Self::BuyNoTrust => "BuyNoTrust",
            Self::SellNotAuthorized => "SellNotAuthorized",
            Self::BuyNotAuthorized => "BuyNotAuthorized",
            Self::LineFull => "LineFull",
            Self::Underfunded => "Underfunded",
            Self::CrossSelf => "CrossSelf",
            Self::SellNoIssuer => "SellNoIssuer",
            Self::BuyNoIssuer => "BuyNoIssuer",
            Self::NotFound => "NotFound",
            Self::LowReserve => "LowReserve",
        }
    }
}

impl fmt::Display for ManageSellOfferResultCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for ManageSellOfferResultCode {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self> {
        let e = match i {
            0 => ManageSellOfferResultCode::Success,
            -1 => ManageSellOfferResultCode::Malformed,
            -2 => ManageSellOfferResultCode::SellNoTrust,
            -3 => ManageSellOfferResultCode::BuyNoTrust,
            -4 => ManageSellOfferResultCode::SellNotAuthorized,
            -5 => ManageSellOfferResultCode::BuyNotAuthorized,
            -6 => ManageSellOfferResultCode::LineFull,
            -7 => ManageSellOfferResultCode::Underfunded,
            -8 => ManageSellOfferResultCode::CrossSelf,
            -9 => ManageSellOfferResultCode::SellNoIssuer,
            -10 => ManageSellOfferResultCode::BuyNoIssuer,
            -11 => ManageSellOfferResultCode::NotFound,
            -12 => ManageSellOfferResultCode::LowReserve,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<ManageSellOfferResultCode> for i32 {
    #[must_use]
    fn from(e: ManageSellOfferResultCode) -> Self {
        e as Self
    }
}

impl ReadXdr for ManageSellOfferResultCode {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXdr for ManageSellOfferResultCode {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// ManageOfferEffect is an XDR Enum defines as:
//
//   enum ManageOfferEffect
//    {
//        MANAGE_OFFER_CREATED = 0,
//        MANAGE_OFFER_UPDATED = 1,
//        MANAGE_OFFER_DELETED = 2
//    };
//
// enum
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum ManageOfferEffect {
    Created = 0,
    Updated = 1,
    Deleted = 2,
}

impl ManageOfferEffect {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Created => "Created",
            Self::Updated => "Updated",
            Self::Deleted => "Deleted",
        }
    }
}

impl fmt::Display for ManageOfferEffect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for ManageOfferEffect {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self> {
        let e = match i {
            0 => ManageOfferEffect::Created,
            1 => ManageOfferEffect::Updated,
            2 => ManageOfferEffect::Deleted,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<ManageOfferEffect> for i32 {
    #[must_use]
    fn from(e: ManageOfferEffect) -> Self {
        e as Self
    }
}

impl ReadXdr for ManageOfferEffect {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXdr for ManageOfferEffect {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// ManageOfferSuccessResultOffer is an XDR NestedUnion defines as:
//
//   union switch (ManageOfferEffect effect)
//        {
//        case MANAGE_OFFER_CREATED:
//        case MANAGE_OFFER_UPDATED:
//            OfferEntry offer;
//        case MANAGE_OFFER_DELETED:
//            void;
//        }
//
// union with discriminant ManageOfferEffect
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ManageOfferSuccessResultOffer {
    Created(OfferEntry),
    Updated(OfferEntry),
    Deleted,
}

impl ManageOfferSuccessResultOffer {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Created(_) => "Created",
            Self::Updated(_) => "Updated",
            Self::Deleted => "Deleted",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> ManageOfferEffect {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Created(_) => ManageOfferEffect::Created,
            Self::Updated(_) => ManageOfferEffect::Updated,
            Self::Deleted => ManageOfferEffect::Deleted,
        }
    }
}

impl ReadXdr for ManageOfferSuccessResultOffer {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: ManageOfferEffect = <ManageOfferEffect as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            ManageOfferEffect::Created => Self::Created(OfferEntry::read_xdr(r)?),
            ManageOfferEffect::Updated => Self::Updated(OfferEntry::read_xdr(r)?),
            ManageOfferEffect::Deleted => Self::Deleted,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for ManageOfferSuccessResultOffer {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Created(v) => v.write_xdr(w)?,
            Self::Updated(v) => v.write_xdr(w)?,
            Self::Deleted => ().write_xdr(w)?,
        };
        Ok(())
    }
}

// ManageOfferSuccessResult is an XDR Struct defines as:
//
//   struct ManageOfferSuccessResult
//    {
//        // offers that got claimed while creating this offer
//        ClaimAtom offersClaimed<>;
//
//        union switch (ManageOfferEffect effect)
//        {
//        case MANAGE_OFFER_CREATED:
//        case MANAGE_OFFER_UPDATED:
//            OfferEntry offer;
//        case MANAGE_OFFER_DELETED:
//            void;
//        }
//        offer;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ManageOfferSuccessResult {
    pub offers_claimed: VecM<ClaimAtom>,
    pub offer: ManageOfferSuccessResultOffer,
}

impl ReadXdr for ManageOfferSuccessResult {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            offers_claimed: VecM::<ClaimAtom>::read_xdr(r)?,
            offer: ManageOfferSuccessResultOffer::read_xdr(r)?,
        })
    }
}

impl WriteXdr for ManageOfferSuccessResult {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.offers_claimed.write_xdr(w)?;
        self.offer.write_xdr(w)?;
        Ok(())
    }
}

// ManageSellOfferResult is an XDR Union defines as:
//
//   union ManageSellOfferResult switch (ManageSellOfferResultCode code)
//    {
//    case MANAGE_SELL_OFFER_SUCCESS:
//        ManageOfferSuccessResult success;
//    case MANAGE_SELL_OFFER_MALFORMED:
//    case MANAGE_SELL_OFFER_SELL_NO_TRUST:
//    case MANAGE_SELL_OFFER_BUY_NO_TRUST:
//    case MANAGE_SELL_OFFER_SELL_NOT_AUTHORIZED:
//    case MANAGE_SELL_OFFER_BUY_NOT_AUTHORIZED:
//    case MANAGE_SELL_OFFER_LINE_FULL:
//    case MANAGE_SELL_OFFER_UNDERFUNDED:
//    case MANAGE_SELL_OFFER_CROSS_SELF:
//    case MANAGE_SELL_OFFER_SELL_NO_ISSUER:
//    case MANAGE_SELL_OFFER_BUY_NO_ISSUER:
//    case MANAGE_SELL_OFFER_NOT_FOUND:
//    case MANAGE_SELL_OFFER_LOW_RESERVE:
//        void;
//    };
//
// union with discriminant ManageSellOfferResultCode
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ManageSellOfferResult {
    Success(ManageOfferSuccessResult),
    Malformed,
    SellNoTrust,
    BuyNoTrust,
    SellNotAuthorized,
    BuyNotAuthorized,
    LineFull,
    Underfunded,
    CrossSelf,
    SellNoIssuer,
    BuyNoIssuer,
    NotFound,
    LowReserve,
}

impl ManageSellOfferResult {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success(_) => "Success",
            Self::Malformed => "Malformed",
            Self::SellNoTrust => "SellNoTrust",
            Self::BuyNoTrust => "BuyNoTrust",
            Self::SellNotAuthorized => "SellNotAuthorized",
            Self::BuyNotAuthorized => "BuyNotAuthorized",
            Self::LineFull => "LineFull",
            Self::Underfunded => "Underfunded",
            Self::CrossSelf => "CrossSelf",
            Self::SellNoIssuer => "SellNoIssuer",
            Self::BuyNoIssuer => "BuyNoIssuer",
            Self::NotFound => "NotFound",
            Self::LowReserve => "LowReserve",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> ManageSellOfferResultCode {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success(_) => ManageSellOfferResultCode::Success,
            Self::Malformed => ManageSellOfferResultCode::Malformed,
            Self::SellNoTrust => ManageSellOfferResultCode::SellNoTrust,
            Self::BuyNoTrust => ManageSellOfferResultCode::BuyNoTrust,
            Self::SellNotAuthorized => ManageSellOfferResultCode::SellNotAuthorized,
            Self::BuyNotAuthorized => ManageSellOfferResultCode::BuyNotAuthorized,
            Self::LineFull => ManageSellOfferResultCode::LineFull,
            Self::Underfunded => ManageSellOfferResultCode::Underfunded,
            Self::CrossSelf => ManageSellOfferResultCode::CrossSelf,
            Self::SellNoIssuer => ManageSellOfferResultCode::SellNoIssuer,
            Self::BuyNoIssuer => ManageSellOfferResultCode::BuyNoIssuer,
            Self::NotFound => ManageSellOfferResultCode::NotFound,
            Self::LowReserve => ManageSellOfferResultCode::LowReserve,
        }
    }
}

impl ReadXdr for ManageSellOfferResult {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: ManageSellOfferResultCode = <ManageSellOfferResultCode as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            ManageSellOfferResultCode::Success => {
                Self::Success(ManageOfferSuccessResult::read_xdr(r)?)
            }
            ManageSellOfferResultCode::Malformed => Self::Malformed,
            ManageSellOfferResultCode::SellNoTrust => Self::SellNoTrust,
            ManageSellOfferResultCode::BuyNoTrust => Self::BuyNoTrust,
            ManageSellOfferResultCode::SellNotAuthorized => Self::SellNotAuthorized,
            ManageSellOfferResultCode::BuyNotAuthorized => Self::BuyNotAuthorized,
            ManageSellOfferResultCode::LineFull => Self::LineFull,
            ManageSellOfferResultCode::Underfunded => Self::Underfunded,
            ManageSellOfferResultCode::CrossSelf => Self::CrossSelf,
            ManageSellOfferResultCode::SellNoIssuer => Self::SellNoIssuer,
            ManageSellOfferResultCode::BuyNoIssuer => Self::BuyNoIssuer,
            ManageSellOfferResultCode::NotFound => Self::NotFound,
            ManageSellOfferResultCode::LowReserve => Self::LowReserve,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for ManageSellOfferResult {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success(v) => v.write_xdr(w)?,
            Self::Malformed => ().write_xdr(w)?,
            Self::SellNoTrust => ().write_xdr(w)?,
            Self::BuyNoTrust => ().write_xdr(w)?,
            Self::SellNotAuthorized => ().write_xdr(w)?,
            Self::BuyNotAuthorized => ().write_xdr(w)?,
            Self::LineFull => ().write_xdr(w)?,
            Self::Underfunded => ().write_xdr(w)?,
            Self::CrossSelf => ().write_xdr(w)?,
            Self::SellNoIssuer => ().write_xdr(w)?,
            Self::BuyNoIssuer => ().write_xdr(w)?,
            Self::NotFound => ().write_xdr(w)?,
            Self::LowReserve => ().write_xdr(w)?,
        };
        Ok(())
    }
}

// ManageBuyOfferResultCode is an XDR Enum defines as:
//
//   enum ManageBuyOfferResultCode
//    {
//        // codes considered as "success" for the operation
//        MANAGE_BUY_OFFER_SUCCESS = 0,
//
//        // codes considered as "failure" for the operation
//        MANAGE_BUY_OFFER_MALFORMED = -1,     // generated offer would be invalid
//        MANAGE_BUY_OFFER_SELL_NO_TRUST = -2, // no trust line for what we're selling
//        MANAGE_BUY_OFFER_BUY_NO_TRUST = -3,  // no trust line for what we're buying
//        MANAGE_BUY_OFFER_SELL_NOT_AUTHORIZED = -4, // not authorized to sell
//        MANAGE_BUY_OFFER_BUY_NOT_AUTHORIZED = -5,  // not authorized to buy
//        MANAGE_BUY_OFFER_LINE_FULL = -6,   // can't receive more of what it's buying
//        MANAGE_BUY_OFFER_UNDERFUNDED = -7, // doesn't hold what it's trying to sell
//        MANAGE_BUY_OFFER_CROSS_SELF = -8, // would cross an offer from the same user
//        MANAGE_BUY_OFFER_SELL_NO_ISSUER = -9, // no issuer for what we're selling
//        MANAGE_BUY_OFFER_BUY_NO_ISSUER = -10, // no issuer for what we're buying
//
//        // update errors
//        MANAGE_BUY_OFFER_NOT_FOUND =
//            -11, // offerID does not match an existing offer
//
//        MANAGE_BUY_OFFER_LOW_RESERVE = -12 // not enough funds to create a new Offer
//    };
//
// enum
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum ManageBuyOfferResultCode {
    Success = 0,
    Malformed = -1,
    SellNoTrust = -2,
    BuyNoTrust = -3,
    SellNotAuthorized = -4,
    BuyNotAuthorized = -5,
    LineFull = -6,
    Underfunded = -7,
    CrossSelf = -8,
    SellNoIssuer = -9,
    BuyNoIssuer = -10,
    NotFound = -11,
    LowReserve = -12,
}

impl ManageBuyOfferResultCode {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => "Success",
            Self::Malformed => "Malformed",
            Self::SellNoTrust => "SellNoTrust",
            Self::BuyNoTrust => "BuyNoTrust",
            Self::SellNotAuthorized => "SellNotAuthorized",
            Self::BuyNotAuthorized => "BuyNotAuthorized",
            Self::LineFull => "LineFull",
            Self::Underfunded => "Underfunded",
            Self::CrossSelf => "CrossSelf",
            Self::SellNoIssuer => "SellNoIssuer",
            Self::BuyNoIssuer => "BuyNoIssuer",
            Self::NotFound => "NotFound",
            Self::LowReserve => "LowReserve",
        }
    }
}

impl fmt::Display for ManageBuyOfferResultCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for ManageBuyOfferResultCode {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self> {
        let e = match i {
            0 => ManageBuyOfferResultCode::Success,
            -1 => ManageBuyOfferResultCode::Malformed,
            -2 => ManageBuyOfferResultCode::SellNoTrust,
            -3 => ManageBuyOfferResultCode::BuyNoTrust,
            -4 => ManageBuyOfferResultCode::SellNotAuthorized,
            -5 => ManageBuyOfferResultCode::BuyNotAuthorized,
            -6 => ManageBuyOfferResultCode::LineFull,
            -7 => ManageBuyOfferResultCode::Underfunded,
            -8 => ManageBuyOfferResultCode::CrossSelf,
            -9 => ManageBuyOfferResultCode::SellNoIssuer,
            -10 => ManageBuyOfferResultCode::BuyNoIssuer,
            -11 => ManageBuyOfferResultCode::NotFound,
            -12 => ManageBuyOfferResultCode::LowReserve,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<ManageBuyOfferResultCode> for i32 {
    #[must_use]
    fn from(e: ManageBuyOfferResultCode) -> Self {
        e as Self
    }
}

impl ReadXdr for ManageBuyOfferResultCode {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXdr for ManageBuyOfferResultCode {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// ManageBuyOfferResult is an XDR Union defines as:
//
//   union ManageBuyOfferResult switch (ManageBuyOfferResultCode code)
//    {
//    case MANAGE_BUY_OFFER_SUCCESS:
//        ManageOfferSuccessResult success;
//    case MANAGE_BUY_OFFER_MALFORMED:
//    case MANAGE_BUY_OFFER_SELL_NO_TRUST:
//    case MANAGE_BUY_OFFER_BUY_NO_TRUST:
//    case MANAGE_BUY_OFFER_SELL_NOT_AUTHORIZED:
//    case MANAGE_BUY_OFFER_BUY_NOT_AUTHORIZED:
//    case MANAGE_BUY_OFFER_LINE_FULL:
//    case MANAGE_BUY_OFFER_UNDERFUNDED:
//    case MANAGE_BUY_OFFER_CROSS_SELF:
//    case MANAGE_BUY_OFFER_SELL_NO_ISSUER:
//    case MANAGE_BUY_OFFER_BUY_NO_ISSUER:
//    case MANAGE_BUY_OFFER_NOT_FOUND:
//    case MANAGE_BUY_OFFER_LOW_RESERVE:
//        void;
//    };
//
// union with discriminant ManageBuyOfferResultCode
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ManageBuyOfferResult {
    Success(ManageOfferSuccessResult),
    Malformed,
    SellNoTrust,
    BuyNoTrust,
    SellNotAuthorized,
    BuyNotAuthorized,
    LineFull,
    Underfunded,
    CrossSelf,
    SellNoIssuer,
    BuyNoIssuer,
    NotFound,
    LowReserve,
}

impl ManageBuyOfferResult {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success(_) => "Success",
            Self::Malformed => "Malformed",
            Self::SellNoTrust => "SellNoTrust",
            Self::BuyNoTrust => "BuyNoTrust",
            Self::SellNotAuthorized => "SellNotAuthorized",
            Self::BuyNotAuthorized => "BuyNotAuthorized",
            Self::LineFull => "LineFull",
            Self::Underfunded => "Underfunded",
            Self::CrossSelf => "CrossSelf",
            Self::SellNoIssuer => "SellNoIssuer",
            Self::BuyNoIssuer => "BuyNoIssuer",
            Self::NotFound => "NotFound",
            Self::LowReserve => "LowReserve",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> ManageBuyOfferResultCode {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success(_) => ManageBuyOfferResultCode::Success,
            Self::Malformed => ManageBuyOfferResultCode::Malformed,
            Self::SellNoTrust => ManageBuyOfferResultCode::SellNoTrust,
            Self::BuyNoTrust => ManageBuyOfferResultCode::BuyNoTrust,
            Self::SellNotAuthorized => ManageBuyOfferResultCode::SellNotAuthorized,
            Self::BuyNotAuthorized => ManageBuyOfferResultCode::BuyNotAuthorized,
            Self::LineFull => ManageBuyOfferResultCode::LineFull,
            Self::Underfunded => ManageBuyOfferResultCode::Underfunded,
            Self::CrossSelf => ManageBuyOfferResultCode::CrossSelf,
            Self::SellNoIssuer => ManageBuyOfferResultCode::SellNoIssuer,
            Self::BuyNoIssuer => ManageBuyOfferResultCode::BuyNoIssuer,
            Self::NotFound => ManageBuyOfferResultCode::NotFound,
            Self::LowReserve => ManageBuyOfferResultCode::LowReserve,
        }
    }
}

impl ReadXdr for ManageBuyOfferResult {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: ManageBuyOfferResultCode = <ManageBuyOfferResultCode as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            ManageBuyOfferResultCode::Success => {
                Self::Success(ManageOfferSuccessResult::read_xdr(r)?)
            }
            ManageBuyOfferResultCode::Malformed => Self::Malformed,
            ManageBuyOfferResultCode::SellNoTrust => Self::SellNoTrust,
            ManageBuyOfferResultCode::BuyNoTrust => Self::BuyNoTrust,
            ManageBuyOfferResultCode::SellNotAuthorized => Self::SellNotAuthorized,
            ManageBuyOfferResultCode::BuyNotAuthorized => Self::BuyNotAuthorized,
            ManageBuyOfferResultCode::LineFull => Self::LineFull,
            ManageBuyOfferResultCode::Underfunded => Self::Underfunded,
            ManageBuyOfferResultCode::CrossSelf => Self::CrossSelf,
            ManageBuyOfferResultCode::SellNoIssuer => Self::SellNoIssuer,
            ManageBuyOfferResultCode::BuyNoIssuer => Self::BuyNoIssuer,
            ManageBuyOfferResultCode::NotFound => Self::NotFound,
            ManageBuyOfferResultCode::LowReserve => Self::LowReserve,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for ManageBuyOfferResult {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success(v) => v.write_xdr(w)?,
            Self::Malformed => ().write_xdr(w)?,
            Self::SellNoTrust => ().write_xdr(w)?,
            Self::BuyNoTrust => ().write_xdr(w)?,
            Self::SellNotAuthorized => ().write_xdr(w)?,
            Self::BuyNotAuthorized => ().write_xdr(w)?,
            Self::LineFull => ().write_xdr(w)?,
            Self::Underfunded => ().write_xdr(w)?,
            Self::CrossSelf => ().write_xdr(w)?,
            Self::SellNoIssuer => ().write_xdr(w)?,
            Self::BuyNoIssuer => ().write_xdr(w)?,
            Self::NotFound => ().write_xdr(w)?,
            Self::LowReserve => ().write_xdr(w)?,
        };
        Ok(())
    }
}

// SetOptionsResultCode is an XDR Enum defines as:
//
//   enum SetOptionsResultCode
//    {
//        // codes considered as "success" for the operation
//        SET_OPTIONS_SUCCESS = 0,
//        // codes considered as "failure" for the operation
//        SET_OPTIONS_LOW_RESERVE = -1,      // not enough funds to add a signer
//        SET_OPTIONS_TOO_MANY_SIGNERS = -2, // max number of signers already reached
//        SET_OPTIONS_BAD_FLAGS = -3,        // invalid combination of clear/set flags
//        SET_OPTIONS_INVALID_INFLATION = -4,      // inflation account does not exist
//        SET_OPTIONS_CANT_CHANGE = -5,            // can no longer change this option
//        SET_OPTIONS_UNKNOWN_FLAG = -6,           // can't set an unknown flag
//        SET_OPTIONS_THRESHOLD_OUT_OF_RANGE = -7, // bad value for weight/threshold
//        SET_OPTIONS_BAD_SIGNER = -8,             // signer cannot be masterkey
//        SET_OPTIONS_INVALID_HOME_DOMAIN = -9,    // malformed home domain
//        SET_OPTIONS_AUTH_REVOCABLE_REQUIRED =
//            -10 // auth revocable is required for clawback
//    };
//
// enum
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum SetOptionsResultCode {
    Success = 0,
    LowReserve = -1,
    TooManySigners = -2,
    BadFlags = -3,
    InvalidInflation = -4,
    CantChange = -5,
    UnknownFlag = -6,
    ThresholdOutOfRange = -7,
    BadSigner = -8,
    InvalidHomeDomain = -9,
    AuthRevocableRequired = -10,
}

impl SetOptionsResultCode {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => "Success",
            Self::LowReserve => "LowReserve",
            Self::TooManySigners => "TooManySigners",
            Self::BadFlags => "BadFlags",
            Self::InvalidInflation => "InvalidInflation",
            Self::CantChange => "CantChange",
            Self::UnknownFlag => "UnknownFlag",
            Self::ThresholdOutOfRange => "ThresholdOutOfRange",
            Self::BadSigner => "BadSigner",
            Self::InvalidHomeDomain => "InvalidHomeDomain",
            Self::AuthRevocableRequired => "AuthRevocableRequired",
        }
    }
}

impl fmt::Display for SetOptionsResultCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for SetOptionsResultCode {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self> {
        let e = match i {
            0 => SetOptionsResultCode::Success,
            -1 => SetOptionsResultCode::LowReserve,
            -2 => SetOptionsResultCode::TooManySigners,
            -3 => SetOptionsResultCode::BadFlags,
            -4 => SetOptionsResultCode::InvalidInflation,
            -5 => SetOptionsResultCode::CantChange,
            -6 => SetOptionsResultCode::UnknownFlag,
            -7 => SetOptionsResultCode::ThresholdOutOfRange,
            -8 => SetOptionsResultCode::BadSigner,
            -9 => SetOptionsResultCode::InvalidHomeDomain,
            -10 => SetOptionsResultCode::AuthRevocableRequired,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<SetOptionsResultCode> for i32 {
    #[must_use]
    fn from(e: SetOptionsResultCode) -> Self {
        e as Self
    }
}

impl ReadXdr for SetOptionsResultCode {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXdr for SetOptionsResultCode {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// SetOptionsResult is an XDR Union defines as:
//
//   union SetOptionsResult switch (SetOptionsResultCode code)
//    {
//    case SET_OPTIONS_SUCCESS:
//        void;
//    case SET_OPTIONS_LOW_RESERVE:
//    case SET_OPTIONS_TOO_MANY_SIGNERS:
//    case SET_OPTIONS_BAD_FLAGS:
//    case SET_OPTIONS_INVALID_INFLATION:
//    case SET_OPTIONS_CANT_CHANGE:
//    case SET_OPTIONS_UNKNOWN_FLAG:
//    case SET_OPTIONS_THRESHOLD_OUT_OF_RANGE:
//    case SET_OPTIONS_BAD_SIGNER:
//    case SET_OPTIONS_INVALID_HOME_DOMAIN:
//    case SET_OPTIONS_AUTH_REVOCABLE_REQUIRED:
//        void;
//    };
//
// union with discriminant SetOptionsResultCode
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum SetOptionsResult {
    Success,
    LowReserve,
    TooManySigners,
    BadFlags,
    InvalidInflation,
    CantChange,
    UnknownFlag,
    ThresholdOutOfRange,
    BadSigner,
    InvalidHomeDomain,
    AuthRevocableRequired,
}

impl SetOptionsResult {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => "Success",
            Self::LowReserve => "LowReserve",
            Self::TooManySigners => "TooManySigners",
            Self::BadFlags => "BadFlags",
            Self::InvalidInflation => "InvalidInflation",
            Self::CantChange => "CantChange",
            Self::UnknownFlag => "UnknownFlag",
            Self::ThresholdOutOfRange => "ThresholdOutOfRange",
            Self::BadSigner => "BadSigner",
            Self::InvalidHomeDomain => "InvalidHomeDomain",
            Self::AuthRevocableRequired => "AuthRevocableRequired",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> SetOptionsResultCode {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => SetOptionsResultCode::Success,
            Self::LowReserve => SetOptionsResultCode::LowReserve,
            Self::TooManySigners => SetOptionsResultCode::TooManySigners,
            Self::BadFlags => SetOptionsResultCode::BadFlags,
            Self::InvalidInflation => SetOptionsResultCode::InvalidInflation,
            Self::CantChange => SetOptionsResultCode::CantChange,
            Self::UnknownFlag => SetOptionsResultCode::UnknownFlag,
            Self::ThresholdOutOfRange => SetOptionsResultCode::ThresholdOutOfRange,
            Self::BadSigner => SetOptionsResultCode::BadSigner,
            Self::InvalidHomeDomain => SetOptionsResultCode::InvalidHomeDomain,
            Self::AuthRevocableRequired => SetOptionsResultCode::AuthRevocableRequired,
        }
    }
}

impl ReadXdr for SetOptionsResult {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: SetOptionsResultCode = <SetOptionsResultCode as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            SetOptionsResultCode::Success => Self::Success,
            SetOptionsResultCode::LowReserve => Self::LowReserve,
            SetOptionsResultCode::TooManySigners => Self::TooManySigners,
            SetOptionsResultCode::BadFlags => Self::BadFlags,
            SetOptionsResultCode::InvalidInflation => Self::InvalidInflation,
            SetOptionsResultCode::CantChange => Self::CantChange,
            SetOptionsResultCode::UnknownFlag => Self::UnknownFlag,
            SetOptionsResultCode::ThresholdOutOfRange => Self::ThresholdOutOfRange,
            SetOptionsResultCode::BadSigner => Self::BadSigner,
            SetOptionsResultCode::InvalidHomeDomain => Self::InvalidHomeDomain,
            SetOptionsResultCode::AuthRevocableRequired => Self::AuthRevocableRequired,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for SetOptionsResult {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => ().write_xdr(w)?,
            Self::LowReserve => ().write_xdr(w)?,
            Self::TooManySigners => ().write_xdr(w)?,
            Self::BadFlags => ().write_xdr(w)?,
            Self::InvalidInflation => ().write_xdr(w)?,
            Self::CantChange => ().write_xdr(w)?,
            Self::UnknownFlag => ().write_xdr(w)?,
            Self::ThresholdOutOfRange => ().write_xdr(w)?,
            Self::BadSigner => ().write_xdr(w)?,
            Self::InvalidHomeDomain => ().write_xdr(w)?,
            Self::AuthRevocableRequired => ().write_xdr(w)?,
        };
        Ok(())
    }
}

// ChangeTrustResultCode is an XDR Enum defines as:
//
//   enum ChangeTrustResultCode
//    {
//        // codes considered as "success" for the operation
//        CHANGE_TRUST_SUCCESS = 0,
//        // codes considered as "failure" for the operation
//        CHANGE_TRUST_MALFORMED = -1,     // bad input
//        CHANGE_TRUST_NO_ISSUER = -2,     // could not find issuer
//        CHANGE_TRUST_INVALID_LIMIT = -3, // cannot drop limit below balance
//                                         // cannot create with a limit of 0
//        CHANGE_TRUST_LOW_RESERVE =
//            -4, // not enough funds to create a new trust line,
//        CHANGE_TRUST_SELF_NOT_ALLOWED = -5,   // trusting self is not allowed
//        CHANGE_TRUST_TRUST_LINE_MISSING = -6, // Asset trustline is missing for pool
//        CHANGE_TRUST_CANNOT_DELETE =
//            -7, // Asset trustline is still referenced in a pool
//        CHANGE_TRUST_NOT_AUTH_MAINTAIN_LIABILITIES =
//            -8 // Asset trustline is deauthorized
//    };
//
// enum
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum ChangeTrustResultCode {
    Success = 0,
    Malformed = -1,
    NoIssuer = -2,
    InvalidLimit = -3,
    LowReserve = -4,
    SelfNotAllowed = -5,
    TrustLineMissing = -6,
    CannotDelete = -7,
    NotAuthMaintainLiabilities = -8,
}

impl ChangeTrustResultCode {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => "Success",
            Self::Malformed => "Malformed",
            Self::NoIssuer => "NoIssuer",
            Self::InvalidLimit => "InvalidLimit",
            Self::LowReserve => "LowReserve",
            Self::SelfNotAllowed => "SelfNotAllowed",
            Self::TrustLineMissing => "TrustLineMissing",
            Self::CannotDelete => "CannotDelete",
            Self::NotAuthMaintainLiabilities => "NotAuthMaintainLiabilities",
        }
    }
}

impl fmt::Display for ChangeTrustResultCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for ChangeTrustResultCode {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self> {
        let e = match i {
            0 => ChangeTrustResultCode::Success,
            -1 => ChangeTrustResultCode::Malformed,
            -2 => ChangeTrustResultCode::NoIssuer,
            -3 => ChangeTrustResultCode::InvalidLimit,
            -4 => ChangeTrustResultCode::LowReserve,
            -5 => ChangeTrustResultCode::SelfNotAllowed,
            -6 => ChangeTrustResultCode::TrustLineMissing,
            -7 => ChangeTrustResultCode::CannotDelete,
            -8 => ChangeTrustResultCode::NotAuthMaintainLiabilities,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<ChangeTrustResultCode> for i32 {
    #[must_use]
    fn from(e: ChangeTrustResultCode) -> Self {
        e as Self
    }
}

impl ReadXdr for ChangeTrustResultCode {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXdr for ChangeTrustResultCode {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// ChangeTrustResult is an XDR Union defines as:
//
//   union ChangeTrustResult switch (ChangeTrustResultCode code)
//    {
//    case CHANGE_TRUST_SUCCESS:
//        void;
//    case CHANGE_TRUST_MALFORMED:
//    case CHANGE_TRUST_NO_ISSUER:
//    case CHANGE_TRUST_INVALID_LIMIT:
//    case CHANGE_TRUST_LOW_RESERVE:
//    case CHANGE_TRUST_SELF_NOT_ALLOWED:
//    case CHANGE_TRUST_TRUST_LINE_MISSING:
//    case CHANGE_TRUST_CANNOT_DELETE:
//    case CHANGE_TRUST_NOT_AUTH_MAINTAIN_LIABILITIES:
//        void;
//    };
//
// union with discriminant ChangeTrustResultCode
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ChangeTrustResult {
    Success,
    Malformed,
    NoIssuer,
    InvalidLimit,
    LowReserve,
    SelfNotAllowed,
    TrustLineMissing,
    CannotDelete,
    NotAuthMaintainLiabilities,
}

impl ChangeTrustResult {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => "Success",
            Self::Malformed => "Malformed",
            Self::NoIssuer => "NoIssuer",
            Self::InvalidLimit => "InvalidLimit",
            Self::LowReserve => "LowReserve",
            Self::SelfNotAllowed => "SelfNotAllowed",
            Self::TrustLineMissing => "TrustLineMissing",
            Self::CannotDelete => "CannotDelete",
            Self::NotAuthMaintainLiabilities => "NotAuthMaintainLiabilities",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> ChangeTrustResultCode {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => ChangeTrustResultCode::Success,
            Self::Malformed => ChangeTrustResultCode::Malformed,
            Self::NoIssuer => ChangeTrustResultCode::NoIssuer,
            Self::InvalidLimit => ChangeTrustResultCode::InvalidLimit,
            Self::LowReserve => ChangeTrustResultCode::LowReserve,
            Self::SelfNotAllowed => ChangeTrustResultCode::SelfNotAllowed,
            Self::TrustLineMissing => ChangeTrustResultCode::TrustLineMissing,
            Self::CannotDelete => ChangeTrustResultCode::CannotDelete,
            Self::NotAuthMaintainLiabilities => ChangeTrustResultCode::NotAuthMaintainLiabilities,
        }
    }
}

impl ReadXdr for ChangeTrustResult {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: ChangeTrustResultCode = <ChangeTrustResultCode as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            ChangeTrustResultCode::Success => Self::Success,
            ChangeTrustResultCode::Malformed => Self::Malformed,
            ChangeTrustResultCode::NoIssuer => Self::NoIssuer,
            ChangeTrustResultCode::InvalidLimit => Self::InvalidLimit,
            ChangeTrustResultCode::LowReserve => Self::LowReserve,
            ChangeTrustResultCode::SelfNotAllowed => Self::SelfNotAllowed,
            ChangeTrustResultCode::TrustLineMissing => Self::TrustLineMissing,
            ChangeTrustResultCode::CannotDelete => Self::CannotDelete,
            ChangeTrustResultCode::NotAuthMaintainLiabilities => Self::NotAuthMaintainLiabilities,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for ChangeTrustResult {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => ().write_xdr(w)?,
            Self::Malformed => ().write_xdr(w)?,
            Self::NoIssuer => ().write_xdr(w)?,
            Self::InvalidLimit => ().write_xdr(w)?,
            Self::LowReserve => ().write_xdr(w)?,
            Self::SelfNotAllowed => ().write_xdr(w)?,
            Self::TrustLineMissing => ().write_xdr(w)?,
            Self::CannotDelete => ().write_xdr(w)?,
            Self::NotAuthMaintainLiabilities => ().write_xdr(w)?,
        };
        Ok(())
    }
}

// AllowTrustResultCode is an XDR Enum defines as:
//
//   enum AllowTrustResultCode
//    {
//        // codes considered as "success" for the operation
//        ALLOW_TRUST_SUCCESS = 0,
//        // codes considered as "failure" for the operation
//        ALLOW_TRUST_MALFORMED = -1,     // asset is not ASSET_TYPE_ALPHANUM
//        ALLOW_TRUST_NO_TRUST_LINE = -2, // trustor does not have a trustline
//                                        // source account does not require trust
//        ALLOW_TRUST_TRUST_NOT_REQUIRED = -3,
//        ALLOW_TRUST_CANT_REVOKE = -4,      // source account can't revoke trust,
//        ALLOW_TRUST_SELF_NOT_ALLOWED = -5, // trusting self is not allowed
//        ALLOW_TRUST_LOW_RESERVE = -6       // claimable balances can't be created
//                                           // on revoke due to low reserves
//    };
//
// enum
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum AllowTrustResultCode {
    Success = 0,
    Malformed = -1,
    NoTrustLine = -2,
    TrustNotRequired = -3,
    CantRevoke = -4,
    SelfNotAllowed = -5,
    LowReserve = -6,
}

impl AllowTrustResultCode {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => "Success",
            Self::Malformed => "Malformed",
            Self::NoTrustLine => "NoTrustLine",
            Self::TrustNotRequired => "TrustNotRequired",
            Self::CantRevoke => "CantRevoke",
            Self::SelfNotAllowed => "SelfNotAllowed",
            Self::LowReserve => "LowReserve",
        }
    }
}

impl fmt::Display for AllowTrustResultCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for AllowTrustResultCode {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self> {
        let e = match i {
            0 => AllowTrustResultCode::Success,
            -1 => AllowTrustResultCode::Malformed,
            -2 => AllowTrustResultCode::NoTrustLine,
            -3 => AllowTrustResultCode::TrustNotRequired,
            -4 => AllowTrustResultCode::CantRevoke,
            -5 => AllowTrustResultCode::SelfNotAllowed,
            -6 => AllowTrustResultCode::LowReserve,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<AllowTrustResultCode> for i32 {
    #[must_use]
    fn from(e: AllowTrustResultCode) -> Self {
        e as Self
    }
}

impl ReadXdr for AllowTrustResultCode {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXdr for AllowTrustResultCode {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// AllowTrustResult is an XDR Union defines as:
//
//   union AllowTrustResult switch (AllowTrustResultCode code)
//    {
//    case ALLOW_TRUST_SUCCESS:
//        void;
//    case ALLOW_TRUST_MALFORMED:
//    case ALLOW_TRUST_NO_TRUST_LINE:
//    case ALLOW_TRUST_TRUST_NOT_REQUIRED:
//    case ALLOW_TRUST_CANT_REVOKE:
//    case ALLOW_TRUST_SELF_NOT_ALLOWED:
//    case ALLOW_TRUST_LOW_RESERVE:
//        void;
//    };
//
// union with discriminant AllowTrustResultCode
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum AllowTrustResult {
    Success,
    Malformed,
    NoTrustLine,
    TrustNotRequired,
    CantRevoke,
    SelfNotAllowed,
    LowReserve,
}

impl AllowTrustResult {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => "Success",
            Self::Malformed => "Malformed",
            Self::NoTrustLine => "NoTrustLine",
            Self::TrustNotRequired => "TrustNotRequired",
            Self::CantRevoke => "CantRevoke",
            Self::SelfNotAllowed => "SelfNotAllowed",
            Self::LowReserve => "LowReserve",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> AllowTrustResultCode {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => AllowTrustResultCode::Success,
            Self::Malformed => AllowTrustResultCode::Malformed,
            Self::NoTrustLine => AllowTrustResultCode::NoTrustLine,
            Self::TrustNotRequired => AllowTrustResultCode::TrustNotRequired,
            Self::CantRevoke => AllowTrustResultCode::CantRevoke,
            Self::SelfNotAllowed => AllowTrustResultCode::SelfNotAllowed,
            Self::LowReserve => AllowTrustResultCode::LowReserve,
        }
    }
}

impl ReadXdr for AllowTrustResult {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: AllowTrustResultCode = <AllowTrustResultCode as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            AllowTrustResultCode::Success => Self::Success,
            AllowTrustResultCode::Malformed => Self::Malformed,
            AllowTrustResultCode::NoTrustLine => Self::NoTrustLine,
            AllowTrustResultCode::TrustNotRequired => Self::TrustNotRequired,
            AllowTrustResultCode::CantRevoke => Self::CantRevoke,
            AllowTrustResultCode::SelfNotAllowed => Self::SelfNotAllowed,
            AllowTrustResultCode::LowReserve => Self::LowReserve,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for AllowTrustResult {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => ().write_xdr(w)?,
            Self::Malformed => ().write_xdr(w)?,
            Self::NoTrustLine => ().write_xdr(w)?,
            Self::TrustNotRequired => ().write_xdr(w)?,
            Self::CantRevoke => ().write_xdr(w)?,
            Self::SelfNotAllowed => ().write_xdr(w)?,
            Self::LowReserve => ().write_xdr(w)?,
        };
        Ok(())
    }
}

// AccountMergeResultCode is an XDR Enum defines as:
//
//   enum AccountMergeResultCode
//    {
//        // codes considered as "success" for the operation
//        ACCOUNT_MERGE_SUCCESS = 0,
//        // codes considered as "failure" for the operation
//        ACCOUNT_MERGE_MALFORMED = -1,       // can't merge onto itself
//        ACCOUNT_MERGE_NO_ACCOUNT = -2,      // destination does not exist
//        ACCOUNT_MERGE_IMMUTABLE_SET = -3,   // source account has AUTH_IMMUTABLE set
//        ACCOUNT_MERGE_HAS_SUB_ENTRIES = -4, // account has trust lines/offers
//        ACCOUNT_MERGE_SEQNUM_TOO_FAR = -5,  // sequence number is over max allowed
//        ACCOUNT_MERGE_DEST_FULL = -6,       // can't add source balance to
//                                            // destination balance
//        ACCOUNT_MERGE_IS_SPONSOR = -7       // can't merge account that is a sponsor
//    };
//
// enum
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum AccountMergeResultCode {
    Success = 0,
    Malformed = -1,
    NoAccount = -2,
    ImmutableSet = -3,
    HasSubEntries = -4,
    SeqnumTooFar = -5,
    DestFull = -6,
    IsSponsor = -7,
}

impl AccountMergeResultCode {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => "Success",
            Self::Malformed => "Malformed",
            Self::NoAccount => "NoAccount",
            Self::ImmutableSet => "ImmutableSet",
            Self::HasSubEntries => "HasSubEntries",
            Self::SeqnumTooFar => "SeqnumTooFar",
            Self::DestFull => "DestFull",
            Self::IsSponsor => "IsSponsor",
        }
    }
}

impl fmt::Display for AccountMergeResultCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for AccountMergeResultCode {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self> {
        let e = match i {
            0 => AccountMergeResultCode::Success,
            -1 => AccountMergeResultCode::Malformed,
            -2 => AccountMergeResultCode::NoAccount,
            -3 => AccountMergeResultCode::ImmutableSet,
            -4 => AccountMergeResultCode::HasSubEntries,
            -5 => AccountMergeResultCode::SeqnumTooFar,
            -6 => AccountMergeResultCode::DestFull,
            -7 => AccountMergeResultCode::IsSponsor,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<AccountMergeResultCode> for i32 {
    #[must_use]
    fn from(e: AccountMergeResultCode) -> Self {
        e as Self
    }
}

impl ReadXdr for AccountMergeResultCode {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXdr for AccountMergeResultCode {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// AccountMergeResult is an XDR Union defines as:
//
//   union AccountMergeResult switch (AccountMergeResultCode code)
//    {
//    case ACCOUNT_MERGE_SUCCESS:
//        int64 sourceAccountBalance; // how much got transferred from source account
//    case ACCOUNT_MERGE_MALFORMED:
//    case ACCOUNT_MERGE_NO_ACCOUNT:
//    case ACCOUNT_MERGE_IMMUTABLE_SET:
//    case ACCOUNT_MERGE_HAS_SUB_ENTRIES:
//    case ACCOUNT_MERGE_SEQNUM_TOO_FAR:
//    case ACCOUNT_MERGE_DEST_FULL:
//    case ACCOUNT_MERGE_IS_SPONSOR:
//        void;
//    };
//
// union with discriminant AccountMergeResultCode
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum AccountMergeResult {
    Success(i64),
    Malformed,
    NoAccount,
    ImmutableSet,
    HasSubEntries,
    SeqnumTooFar,
    DestFull,
    IsSponsor,
}

impl AccountMergeResult {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success(_) => "Success",
            Self::Malformed => "Malformed",
            Self::NoAccount => "NoAccount",
            Self::ImmutableSet => "ImmutableSet",
            Self::HasSubEntries => "HasSubEntries",
            Self::SeqnumTooFar => "SeqnumTooFar",
            Self::DestFull => "DestFull",
            Self::IsSponsor => "IsSponsor",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> AccountMergeResultCode {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success(_) => AccountMergeResultCode::Success,
            Self::Malformed => AccountMergeResultCode::Malformed,
            Self::NoAccount => AccountMergeResultCode::NoAccount,
            Self::ImmutableSet => AccountMergeResultCode::ImmutableSet,
            Self::HasSubEntries => AccountMergeResultCode::HasSubEntries,
            Self::SeqnumTooFar => AccountMergeResultCode::SeqnumTooFar,
            Self::DestFull => AccountMergeResultCode::DestFull,
            Self::IsSponsor => AccountMergeResultCode::IsSponsor,
        }
    }
}

impl ReadXdr for AccountMergeResult {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: AccountMergeResultCode = <AccountMergeResultCode as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            AccountMergeResultCode::Success => Self::Success(i64::read_xdr(r)?),
            AccountMergeResultCode::Malformed => Self::Malformed,
            AccountMergeResultCode::NoAccount => Self::NoAccount,
            AccountMergeResultCode::ImmutableSet => Self::ImmutableSet,
            AccountMergeResultCode::HasSubEntries => Self::HasSubEntries,
            AccountMergeResultCode::SeqnumTooFar => Self::SeqnumTooFar,
            AccountMergeResultCode::DestFull => Self::DestFull,
            AccountMergeResultCode::IsSponsor => Self::IsSponsor,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for AccountMergeResult {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success(v) => v.write_xdr(w)?,
            Self::Malformed => ().write_xdr(w)?,
            Self::NoAccount => ().write_xdr(w)?,
            Self::ImmutableSet => ().write_xdr(w)?,
            Self::HasSubEntries => ().write_xdr(w)?,
            Self::SeqnumTooFar => ().write_xdr(w)?,
            Self::DestFull => ().write_xdr(w)?,
            Self::IsSponsor => ().write_xdr(w)?,
        };
        Ok(())
    }
}

// InflationResultCode is an XDR Enum defines as:
//
//   enum InflationResultCode
//    {
//        // codes considered as "success" for the operation
//        INFLATION_SUCCESS = 0,
//        // codes considered as "failure" for the operation
//        INFLATION_NOT_TIME = -1
//    };
//
// enum
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum InflationResultCode {
    Success = 0,
    NotTime = -1,
}

impl InflationResultCode {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => "Success",
            Self::NotTime => "NotTime",
        }
    }
}

impl fmt::Display for InflationResultCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for InflationResultCode {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self> {
        let e = match i {
            0 => InflationResultCode::Success,
            -1 => InflationResultCode::NotTime,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<InflationResultCode> for i32 {
    #[must_use]
    fn from(e: InflationResultCode) -> Self {
        e as Self
    }
}

impl ReadXdr for InflationResultCode {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXdr for InflationResultCode {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// InflationPayout is an XDR Struct defines as:
//
//   struct InflationPayout // or use PaymentResultAtom to limit types?
//    {
//        AccountID destination;
//        int64 amount;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct InflationPayout {
    pub destination: AccountId,
    pub amount: i64,
}

impl ReadXdr for InflationPayout {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            destination: AccountId::read_xdr(r)?,
            amount: i64::read_xdr(r)?,
        })
    }
}

impl WriteXdr for InflationPayout {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.destination.write_xdr(w)?;
        self.amount.write_xdr(w)?;
        Ok(())
    }
}

// InflationResult is an XDR Union defines as:
//
//   union InflationResult switch (InflationResultCode code)
//    {
//    case INFLATION_SUCCESS:
//        InflationPayout payouts<>;
//    case INFLATION_NOT_TIME:
//        void;
//    };
//
// union with discriminant InflationResultCode
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum InflationResult {
    Success(VecM<InflationPayout>),
    NotTime,
}

impl InflationResult {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success(_) => "Success",
            Self::NotTime => "NotTime",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> InflationResultCode {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success(_) => InflationResultCode::Success,
            Self::NotTime => InflationResultCode::NotTime,
        }
    }
}

impl ReadXdr for InflationResult {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: InflationResultCode = <InflationResultCode as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            InflationResultCode::Success => Self::Success(VecM::<InflationPayout>::read_xdr(r)?),
            InflationResultCode::NotTime => Self::NotTime,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for InflationResult {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success(v) => v.write_xdr(w)?,
            Self::NotTime => ().write_xdr(w)?,
        };
        Ok(())
    }
}

// ManageDataResultCode is an XDR Enum defines as:
//
//   enum ManageDataResultCode
//    {
//        // codes considered as "success" for the operation
//        MANAGE_DATA_SUCCESS = 0,
//        // codes considered as "failure" for the operation
//        MANAGE_DATA_NOT_SUPPORTED_YET =
//            -1, // The network hasn't moved to this protocol change yet
//        MANAGE_DATA_NAME_NOT_FOUND =
//            -2, // Trying to remove a Data Entry that isn't there
//        MANAGE_DATA_LOW_RESERVE = -3, // not enough funds to create a new Data Entry
//        MANAGE_DATA_INVALID_NAME = -4 // Name not a valid string
//    };
//
// enum
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum ManageDataResultCode {
    Success = 0,
    NotSupportedYet = -1,
    NameNotFound = -2,
    LowReserve = -3,
    InvalidName = -4,
}

impl ManageDataResultCode {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => "Success",
            Self::NotSupportedYet => "NotSupportedYet",
            Self::NameNotFound => "NameNotFound",
            Self::LowReserve => "LowReserve",
            Self::InvalidName => "InvalidName",
        }
    }
}

impl fmt::Display for ManageDataResultCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for ManageDataResultCode {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self> {
        let e = match i {
            0 => ManageDataResultCode::Success,
            -1 => ManageDataResultCode::NotSupportedYet,
            -2 => ManageDataResultCode::NameNotFound,
            -3 => ManageDataResultCode::LowReserve,
            -4 => ManageDataResultCode::InvalidName,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<ManageDataResultCode> for i32 {
    #[must_use]
    fn from(e: ManageDataResultCode) -> Self {
        e as Self
    }
}

impl ReadXdr for ManageDataResultCode {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXdr for ManageDataResultCode {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// ManageDataResult is an XDR Union defines as:
//
//   union ManageDataResult switch (ManageDataResultCode code)
//    {
//    case MANAGE_DATA_SUCCESS:
//        void;
//    case MANAGE_DATA_NOT_SUPPORTED_YET:
//    case MANAGE_DATA_NAME_NOT_FOUND:
//    case MANAGE_DATA_LOW_RESERVE:
//    case MANAGE_DATA_INVALID_NAME:
//        void;
//    };
//
// union with discriminant ManageDataResultCode
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ManageDataResult {
    Success,
    NotSupportedYet,
    NameNotFound,
    LowReserve,
    InvalidName,
}

impl ManageDataResult {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => "Success",
            Self::NotSupportedYet => "NotSupportedYet",
            Self::NameNotFound => "NameNotFound",
            Self::LowReserve => "LowReserve",
            Self::InvalidName => "InvalidName",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> ManageDataResultCode {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => ManageDataResultCode::Success,
            Self::NotSupportedYet => ManageDataResultCode::NotSupportedYet,
            Self::NameNotFound => ManageDataResultCode::NameNotFound,
            Self::LowReserve => ManageDataResultCode::LowReserve,
            Self::InvalidName => ManageDataResultCode::InvalidName,
        }
    }
}

impl ReadXdr for ManageDataResult {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: ManageDataResultCode = <ManageDataResultCode as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            ManageDataResultCode::Success => Self::Success,
            ManageDataResultCode::NotSupportedYet => Self::NotSupportedYet,
            ManageDataResultCode::NameNotFound => Self::NameNotFound,
            ManageDataResultCode::LowReserve => Self::LowReserve,
            ManageDataResultCode::InvalidName => Self::InvalidName,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for ManageDataResult {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => ().write_xdr(w)?,
            Self::NotSupportedYet => ().write_xdr(w)?,
            Self::NameNotFound => ().write_xdr(w)?,
            Self::LowReserve => ().write_xdr(w)?,
            Self::InvalidName => ().write_xdr(w)?,
        };
        Ok(())
    }
}

// BumpSequenceResultCode is an XDR Enum defines as:
//
//   enum BumpSequenceResultCode
//    {
//        // codes considered as "success" for the operation
//        BUMP_SEQUENCE_SUCCESS = 0,
//        // codes considered as "failure" for the operation
//        BUMP_SEQUENCE_BAD_SEQ = -1 // `bumpTo` is not within bounds
//    };
//
// enum
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum BumpSequenceResultCode {
    Success = 0,
    BadSeq = -1,
}

impl BumpSequenceResultCode {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => "Success",
            Self::BadSeq => "BadSeq",
        }
    }
}

impl fmt::Display for BumpSequenceResultCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for BumpSequenceResultCode {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self> {
        let e = match i {
            0 => BumpSequenceResultCode::Success,
            -1 => BumpSequenceResultCode::BadSeq,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<BumpSequenceResultCode> for i32 {
    #[must_use]
    fn from(e: BumpSequenceResultCode) -> Self {
        e as Self
    }
}

impl ReadXdr for BumpSequenceResultCode {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXdr for BumpSequenceResultCode {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// BumpSequenceResult is an XDR Union defines as:
//
//   union BumpSequenceResult switch (BumpSequenceResultCode code)
//    {
//    case BUMP_SEQUENCE_SUCCESS:
//        void;
//    case BUMP_SEQUENCE_BAD_SEQ:
//        void;
//    };
//
// union with discriminant BumpSequenceResultCode
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum BumpSequenceResult {
    Success,
    BadSeq,
}

impl BumpSequenceResult {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => "Success",
            Self::BadSeq => "BadSeq",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> BumpSequenceResultCode {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => BumpSequenceResultCode::Success,
            Self::BadSeq => BumpSequenceResultCode::BadSeq,
        }
    }
}

impl ReadXdr for BumpSequenceResult {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: BumpSequenceResultCode = <BumpSequenceResultCode as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            BumpSequenceResultCode::Success => Self::Success,
            BumpSequenceResultCode::BadSeq => Self::BadSeq,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for BumpSequenceResult {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => ().write_xdr(w)?,
            Self::BadSeq => ().write_xdr(w)?,
        };
        Ok(())
    }
}

// CreateClaimableBalanceResultCode is an XDR Enum defines as:
//
//   enum CreateClaimableBalanceResultCode
//    {
//        CREATE_CLAIMABLE_BALANCE_SUCCESS = 0,
//        CREATE_CLAIMABLE_BALANCE_MALFORMED = -1,
//        CREATE_CLAIMABLE_BALANCE_LOW_RESERVE = -2,
//        CREATE_CLAIMABLE_BALANCE_NO_TRUST = -3,
//        CREATE_CLAIMABLE_BALANCE_NOT_AUTHORIZED = -4,
//        CREATE_CLAIMABLE_BALANCE_UNDERFUNDED = -5
//    };
//
// enum
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum CreateClaimableBalanceResultCode {
    Success = 0,
    Malformed = -1,
    LowReserve = -2,
    NoTrust = -3,
    NotAuthorized = -4,
    Underfunded = -5,
}

impl CreateClaimableBalanceResultCode {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => "Success",
            Self::Malformed => "Malformed",
            Self::LowReserve => "LowReserve",
            Self::NoTrust => "NoTrust",
            Self::NotAuthorized => "NotAuthorized",
            Self::Underfunded => "Underfunded",
        }
    }
}

impl fmt::Display for CreateClaimableBalanceResultCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for CreateClaimableBalanceResultCode {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self> {
        let e = match i {
            0 => CreateClaimableBalanceResultCode::Success,
            -1 => CreateClaimableBalanceResultCode::Malformed,
            -2 => CreateClaimableBalanceResultCode::LowReserve,
            -3 => CreateClaimableBalanceResultCode::NoTrust,
            -4 => CreateClaimableBalanceResultCode::NotAuthorized,
            -5 => CreateClaimableBalanceResultCode::Underfunded,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<CreateClaimableBalanceResultCode> for i32 {
    #[must_use]
    fn from(e: CreateClaimableBalanceResultCode) -> Self {
        e as Self
    }
}

impl ReadXdr for CreateClaimableBalanceResultCode {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXdr for CreateClaimableBalanceResultCode {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// CreateClaimableBalanceResult is an XDR Union defines as:
//
//   union CreateClaimableBalanceResult switch (
//        CreateClaimableBalanceResultCode code)
//    {
//    case CREATE_CLAIMABLE_BALANCE_SUCCESS:
//        ClaimableBalanceID balanceID;
//    case CREATE_CLAIMABLE_BALANCE_MALFORMED:
//    case CREATE_CLAIMABLE_BALANCE_LOW_RESERVE:
//    case CREATE_CLAIMABLE_BALANCE_NO_TRUST:
//    case CREATE_CLAIMABLE_BALANCE_NOT_AUTHORIZED:
//    case CREATE_CLAIMABLE_BALANCE_UNDERFUNDED:
//        void;
//    };
//
// union with discriminant CreateClaimableBalanceResultCode
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum CreateClaimableBalanceResult {
    Success(ClaimableBalanceId),
    Malformed,
    LowReserve,
    NoTrust,
    NotAuthorized,
    Underfunded,
}

impl CreateClaimableBalanceResult {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success(_) => "Success",
            Self::Malformed => "Malformed",
            Self::LowReserve => "LowReserve",
            Self::NoTrust => "NoTrust",
            Self::NotAuthorized => "NotAuthorized",
            Self::Underfunded => "Underfunded",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> CreateClaimableBalanceResultCode {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success(_) => CreateClaimableBalanceResultCode::Success,
            Self::Malformed => CreateClaimableBalanceResultCode::Malformed,
            Self::LowReserve => CreateClaimableBalanceResultCode::LowReserve,
            Self::NoTrust => CreateClaimableBalanceResultCode::NoTrust,
            Self::NotAuthorized => CreateClaimableBalanceResultCode::NotAuthorized,
            Self::Underfunded => CreateClaimableBalanceResultCode::Underfunded,
        }
    }
}

impl ReadXdr for CreateClaimableBalanceResult {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: CreateClaimableBalanceResultCode =
            <CreateClaimableBalanceResultCode as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            CreateClaimableBalanceResultCode::Success => {
                Self::Success(ClaimableBalanceId::read_xdr(r)?)
            }
            CreateClaimableBalanceResultCode::Malformed => Self::Malformed,
            CreateClaimableBalanceResultCode::LowReserve => Self::LowReserve,
            CreateClaimableBalanceResultCode::NoTrust => Self::NoTrust,
            CreateClaimableBalanceResultCode::NotAuthorized => Self::NotAuthorized,
            CreateClaimableBalanceResultCode::Underfunded => Self::Underfunded,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for CreateClaimableBalanceResult {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success(v) => v.write_xdr(w)?,
            Self::Malformed => ().write_xdr(w)?,
            Self::LowReserve => ().write_xdr(w)?,
            Self::NoTrust => ().write_xdr(w)?,
            Self::NotAuthorized => ().write_xdr(w)?,
            Self::Underfunded => ().write_xdr(w)?,
        };
        Ok(())
    }
}

// ClaimClaimableBalanceResultCode is an XDR Enum defines as:
//
//   enum ClaimClaimableBalanceResultCode
//    {
//        CLAIM_CLAIMABLE_BALANCE_SUCCESS = 0,
//        CLAIM_CLAIMABLE_BALANCE_DOES_NOT_EXIST = -1,
//        CLAIM_CLAIMABLE_BALANCE_CANNOT_CLAIM = -2,
//        CLAIM_CLAIMABLE_BALANCE_LINE_FULL = -3,
//        CLAIM_CLAIMABLE_BALANCE_NO_TRUST = -4,
//        CLAIM_CLAIMABLE_BALANCE_NOT_AUTHORIZED = -5
//    };
//
// enum
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum ClaimClaimableBalanceResultCode {
    Success = 0,
    DoesNotExist = -1,
    CannotClaim = -2,
    LineFull = -3,
    NoTrust = -4,
    NotAuthorized = -5,
}

impl ClaimClaimableBalanceResultCode {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => "Success",
            Self::DoesNotExist => "DoesNotExist",
            Self::CannotClaim => "CannotClaim",
            Self::LineFull => "LineFull",
            Self::NoTrust => "NoTrust",
            Self::NotAuthorized => "NotAuthorized",
        }
    }
}

impl fmt::Display for ClaimClaimableBalanceResultCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for ClaimClaimableBalanceResultCode {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self> {
        let e = match i {
            0 => ClaimClaimableBalanceResultCode::Success,
            -1 => ClaimClaimableBalanceResultCode::DoesNotExist,
            -2 => ClaimClaimableBalanceResultCode::CannotClaim,
            -3 => ClaimClaimableBalanceResultCode::LineFull,
            -4 => ClaimClaimableBalanceResultCode::NoTrust,
            -5 => ClaimClaimableBalanceResultCode::NotAuthorized,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<ClaimClaimableBalanceResultCode> for i32 {
    #[must_use]
    fn from(e: ClaimClaimableBalanceResultCode) -> Self {
        e as Self
    }
}

impl ReadXdr for ClaimClaimableBalanceResultCode {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXdr for ClaimClaimableBalanceResultCode {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// ClaimClaimableBalanceResult is an XDR Union defines as:
//
//   union ClaimClaimableBalanceResult switch (ClaimClaimableBalanceResultCode code)
//    {
//    case CLAIM_CLAIMABLE_BALANCE_SUCCESS:
//        void;
//    case CLAIM_CLAIMABLE_BALANCE_DOES_NOT_EXIST:
//    case CLAIM_CLAIMABLE_BALANCE_CANNOT_CLAIM:
//    case CLAIM_CLAIMABLE_BALANCE_LINE_FULL:
//    case CLAIM_CLAIMABLE_BALANCE_NO_TRUST:
//    case CLAIM_CLAIMABLE_BALANCE_NOT_AUTHORIZED:
//        void;
//    };
//
// union with discriminant ClaimClaimableBalanceResultCode
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ClaimClaimableBalanceResult {
    Success,
    DoesNotExist,
    CannotClaim,
    LineFull,
    NoTrust,
    NotAuthorized,
}

impl ClaimClaimableBalanceResult {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => "Success",
            Self::DoesNotExist => "DoesNotExist",
            Self::CannotClaim => "CannotClaim",
            Self::LineFull => "LineFull",
            Self::NoTrust => "NoTrust",
            Self::NotAuthorized => "NotAuthorized",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> ClaimClaimableBalanceResultCode {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => ClaimClaimableBalanceResultCode::Success,
            Self::DoesNotExist => ClaimClaimableBalanceResultCode::DoesNotExist,
            Self::CannotClaim => ClaimClaimableBalanceResultCode::CannotClaim,
            Self::LineFull => ClaimClaimableBalanceResultCode::LineFull,
            Self::NoTrust => ClaimClaimableBalanceResultCode::NoTrust,
            Self::NotAuthorized => ClaimClaimableBalanceResultCode::NotAuthorized,
        }
    }
}

impl ReadXdr for ClaimClaimableBalanceResult {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: ClaimClaimableBalanceResultCode =
            <ClaimClaimableBalanceResultCode as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            ClaimClaimableBalanceResultCode::Success => Self::Success,
            ClaimClaimableBalanceResultCode::DoesNotExist => Self::DoesNotExist,
            ClaimClaimableBalanceResultCode::CannotClaim => Self::CannotClaim,
            ClaimClaimableBalanceResultCode::LineFull => Self::LineFull,
            ClaimClaimableBalanceResultCode::NoTrust => Self::NoTrust,
            ClaimClaimableBalanceResultCode::NotAuthorized => Self::NotAuthorized,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for ClaimClaimableBalanceResult {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => ().write_xdr(w)?,
            Self::DoesNotExist => ().write_xdr(w)?,
            Self::CannotClaim => ().write_xdr(w)?,
            Self::LineFull => ().write_xdr(w)?,
            Self::NoTrust => ().write_xdr(w)?,
            Self::NotAuthorized => ().write_xdr(w)?,
        };
        Ok(())
    }
}

// BeginSponsoringFutureReservesResultCode is an XDR Enum defines as:
//
//   enum BeginSponsoringFutureReservesResultCode
//    {
//        // codes considered as "success" for the operation
//        BEGIN_SPONSORING_FUTURE_RESERVES_SUCCESS = 0,
//
//        // codes considered as "failure" for the operation
//        BEGIN_SPONSORING_FUTURE_RESERVES_MALFORMED = -1,
//        BEGIN_SPONSORING_FUTURE_RESERVES_ALREADY_SPONSORED = -2,
//        BEGIN_SPONSORING_FUTURE_RESERVES_RECURSIVE = -3
//    };
//
// enum
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum BeginSponsoringFutureReservesResultCode {
    Success = 0,
    Malformed = -1,
    AlreadySponsored = -2,
    Recursive = -3,
}

impl BeginSponsoringFutureReservesResultCode {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => "Success",
            Self::Malformed => "Malformed",
            Self::AlreadySponsored => "AlreadySponsored",
            Self::Recursive => "Recursive",
        }
    }
}

impl fmt::Display for BeginSponsoringFutureReservesResultCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for BeginSponsoringFutureReservesResultCode {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self> {
        let e = match i {
            0 => BeginSponsoringFutureReservesResultCode::Success,
            -1 => BeginSponsoringFutureReservesResultCode::Malformed,
            -2 => BeginSponsoringFutureReservesResultCode::AlreadySponsored,
            -3 => BeginSponsoringFutureReservesResultCode::Recursive,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<BeginSponsoringFutureReservesResultCode> for i32 {
    #[must_use]
    fn from(e: BeginSponsoringFutureReservesResultCode) -> Self {
        e as Self
    }
}

impl ReadXdr for BeginSponsoringFutureReservesResultCode {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXdr for BeginSponsoringFutureReservesResultCode {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// BeginSponsoringFutureReservesResult is an XDR Union defines as:
//
//   union BeginSponsoringFutureReservesResult switch (
//        BeginSponsoringFutureReservesResultCode code)
//    {
//    case BEGIN_SPONSORING_FUTURE_RESERVES_SUCCESS:
//        void;
//    case BEGIN_SPONSORING_FUTURE_RESERVES_MALFORMED:
//    case BEGIN_SPONSORING_FUTURE_RESERVES_ALREADY_SPONSORED:
//    case BEGIN_SPONSORING_FUTURE_RESERVES_RECURSIVE:
//        void;
//    };
//
// union with discriminant BeginSponsoringFutureReservesResultCode
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum BeginSponsoringFutureReservesResult {
    Success,
    Malformed,
    AlreadySponsored,
    Recursive,
}

impl BeginSponsoringFutureReservesResult {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => "Success",
            Self::Malformed => "Malformed",
            Self::AlreadySponsored => "AlreadySponsored",
            Self::Recursive => "Recursive",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> BeginSponsoringFutureReservesResultCode {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => BeginSponsoringFutureReservesResultCode::Success,
            Self::Malformed => BeginSponsoringFutureReservesResultCode::Malformed,
            Self::AlreadySponsored => BeginSponsoringFutureReservesResultCode::AlreadySponsored,
            Self::Recursive => BeginSponsoringFutureReservesResultCode::Recursive,
        }
    }
}

impl ReadXdr for BeginSponsoringFutureReservesResult {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: BeginSponsoringFutureReservesResultCode =
            <BeginSponsoringFutureReservesResultCode as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            BeginSponsoringFutureReservesResultCode::Success => Self::Success,
            BeginSponsoringFutureReservesResultCode::Malformed => Self::Malformed,
            BeginSponsoringFutureReservesResultCode::AlreadySponsored => Self::AlreadySponsored,
            BeginSponsoringFutureReservesResultCode::Recursive => Self::Recursive,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for BeginSponsoringFutureReservesResult {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => ().write_xdr(w)?,
            Self::Malformed => ().write_xdr(w)?,
            Self::AlreadySponsored => ().write_xdr(w)?,
            Self::Recursive => ().write_xdr(w)?,
        };
        Ok(())
    }
}

// EndSponsoringFutureReservesResultCode is an XDR Enum defines as:
//
//   enum EndSponsoringFutureReservesResultCode
//    {
//        // codes considered as "success" for the operation
//        END_SPONSORING_FUTURE_RESERVES_SUCCESS = 0,
//
//        // codes considered as "failure" for the operation
//        END_SPONSORING_FUTURE_RESERVES_NOT_SPONSORED = -1
//    };
//
// enum
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum EndSponsoringFutureReservesResultCode {
    Success = 0,
    NotSponsored = -1,
}

impl EndSponsoringFutureReservesResultCode {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => "Success",
            Self::NotSponsored => "NotSponsored",
        }
    }
}

impl fmt::Display for EndSponsoringFutureReservesResultCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for EndSponsoringFutureReservesResultCode {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self> {
        let e = match i {
            0 => EndSponsoringFutureReservesResultCode::Success,
            -1 => EndSponsoringFutureReservesResultCode::NotSponsored,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<EndSponsoringFutureReservesResultCode> for i32 {
    #[must_use]
    fn from(e: EndSponsoringFutureReservesResultCode) -> Self {
        e as Self
    }
}

impl ReadXdr for EndSponsoringFutureReservesResultCode {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXdr for EndSponsoringFutureReservesResultCode {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// EndSponsoringFutureReservesResult is an XDR Union defines as:
//
//   union EndSponsoringFutureReservesResult switch (
//        EndSponsoringFutureReservesResultCode code)
//    {
//    case END_SPONSORING_FUTURE_RESERVES_SUCCESS:
//        void;
//    case END_SPONSORING_FUTURE_RESERVES_NOT_SPONSORED:
//        void;
//    };
//
// union with discriminant EndSponsoringFutureReservesResultCode
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum EndSponsoringFutureReservesResult {
    Success,
    NotSponsored,
}

impl EndSponsoringFutureReservesResult {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => "Success",
            Self::NotSponsored => "NotSponsored",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> EndSponsoringFutureReservesResultCode {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => EndSponsoringFutureReservesResultCode::Success,
            Self::NotSponsored => EndSponsoringFutureReservesResultCode::NotSponsored,
        }
    }
}

impl ReadXdr for EndSponsoringFutureReservesResult {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: EndSponsoringFutureReservesResultCode =
            <EndSponsoringFutureReservesResultCode as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            EndSponsoringFutureReservesResultCode::Success => Self::Success,
            EndSponsoringFutureReservesResultCode::NotSponsored => Self::NotSponsored,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for EndSponsoringFutureReservesResult {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => ().write_xdr(w)?,
            Self::NotSponsored => ().write_xdr(w)?,
        };
        Ok(())
    }
}

// RevokeSponsorshipResultCode is an XDR Enum defines as:
//
//   enum RevokeSponsorshipResultCode
//    {
//        // codes considered as "success" for the operation
//        REVOKE_SPONSORSHIP_SUCCESS = 0,
//
//        // codes considered as "failure" for the operation
//        REVOKE_SPONSORSHIP_DOES_NOT_EXIST = -1,
//        REVOKE_SPONSORSHIP_NOT_SPONSOR = -2,
//        REVOKE_SPONSORSHIP_LOW_RESERVE = -3,
//        REVOKE_SPONSORSHIP_ONLY_TRANSFERABLE = -4,
//        REVOKE_SPONSORSHIP_MALFORMED = -5
//    };
//
// enum
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum RevokeSponsorshipResultCode {
    Success = 0,
    DoesNotExist = -1,
    NotSponsor = -2,
    LowReserve = -3,
    OnlyTransferable = -4,
    Malformed = -5,
}

impl RevokeSponsorshipResultCode {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => "Success",
            Self::DoesNotExist => "DoesNotExist",
            Self::NotSponsor => "NotSponsor",
            Self::LowReserve => "LowReserve",
            Self::OnlyTransferable => "OnlyTransferable",
            Self::Malformed => "Malformed",
        }
    }
}

impl fmt::Display for RevokeSponsorshipResultCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for RevokeSponsorshipResultCode {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self> {
        let e = match i {
            0 => RevokeSponsorshipResultCode::Success,
            -1 => RevokeSponsorshipResultCode::DoesNotExist,
            -2 => RevokeSponsorshipResultCode::NotSponsor,
            -3 => RevokeSponsorshipResultCode::LowReserve,
            -4 => RevokeSponsorshipResultCode::OnlyTransferable,
            -5 => RevokeSponsorshipResultCode::Malformed,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<RevokeSponsorshipResultCode> for i32 {
    #[must_use]
    fn from(e: RevokeSponsorshipResultCode) -> Self {
        e as Self
    }
}

impl ReadXdr for RevokeSponsorshipResultCode {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXdr for RevokeSponsorshipResultCode {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// RevokeSponsorshipResult is an XDR Union defines as:
//
//   union RevokeSponsorshipResult switch (RevokeSponsorshipResultCode code)
//    {
//    case REVOKE_SPONSORSHIP_SUCCESS:
//        void;
//    case REVOKE_SPONSORSHIP_DOES_NOT_EXIST:
//    case REVOKE_SPONSORSHIP_NOT_SPONSOR:
//    case REVOKE_SPONSORSHIP_LOW_RESERVE:
//    case REVOKE_SPONSORSHIP_ONLY_TRANSFERABLE:
//    case REVOKE_SPONSORSHIP_MALFORMED:
//        void;
//    };
//
// union with discriminant RevokeSponsorshipResultCode
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum RevokeSponsorshipResult {
    Success,
    DoesNotExist,
    NotSponsor,
    LowReserve,
    OnlyTransferable,
    Malformed,
}

impl RevokeSponsorshipResult {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => "Success",
            Self::DoesNotExist => "DoesNotExist",
            Self::NotSponsor => "NotSponsor",
            Self::LowReserve => "LowReserve",
            Self::OnlyTransferable => "OnlyTransferable",
            Self::Malformed => "Malformed",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> RevokeSponsorshipResultCode {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => RevokeSponsorshipResultCode::Success,
            Self::DoesNotExist => RevokeSponsorshipResultCode::DoesNotExist,
            Self::NotSponsor => RevokeSponsorshipResultCode::NotSponsor,
            Self::LowReserve => RevokeSponsorshipResultCode::LowReserve,
            Self::OnlyTransferable => RevokeSponsorshipResultCode::OnlyTransferable,
            Self::Malformed => RevokeSponsorshipResultCode::Malformed,
        }
    }
}

impl ReadXdr for RevokeSponsorshipResult {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: RevokeSponsorshipResultCode =
            <RevokeSponsorshipResultCode as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            RevokeSponsorshipResultCode::Success => Self::Success,
            RevokeSponsorshipResultCode::DoesNotExist => Self::DoesNotExist,
            RevokeSponsorshipResultCode::NotSponsor => Self::NotSponsor,
            RevokeSponsorshipResultCode::LowReserve => Self::LowReserve,
            RevokeSponsorshipResultCode::OnlyTransferable => Self::OnlyTransferable,
            RevokeSponsorshipResultCode::Malformed => Self::Malformed,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for RevokeSponsorshipResult {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => ().write_xdr(w)?,
            Self::DoesNotExist => ().write_xdr(w)?,
            Self::NotSponsor => ().write_xdr(w)?,
            Self::LowReserve => ().write_xdr(w)?,
            Self::OnlyTransferable => ().write_xdr(w)?,
            Self::Malformed => ().write_xdr(w)?,
        };
        Ok(())
    }
}

// ClawbackResultCode is an XDR Enum defines as:
//
//   enum ClawbackResultCode
//    {
//        // codes considered as "success" for the operation
//        CLAWBACK_SUCCESS = 0,
//
//        // codes considered as "failure" for the operation
//        CLAWBACK_MALFORMED = -1,
//        CLAWBACK_NOT_CLAWBACK_ENABLED = -2,
//        CLAWBACK_NO_TRUST = -3,
//        CLAWBACK_UNDERFUNDED = -4
//    };
//
// enum
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum ClawbackResultCode {
    Success = 0,
    Malformed = -1,
    NotClawbackEnabled = -2,
    NoTrust = -3,
    Underfunded = -4,
}

impl ClawbackResultCode {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => "Success",
            Self::Malformed => "Malformed",
            Self::NotClawbackEnabled => "NotClawbackEnabled",
            Self::NoTrust => "NoTrust",
            Self::Underfunded => "Underfunded",
        }
    }
}

impl fmt::Display for ClawbackResultCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for ClawbackResultCode {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self> {
        let e = match i {
            0 => ClawbackResultCode::Success,
            -1 => ClawbackResultCode::Malformed,
            -2 => ClawbackResultCode::NotClawbackEnabled,
            -3 => ClawbackResultCode::NoTrust,
            -4 => ClawbackResultCode::Underfunded,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<ClawbackResultCode> for i32 {
    #[must_use]
    fn from(e: ClawbackResultCode) -> Self {
        e as Self
    }
}

impl ReadXdr for ClawbackResultCode {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXdr for ClawbackResultCode {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// ClawbackResult is an XDR Union defines as:
//
//   union ClawbackResult switch (ClawbackResultCode code)
//    {
//    case CLAWBACK_SUCCESS:
//        void;
//    case CLAWBACK_MALFORMED:
//    case CLAWBACK_NOT_CLAWBACK_ENABLED:
//    case CLAWBACK_NO_TRUST:
//    case CLAWBACK_UNDERFUNDED:
//        void;
//    };
//
// union with discriminant ClawbackResultCode
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ClawbackResult {
    Success,
    Malformed,
    NotClawbackEnabled,
    NoTrust,
    Underfunded,
}

impl ClawbackResult {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => "Success",
            Self::Malformed => "Malformed",
            Self::NotClawbackEnabled => "NotClawbackEnabled",
            Self::NoTrust => "NoTrust",
            Self::Underfunded => "Underfunded",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> ClawbackResultCode {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => ClawbackResultCode::Success,
            Self::Malformed => ClawbackResultCode::Malformed,
            Self::NotClawbackEnabled => ClawbackResultCode::NotClawbackEnabled,
            Self::NoTrust => ClawbackResultCode::NoTrust,
            Self::Underfunded => ClawbackResultCode::Underfunded,
        }
    }
}

impl ReadXdr for ClawbackResult {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: ClawbackResultCode = <ClawbackResultCode as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            ClawbackResultCode::Success => Self::Success,
            ClawbackResultCode::Malformed => Self::Malformed,
            ClawbackResultCode::NotClawbackEnabled => Self::NotClawbackEnabled,
            ClawbackResultCode::NoTrust => Self::NoTrust,
            ClawbackResultCode::Underfunded => Self::Underfunded,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for ClawbackResult {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => ().write_xdr(w)?,
            Self::Malformed => ().write_xdr(w)?,
            Self::NotClawbackEnabled => ().write_xdr(w)?,
            Self::NoTrust => ().write_xdr(w)?,
            Self::Underfunded => ().write_xdr(w)?,
        };
        Ok(())
    }
}

// ClawbackClaimableBalanceResultCode is an XDR Enum defines as:
//
//   enum ClawbackClaimableBalanceResultCode
//    {
//        // codes considered as "success" for the operation
//        CLAWBACK_CLAIMABLE_BALANCE_SUCCESS = 0,
//
//        // codes considered as "failure" for the operation
//        CLAWBACK_CLAIMABLE_BALANCE_DOES_NOT_EXIST = -1,
//        CLAWBACK_CLAIMABLE_BALANCE_NOT_ISSUER = -2,
//        CLAWBACK_CLAIMABLE_BALANCE_NOT_CLAWBACK_ENABLED = -3
//    };
//
// enum
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum ClawbackClaimableBalanceResultCode {
    Success = 0,
    DoesNotExist = -1,
    NotIssuer = -2,
    NotClawbackEnabled = -3,
}

impl ClawbackClaimableBalanceResultCode {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => "Success",
            Self::DoesNotExist => "DoesNotExist",
            Self::NotIssuer => "NotIssuer",
            Self::NotClawbackEnabled => "NotClawbackEnabled",
        }
    }
}

impl fmt::Display for ClawbackClaimableBalanceResultCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for ClawbackClaimableBalanceResultCode {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self> {
        let e = match i {
            0 => ClawbackClaimableBalanceResultCode::Success,
            -1 => ClawbackClaimableBalanceResultCode::DoesNotExist,
            -2 => ClawbackClaimableBalanceResultCode::NotIssuer,
            -3 => ClawbackClaimableBalanceResultCode::NotClawbackEnabled,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<ClawbackClaimableBalanceResultCode> for i32 {
    #[must_use]
    fn from(e: ClawbackClaimableBalanceResultCode) -> Self {
        e as Self
    }
}

impl ReadXdr for ClawbackClaimableBalanceResultCode {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXdr for ClawbackClaimableBalanceResultCode {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// ClawbackClaimableBalanceResult is an XDR Union defines as:
//
//   union ClawbackClaimableBalanceResult switch (
//        ClawbackClaimableBalanceResultCode code)
//    {
//    case CLAWBACK_CLAIMABLE_BALANCE_SUCCESS:
//        void;
//    case CLAWBACK_CLAIMABLE_BALANCE_DOES_NOT_EXIST:
//    case CLAWBACK_CLAIMABLE_BALANCE_NOT_ISSUER:
//    case CLAWBACK_CLAIMABLE_BALANCE_NOT_CLAWBACK_ENABLED:
//        void;
//    };
//
// union with discriminant ClawbackClaimableBalanceResultCode
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ClawbackClaimableBalanceResult {
    Success,
    DoesNotExist,
    NotIssuer,
    NotClawbackEnabled,
}

impl ClawbackClaimableBalanceResult {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => "Success",
            Self::DoesNotExist => "DoesNotExist",
            Self::NotIssuer => "NotIssuer",
            Self::NotClawbackEnabled => "NotClawbackEnabled",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> ClawbackClaimableBalanceResultCode {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => ClawbackClaimableBalanceResultCode::Success,
            Self::DoesNotExist => ClawbackClaimableBalanceResultCode::DoesNotExist,
            Self::NotIssuer => ClawbackClaimableBalanceResultCode::NotIssuer,
            Self::NotClawbackEnabled => ClawbackClaimableBalanceResultCode::NotClawbackEnabled,
        }
    }
}

impl ReadXdr for ClawbackClaimableBalanceResult {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: ClawbackClaimableBalanceResultCode =
            <ClawbackClaimableBalanceResultCode as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            ClawbackClaimableBalanceResultCode::Success => Self::Success,
            ClawbackClaimableBalanceResultCode::DoesNotExist => Self::DoesNotExist,
            ClawbackClaimableBalanceResultCode::NotIssuer => Self::NotIssuer,
            ClawbackClaimableBalanceResultCode::NotClawbackEnabled => Self::NotClawbackEnabled,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for ClawbackClaimableBalanceResult {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => ().write_xdr(w)?,
            Self::DoesNotExist => ().write_xdr(w)?,
            Self::NotIssuer => ().write_xdr(w)?,
            Self::NotClawbackEnabled => ().write_xdr(w)?,
        };
        Ok(())
    }
}

// SetTrustLineFlagsResultCode is an XDR Enum defines as:
//
//   enum SetTrustLineFlagsResultCode
//    {
//        // codes considered as "success" for the operation
//        SET_TRUST_LINE_FLAGS_SUCCESS = 0,
//
//        // codes considered as "failure" for the operation
//        SET_TRUST_LINE_FLAGS_MALFORMED = -1,
//        SET_TRUST_LINE_FLAGS_NO_TRUST_LINE = -2,
//        SET_TRUST_LINE_FLAGS_CANT_REVOKE = -3,
//        SET_TRUST_LINE_FLAGS_INVALID_STATE = -4,
//        SET_TRUST_LINE_FLAGS_LOW_RESERVE = -5 // claimable balances can't be created
//                                              // on revoke due to low reserves
//    };
//
// enum
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum SetTrustLineFlagsResultCode {
    Success = 0,
    Malformed = -1,
    NoTrustLine = -2,
    CantRevoke = -3,
    InvalidState = -4,
    LowReserve = -5,
}

impl SetTrustLineFlagsResultCode {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => "Success",
            Self::Malformed => "Malformed",
            Self::NoTrustLine => "NoTrustLine",
            Self::CantRevoke => "CantRevoke",
            Self::InvalidState => "InvalidState",
            Self::LowReserve => "LowReserve",
        }
    }
}

impl fmt::Display for SetTrustLineFlagsResultCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for SetTrustLineFlagsResultCode {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self> {
        let e = match i {
            0 => SetTrustLineFlagsResultCode::Success,
            -1 => SetTrustLineFlagsResultCode::Malformed,
            -2 => SetTrustLineFlagsResultCode::NoTrustLine,
            -3 => SetTrustLineFlagsResultCode::CantRevoke,
            -4 => SetTrustLineFlagsResultCode::InvalidState,
            -5 => SetTrustLineFlagsResultCode::LowReserve,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<SetTrustLineFlagsResultCode> for i32 {
    #[must_use]
    fn from(e: SetTrustLineFlagsResultCode) -> Self {
        e as Self
    }
}

impl ReadXdr for SetTrustLineFlagsResultCode {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXdr for SetTrustLineFlagsResultCode {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// SetTrustLineFlagsResult is an XDR Union defines as:
//
//   union SetTrustLineFlagsResult switch (SetTrustLineFlagsResultCode code)
//    {
//    case SET_TRUST_LINE_FLAGS_SUCCESS:
//        void;
//    case SET_TRUST_LINE_FLAGS_MALFORMED:
//    case SET_TRUST_LINE_FLAGS_NO_TRUST_LINE:
//    case SET_TRUST_LINE_FLAGS_CANT_REVOKE:
//    case SET_TRUST_LINE_FLAGS_INVALID_STATE:
//    case SET_TRUST_LINE_FLAGS_LOW_RESERVE:
//        void;
//    };
//
// union with discriminant SetTrustLineFlagsResultCode
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum SetTrustLineFlagsResult {
    Success,
    Malformed,
    NoTrustLine,
    CantRevoke,
    InvalidState,
    LowReserve,
}

impl SetTrustLineFlagsResult {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => "Success",
            Self::Malformed => "Malformed",
            Self::NoTrustLine => "NoTrustLine",
            Self::CantRevoke => "CantRevoke",
            Self::InvalidState => "InvalidState",
            Self::LowReserve => "LowReserve",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> SetTrustLineFlagsResultCode {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => SetTrustLineFlagsResultCode::Success,
            Self::Malformed => SetTrustLineFlagsResultCode::Malformed,
            Self::NoTrustLine => SetTrustLineFlagsResultCode::NoTrustLine,
            Self::CantRevoke => SetTrustLineFlagsResultCode::CantRevoke,
            Self::InvalidState => SetTrustLineFlagsResultCode::InvalidState,
            Self::LowReserve => SetTrustLineFlagsResultCode::LowReserve,
        }
    }
}

impl ReadXdr for SetTrustLineFlagsResult {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: SetTrustLineFlagsResultCode =
            <SetTrustLineFlagsResultCode as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            SetTrustLineFlagsResultCode::Success => Self::Success,
            SetTrustLineFlagsResultCode::Malformed => Self::Malformed,
            SetTrustLineFlagsResultCode::NoTrustLine => Self::NoTrustLine,
            SetTrustLineFlagsResultCode::CantRevoke => Self::CantRevoke,
            SetTrustLineFlagsResultCode::InvalidState => Self::InvalidState,
            SetTrustLineFlagsResultCode::LowReserve => Self::LowReserve,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for SetTrustLineFlagsResult {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => ().write_xdr(w)?,
            Self::Malformed => ().write_xdr(w)?,
            Self::NoTrustLine => ().write_xdr(w)?,
            Self::CantRevoke => ().write_xdr(w)?,
            Self::InvalidState => ().write_xdr(w)?,
            Self::LowReserve => ().write_xdr(w)?,
        };
        Ok(())
    }
}

// LiquidityPoolDepositResultCode is an XDR Enum defines as:
//
//   enum LiquidityPoolDepositResultCode
//    {
//        // codes considered as "success" for the operation
//        LIQUIDITY_POOL_DEPOSIT_SUCCESS = 0,
//
//        // codes considered as "failure" for the operation
//        LIQUIDITY_POOL_DEPOSIT_MALFORMED = -1,      // bad input
//        LIQUIDITY_POOL_DEPOSIT_NO_TRUST = -2,       // no trust line for one of the
//                                                    // assets
//        LIQUIDITY_POOL_DEPOSIT_NOT_AUTHORIZED = -3, // not authorized for one of the
//                                                    // assets
//        LIQUIDITY_POOL_DEPOSIT_UNDERFUNDED = -4,    // not enough balance for one of
//                                                    // the assets
//        LIQUIDITY_POOL_DEPOSIT_LINE_FULL = -5,      // pool share trust line doesn't
//                                                    // have sufficient limit
//        LIQUIDITY_POOL_DEPOSIT_BAD_PRICE = -6,      // deposit price outside bounds
//        LIQUIDITY_POOL_DEPOSIT_POOL_FULL = -7       // pool reserves are full
//    };
//
// enum
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum LiquidityPoolDepositResultCode {
    Success = 0,
    Malformed = -1,
    NoTrust = -2,
    NotAuthorized = -3,
    Underfunded = -4,
    LineFull = -5,
    BadPrice = -6,
    PoolFull = -7,
}

impl LiquidityPoolDepositResultCode {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => "Success",
            Self::Malformed => "Malformed",
            Self::NoTrust => "NoTrust",
            Self::NotAuthorized => "NotAuthorized",
            Self::Underfunded => "Underfunded",
            Self::LineFull => "LineFull",
            Self::BadPrice => "BadPrice",
            Self::PoolFull => "PoolFull",
        }
    }
}

impl fmt::Display for LiquidityPoolDepositResultCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for LiquidityPoolDepositResultCode {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self> {
        let e = match i {
            0 => LiquidityPoolDepositResultCode::Success,
            -1 => LiquidityPoolDepositResultCode::Malformed,
            -2 => LiquidityPoolDepositResultCode::NoTrust,
            -3 => LiquidityPoolDepositResultCode::NotAuthorized,
            -4 => LiquidityPoolDepositResultCode::Underfunded,
            -5 => LiquidityPoolDepositResultCode::LineFull,
            -6 => LiquidityPoolDepositResultCode::BadPrice,
            -7 => LiquidityPoolDepositResultCode::PoolFull,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<LiquidityPoolDepositResultCode> for i32 {
    #[must_use]
    fn from(e: LiquidityPoolDepositResultCode) -> Self {
        e as Self
    }
}

impl ReadXdr for LiquidityPoolDepositResultCode {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXdr for LiquidityPoolDepositResultCode {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// LiquidityPoolDepositResult is an XDR Union defines as:
//
//   union LiquidityPoolDepositResult switch (LiquidityPoolDepositResultCode code)
//    {
//    case LIQUIDITY_POOL_DEPOSIT_SUCCESS:
//        void;
//    case LIQUIDITY_POOL_DEPOSIT_MALFORMED:
//    case LIQUIDITY_POOL_DEPOSIT_NO_TRUST:
//    case LIQUIDITY_POOL_DEPOSIT_NOT_AUTHORIZED:
//    case LIQUIDITY_POOL_DEPOSIT_UNDERFUNDED:
//    case LIQUIDITY_POOL_DEPOSIT_LINE_FULL:
//    case LIQUIDITY_POOL_DEPOSIT_BAD_PRICE:
//    case LIQUIDITY_POOL_DEPOSIT_POOL_FULL:
//        void;
//    };
//
// union with discriminant LiquidityPoolDepositResultCode
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum LiquidityPoolDepositResult {
    Success,
    Malformed,
    NoTrust,
    NotAuthorized,
    Underfunded,
    LineFull,
    BadPrice,
    PoolFull,
}

impl LiquidityPoolDepositResult {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => "Success",
            Self::Malformed => "Malformed",
            Self::NoTrust => "NoTrust",
            Self::NotAuthorized => "NotAuthorized",
            Self::Underfunded => "Underfunded",
            Self::LineFull => "LineFull",
            Self::BadPrice => "BadPrice",
            Self::PoolFull => "PoolFull",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> LiquidityPoolDepositResultCode {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => LiquidityPoolDepositResultCode::Success,
            Self::Malformed => LiquidityPoolDepositResultCode::Malformed,
            Self::NoTrust => LiquidityPoolDepositResultCode::NoTrust,
            Self::NotAuthorized => LiquidityPoolDepositResultCode::NotAuthorized,
            Self::Underfunded => LiquidityPoolDepositResultCode::Underfunded,
            Self::LineFull => LiquidityPoolDepositResultCode::LineFull,
            Self::BadPrice => LiquidityPoolDepositResultCode::BadPrice,
            Self::PoolFull => LiquidityPoolDepositResultCode::PoolFull,
        }
    }
}

impl ReadXdr for LiquidityPoolDepositResult {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: LiquidityPoolDepositResultCode =
            <LiquidityPoolDepositResultCode as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            LiquidityPoolDepositResultCode::Success => Self::Success,
            LiquidityPoolDepositResultCode::Malformed => Self::Malformed,
            LiquidityPoolDepositResultCode::NoTrust => Self::NoTrust,
            LiquidityPoolDepositResultCode::NotAuthorized => Self::NotAuthorized,
            LiquidityPoolDepositResultCode::Underfunded => Self::Underfunded,
            LiquidityPoolDepositResultCode::LineFull => Self::LineFull,
            LiquidityPoolDepositResultCode::BadPrice => Self::BadPrice,
            LiquidityPoolDepositResultCode::PoolFull => Self::PoolFull,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for LiquidityPoolDepositResult {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => ().write_xdr(w)?,
            Self::Malformed => ().write_xdr(w)?,
            Self::NoTrust => ().write_xdr(w)?,
            Self::NotAuthorized => ().write_xdr(w)?,
            Self::Underfunded => ().write_xdr(w)?,
            Self::LineFull => ().write_xdr(w)?,
            Self::BadPrice => ().write_xdr(w)?,
            Self::PoolFull => ().write_xdr(w)?,
        };
        Ok(())
    }
}

// LiquidityPoolWithdrawResultCode is an XDR Enum defines as:
//
//   enum LiquidityPoolWithdrawResultCode
//    {
//        // codes considered as "success" for the operation
//        LIQUIDITY_POOL_WITHDRAW_SUCCESS = 0,
//
//        // codes considered as "failure" for the operation
//        LIQUIDITY_POOL_WITHDRAW_MALFORMED = -1,    // bad input
//        LIQUIDITY_POOL_WITHDRAW_NO_TRUST = -2,     // no trust line for one of the
//                                                   // assets
//        LIQUIDITY_POOL_WITHDRAW_UNDERFUNDED = -3,  // not enough balance of the
//                                                   // pool share
//        LIQUIDITY_POOL_WITHDRAW_LINE_FULL = -4,    // would go above limit for one
//                                                   // of the assets
//        LIQUIDITY_POOL_WITHDRAW_UNDER_MINIMUM = -5 // didn't withdraw enough
//    };
//
// enum
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum LiquidityPoolWithdrawResultCode {
    Success = 0,
    Malformed = -1,
    NoTrust = -2,
    Underfunded = -3,
    LineFull = -4,
    UnderMinimum = -5,
}

impl LiquidityPoolWithdrawResultCode {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => "Success",
            Self::Malformed => "Malformed",
            Self::NoTrust => "NoTrust",
            Self::Underfunded => "Underfunded",
            Self::LineFull => "LineFull",
            Self::UnderMinimum => "UnderMinimum",
        }
    }
}

impl fmt::Display for LiquidityPoolWithdrawResultCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for LiquidityPoolWithdrawResultCode {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self> {
        let e = match i {
            0 => LiquidityPoolWithdrawResultCode::Success,
            -1 => LiquidityPoolWithdrawResultCode::Malformed,
            -2 => LiquidityPoolWithdrawResultCode::NoTrust,
            -3 => LiquidityPoolWithdrawResultCode::Underfunded,
            -4 => LiquidityPoolWithdrawResultCode::LineFull,
            -5 => LiquidityPoolWithdrawResultCode::UnderMinimum,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<LiquidityPoolWithdrawResultCode> for i32 {
    #[must_use]
    fn from(e: LiquidityPoolWithdrawResultCode) -> Self {
        e as Self
    }
}

impl ReadXdr for LiquidityPoolWithdrawResultCode {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXdr for LiquidityPoolWithdrawResultCode {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// LiquidityPoolWithdrawResult is an XDR Union defines as:
//
//   union LiquidityPoolWithdrawResult switch (LiquidityPoolWithdrawResultCode code)
//    {
//    case LIQUIDITY_POOL_WITHDRAW_SUCCESS:
//        void;
//    case LIQUIDITY_POOL_WITHDRAW_MALFORMED:
//    case LIQUIDITY_POOL_WITHDRAW_NO_TRUST:
//    case LIQUIDITY_POOL_WITHDRAW_UNDERFUNDED:
//    case LIQUIDITY_POOL_WITHDRAW_LINE_FULL:
//    case LIQUIDITY_POOL_WITHDRAW_UNDER_MINIMUM:
//        void;
//    };
//
// union with discriminant LiquidityPoolWithdrawResultCode
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum LiquidityPoolWithdrawResult {
    Success,
    Malformed,
    NoTrust,
    Underfunded,
    LineFull,
    UnderMinimum,
}

impl LiquidityPoolWithdrawResult {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => "Success",
            Self::Malformed => "Malformed",
            Self::NoTrust => "NoTrust",
            Self::Underfunded => "Underfunded",
            Self::LineFull => "LineFull",
            Self::UnderMinimum => "UnderMinimum",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> LiquidityPoolWithdrawResultCode {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => LiquidityPoolWithdrawResultCode::Success,
            Self::Malformed => LiquidityPoolWithdrawResultCode::Malformed,
            Self::NoTrust => LiquidityPoolWithdrawResultCode::NoTrust,
            Self::Underfunded => LiquidityPoolWithdrawResultCode::Underfunded,
            Self::LineFull => LiquidityPoolWithdrawResultCode::LineFull,
            Self::UnderMinimum => LiquidityPoolWithdrawResultCode::UnderMinimum,
        }
    }
}

impl ReadXdr for LiquidityPoolWithdrawResult {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: LiquidityPoolWithdrawResultCode =
            <LiquidityPoolWithdrawResultCode as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            LiquidityPoolWithdrawResultCode::Success => Self::Success,
            LiquidityPoolWithdrawResultCode::Malformed => Self::Malformed,
            LiquidityPoolWithdrawResultCode::NoTrust => Self::NoTrust,
            LiquidityPoolWithdrawResultCode::Underfunded => Self::Underfunded,
            LiquidityPoolWithdrawResultCode::LineFull => Self::LineFull,
            LiquidityPoolWithdrawResultCode::UnderMinimum => Self::UnderMinimum,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for LiquidityPoolWithdrawResult {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => ().write_xdr(w)?,
            Self::Malformed => ().write_xdr(w)?,
            Self::NoTrust => ().write_xdr(w)?,
            Self::Underfunded => ().write_xdr(w)?,
            Self::LineFull => ().write_xdr(w)?,
            Self::UnderMinimum => ().write_xdr(w)?,
        };
        Ok(())
    }
}

// InvokeHostFunctionResultCode is an XDR Enum defines as:
//
//   enum InvokeHostFunctionResultCode
//    {
//        // codes considered as "success" for the operation
//        INVOKE_HOST_FUNCTION_SUCCESS = 0,
//
//        // codes considered as "failure" for the operation
//        INVOKE_HOST_FUNCTION_MALFORMED = -1,
//        INVOKE_HOST_FUNCTION_TRAPPED = -2
//    };
//
// enum
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum InvokeHostFunctionResultCode {
    Success = 0,
    Malformed = -1,
    Trapped = -2,
}

impl InvokeHostFunctionResultCode {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => "Success",
            Self::Malformed => "Malformed",
            Self::Trapped => "Trapped",
        }
    }
}

impl fmt::Display for InvokeHostFunctionResultCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for InvokeHostFunctionResultCode {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self> {
        let e = match i {
            0 => InvokeHostFunctionResultCode::Success,
            -1 => InvokeHostFunctionResultCode::Malformed,
            -2 => InvokeHostFunctionResultCode::Trapped,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<InvokeHostFunctionResultCode> for i32 {
    #[must_use]
    fn from(e: InvokeHostFunctionResultCode) -> Self {
        e as Self
    }
}

impl ReadXdr for InvokeHostFunctionResultCode {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXdr for InvokeHostFunctionResultCode {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// InvokeHostFunctionResult is an XDR Union defines as:
//
//   union InvokeHostFunctionResult switch (InvokeHostFunctionResultCode code)
//    {
//    case INVOKE_HOST_FUNCTION_SUCCESS:
//        void;
//    case INVOKE_HOST_FUNCTION_MALFORMED:
//    case INVOKE_HOST_FUNCTION_TRAPPED:
//        void;
//    };
//
// union with discriminant InvokeHostFunctionResultCode
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum InvokeHostFunctionResult {
    Success,
    Malformed,
    Trapped,
}

impl InvokeHostFunctionResult {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => "Success",
            Self::Malformed => "Malformed",
            Self::Trapped => "Trapped",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> InvokeHostFunctionResultCode {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => InvokeHostFunctionResultCode::Success,
            Self::Malformed => InvokeHostFunctionResultCode::Malformed,
            Self::Trapped => InvokeHostFunctionResultCode::Trapped,
        }
    }
}

impl ReadXdr for InvokeHostFunctionResult {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: InvokeHostFunctionResultCode =
            <InvokeHostFunctionResultCode as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            InvokeHostFunctionResultCode::Success => Self::Success,
            InvokeHostFunctionResultCode::Malformed => Self::Malformed,
            InvokeHostFunctionResultCode::Trapped => Self::Trapped,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for InvokeHostFunctionResult {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => ().write_xdr(w)?,
            Self::Malformed => ().write_xdr(w)?,
            Self::Trapped => ().write_xdr(w)?,
        };
        Ok(())
    }
}

// OperationResultCode is an XDR Enum defines as:
//
//   enum OperationResultCode
//    {
//        opINNER = 0, // inner object result is valid
//
//        opBAD_AUTH = -1,            // too few valid signatures / wrong network
//        opNO_ACCOUNT = -2,          // source account was not found
//        opNOT_SUPPORTED = -3,       // operation not supported at this time
//        opTOO_MANY_SUBENTRIES = -4, // max number of subentries already reached
//        opEXCEEDED_WORK_LIMIT = -5, // operation did too much work
//        opTOO_MANY_SPONSORING = -6  // account is sponsoring too many entries
//    };
//
// enum
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum OperationResultCode {
    OpInner = 0,
    OpBadAuth = -1,
    OpNoAccount = -2,
    OpNotSupported = -3,
    OpTooManySubentries = -4,
    OpExceededWorkLimit = -5,
    OpTooManySponsoring = -6,
}

impl OperationResultCode {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::OpInner => "OpInner",
            Self::OpBadAuth => "OpBadAuth",
            Self::OpNoAccount => "OpNoAccount",
            Self::OpNotSupported => "OpNotSupported",
            Self::OpTooManySubentries => "OpTooManySubentries",
            Self::OpExceededWorkLimit => "OpExceededWorkLimit",
            Self::OpTooManySponsoring => "OpTooManySponsoring",
        }
    }
}

impl fmt::Display for OperationResultCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for OperationResultCode {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self> {
        let e = match i {
            0 => OperationResultCode::OpInner,
            -1 => OperationResultCode::OpBadAuth,
            -2 => OperationResultCode::OpNoAccount,
            -3 => OperationResultCode::OpNotSupported,
            -4 => OperationResultCode::OpTooManySubentries,
            -5 => OperationResultCode::OpExceededWorkLimit,
            -6 => OperationResultCode::OpTooManySponsoring,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<OperationResultCode> for i32 {
    #[must_use]
    fn from(e: OperationResultCode) -> Self {
        e as Self
    }
}

impl ReadXdr for OperationResultCode {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXdr for OperationResultCode {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// OperationResultTr is an XDR NestedUnion defines as:
//
//   union switch (OperationType type)
//        {
//        case CREATE_ACCOUNT:
//            CreateAccountResult createAccountResult;
//        case PAYMENT:
//            PaymentResult paymentResult;
//        case PATH_PAYMENT_STRICT_RECEIVE:
//            PathPaymentStrictReceiveResult pathPaymentStrictReceiveResult;
//        case MANAGE_SELL_OFFER:
//            ManageSellOfferResult manageSellOfferResult;
//        case CREATE_PASSIVE_SELL_OFFER:
//            ManageSellOfferResult createPassiveSellOfferResult;
//        case SET_OPTIONS:
//            SetOptionsResult setOptionsResult;
//        case CHANGE_TRUST:
//            ChangeTrustResult changeTrustResult;
//        case ALLOW_TRUST:
//            AllowTrustResult allowTrustResult;
//        case ACCOUNT_MERGE:
//            AccountMergeResult accountMergeResult;
//        case INFLATION:
//            InflationResult inflationResult;
//        case MANAGE_DATA:
//            ManageDataResult manageDataResult;
//        case BUMP_SEQUENCE:
//            BumpSequenceResult bumpSeqResult;
//        case MANAGE_BUY_OFFER:
//            ManageBuyOfferResult manageBuyOfferResult;
//        case PATH_PAYMENT_STRICT_SEND:
//            PathPaymentStrictSendResult pathPaymentStrictSendResult;
//        case CREATE_CLAIMABLE_BALANCE:
//            CreateClaimableBalanceResult createClaimableBalanceResult;
//        case CLAIM_CLAIMABLE_BALANCE:
//            ClaimClaimableBalanceResult claimClaimableBalanceResult;
//        case BEGIN_SPONSORING_FUTURE_RESERVES:
//            BeginSponsoringFutureReservesResult beginSponsoringFutureReservesResult;
//        case END_SPONSORING_FUTURE_RESERVES:
//            EndSponsoringFutureReservesResult endSponsoringFutureReservesResult;
//        case REVOKE_SPONSORSHIP:
//            RevokeSponsorshipResult revokeSponsorshipResult;
//        case CLAWBACK:
//            ClawbackResult clawbackResult;
//        case CLAWBACK_CLAIMABLE_BALANCE:
//            ClawbackClaimableBalanceResult clawbackClaimableBalanceResult;
//        case SET_TRUST_LINE_FLAGS:
//            SetTrustLineFlagsResult setTrustLineFlagsResult;
//        case LIQUIDITY_POOL_DEPOSIT:
//            LiquidityPoolDepositResult liquidityPoolDepositResult;
//        case LIQUIDITY_POOL_WITHDRAW:
//            LiquidityPoolWithdrawResult liquidityPoolWithdrawResult;
//        case INVOKE_HOST_FUNCTION:
//            InvokeHostFunctionResult invokeHostFunctionResult;
//        }
//
// union with discriminant OperationType
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum OperationResultTr {
    CreateAccount(CreateAccountResult),
    Payment(PaymentResult),
    PathPaymentStrictReceive(PathPaymentStrictReceiveResult),
    ManageSellOffer(ManageSellOfferResult),
    CreatePassiveSellOffer(ManageSellOfferResult),
    SetOptions(SetOptionsResult),
    ChangeTrust(ChangeTrustResult),
    AllowTrust(AllowTrustResult),
    AccountMerge(AccountMergeResult),
    Inflation(InflationResult),
    ManageData(ManageDataResult),
    BumpSequence(BumpSequenceResult),
    ManageBuyOffer(ManageBuyOfferResult),
    PathPaymentStrictSend(PathPaymentStrictSendResult),
    CreateClaimableBalance(CreateClaimableBalanceResult),
    ClaimClaimableBalance(ClaimClaimableBalanceResult),
    BeginSponsoringFutureReserves(BeginSponsoringFutureReservesResult),
    EndSponsoringFutureReserves(EndSponsoringFutureReservesResult),
    RevokeSponsorship(RevokeSponsorshipResult),
    Clawback(ClawbackResult),
    ClawbackClaimableBalance(ClawbackClaimableBalanceResult),
    SetTrustLineFlags(SetTrustLineFlagsResult),
    LiquidityPoolDeposit(LiquidityPoolDepositResult),
    LiquidityPoolWithdraw(LiquidityPoolWithdrawResult),
    InvokeHostFunction(InvokeHostFunctionResult),
}

impl OperationResultTr {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::CreateAccount(_) => "CreateAccount",
            Self::Payment(_) => "Payment",
            Self::PathPaymentStrictReceive(_) => "PathPaymentStrictReceive",
            Self::ManageSellOffer(_) => "ManageSellOffer",
            Self::CreatePassiveSellOffer(_) => "CreatePassiveSellOffer",
            Self::SetOptions(_) => "SetOptions",
            Self::ChangeTrust(_) => "ChangeTrust",
            Self::AllowTrust(_) => "AllowTrust",
            Self::AccountMerge(_) => "AccountMerge",
            Self::Inflation(_) => "Inflation",
            Self::ManageData(_) => "ManageData",
            Self::BumpSequence(_) => "BumpSequence",
            Self::ManageBuyOffer(_) => "ManageBuyOffer",
            Self::PathPaymentStrictSend(_) => "PathPaymentStrictSend",
            Self::CreateClaimableBalance(_) => "CreateClaimableBalance",
            Self::ClaimClaimableBalance(_) => "ClaimClaimableBalance",
            Self::BeginSponsoringFutureReserves(_) => "BeginSponsoringFutureReserves",
            Self::EndSponsoringFutureReserves(_) => "EndSponsoringFutureReserves",
            Self::RevokeSponsorship(_) => "RevokeSponsorship",
            Self::Clawback(_) => "Clawback",
            Self::ClawbackClaimableBalance(_) => "ClawbackClaimableBalance",
            Self::SetTrustLineFlags(_) => "SetTrustLineFlags",
            Self::LiquidityPoolDeposit(_) => "LiquidityPoolDeposit",
            Self::LiquidityPoolWithdraw(_) => "LiquidityPoolWithdraw",
            Self::InvokeHostFunction(_) => "InvokeHostFunction",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> OperationType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::CreateAccount(_) => OperationType::CreateAccount,
            Self::Payment(_) => OperationType::Payment,
            Self::PathPaymentStrictReceive(_) => OperationType::PathPaymentStrictReceive,
            Self::ManageSellOffer(_) => OperationType::ManageSellOffer,
            Self::CreatePassiveSellOffer(_) => OperationType::CreatePassiveSellOffer,
            Self::SetOptions(_) => OperationType::SetOptions,
            Self::ChangeTrust(_) => OperationType::ChangeTrust,
            Self::AllowTrust(_) => OperationType::AllowTrust,
            Self::AccountMerge(_) => OperationType::AccountMerge,
            Self::Inflation(_) => OperationType::Inflation,
            Self::ManageData(_) => OperationType::ManageData,
            Self::BumpSequence(_) => OperationType::BumpSequence,
            Self::ManageBuyOffer(_) => OperationType::ManageBuyOffer,
            Self::PathPaymentStrictSend(_) => OperationType::PathPaymentStrictSend,
            Self::CreateClaimableBalance(_) => OperationType::CreateClaimableBalance,
            Self::ClaimClaimableBalance(_) => OperationType::ClaimClaimableBalance,
            Self::BeginSponsoringFutureReserves(_) => OperationType::BeginSponsoringFutureReserves,
            Self::EndSponsoringFutureReserves(_) => OperationType::EndSponsoringFutureReserves,
            Self::RevokeSponsorship(_) => OperationType::RevokeSponsorship,
            Self::Clawback(_) => OperationType::Clawback,
            Self::ClawbackClaimableBalance(_) => OperationType::ClawbackClaimableBalance,
            Self::SetTrustLineFlags(_) => OperationType::SetTrustLineFlags,
            Self::LiquidityPoolDeposit(_) => OperationType::LiquidityPoolDeposit,
            Self::LiquidityPoolWithdraw(_) => OperationType::LiquidityPoolWithdraw,
            Self::InvokeHostFunction(_) => OperationType::InvokeHostFunction,
        }
    }
}

impl ReadXdr for OperationResultTr {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: OperationType = <OperationType as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            OperationType::CreateAccount => Self::CreateAccount(CreateAccountResult::read_xdr(r)?),
            OperationType::Payment => Self::Payment(PaymentResult::read_xdr(r)?),
            OperationType::PathPaymentStrictReceive => {
                Self::PathPaymentStrictReceive(PathPaymentStrictReceiveResult::read_xdr(r)?)
            }
            OperationType::ManageSellOffer => {
                Self::ManageSellOffer(ManageSellOfferResult::read_xdr(r)?)
            }
            OperationType::CreatePassiveSellOffer => {
                Self::CreatePassiveSellOffer(ManageSellOfferResult::read_xdr(r)?)
            }
            OperationType::SetOptions => Self::SetOptions(SetOptionsResult::read_xdr(r)?),
            OperationType::ChangeTrust => Self::ChangeTrust(ChangeTrustResult::read_xdr(r)?),
            OperationType::AllowTrust => Self::AllowTrust(AllowTrustResult::read_xdr(r)?),
            OperationType::AccountMerge => Self::AccountMerge(AccountMergeResult::read_xdr(r)?),
            OperationType::Inflation => Self::Inflation(InflationResult::read_xdr(r)?),
            OperationType::ManageData => Self::ManageData(ManageDataResult::read_xdr(r)?),
            OperationType::BumpSequence => Self::BumpSequence(BumpSequenceResult::read_xdr(r)?),
            OperationType::ManageBuyOffer => {
                Self::ManageBuyOffer(ManageBuyOfferResult::read_xdr(r)?)
            }
            OperationType::PathPaymentStrictSend => {
                Self::PathPaymentStrictSend(PathPaymentStrictSendResult::read_xdr(r)?)
            }
            OperationType::CreateClaimableBalance => {
                Self::CreateClaimableBalance(CreateClaimableBalanceResult::read_xdr(r)?)
            }
            OperationType::ClaimClaimableBalance => {
                Self::ClaimClaimableBalance(ClaimClaimableBalanceResult::read_xdr(r)?)
            }
            OperationType::BeginSponsoringFutureReserves => Self::BeginSponsoringFutureReserves(
                BeginSponsoringFutureReservesResult::read_xdr(r)?,
            ),
            OperationType::EndSponsoringFutureReserves => {
                Self::EndSponsoringFutureReserves(EndSponsoringFutureReservesResult::read_xdr(r)?)
            }
            OperationType::RevokeSponsorship => {
                Self::RevokeSponsorship(RevokeSponsorshipResult::read_xdr(r)?)
            }
            OperationType::Clawback => Self::Clawback(ClawbackResult::read_xdr(r)?),
            OperationType::ClawbackClaimableBalance => {
                Self::ClawbackClaimableBalance(ClawbackClaimableBalanceResult::read_xdr(r)?)
            }
            OperationType::SetTrustLineFlags => {
                Self::SetTrustLineFlags(SetTrustLineFlagsResult::read_xdr(r)?)
            }
            OperationType::LiquidityPoolDeposit => {
                Self::LiquidityPoolDeposit(LiquidityPoolDepositResult::read_xdr(r)?)
            }
            OperationType::LiquidityPoolWithdraw => {
                Self::LiquidityPoolWithdraw(LiquidityPoolWithdrawResult::read_xdr(r)?)
            }
            OperationType::InvokeHostFunction => {
                Self::InvokeHostFunction(InvokeHostFunctionResult::read_xdr(r)?)
            }
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for OperationResultTr {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::CreateAccount(v) => v.write_xdr(w)?,
            Self::Payment(v) => v.write_xdr(w)?,
            Self::PathPaymentStrictReceive(v) => v.write_xdr(w)?,
            Self::ManageSellOffer(v) => v.write_xdr(w)?,
            Self::CreatePassiveSellOffer(v) => v.write_xdr(w)?,
            Self::SetOptions(v) => v.write_xdr(w)?,
            Self::ChangeTrust(v) => v.write_xdr(w)?,
            Self::AllowTrust(v) => v.write_xdr(w)?,
            Self::AccountMerge(v) => v.write_xdr(w)?,
            Self::Inflation(v) => v.write_xdr(w)?,
            Self::ManageData(v) => v.write_xdr(w)?,
            Self::BumpSequence(v) => v.write_xdr(w)?,
            Self::ManageBuyOffer(v) => v.write_xdr(w)?,
            Self::PathPaymentStrictSend(v) => v.write_xdr(w)?,
            Self::CreateClaimableBalance(v) => v.write_xdr(w)?,
            Self::ClaimClaimableBalance(v) => v.write_xdr(w)?,
            Self::BeginSponsoringFutureReserves(v) => v.write_xdr(w)?,
            Self::EndSponsoringFutureReserves(v) => v.write_xdr(w)?,
            Self::RevokeSponsorship(v) => v.write_xdr(w)?,
            Self::Clawback(v) => v.write_xdr(w)?,
            Self::ClawbackClaimableBalance(v) => v.write_xdr(w)?,
            Self::SetTrustLineFlags(v) => v.write_xdr(w)?,
            Self::LiquidityPoolDeposit(v) => v.write_xdr(w)?,
            Self::LiquidityPoolWithdraw(v) => v.write_xdr(w)?,
            Self::InvokeHostFunction(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// OperationResult is an XDR Union defines as:
//
//   union OperationResult switch (OperationResultCode code)
//    {
//    case opINNER:
//        union switch (OperationType type)
//        {
//        case CREATE_ACCOUNT:
//            CreateAccountResult createAccountResult;
//        case PAYMENT:
//            PaymentResult paymentResult;
//        case PATH_PAYMENT_STRICT_RECEIVE:
//            PathPaymentStrictReceiveResult pathPaymentStrictReceiveResult;
//        case MANAGE_SELL_OFFER:
//            ManageSellOfferResult manageSellOfferResult;
//        case CREATE_PASSIVE_SELL_OFFER:
//            ManageSellOfferResult createPassiveSellOfferResult;
//        case SET_OPTIONS:
//            SetOptionsResult setOptionsResult;
//        case CHANGE_TRUST:
//            ChangeTrustResult changeTrustResult;
//        case ALLOW_TRUST:
//            AllowTrustResult allowTrustResult;
//        case ACCOUNT_MERGE:
//            AccountMergeResult accountMergeResult;
//        case INFLATION:
//            InflationResult inflationResult;
//        case MANAGE_DATA:
//            ManageDataResult manageDataResult;
//        case BUMP_SEQUENCE:
//            BumpSequenceResult bumpSeqResult;
//        case MANAGE_BUY_OFFER:
//            ManageBuyOfferResult manageBuyOfferResult;
//        case PATH_PAYMENT_STRICT_SEND:
//            PathPaymentStrictSendResult pathPaymentStrictSendResult;
//        case CREATE_CLAIMABLE_BALANCE:
//            CreateClaimableBalanceResult createClaimableBalanceResult;
//        case CLAIM_CLAIMABLE_BALANCE:
//            ClaimClaimableBalanceResult claimClaimableBalanceResult;
//        case BEGIN_SPONSORING_FUTURE_RESERVES:
//            BeginSponsoringFutureReservesResult beginSponsoringFutureReservesResult;
//        case END_SPONSORING_FUTURE_RESERVES:
//            EndSponsoringFutureReservesResult endSponsoringFutureReservesResult;
//        case REVOKE_SPONSORSHIP:
//            RevokeSponsorshipResult revokeSponsorshipResult;
//        case CLAWBACK:
//            ClawbackResult clawbackResult;
//        case CLAWBACK_CLAIMABLE_BALANCE:
//            ClawbackClaimableBalanceResult clawbackClaimableBalanceResult;
//        case SET_TRUST_LINE_FLAGS:
//            SetTrustLineFlagsResult setTrustLineFlagsResult;
//        case LIQUIDITY_POOL_DEPOSIT:
//            LiquidityPoolDepositResult liquidityPoolDepositResult;
//        case LIQUIDITY_POOL_WITHDRAW:
//            LiquidityPoolWithdrawResult liquidityPoolWithdrawResult;
//        case INVOKE_HOST_FUNCTION:
//            InvokeHostFunctionResult invokeHostFunctionResult;
//        }
//        tr;
//    case opBAD_AUTH:
//    case opNO_ACCOUNT:
//    case opNOT_SUPPORTED:
//    case opTOO_MANY_SUBENTRIES:
//    case opEXCEEDED_WORK_LIMIT:
//    case opTOO_MANY_SPONSORING:
//        void;
//    };
//
// union with discriminant OperationResultCode
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum OperationResult {
    OpInner(OperationResultTr),
    OpBadAuth,
    OpNoAccount,
    OpNotSupported,
    OpTooManySubentries,
    OpExceededWorkLimit,
    OpTooManySponsoring,
}

impl OperationResult {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::OpInner(_) => "OpInner",
            Self::OpBadAuth => "OpBadAuth",
            Self::OpNoAccount => "OpNoAccount",
            Self::OpNotSupported => "OpNotSupported",
            Self::OpTooManySubentries => "OpTooManySubentries",
            Self::OpExceededWorkLimit => "OpExceededWorkLimit",
            Self::OpTooManySponsoring => "OpTooManySponsoring",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> OperationResultCode {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::OpInner(_) => OperationResultCode::OpInner,
            Self::OpBadAuth => OperationResultCode::OpBadAuth,
            Self::OpNoAccount => OperationResultCode::OpNoAccount,
            Self::OpNotSupported => OperationResultCode::OpNotSupported,
            Self::OpTooManySubentries => OperationResultCode::OpTooManySubentries,
            Self::OpExceededWorkLimit => OperationResultCode::OpExceededWorkLimit,
            Self::OpTooManySponsoring => OperationResultCode::OpTooManySponsoring,
        }
    }
}

impl ReadXdr for OperationResult {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: OperationResultCode = <OperationResultCode as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            OperationResultCode::OpInner => Self::OpInner(OperationResultTr::read_xdr(r)?),
            OperationResultCode::OpBadAuth => Self::OpBadAuth,
            OperationResultCode::OpNoAccount => Self::OpNoAccount,
            OperationResultCode::OpNotSupported => Self::OpNotSupported,
            OperationResultCode::OpTooManySubentries => Self::OpTooManySubentries,
            OperationResultCode::OpExceededWorkLimit => Self::OpExceededWorkLimit,
            OperationResultCode::OpTooManySponsoring => Self::OpTooManySponsoring,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for OperationResult {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::OpInner(v) => v.write_xdr(w)?,
            Self::OpBadAuth => ().write_xdr(w)?,
            Self::OpNoAccount => ().write_xdr(w)?,
            Self::OpNotSupported => ().write_xdr(w)?,
            Self::OpTooManySubentries => ().write_xdr(w)?,
            Self::OpExceededWorkLimit => ().write_xdr(w)?,
            Self::OpTooManySponsoring => ().write_xdr(w)?,
        };
        Ok(())
    }
}

// TransactionResultCode is an XDR Enum defines as:
//
//   enum TransactionResultCode
//    {
//        txFEE_BUMP_INNER_SUCCESS = 1, // fee bump inner transaction succeeded
//        txSUCCESS = 0,                // all operations succeeded
//
//        txFAILED = -1, // one of the operations failed (none were applied)
//
//        txTOO_EARLY = -2,         // ledger closeTime before minTime
//        txTOO_LATE = -3,          // ledger closeTime after maxTime
//        txMISSING_OPERATION = -4, // no operation was specified
//        txBAD_SEQ = -5,           // sequence number does not match source account
//
//        txBAD_AUTH = -6,             // too few valid signatures / wrong network
//        txINSUFFICIENT_BALANCE = -7, // fee would bring account below reserve
//        txNO_ACCOUNT = -8,           // source account not found
//        txINSUFFICIENT_FEE = -9,     // fee is too small
//        txBAD_AUTH_EXTRA = -10,      // unused signatures attached to transaction
//        txINTERNAL_ERROR = -11,      // an unknown error occurred
//
//        txNOT_SUPPORTED = -12,         // transaction type not supported
//        txFEE_BUMP_INNER_FAILED = -13, // fee bump inner transaction failed
//        txBAD_SPONSORSHIP = -14,       // sponsorship not confirmed
//        txBAD_MIN_SEQ_AGE_OR_GAP =
//            -15, // minSeqAge or minSeqLedgerGap conditions not met
//        txMALFORMED = -16 // precondition is invalid
//    };
//
// enum
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum TransactionResultCode {
    TxFeeBumpInnerSuccess = 1,
    TxSuccess = 0,
    TxFailed = -1,
    TxTooEarly = -2,
    TxTooLate = -3,
    TxMissingOperation = -4,
    TxBadSeq = -5,
    TxBadAuth = -6,
    TxInsufficientBalance = -7,
    TxNoAccount = -8,
    TxInsufficientFee = -9,
    TxBadAuthExtra = -10,
    TxInternalError = -11,
    TxNotSupported = -12,
    TxFeeBumpInnerFailed = -13,
    TxBadSponsorship = -14,
    TxBadMinSeqAgeOrGap = -15,
    TxMalformed = -16,
}

impl TransactionResultCode {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::TxFeeBumpInnerSuccess => "TxFeeBumpInnerSuccess",
            Self::TxSuccess => "TxSuccess",
            Self::TxFailed => "TxFailed",
            Self::TxTooEarly => "TxTooEarly",
            Self::TxTooLate => "TxTooLate",
            Self::TxMissingOperation => "TxMissingOperation",
            Self::TxBadSeq => "TxBadSeq",
            Self::TxBadAuth => "TxBadAuth",
            Self::TxInsufficientBalance => "TxInsufficientBalance",
            Self::TxNoAccount => "TxNoAccount",
            Self::TxInsufficientFee => "TxInsufficientFee",
            Self::TxBadAuthExtra => "TxBadAuthExtra",
            Self::TxInternalError => "TxInternalError",
            Self::TxNotSupported => "TxNotSupported",
            Self::TxFeeBumpInnerFailed => "TxFeeBumpInnerFailed",
            Self::TxBadSponsorship => "TxBadSponsorship",
            Self::TxBadMinSeqAgeOrGap => "TxBadMinSeqAgeOrGap",
            Self::TxMalformed => "TxMalformed",
        }
    }
}

impl fmt::Display for TransactionResultCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for TransactionResultCode {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self> {
        let e = match i {
            1 => TransactionResultCode::TxFeeBumpInnerSuccess,
            0 => TransactionResultCode::TxSuccess,
            -1 => TransactionResultCode::TxFailed,
            -2 => TransactionResultCode::TxTooEarly,
            -3 => TransactionResultCode::TxTooLate,
            -4 => TransactionResultCode::TxMissingOperation,
            -5 => TransactionResultCode::TxBadSeq,
            -6 => TransactionResultCode::TxBadAuth,
            -7 => TransactionResultCode::TxInsufficientBalance,
            -8 => TransactionResultCode::TxNoAccount,
            -9 => TransactionResultCode::TxInsufficientFee,
            -10 => TransactionResultCode::TxBadAuthExtra,
            -11 => TransactionResultCode::TxInternalError,
            -12 => TransactionResultCode::TxNotSupported,
            -13 => TransactionResultCode::TxFeeBumpInnerFailed,
            -14 => TransactionResultCode::TxBadSponsorship,
            -15 => TransactionResultCode::TxBadMinSeqAgeOrGap,
            -16 => TransactionResultCode::TxMalformed,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<TransactionResultCode> for i32 {
    #[must_use]
    fn from(e: TransactionResultCode) -> Self {
        e as Self
    }
}

impl ReadXdr for TransactionResultCode {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXdr for TransactionResultCode {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// InnerTransactionResultResult is an XDR NestedUnion defines as:
//
//   union switch (TransactionResultCode code)
//        {
//        // txFEE_BUMP_INNER_SUCCESS is not included
//        case txSUCCESS:
//        case txFAILED:
//            OperationResult results<>;
//        case txTOO_EARLY:
//        case txTOO_LATE:
//        case txMISSING_OPERATION:
//        case txBAD_SEQ:
//        case txBAD_AUTH:
//        case txINSUFFICIENT_BALANCE:
//        case txNO_ACCOUNT:
//        case txINSUFFICIENT_FEE:
//        case txBAD_AUTH_EXTRA:
//        case txINTERNAL_ERROR:
//        case txNOT_SUPPORTED:
//        // txFEE_BUMP_INNER_FAILED is not included
//        case txBAD_SPONSORSHIP:
//        case txBAD_MIN_SEQ_AGE_OR_GAP:
//        case txMALFORMED:
//            void;
//        }
//
// union with discriminant TransactionResultCode
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum InnerTransactionResultResult {
    TxSuccess(VecM<OperationResult>),
    TxFailed(VecM<OperationResult>),
    TxTooEarly,
    TxTooLate,
    TxMissingOperation,
    TxBadSeq,
    TxBadAuth,
    TxInsufficientBalance,
    TxNoAccount,
    TxInsufficientFee,
    TxBadAuthExtra,
    TxInternalError,
    TxNotSupported,
    TxBadSponsorship,
    TxBadMinSeqAgeOrGap,
    TxMalformed,
}

impl InnerTransactionResultResult {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::TxSuccess(_) => "TxSuccess",
            Self::TxFailed(_) => "TxFailed",
            Self::TxTooEarly => "TxTooEarly",
            Self::TxTooLate => "TxTooLate",
            Self::TxMissingOperation => "TxMissingOperation",
            Self::TxBadSeq => "TxBadSeq",
            Self::TxBadAuth => "TxBadAuth",
            Self::TxInsufficientBalance => "TxInsufficientBalance",
            Self::TxNoAccount => "TxNoAccount",
            Self::TxInsufficientFee => "TxInsufficientFee",
            Self::TxBadAuthExtra => "TxBadAuthExtra",
            Self::TxInternalError => "TxInternalError",
            Self::TxNotSupported => "TxNotSupported",
            Self::TxBadSponsorship => "TxBadSponsorship",
            Self::TxBadMinSeqAgeOrGap => "TxBadMinSeqAgeOrGap",
            Self::TxMalformed => "TxMalformed",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> TransactionResultCode {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::TxSuccess(_) => TransactionResultCode::TxSuccess,
            Self::TxFailed(_) => TransactionResultCode::TxFailed,
            Self::TxTooEarly => TransactionResultCode::TxTooEarly,
            Self::TxTooLate => TransactionResultCode::TxTooLate,
            Self::TxMissingOperation => TransactionResultCode::TxMissingOperation,
            Self::TxBadSeq => TransactionResultCode::TxBadSeq,
            Self::TxBadAuth => TransactionResultCode::TxBadAuth,
            Self::TxInsufficientBalance => TransactionResultCode::TxInsufficientBalance,
            Self::TxNoAccount => TransactionResultCode::TxNoAccount,
            Self::TxInsufficientFee => TransactionResultCode::TxInsufficientFee,
            Self::TxBadAuthExtra => TransactionResultCode::TxBadAuthExtra,
            Self::TxInternalError => TransactionResultCode::TxInternalError,
            Self::TxNotSupported => TransactionResultCode::TxNotSupported,
            Self::TxBadSponsorship => TransactionResultCode::TxBadSponsorship,
            Self::TxBadMinSeqAgeOrGap => TransactionResultCode::TxBadMinSeqAgeOrGap,
            Self::TxMalformed => TransactionResultCode::TxMalformed,
        }
    }
}

impl ReadXdr for InnerTransactionResultResult {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: TransactionResultCode = <TransactionResultCode as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            TransactionResultCode::TxSuccess => {
                Self::TxSuccess(VecM::<OperationResult>::read_xdr(r)?)
            }
            TransactionResultCode::TxFailed => {
                Self::TxFailed(VecM::<OperationResult>::read_xdr(r)?)
            }
            TransactionResultCode::TxTooEarly => Self::TxTooEarly,
            TransactionResultCode::TxTooLate => Self::TxTooLate,
            TransactionResultCode::TxMissingOperation => Self::TxMissingOperation,
            TransactionResultCode::TxBadSeq => Self::TxBadSeq,
            TransactionResultCode::TxBadAuth => Self::TxBadAuth,
            TransactionResultCode::TxInsufficientBalance => Self::TxInsufficientBalance,
            TransactionResultCode::TxNoAccount => Self::TxNoAccount,
            TransactionResultCode::TxInsufficientFee => Self::TxInsufficientFee,
            TransactionResultCode::TxBadAuthExtra => Self::TxBadAuthExtra,
            TransactionResultCode::TxInternalError => Self::TxInternalError,
            TransactionResultCode::TxNotSupported => Self::TxNotSupported,
            TransactionResultCode::TxBadSponsorship => Self::TxBadSponsorship,
            TransactionResultCode::TxBadMinSeqAgeOrGap => Self::TxBadMinSeqAgeOrGap,
            TransactionResultCode::TxMalformed => Self::TxMalformed,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for InnerTransactionResultResult {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::TxSuccess(v) => v.write_xdr(w)?,
            Self::TxFailed(v) => v.write_xdr(w)?,
            Self::TxTooEarly => ().write_xdr(w)?,
            Self::TxTooLate => ().write_xdr(w)?,
            Self::TxMissingOperation => ().write_xdr(w)?,
            Self::TxBadSeq => ().write_xdr(w)?,
            Self::TxBadAuth => ().write_xdr(w)?,
            Self::TxInsufficientBalance => ().write_xdr(w)?,
            Self::TxNoAccount => ().write_xdr(w)?,
            Self::TxInsufficientFee => ().write_xdr(w)?,
            Self::TxBadAuthExtra => ().write_xdr(w)?,
            Self::TxInternalError => ().write_xdr(w)?,
            Self::TxNotSupported => ().write_xdr(w)?,
            Self::TxBadSponsorship => ().write_xdr(w)?,
            Self::TxBadMinSeqAgeOrGap => ().write_xdr(w)?,
            Self::TxMalformed => ().write_xdr(w)?,
        };
        Ok(())
    }
}

// InnerTransactionResultExt is an XDR NestedUnion defines as:
//
//   union switch (int v)
//        {
//        case 0:
//            void;
//        }
//
// union with discriminant i32
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum InnerTransactionResultExt {
    V0,
}

impl InnerTransactionResultExt {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0 => "V0",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> i32 {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0 => 0,
        }
    }
}

impl ReadXdr for InnerTransactionResultExt {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: i32 = <i32 as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            0 => Self::V0,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for InnerTransactionResultExt {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0 => ().write_xdr(w)?,
        };
        Ok(())
    }
}

// InnerTransactionResult is an XDR Struct defines as:
//
//   struct InnerTransactionResult
//    {
//        // Always 0. Here for binary compatibility.
//        int64 feeCharged;
//
//        union switch (TransactionResultCode code)
//        {
//        // txFEE_BUMP_INNER_SUCCESS is not included
//        case txSUCCESS:
//        case txFAILED:
//            OperationResult results<>;
//        case txTOO_EARLY:
//        case txTOO_LATE:
//        case txMISSING_OPERATION:
//        case txBAD_SEQ:
//        case txBAD_AUTH:
//        case txINSUFFICIENT_BALANCE:
//        case txNO_ACCOUNT:
//        case txINSUFFICIENT_FEE:
//        case txBAD_AUTH_EXTRA:
//        case txINTERNAL_ERROR:
//        case txNOT_SUPPORTED:
//        // txFEE_BUMP_INNER_FAILED is not included
//        case txBAD_SPONSORSHIP:
//        case txBAD_MIN_SEQ_AGE_OR_GAP:
//        case txMALFORMED:
//            void;
//        }
//        result;
//
//        // reserved for future use
//        union switch (int v)
//        {
//        case 0:
//            void;
//        }
//        ext;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct InnerTransactionResult {
    pub fee_charged: i64,
    pub result: InnerTransactionResultResult,
    pub ext: InnerTransactionResultExt,
}

impl ReadXdr for InnerTransactionResult {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            fee_charged: i64::read_xdr(r)?,
            result: InnerTransactionResultResult::read_xdr(r)?,
            ext: InnerTransactionResultExt::read_xdr(r)?,
        })
    }
}

impl WriteXdr for InnerTransactionResult {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.fee_charged.write_xdr(w)?;
        self.result.write_xdr(w)?;
        self.ext.write_xdr(w)?;
        Ok(())
    }
}

// InnerTransactionResultPair is an XDR Struct defines as:
//
//   struct InnerTransactionResultPair
//    {
//        Hash transactionHash;          // hash of the inner transaction
//        InnerTransactionResult result; // result for the inner transaction
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct InnerTransactionResultPair {
    pub transaction_hash: Hash,
    pub result: InnerTransactionResult,
}

impl ReadXdr for InnerTransactionResultPair {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            transaction_hash: Hash::read_xdr(r)?,
            result: InnerTransactionResult::read_xdr(r)?,
        })
    }
}

impl WriteXdr for InnerTransactionResultPair {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.transaction_hash.write_xdr(w)?;
        self.result.write_xdr(w)?;
        Ok(())
    }
}

// TransactionResultResult is an XDR NestedUnion defines as:
//
//   union switch (TransactionResultCode code)
//        {
//        case txFEE_BUMP_INNER_SUCCESS:
//        case txFEE_BUMP_INNER_FAILED:
//            InnerTransactionResultPair innerResultPair;
//        case txSUCCESS:
//        case txFAILED:
//            OperationResult results<>;
//        case txTOO_EARLY:
//        case txTOO_LATE:
//        case txMISSING_OPERATION:
//        case txBAD_SEQ:
//        case txBAD_AUTH:
//        case txINSUFFICIENT_BALANCE:
//        case txNO_ACCOUNT:
//        case txINSUFFICIENT_FEE:
//        case txBAD_AUTH_EXTRA:
//        case txINTERNAL_ERROR:
//        case txNOT_SUPPORTED:
//        // case txFEE_BUMP_INNER_FAILED: handled above
//        case txBAD_SPONSORSHIP:
//        case txBAD_MIN_SEQ_AGE_OR_GAP:
//        case txMALFORMED:
//            void;
//        }
//
// union with discriminant TransactionResultCode
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum TransactionResultResult {
    TxFeeBumpInnerSuccess(InnerTransactionResultPair),
    TxFeeBumpInnerFailed(InnerTransactionResultPair),
    TxSuccess(VecM<OperationResult>),
    TxFailed(VecM<OperationResult>),
    TxTooEarly,
    TxTooLate,
    TxMissingOperation,
    TxBadSeq,
    TxBadAuth,
    TxInsufficientBalance,
    TxNoAccount,
    TxInsufficientFee,
    TxBadAuthExtra,
    TxInternalError,
    TxNotSupported,
    TxBadSponsorship,
    TxBadMinSeqAgeOrGap,
    TxMalformed,
}

impl TransactionResultResult {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::TxFeeBumpInnerSuccess(_) => "TxFeeBumpInnerSuccess",
            Self::TxFeeBumpInnerFailed(_) => "TxFeeBumpInnerFailed",
            Self::TxSuccess(_) => "TxSuccess",
            Self::TxFailed(_) => "TxFailed",
            Self::TxTooEarly => "TxTooEarly",
            Self::TxTooLate => "TxTooLate",
            Self::TxMissingOperation => "TxMissingOperation",
            Self::TxBadSeq => "TxBadSeq",
            Self::TxBadAuth => "TxBadAuth",
            Self::TxInsufficientBalance => "TxInsufficientBalance",
            Self::TxNoAccount => "TxNoAccount",
            Self::TxInsufficientFee => "TxInsufficientFee",
            Self::TxBadAuthExtra => "TxBadAuthExtra",
            Self::TxInternalError => "TxInternalError",
            Self::TxNotSupported => "TxNotSupported",
            Self::TxBadSponsorship => "TxBadSponsorship",
            Self::TxBadMinSeqAgeOrGap => "TxBadMinSeqAgeOrGap",
            Self::TxMalformed => "TxMalformed",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> TransactionResultCode {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::TxFeeBumpInnerSuccess(_) => TransactionResultCode::TxFeeBumpInnerSuccess,
            Self::TxFeeBumpInnerFailed(_) => TransactionResultCode::TxFeeBumpInnerFailed,
            Self::TxSuccess(_) => TransactionResultCode::TxSuccess,
            Self::TxFailed(_) => TransactionResultCode::TxFailed,
            Self::TxTooEarly => TransactionResultCode::TxTooEarly,
            Self::TxTooLate => TransactionResultCode::TxTooLate,
            Self::TxMissingOperation => TransactionResultCode::TxMissingOperation,
            Self::TxBadSeq => TransactionResultCode::TxBadSeq,
            Self::TxBadAuth => TransactionResultCode::TxBadAuth,
            Self::TxInsufficientBalance => TransactionResultCode::TxInsufficientBalance,
            Self::TxNoAccount => TransactionResultCode::TxNoAccount,
            Self::TxInsufficientFee => TransactionResultCode::TxInsufficientFee,
            Self::TxBadAuthExtra => TransactionResultCode::TxBadAuthExtra,
            Self::TxInternalError => TransactionResultCode::TxInternalError,
            Self::TxNotSupported => TransactionResultCode::TxNotSupported,
            Self::TxBadSponsorship => TransactionResultCode::TxBadSponsorship,
            Self::TxBadMinSeqAgeOrGap => TransactionResultCode::TxBadMinSeqAgeOrGap,
            Self::TxMalformed => TransactionResultCode::TxMalformed,
        }
    }
}

impl ReadXdr for TransactionResultResult {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: TransactionResultCode = <TransactionResultCode as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            TransactionResultCode::TxFeeBumpInnerSuccess => {
                Self::TxFeeBumpInnerSuccess(InnerTransactionResultPair::read_xdr(r)?)
            }
            TransactionResultCode::TxFeeBumpInnerFailed => {
                Self::TxFeeBumpInnerFailed(InnerTransactionResultPair::read_xdr(r)?)
            }
            TransactionResultCode::TxSuccess => {
                Self::TxSuccess(VecM::<OperationResult>::read_xdr(r)?)
            }
            TransactionResultCode::TxFailed => {
                Self::TxFailed(VecM::<OperationResult>::read_xdr(r)?)
            }
            TransactionResultCode::TxTooEarly => Self::TxTooEarly,
            TransactionResultCode::TxTooLate => Self::TxTooLate,
            TransactionResultCode::TxMissingOperation => Self::TxMissingOperation,
            TransactionResultCode::TxBadSeq => Self::TxBadSeq,
            TransactionResultCode::TxBadAuth => Self::TxBadAuth,
            TransactionResultCode::TxInsufficientBalance => Self::TxInsufficientBalance,
            TransactionResultCode::TxNoAccount => Self::TxNoAccount,
            TransactionResultCode::TxInsufficientFee => Self::TxInsufficientFee,
            TransactionResultCode::TxBadAuthExtra => Self::TxBadAuthExtra,
            TransactionResultCode::TxInternalError => Self::TxInternalError,
            TransactionResultCode::TxNotSupported => Self::TxNotSupported,
            TransactionResultCode::TxBadSponsorship => Self::TxBadSponsorship,
            TransactionResultCode::TxBadMinSeqAgeOrGap => Self::TxBadMinSeqAgeOrGap,
            TransactionResultCode::TxMalformed => Self::TxMalformed,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for TransactionResultResult {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::TxFeeBumpInnerSuccess(v) => v.write_xdr(w)?,
            Self::TxFeeBumpInnerFailed(v) => v.write_xdr(w)?,
            Self::TxSuccess(v) => v.write_xdr(w)?,
            Self::TxFailed(v) => v.write_xdr(w)?,
            Self::TxTooEarly => ().write_xdr(w)?,
            Self::TxTooLate => ().write_xdr(w)?,
            Self::TxMissingOperation => ().write_xdr(w)?,
            Self::TxBadSeq => ().write_xdr(w)?,
            Self::TxBadAuth => ().write_xdr(w)?,
            Self::TxInsufficientBalance => ().write_xdr(w)?,
            Self::TxNoAccount => ().write_xdr(w)?,
            Self::TxInsufficientFee => ().write_xdr(w)?,
            Self::TxBadAuthExtra => ().write_xdr(w)?,
            Self::TxInternalError => ().write_xdr(w)?,
            Self::TxNotSupported => ().write_xdr(w)?,
            Self::TxBadSponsorship => ().write_xdr(w)?,
            Self::TxBadMinSeqAgeOrGap => ().write_xdr(w)?,
            Self::TxMalformed => ().write_xdr(w)?,
        };
        Ok(())
    }
}

// TransactionResultExt is an XDR NestedUnion defines as:
//
//   union switch (int v)
//        {
//        case 0:
//            void;
//        }
//
// union with discriminant i32
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum TransactionResultExt {
    V0,
}

impl TransactionResultExt {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0 => "V0",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> i32 {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0 => 0,
        }
    }
}

impl ReadXdr for TransactionResultExt {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: i32 = <i32 as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            0 => Self::V0,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for TransactionResultExt {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0 => ().write_xdr(w)?,
        };
        Ok(())
    }
}

// TransactionResult is an XDR Struct defines as:
//
//   struct TransactionResult
//    {
//        int64 feeCharged; // actual fee charged for the transaction
//
//        union switch (TransactionResultCode code)
//        {
//        case txFEE_BUMP_INNER_SUCCESS:
//        case txFEE_BUMP_INNER_FAILED:
//            InnerTransactionResultPair innerResultPair;
//        case txSUCCESS:
//        case txFAILED:
//            OperationResult results<>;
//        case txTOO_EARLY:
//        case txTOO_LATE:
//        case txMISSING_OPERATION:
//        case txBAD_SEQ:
//        case txBAD_AUTH:
//        case txINSUFFICIENT_BALANCE:
//        case txNO_ACCOUNT:
//        case txINSUFFICIENT_FEE:
//        case txBAD_AUTH_EXTRA:
//        case txINTERNAL_ERROR:
//        case txNOT_SUPPORTED:
//        // case txFEE_BUMP_INNER_FAILED: handled above
//        case txBAD_SPONSORSHIP:
//        case txBAD_MIN_SEQ_AGE_OR_GAP:
//        case txMALFORMED:
//            void;
//        }
//        result;
//
//        // reserved for future use
//        union switch (int v)
//        {
//        case 0:
//            void;
//        }
//        ext;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TransactionResult {
    pub fee_charged: i64,
    pub result: TransactionResultResult,
    pub ext: TransactionResultExt,
}

impl ReadXdr for TransactionResult {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            fee_charged: i64::read_xdr(r)?,
            result: TransactionResultResult::read_xdr(r)?,
            ext: TransactionResultExt::read_xdr(r)?,
        })
    }
}

impl WriteXdr for TransactionResult {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.fee_charged.write_xdr(w)?;
        self.result.write_xdr(w)?;
        self.ext.write_xdr(w)?;
        Ok(())
    }
}

// Hash is an XDR Typedef defines as:
//
//   typedef opaque Hash[32];
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Hash(pub [u8; 32]);

impl From<Hash> for [u8; 32] {
    #[must_use]
    fn from(x: Hash) -> Self {
        x.0
    }
}

impl From<[u8; 32]> for Hash {
    #[must_use]
    fn from(x: [u8; 32]) -> Self {
        Hash(x)
    }
}

impl AsRef<[u8; 32]> for Hash {
    #[must_use]
    fn as_ref(&self) -> &[u8; 32] {
        &self.0
    }
}

impl ReadXdr for Hash {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let i = <[u8; 32]>::read_xdr(r)?;
        let v = Hash(i);
        Ok(v)
    }
}

impl WriteXdr for Hash {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.0.write_xdr(w)
    }
}

// Uint256 is an XDR Typedef defines as:
//
//   typedef opaque uint256[32];
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Uint256(pub [u8; 32]);

impl From<Uint256> for [u8; 32] {
    #[must_use]
    fn from(x: Uint256) -> Self {
        x.0
    }
}

impl From<[u8; 32]> for Uint256 {
    #[must_use]
    fn from(x: [u8; 32]) -> Self {
        Uint256(x)
    }
}

impl AsRef<[u8; 32]> for Uint256 {
    #[must_use]
    fn as_ref(&self) -> &[u8; 32] {
        &self.0
    }
}

impl ReadXdr for Uint256 {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let i = <[u8; 32]>::read_xdr(r)?;
        let v = Uint256(i);
        Ok(v)
    }
}

impl WriteXdr for Uint256 {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.0.write_xdr(w)
    }
}

// Uint32 is an XDR Typedef defines as:
//
//   typedef unsigned int uint32;
//
pub type Uint32 = u32;

// Int32 is an XDR Typedef defines as:
//
//   typedef int int32;
//
pub type Int32 = i32;

// Uint64 is an XDR Typedef defines as:
//
//   typedef unsigned hyper uint64;
//
pub type Uint64 = u64;

// Int64 is an XDR Typedef defines as:
//
//   typedef hyper int64;
//
pub type Int64 = i64;

// ExtensionPoint is an XDR Union defines as:
//
//   union ExtensionPoint switch (int v)
//    {
//    case 0:
//        void;
//    };
//
// union with discriminant i32
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ExtensionPoint {
    V0,
}

impl ExtensionPoint {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0 => "V0",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> i32 {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0 => 0,
        }
    }
}

impl ReadXdr for ExtensionPoint {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: i32 = <i32 as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            0 => Self::V0,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for ExtensionPoint {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0 => ().write_xdr(w)?,
        };
        Ok(())
    }
}

// CryptoKeyType is an XDR Enum defines as:
//
//   enum CryptoKeyType
//    {
//        KEY_TYPE_ED25519 = 0,
//        KEY_TYPE_PRE_AUTH_TX = 1,
//        KEY_TYPE_HASH_X = 2,
//        KEY_TYPE_ED25519_SIGNED_PAYLOAD = 3,
//        // MUXED enum values for supported type are derived from the enum values
//        // above by ORing them with 0x100
//        KEY_TYPE_MUXED_ED25519 = 0x100
//    };
//
// enum
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum CryptoKeyType {
    Ed25519 = 0,
    PreAuthTx = 1,
    HashX = 2,
    Ed25519SignedPayload = 3,
    MuxedEd25519 = 256,
}

impl CryptoKeyType {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Ed25519 => "Ed25519",
            Self::PreAuthTx => "PreAuthTx",
            Self::HashX => "HashX",
            Self::Ed25519SignedPayload => "Ed25519SignedPayload",
            Self::MuxedEd25519 => "MuxedEd25519",
        }
    }
}

impl fmt::Display for CryptoKeyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for CryptoKeyType {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self> {
        let e = match i {
            0 => CryptoKeyType::Ed25519,
            1 => CryptoKeyType::PreAuthTx,
            2 => CryptoKeyType::HashX,
            3 => CryptoKeyType::Ed25519SignedPayload,
            256 => CryptoKeyType::MuxedEd25519,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<CryptoKeyType> for i32 {
    #[must_use]
    fn from(e: CryptoKeyType) -> Self {
        e as Self
    }
}

impl ReadXdr for CryptoKeyType {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXdr for CryptoKeyType {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// PublicKeyType is an XDR Enum defines as:
//
//   enum PublicKeyType
//    {
//        PUBLIC_KEY_TYPE_ED25519 = KEY_TYPE_ED25519
//    };
//
// enum
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum PublicKeyType {
    PublicKeyTypeEd25519 = 0,
}

impl PublicKeyType {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::PublicKeyTypeEd25519 => "PublicKeyTypeEd25519",
        }
    }
}

impl fmt::Display for PublicKeyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for PublicKeyType {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self> {
        let e = match i {
            0 => PublicKeyType::PublicKeyTypeEd25519,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<PublicKeyType> for i32 {
    #[must_use]
    fn from(e: PublicKeyType) -> Self {
        e as Self
    }
}

impl ReadXdr for PublicKeyType {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXdr for PublicKeyType {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// SignerKeyType is an XDR Enum defines as:
//
//   enum SignerKeyType
//    {
//        SIGNER_KEY_TYPE_ED25519 = KEY_TYPE_ED25519,
//        SIGNER_KEY_TYPE_PRE_AUTH_TX = KEY_TYPE_PRE_AUTH_TX,
//        SIGNER_KEY_TYPE_HASH_X = KEY_TYPE_HASH_X,
//        SIGNER_KEY_TYPE_ED25519_SIGNED_PAYLOAD = KEY_TYPE_ED25519_SIGNED_PAYLOAD
//    };
//
// enum
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum SignerKeyType {
    Ed25519 = 0,
    PreAuthTx = 1,
    HashX = 2,
    Ed25519SignedPayload = 3,
}

impl SignerKeyType {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Ed25519 => "Ed25519",
            Self::PreAuthTx => "PreAuthTx",
            Self::HashX => "HashX",
            Self::Ed25519SignedPayload => "Ed25519SignedPayload",
        }
    }
}

impl fmt::Display for SignerKeyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for SignerKeyType {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self> {
        let e = match i {
            0 => SignerKeyType::Ed25519,
            1 => SignerKeyType::PreAuthTx,
            2 => SignerKeyType::HashX,
            3 => SignerKeyType::Ed25519SignedPayload,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<SignerKeyType> for i32 {
    #[must_use]
    fn from(e: SignerKeyType) -> Self {
        e as Self
    }
}

impl ReadXdr for SignerKeyType {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXdr for SignerKeyType {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// PublicKey is an XDR Union defines as:
//
//   union PublicKey switch (PublicKeyType type)
//    {
//    case PUBLIC_KEY_TYPE_ED25519:
//        uint256 ed25519;
//    };
//
// union with discriminant PublicKeyType
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum PublicKey {
    PublicKeyTypeEd25519(Uint256),
}

impl PublicKey {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::PublicKeyTypeEd25519(_) => "PublicKeyTypeEd25519",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> PublicKeyType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::PublicKeyTypeEd25519(_) => PublicKeyType::PublicKeyTypeEd25519,
        }
    }
}

impl ReadXdr for PublicKey {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: PublicKeyType = <PublicKeyType as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            PublicKeyType::PublicKeyTypeEd25519 => {
                Self::PublicKeyTypeEd25519(Uint256::read_xdr(r)?)
            }
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for PublicKey {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::PublicKeyTypeEd25519(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// SignerKeyEd25519SignedPayload is an XDR NestedStruct defines as:
//
//   struct
//        {
//            /* Public key that must sign the payload. */
//            uint256 ed25519;
//            /* Payload to be raw signed by ed25519. */
//            opaque payload<64>;
//        }
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SignerKeyEd25519SignedPayload {
    pub ed25519: Uint256,
    pub payload: VecM<u8, 64>,
}

impl ReadXdr for SignerKeyEd25519SignedPayload {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            ed25519: Uint256::read_xdr(r)?,
            payload: VecM::<u8, 64>::read_xdr(r)?,
        })
    }
}

impl WriteXdr for SignerKeyEd25519SignedPayload {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.ed25519.write_xdr(w)?;
        self.payload.write_xdr(w)?;
        Ok(())
    }
}

// SignerKey is an XDR Union defines as:
//
//   union SignerKey switch (SignerKeyType type)
//    {
//    case SIGNER_KEY_TYPE_ED25519:
//        uint256 ed25519;
//    case SIGNER_KEY_TYPE_PRE_AUTH_TX:
//        /* SHA-256 Hash of TransactionSignaturePayload structure */
//        uint256 preAuthTx;
//    case SIGNER_KEY_TYPE_HASH_X:
//        /* Hash of random 256 bit preimage X */
//        uint256 hashX;
//    case SIGNER_KEY_TYPE_ED25519_SIGNED_PAYLOAD:
//        struct
//        {
//            /* Public key that must sign the payload. */
//            uint256 ed25519;
//            /* Payload to be raw signed by ed25519. */
//            opaque payload<64>;
//        } ed25519SignedPayload;
//    };
//
// union with discriminant SignerKeyType
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum SignerKey {
    Ed25519(Uint256),
    PreAuthTx(Uint256),
    HashX(Uint256),
    Ed25519SignedPayload(SignerKeyEd25519SignedPayload),
}

impl SignerKey {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Ed25519(_) => "Ed25519",
            Self::PreAuthTx(_) => "PreAuthTx",
            Self::HashX(_) => "HashX",
            Self::Ed25519SignedPayload(_) => "Ed25519SignedPayload",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> SignerKeyType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Ed25519(_) => SignerKeyType::Ed25519,
            Self::PreAuthTx(_) => SignerKeyType::PreAuthTx,
            Self::HashX(_) => SignerKeyType::HashX,
            Self::Ed25519SignedPayload(_) => SignerKeyType::Ed25519SignedPayload,
        }
    }
}

impl ReadXdr for SignerKey {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: SignerKeyType = <SignerKeyType as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            SignerKeyType::Ed25519 => Self::Ed25519(Uint256::read_xdr(r)?),
            SignerKeyType::PreAuthTx => Self::PreAuthTx(Uint256::read_xdr(r)?),
            SignerKeyType::HashX => Self::HashX(Uint256::read_xdr(r)?),
            SignerKeyType::Ed25519SignedPayload => {
                Self::Ed25519SignedPayload(SignerKeyEd25519SignedPayload::read_xdr(r)?)
            }
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for SignerKey {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Ed25519(v) => v.write_xdr(w)?,
            Self::PreAuthTx(v) => v.write_xdr(w)?,
            Self::HashX(v) => v.write_xdr(w)?,
            Self::Ed25519SignedPayload(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// Signature is an XDR Typedef defines as:
//
//   typedef opaque Signature<64>;
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Signature(pub VecM<u8, 64>);

impl From<Signature> for VecM<u8, 64> {
    #[must_use]
    fn from(x: Signature) -> Self {
        x.0
    }
}

impl From<VecM<u8, 64>> for Signature {
    #[must_use]
    fn from(x: VecM<u8, 64>) -> Self {
        Signature(x)
    }
}

impl AsRef<VecM<u8, 64>> for Signature {
    #[must_use]
    fn as_ref(&self) -> &VecM<u8, 64> {
        &self.0
    }
}

impl ReadXdr for Signature {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let i = VecM::<u8, 64>::read_xdr(r)?;
        let v = Signature(i);
        Ok(v)
    }
}

impl WriteXdr for Signature {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.0.write_xdr(w)
    }
}

impl Deref for Signature {
    type Target = VecM<u8, 64>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Signature> for Vec<u8> {
    #[must_use]
    fn from(x: Signature) -> Self {
        x.0 .0
    }
}

impl TryFrom<Vec<u8>> for Signature {
    type Error = Error;
    fn try_from(x: Vec<u8>) -> Result<Self> {
        Ok(Signature(x.try_into()?))
    }
}

impl AsRef<Vec<u8>> for Signature {
    #[must_use]
    fn as_ref(&self) -> &Vec<u8> {
        &self.0 .0
    }
}

impl AsRef<[u8]> for Signature {
    #[cfg(feature = "alloc")]
    #[must_use]
    fn as_ref(&self) -> &[u8] {
        &self.0 .0
    }
    #[cfg(not(feature = "alloc"))]
    #[must_use]
    fn as_ref(&self) -> &[u8] {
        self.0 .0
    }
}

// SignatureHint is an XDR Typedef defines as:
//
//   typedef opaque SignatureHint[4];
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SignatureHint(pub [u8; 4]);

impl From<SignatureHint> for [u8; 4] {
    #[must_use]
    fn from(x: SignatureHint) -> Self {
        x.0
    }
}

impl From<[u8; 4]> for SignatureHint {
    #[must_use]
    fn from(x: [u8; 4]) -> Self {
        SignatureHint(x)
    }
}

impl AsRef<[u8; 4]> for SignatureHint {
    #[must_use]
    fn as_ref(&self) -> &[u8; 4] {
        &self.0
    }
}

impl ReadXdr for SignatureHint {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let i = <[u8; 4]>::read_xdr(r)?;
        let v = SignatureHint(i);
        Ok(v)
    }
}

impl WriteXdr for SignatureHint {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.0.write_xdr(w)
    }
}

// NodeId is an XDR Typedef defines as:
//
//   typedef PublicKey NodeID;
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct NodeId(pub PublicKey);

impl From<NodeId> for PublicKey {
    #[must_use]
    fn from(x: NodeId) -> Self {
        x.0
    }
}

impl From<PublicKey> for NodeId {
    #[must_use]
    fn from(x: PublicKey) -> Self {
        NodeId(x)
    }
}

impl AsRef<PublicKey> for NodeId {
    #[must_use]
    fn as_ref(&self) -> &PublicKey {
        &self.0
    }
}

impl ReadXdr for NodeId {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let i = PublicKey::read_xdr(r)?;
        let v = NodeId(i);
        Ok(v)
    }
}

impl WriteXdr for NodeId {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.0.write_xdr(w)
    }
}

// Curve25519Secret is an XDR Struct defines as:
//
//   struct Curve25519Secret
//    {
//        opaque key[32];
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Curve25519Secret {
    pub key: [u8; 32],
}

impl ReadXdr for Curve25519Secret {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            key: <[u8; 32]>::read_xdr(r)?,
        })
    }
}

impl WriteXdr for Curve25519Secret {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.key.write_xdr(w)?;
        Ok(())
    }
}

// Curve25519Public is an XDR Struct defines as:
//
//   struct Curve25519Public
//    {
//        opaque key[32];
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Curve25519Public {
    pub key: [u8; 32],
}

impl ReadXdr for Curve25519Public {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            key: <[u8; 32]>::read_xdr(r)?,
        })
    }
}

impl WriteXdr for Curve25519Public {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.key.write_xdr(w)?;
        Ok(())
    }
}

// HmacSha256Key is an XDR Struct defines as:
//
//   struct HmacSha256Key
//    {
//        opaque key[32];
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct HmacSha256Key {
    pub key: [u8; 32],
}

impl ReadXdr for HmacSha256Key {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            key: <[u8; 32]>::read_xdr(r)?,
        })
    }
}

impl WriteXdr for HmacSha256Key {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.key.write_xdr(w)?;
        Ok(())
    }
}

// HmacSha256Mac is an XDR Struct defines as:
//
//   struct HmacSha256Mac
//    {
//        opaque mac[32];
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct HmacSha256Mac {
    pub mac: [u8; 32],
}

impl ReadXdr for HmacSha256Mac {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            mac: <[u8; 32]>::read_xdr(r)?,
        })
    }
}

impl WriteXdr for HmacSha256Mac {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.mac.write_xdr(w)?;
        Ok(())
    }
}

// ScSymbol is an XDR Typedef defines as:
//
//   typedef string SCSymbol<10>;
//
pub type ScSymbol = VecM<u8, 10>;

// ScValType is an XDR Enum defines as:
//
//   enum SCValType
//    {
//        SCV_U63 = 0,
//        SCV_U32 = 1,
//        SCV_I32 = 2,
//        SCV_STATIC = 3,
//        SCV_OBJECT = 4,
//        SCV_SYMBOL = 5,
//        SCV_BITSET = 6,
//        SCV_STATUS = 7
//    };
//
// enum
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum ScValType {
    U63 = 0,
    U32 = 1,
    I32 = 2,
    Static = 3,
    Object = 4,
    Symbol = 5,
    Bitset = 6,
    Status = 7,
}

impl ScValType {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::U63 => "U63",
            Self::U32 => "U32",
            Self::I32 => "I32",
            Self::Static => "Static",
            Self::Object => "Object",
            Self::Symbol => "Symbol",
            Self::Bitset => "Bitset",
            Self::Status => "Status",
        }
    }
}

impl fmt::Display for ScValType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for ScValType {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self> {
        let e = match i {
            0 => ScValType::U63,
            1 => ScValType::U32,
            2 => ScValType::I32,
            3 => ScValType::Static,
            4 => ScValType::Object,
            5 => ScValType::Symbol,
            6 => ScValType::Bitset,
            7 => ScValType::Status,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<ScValType> for i32 {
    #[must_use]
    fn from(e: ScValType) -> Self {
        e as Self
    }
}

impl ReadXdr for ScValType {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXdr for ScValType {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// ScStatic is an XDR Enum defines as:
//
//   enum SCStatic
//    {
//        SCS_VOID = 0,
//        SCS_TRUE = 1,
//        SCS_FALSE = 2,
//        SCS_LEDGER_KEY_CONTRACT_CODE_WASM = 3
//    };
//
// enum
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum ScStatic {
    Void = 0,
    True = 1,
    False = 2,
    LedgerKeyContractCodeWasm = 3,
}

impl ScStatic {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Void => "Void",
            Self::True => "True",
            Self::False => "False",
            Self::LedgerKeyContractCodeWasm => "LedgerKeyContractCodeWasm",
        }
    }
}

impl fmt::Display for ScStatic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for ScStatic {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self> {
        let e = match i {
            0 => ScStatic::Void,
            1 => ScStatic::True,
            2 => ScStatic::False,
            3 => ScStatic::LedgerKeyContractCodeWasm,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<ScStatic> for i32 {
    #[must_use]
    fn from(e: ScStatic) -> Self {
        e as Self
    }
}

impl ReadXdr for ScStatic {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXdr for ScStatic {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// ScStatusType is an XDR Enum defines as:
//
//   enum SCStatusType
//    {
//        SST_OK = 0,
//        SST_UNKNOWN_ERROR = 1
//        // TODO: add more
//    };
//
// enum
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum ScStatusType {
    Ok = 0,
    UnknownError = 1,
}

impl ScStatusType {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Ok => "Ok",
            Self::UnknownError => "UnknownError",
        }
    }
}

impl fmt::Display for ScStatusType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for ScStatusType {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self> {
        let e = match i {
            0 => ScStatusType::Ok,
            1 => ScStatusType::UnknownError,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<ScStatusType> for i32 {
    #[must_use]
    fn from(e: ScStatusType) -> Self {
        e as Self
    }
}

impl ReadXdr for ScStatusType {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXdr for ScStatusType {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// ScStatus is an XDR Union defines as:
//
//   union SCStatus switch (SCStatusType type)
//    {
//    case SST_OK:
//        void;
//    case SST_UNKNOWN_ERROR:
//        uint32 unknownCode;
//    };
//
// union with discriminant ScStatusType
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ScStatus {
    Ok,
    UnknownError(u32),
}

impl ScStatus {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Ok => "Ok",
            Self::UnknownError(_) => "UnknownError",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> ScStatusType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Ok => ScStatusType::Ok,
            Self::UnknownError(_) => ScStatusType::UnknownError,
        }
    }
}

impl ReadXdr for ScStatus {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: ScStatusType = <ScStatusType as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            ScStatusType::Ok => Self::Ok,
            ScStatusType::UnknownError => Self::UnknownError(u32::read_xdr(r)?),
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for ScStatus {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Ok => ().write_xdr(w)?,
            Self::UnknownError(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// ScVal is an XDR Union defines as:
//
//   union SCVal switch (SCValType type)
//    {
//    case SCV_U63:
//        int64 u63;
//    case SCV_U32:
//        uint32 u32;
//    case SCV_I32:
//        int32 i32;
//    case SCV_STATIC:
//        SCStatic ic;
//    case SCV_OBJECT:
//        SCObject* obj;
//    case SCV_SYMBOL:
//        SCSymbol sym;
//    case SCV_BITSET:
//        uint64 bits;
//    case SCV_STATUS:
//        SCStatus status;
//    };
//
// union with discriminant ScValType
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ScVal {
    U63(i64),
    U32(u32),
    I32(i32),
    Static(ScStatic),
    Object(Option<ScObject>),
    Symbol(VecM<u8, 10>),
    Bitset(u64),
    Status(ScStatus),
}

impl ScVal {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::U63(_) => "U63",
            Self::U32(_) => "U32",
            Self::I32(_) => "I32",
            Self::Static(_) => "Static",
            Self::Object(_) => "Object",
            Self::Symbol(_) => "Symbol",
            Self::Bitset(_) => "Bitset",
            Self::Status(_) => "Status",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> ScValType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::U63(_) => ScValType::U63,
            Self::U32(_) => ScValType::U32,
            Self::I32(_) => ScValType::I32,
            Self::Static(_) => ScValType::Static,
            Self::Object(_) => ScValType::Object,
            Self::Symbol(_) => ScValType::Symbol,
            Self::Bitset(_) => ScValType::Bitset,
            Self::Status(_) => ScValType::Status,
        }
    }
}

impl ReadXdr for ScVal {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: ScValType = <ScValType as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            ScValType::U63 => Self::U63(i64::read_xdr(r)?),
            ScValType::U32 => Self::U32(u32::read_xdr(r)?),
            ScValType::I32 => Self::I32(i32::read_xdr(r)?),
            ScValType::Static => Self::Static(ScStatic::read_xdr(r)?),
            ScValType::Object => Self::Object(Option::<ScObject>::read_xdr(r)?),
            ScValType::Symbol => Self::Symbol(VecM::<u8, 10>::read_xdr(r)?),
            ScValType::Bitset => Self::Bitset(u64::read_xdr(r)?),
            ScValType::Status => Self::Status(ScStatus::read_xdr(r)?),
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for ScVal {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::U63(v) => v.write_xdr(w)?,
            Self::U32(v) => v.write_xdr(w)?,
            Self::I32(v) => v.write_xdr(w)?,
            Self::Static(v) => v.write_xdr(w)?,
            Self::Object(v) => v.write_xdr(w)?,
            Self::Symbol(v) => v.write_xdr(w)?,
            Self::Bitset(v) => v.write_xdr(w)?,
            Self::Status(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// ScObjectType is an XDR Enum defines as:
//
//   enum SCObjectType
//    {
//        // We have a few objects that represent non-stellar-specific concepts
//        // like general-purpose maps, vectors, numbers, blobs.
//
//        SCO_VEC = 0,
//        SCO_MAP = 1,
//        SCO_U64 = 2,
//        SCO_I64 = 3,
//        SCO_BINARY = 4
//
//        // TODO: add more
//    };
//
// enum
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum ScObjectType {
    Vec = 0,
    Map = 1,
    U64 = 2,
    I64 = 3,
    Binary = 4,
}

impl ScObjectType {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Vec => "Vec",
            Self::Map => "Map",
            Self::U64 => "U64",
            Self::I64 => "I64",
            Self::Binary => "Binary",
        }
    }
}

impl fmt::Display for ScObjectType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for ScObjectType {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self> {
        let e = match i {
            0 => ScObjectType::Vec,
            1 => ScObjectType::Map,
            2 => ScObjectType::U64,
            3 => ScObjectType::I64,
            4 => ScObjectType::Binary,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<ScObjectType> for i32 {
    #[must_use]
    fn from(e: ScObjectType) -> Self {
        e as Self
    }
}

impl ReadXdr for ScObjectType {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXdr for ScObjectType {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// ScMapEntry is an XDR Struct defines as:
//
//   struct SCMapEntry
//    {
//        SCVal key;
//        SCVal val;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScMapEntry {
    pub key: ScVal,
    pub val: ScVal,
}

impl ReadXdr for ScMapEntry {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            key: ScVal::read_xdr(r)?,
            val: ScVal::read_xdr(r)?,
        })
    }
}

impl WriteXdr for ScMapEntry {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.key.write_xdr(w)?;
        self.val.write_xdr(w)?;
        Ok(())
    }
}

// ScvalLimit is an XDR Const defines as:
//
//   const SCVAL_LIMIT = 256000;
//
pub const SCVAL_LIMIT: u64 = 256000;

// ScVec is an XDR Typedef defines as:
//
//   typedef SCVal SCVec<SCVAL_LIMIT>;
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScVec(pub VecM<ScVal, 256000>);

impl From<ScVec> for VecM<ScVal, 256000> {
    #[must_use]
    fn from(x: ScVec) -> Self {
        x.0
    }
}

impl From<VecM<ScVal, 256000>> for ScVec {
    #[must_use]
    fn from(x: VecM<ScVal, 256000>) -> Self {
        ScVec(x)
    }
}

impl AsRef<VecM<ScVal, 256000>> for ScVec {
    #[must_use]
    fn as_ref(&self) -> &VecM<ScVal, 256000> {
        &self.0
    }
}

impl ReadXdr for ScVec {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let i = VecM::<ScVal, 256000>::read_xdr(r)?;
        let v = ScVec(i);
        Ok(v)
    }
}

impl WriteXdr for ScVec {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.0.write_xdr(w)
    }
}

impl Deref for ScVec {
    type Target = VecM<ScVal, 256000>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<ScVec> for Vec<ScVal> {
    #[must_use]
    fn from(x: ScVec) -> Self {
        x.0 .0
    }
}

impl TryFrom<Vec<ScVal>> for ScVec {
    type Error = Error;
    fn try_from(x: Vec<ScVal>) -> Result<Self> {
        Ok(ScVec(x.try_into()?))
    }
}

impl AsRef<Vec<ScVal>> for ScVec {
    #[must_use]
    fn as_ref(&self) -> &Vec<ScVal> {
        &self.0 .0
    }
}

impl AsRef<[ScVal]> for ScVec {
    #[cfg(feature = "alloc")]
    #[must_use]
    fn as_ref(&self) -> &[ScVal] {
        &self.0 .0
    }
    #[cfg(not(feature = "alloc"))]
    #[must_use]
    fn as_ref(&self) -> &[ScVal] {
        self.0 .0
    }
}

// ScMap is an XDR Typedef defines as:
//
//   typedef SCMapEntry SCMap<SCVAL_LIMIT>;
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScMap(pub VecM<ScMapEntry, 256000>);

impl From<ScMap> for VecM<ScMapEntry, 256000> {
    #[must_use]
    fn from(x: ScMap) -> Self {
        x.0
    }
}

impl From<VecM<ScMapEntry, 256000>> for ScMap {
    #[must_use]
    fn from(x: VecM<ScMapEntry, 256000>) -> Self {
        ScMap(x)
    }
}

impl AsRef<VecM<ScMapEntry, 256000>> for ScMap {
    #[must_use]
    fn as_ref(&self) -> &VecM<ScMapEntry, 256000> {
        &self.0
    }
}

impl ReadXdr for ScMap {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let i = VecM::<ScMapEntry, 256000>::read_xdr(r)?;
        let v = ScMap(i);
        Ok(v)
    }
}

impl WriteXdr for ScMap {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.0.write_xdr(w)
    }
}

impl Deref for ScMap {
    type Target = VecM<ScMapEntry, 256000>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<ScMap> for Vec<ScMapEntry> {
    #[must_use]
    fn from(x: ScMap) -> Self {
        x.0 .0
    }
}

impl TryFrom<Vec<ScMapEntry>> for ScMap {
    type Error = Error;
    fn try_from(x: Vec<ScMapEntry>) -> Result<Self> {
        Ok(ScMap(x.try_into()?))
    }
}

impl AsRef<Vec<ScMapEntry>> for ScMap {
    #[must_use]
    fn as_ref(&self) -> &Vec<ScMapEntry> {
        &self.0 .0
    }
}

impl AsRef<[ScMapEntry]> for ScMap {
    #[cfg(feature = "alloc")]
    #[must_use]
    fn as_ref(&self) -> &[ScMapEntry] {
        &self.0 .0
    }
    #[cfg(not(feature = "alloc"))]
    #[must_use]
    fn as_ref(&self) -> &[ScMapEntry] {
        self.0 .0
    }
}

// ScObject is an XDR Union defines as:
//
//   union SCObject switch (SCObjectType type)
//    {
//    case SCO_VEC:
//        SCVec vec;
//    case SCO_MAP:
//        SCMap map;
//    case SCO_U64:
//        uint64 u64;
//    case SCO_I64:
//        int64 i64;
//    case SCO_BINARY:
//        opaque bin<SCVAL_LIMIT>;
//    };
//
// union with discriminant ScObjectType
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ScObject {
    Vec(ScVec),
    Map(ScMap),
    U64(u64),
    I64(i64),
    Binary(VecM<u8, 256000>),
}

impl ScObject {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Vec(_) => "Vec",
            Self::Map(_) => "Map",
            Self::U64(_) => "U64",
            Self::I64(_) => "I64",
            Self::Binary(_) => "Binary",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> ScObjectType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Vec(_) => ScObjectType::Vec,
            Self::Map(_) => ScObjectType::Map,
            Self::U64(_) => ScObjectType::U64,
            Self::I64(_) => ScObjectType::I64,
            Self::Binary(_) => ScObjectType::Binary,
        }
    }
}

impl ReadXdr for ScObject {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: ScObjectType = <ScObjectType as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            ScObjectType::Vec => Self::Vec(ScVec::read_xdr(r)?),
            ScObjectType::Map => Self::Map(ScMap::read_xdr(r)?),
            ScObjectType::U64 => Self::U64(u64::read_xdr(r)?),
            ScObjectType::I64 => Self::I64(i64::read_xdr(r)?),
            ScObjectType::Binary => Self::Binary(VecM::<u8, 256000>::read_xdr(r)?),
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for ScObject {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Vec(v) => v.write_xdr(w)?,
            Self::Map(v) => v.write_xdr(w)?,
            Self::U64(v) => v.write_xdr(w)?,
            Self::I64(v) => v.write_xdr(w)?,
            Self::Binary(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// SpecType is an XDR Enum defines as:
//
//   enum SpecType
//    {
//        // Types with no parameters.
//        SPEC_TYPE_U32 = 1,
//        SPEC_TYPE_I32 = 2,
//        SPEC_TYPE_U64 = 3,
//        SPEC_TYPE_I64 = 4,
//        SPEC_TYPE_BOOL = 5,
//        SPEC_TYPE_SYMBOL = 6,
//        SPEC_TYPE_BITSET = 7,
//        SPEC_TYPE_STATUS = 8,
//        SPEC_TYPE_BINARY = 9,
//        SPEC_TYPE_BIG_INT = 10,
//
//        // Types with parameters.
//        SPEC_TYPE_OPTION = 1000,
//        SPEC_TYPE_RESULT = 1001,
//        SPEC_TYPE_VEC = 1002,
//        SPEC_TYPE_SET = 1003,
//        SPEC_TYPE_MAP = 1004,
//        SPEC_TYPE_TUPLE = 1005,
//
//        // User defined types.
//        SPEC_TYPE_UDT = 2000
//    };
//
// enum
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum SpecType {
    U32 = 1,
    I32 = 2,
    U64 = 3,
    I64 = 4,
    Bool = 5,
    Symbol = 6,
    Bitset = 7,
    Status = 8,
    Binary = 9,
    BigInt = 10,
    Option = 1000,
    Result = 1001,
    Vec = 1002,
    Set = 1003,
    Map = 1004,
    Tuple = 1005,
    Udt = 2000,
}

impl SpecType {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::U32 => "U32",
            Self::I32 => "I32",
            Self::U64 => "U64",
            Self::I64 => "I64",
            Self::Bool => "Bool",
            Self::Symbol => "Symbol",
            Self::Bitset => "Bitset",
            Self::Status => "Status",
            Self::Binary => "Binary",
            Self::BigInt => "BigInt",
            Self::Option => "Option",
            Self::Result => "Result",
            Self::Vec => "Vec",
            Self::Set => "Set",
            Self::Map => "Map",
            Self::Tuple => "Tuple",
            Self::Udt => "Udt",
        }
    }
}

impl fmt::Display for SpecType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for SpecType {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self> {
        let e = match i {
            1 => SpecType::U32,
            2 => SpecType::I32,
            3 => SpecType::U64,
            4 => SpecType::I64,
            5 => SpecType::Bool,
            6 => SpecType::Symbol,
            7 => SpecType::Bitset,
            8 => SpecType::Status,
            9 => SpecType::Binary,
            10 => SpecType::BigInt,
            1000 => SpecType::Option,
            1001 => SpecType::Result,
            1002 => SpecType::Vec,
            1003 => SpecType::Set,
            1004 => SpecType::Map,
            1005 => SpecType::Tuple,
            2000 => SpecType::Udt,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<SpecType> for i32 {
    #[must_use]
    fn from(e: SpecType) -> Self {
        e as Self
    }
}

impl ReadXdr for SpecType {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXdr for SpecType {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// SpecTypeOption is an XDR Struct defines as:
//
//   struct SpecTypeOption
//    {
//        SpecTypeDef valueType;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SpecTypeOption {
    pub value_type: Box<SpecTypeDef>,
}

impl ReadXdr for SpecTypeOption {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            value_type: Box::<SpecTypeDef>::read_xdr(r)?,
        })
    }
}

impl WriteXdr for SpecTypeOption {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.value_type.write_xdr(w)?;
        Ok(())
    }
}

// SpecTypeResult is an XDR Struct defines as:
//
//   struct SpecTypeResult
//    {
//        SpecTypeDef okType;
//        SpecTypeDef errorType;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SpecTypeResult {
    pub ok_type: Box<SpecTypeDef>,
    pub error_type: Box<SpecTypeDef>,
}

impl ReadXdr for SpecTypeResult {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            ok_type: Box::<SpecTypeDef>::read_xdr(r)?,
            error_type: Box::<SpecTypeDef>::read_xdr(r)?,
        })
    }
}

impl WriteXdr for SpecTypeResult {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.ok_type.write_xdr(w)?;
        self.error_type.write_xdr(w)?;
        Ok(())
    }
}

// SpecTypeVec is an XDR Struct defines as:
//
//   struct SpecTypeVec
//    {
//        SpecTypeDef elementType;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SpecTypeVec {
    pub element_type: Box<SpecTypeDef>,
}

impl ReadXdr for SpecTypeVec {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            element_type: Box::<SpecTypeDef>::read_xdr(r)?,
        })
    }
}

impl WriteXdr for SpecTypeVec {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.element_type.write_xdr(w)?;
        Ok(())
    }
}

// SpecTypeMap is an XDR Struct defines as:
//
//   struct SpecTypeMap
//    {
//        SpecTypeDef keyType;
//        SpecTypeDef valueType;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SpecTypeMap {
    pub key_type: Box<SpecTypeDef>,
    pub value_type: Box<SpecTypeDef>,
}

impl ReadXdr for SpecTypeMap {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            key_type: Box::<SpecTypeDef>::read_xdr(r)?,
            value_type: Box::<SpecTypeDef>::read_xdr(r)?,
        })
    }
}

impl WriteXdr for SpecTypeMap {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.key_type.write_xdr(w)?;
        self.value_type.write_xdr(w)?;
        Ok(())
    }
}

// SpecTypeSet is an XDR Struct defines as:
//
//   struct SpecTypeSet
//    {
//        SpecTypeDef elementType;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SpecTypeSet {
    pub element_type: Box<SpecTypeDef>,
}

impl ReadXdr for SpecTypeSet {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            element_type: Box::<SpecTypeDef>::read_xdr(r)?,
        })
    }
}

impl WriteXdr for SpecTypeSet {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.element_type.write_xdr(w)?;
        Ok(())
    }
}

// SpecTypeTuple is an XDR Struct defines as:
//
//   struct SpecTypeTuple
//    {
//        SpecTypeDef valueTypes<12>;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SpecTypeTuple {
    pub value_types: VecM<SpecTypeDef, 12>,
}

impl ReadXdr for SpecTypeTuple {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            value_types: VecM::<SpecTypeDef, 12>::read_xdr(r)?,
        })
    }
}

impl WriteXdr for SpecTypeTuple {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.value_types.write_xdr(w)?;
        Ok(())
    }
}

// SpecTypeUdt is an XDR Struct defines as:
//
//   struct SpecTypeUDT
//    {
//        string name<60>;
//        SpecUDTDef *udtDef;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SpecTypeUdt {
    pub name: VecM<u8, 60>,
    pub udt_def: Option<Box<SpecUdtDef>>,
}

impl ReadXdr for SpecTypeUdt {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            name: VecM::<u8, 60>::read_xdr(r)?,
            udt_def: Option::<Box<SpecUdtDef>>::read_xdr(r)?,
        })
    }
}

impl WriteXdr for SpecTypeUdt {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.name.write_xdr(w)?;
        self.udt_def.write_xdr(w)?;
        Ok(())
    }
}

// SpecTypeDef is an XDR Union defines as:
//
//   union SpecTypeDef switch (SpecType type)
//    {
//    case SPEC_TYPE_U64:
//    case SPEC_TYPE_I64:
//    case SPEC_TYPE_U32:
//    case SPEC_TYPE_I32:
//    case SPEC_TYPE_BOOL:
//    case SPEC_TYPE_SYMBOL:
//    case SPEC_TYPE_BITSET:
//    case SPEC_TYPE_STATUS:
//    case SPEC_TYPE_BINARY:
//    case SPEC_TYPE_BIG_INT:
//        void;
//    case SPEC_TYPE_OPTION:
//        SpecTypeOption option;
//    case SPEC_TYPE_RESULT:
//        SpecTypeResult result;
//    case SPEC_TYPE_VEC:
//        SpecTypeVec vec;
//    case SPEC_TYPE_MAP:
//        SpecTypeMap map;
//    case SPEC_TYPE_SET:
//        SpecTypeSet set;
//    case SPEC_TYPE_TUPLE:
//        SpecTypeTuple tuple;
//    case SPEC_TYPE_UDT:
//        SpecTypeUDT udt;
//    };
//
// union with discriminant SpecType
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum SpecTypeDef {
    U64,
    I64,
    U32,
    I32,
    Bool,
    Symbol,
    Bitset,
    Status,
    Binary,
    BigInt,
    Option(Box<SpecTypeOption>),
    Result(Box<SpecTypeResult>),
    Vec(Box<SpecTypeVec>),
    Map(Box<SpecTypeMap>),
    Set(Box<SpecTypeSet>),
    Tuple(Box<SpecTypeTuple>),
    Udt(Box<SpecTypeUdt>),
}

impl SpecTypeDef {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::U64 => "U64",
            Self::I64 => "I64",
            Self::U32 => "U32",
            Self::I32 => "I32",
            Self::Bool => "Bool",
            Self::Symbol => "Symbol",
            Self::Bitset => "Bitset",
            Self::Status => "Status",
            Self::Binary => "Binary",
            Self::BigInt => "BigInt",
            Self::Option(_) => "Option",
            Self::Result(_) => "Result",
            Self::Vec(_) => "Vec",
            Self::Map(_) => "Map",
            Self::Set(_) => "Set",
            Self::Tuple(_) => "Tuple",
            Self::Udt(_) => "Udt",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> SpecType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::U64 => SpecType::U64,
            Self::I64 => SpecType::I64,
            Self::U32 => SpecType::U32,
            Self::I32 => SpecType::I32,
            Self::Bool => SpecType::Bool,
            Self::Symbol => SpecType::Symbol,
            Self::Bitset => SpecType::Bitset,
            Self::Status => SpecType::Status,
            Self::Binary => SpecType::Binary,
            Self::BigInt => SpecType::BigInt,
            Self::Option(_) => SpecType::Option,
            Self::Result(_) => SpecType::Result,
            Self::Vec(_) => SpecType::Vec,
            Self::Map(_) => SpecType::Map,
            Self::Set(_) => SpecType::Set,
            Self::Tuple(_) => SpecType::Tuple,
            Self::Udt(_) => SpecType::Udt,
        }
    }
}

impl ReadXdr for SpecTypeDef {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: SpecType = <SpecType as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            SpecType::U64 => Self::U64,
            SpecType::I64 => Self::I64,
            SpecType::U32 => Self::U32,
            SpecType::I32 => Self::I32,
            SpecType::Bool => Self::Bool,
            SpecType::Symbol => Self::Symbol,
            SpecType::Bitset => Self::Bitset,
            SpecType::Status => Self::Status,
            SpecType::Binary => Self::Binary,
            SpecType::BigInt => Self::BigInt,
            SpecType::Option => Self::Option(Box::<SpecTypeOption>::read_xdr(r)?),
            SpecType::Result => Self::Result(Box::<SpecTypeResult>::read_xdr(r)?),
            SpecType::Vec => Self::Vec(Box::<SpecTypeVec>::read_xdr(r)?),
            SpecType::Map => Self::Map(Box::<SpecTypeMap>::read_xdr(r)?),
            SpecType::Set => Self::Set(Box::<SpecTypeSet>::read_xdr(r)?),
            SpecType::Tuple => Self::Tuple(Box::<SpecTypeTuple>::read_xdr(r)?),
            SpecType::Udt => Self::Udt(Box::<SpecTypeUdt>::read_xdr(r)?),
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for SpecTypeDef {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::U64 => ().write_xdr(w)?,
            Self::I64 => ().write_xdr(w)?,
            Self::U32 => ().write_xdr(w)?,
            Self::I32 => ().write_xdr(w)?,
            Self::Bool => ().write_xdr(w)?,
            Self::Symbol => ().write_xdr(w)?,
            Self::Bitset => ().write_xdr(w)?,
            Self::Status => ().write_xdr(w)?,
            Self::Binary => ().write_xdr(w)?,
            Self::BigInt => ().write_xdr(w)?,
            Self::Option(v) => v.write_xdr(w)?,
            Self::Result(v) => v.write_xdr(w)?,
            Self::Vec(v) => v.write_xdr(w)?,
            Self::Map(v) => v.write_xdr(w)?,
            Self::Set(v) => v.write_xdr(w)?,
            Self::Tuple(v) => v.write_xdr(w)?,
            Self::Udt(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// SpecUdtType is an XDR Enum defines as:
//
//   enum SpecUDTType
//    {
//        SPEC_UDT_STRUCT = 0,
//        SPEC_UDT_UNION = 1
//    };
//
// enum
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum SpecUdtType {
    Struct = 0,
    Union = 1,
}

impl SpecUdtType {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Struct => "Struct",
            Self::Union => "Union",
        }
    }
}

impl fmt::Display for SpecUdtType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for SpecUdtType {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self> {
        let e = match i {
            0 => SpecUdtType::Struct,
            1 => SpecUdtType::Union,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<SpecUdtType> for i32 {
    #[must_use]
    fn from(e: SpecUdtType) -> Self {
        e as Self
    }
}

impl ReadXdr for SpecUdtType {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXdr for SpecUdtType {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// SpecUdtStructField is an XDR Struct defines as:
//
//   struct SpecUDTStructField
//    {
//        string name<30>;
//        SpecTypeDef type;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SpecUdtStructField {
    pub name: VecM<u8, 30>,
    pub type_: Box<SpecTypeDef>,
}

impl ReadXdr for SpecUdtStructField {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            name: VecM::<u8, 30>::read_xdr(r)?,
            type_: Box::<SpecTypeDef>::read_xdr(r)?,
        })
    }
}

impl WriteXdr for SpecUdtStructField {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.name.write_xdr(w)?;
        self.type_.write_xdr(w)?;
        Ok(())
    }
}

// SpecUdtStruct is an XDR Struct defines as:
//
//   struct SpecUDTStruct
//    {
//        SpecUDTStructField fields<40>;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SpecUdtStruct {
    pub fields: VecM<SpecUdtStructField, 40>,
}

impl ReadXdr for SpecUdtStruct {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            fields: VecM::<SpecUdtStructField, 40>::read_xdr(r)?,
        })
    }
}

impl WriteXdr for SpecUdtStruct {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.fields.write_xdr(w)?;
        Ok(())
    }
}

// SpecUdtUnionCase is an XDR Struct defines as:
//
//   struct SpecUDTUnionCase
//    {
//        string name<60>;
//        SpecTypeDef *type;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SpecUdtUnionCase {
    pub name: VecM<u8, 60>,
    pub type_: Option<Box<SpecTypeDef>>,
}

impl ReadXdr for SpecUdtUnionCase {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            name: VecM::<u8, 60>::read_xdr(r)?,
            type_: Option::<Box<SpecTypeDef>>::read_xdr(r)?,
        })
    }
}

impl WriteXdr for SpecUdtUnionCase {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.name.write_xdr(w)?;
        self.type_.write_xdr(w)?;
        Ok(())
    }
}

// SpecUdtUnion is an XDR Struct defines as:
//
//   struct SpecUDTUnion
//    {
//        SpecUDTUnionCase cases<50>;
//    };
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SpecUdtUnion {
    pub cases: VecM<SpecUdtUnionCase, 50>,
}

impl ReadXdr for SpecUdtUnion {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            cases: VecM::<SpecUdtUnionCase, 50>::read_xdr(r)?,
        })
    }
}

impl WriteXdr for SpecUdtUnion {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.cases.write_xdr(w)?;
        Ok(())
    }
}

// SpecUdtDef is an XDR Union defines as:
//
//   union SpecUDTDef switch (SpecUDTType type)
//    {
//    case SPEC_UDT_STRUCT:
//        SpecUDTStruct struct;
//    case SPEC_UDT_UNION:
//        SpecUDTUnion union;
//    };
//
// union with discriminant SpecUdtType
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum SpecUdtDef {
    Struct(Box<SpecUdtStruct>),
    Union(Box<SpecUdtUnion>),
}

impl SpecUdtDef {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Struct(_) => "Struct",
            Self::Union(_) => "Union",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> SpecUdtType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Struct(_) => SpecUdtType::Struct,
            Self::Union(_) => SpecUdtType::Union,
        }
    }
}

impl ReadXdr for SpecUdtDef {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: SpecUdtType = <SpecUdtType as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            SpecUdtType::Struct => Self::Struct(Box::<SpecUdtStruct>::read_xdr(r)?),
            SpecUdtType::Union => Self::Union(Box::<SpecUdtUnion>::read_xdr(r)?),
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for SpecUdtDef {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Struct(v) => v.write_xdr(w)?,
            Self::Union(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// SpecEntryFunctionV0 is an XDR NestedStruct defines as:
//
//   struct {
//            SCSymbol name;
//            SpecTypeDef inputTypes<10>;
//            SpecTypeDef outputTypes<1>;
//        }
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SpecEntryFunctionV0 {
    pub name: VecM<u8, 10>,
    pub input_types: VecM<SpecTypeDef, 10>,
    pub output_types: VecM<SpecTypeDef, 1>,
}

impl ReadXdr for SpecEntryFunctionV0 {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            name: VecM::<u8, 10>::read_xdr(r)?,
            input_types: VecM::<SpecTypeDef, 10>::read_xdr(r)?,
            output_types: VecM::<SpecTypeDef, 1>::read_xdr(r)?,
        })
    }
}

impl WriteXdr for SpecEntryFunctionV0 {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.name.write_xdr(w)?;
        self.input_types.write_xdr(w)?;
        self.output_types.write_xdr(w)?;
        Ok(())
    }
}

// SpecEntryFunction is an XDR Union defines as:
//
//   union SpecEntryFunction switch (int v)
//    {
//    case 0:
//        struct {
//            SCSymbol name;
//            SpecTypeDef inputTypes<10>;
//            SpecTypeDef outputTypes<1>;
//        } v0;
//    };
//
// union with discriminant i32
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum SpecEntryFunction {
    V0(SpecEntryFunctionV0),
}

impl SpecEntryFunction {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0(_) => "V0",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> i32 {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0(_) => 0,
        }
    }
}

impl ReadXdr for SpecEntryFunction {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: i32 = <i32 as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            0 => Self::V0(SpecEntryFunctionV0::read_xdr(r)?),
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for SpecEntryFunction {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// SpecEntryUdtV0 is an XDR NestedStruct defines as:
//
//   struct {
//            string name<60>;
//            SpecUDTDef typ;
//        }
//
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SpecEntryUdtV0 {
    pub name: VecM<u8, 60>,
    pub typ: SpecUdtDef,
}

impl ReadXdr for SpecEntryUdtV0 {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        Ok(Self {
            name: VecM::<u8, 60>::read_xdr(r)?,
            typ: SpecUdtDef::read_xdr(r)?,
        })
    }
}

impl WriteXdr for SpecEntryUdtV0 {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.name.write_xdr(w)?;
        self.typ.write_xdr(w)?;
        Ok(())
    }
}

// SpecEntryUdt is an XDR Union defines as:
//
//   union SpecEntryUDT switch (int v)
//    {
//    case 0:
//        struct {
//            string name<60>;
//            SpecUDTDef typ;
//        } v0;
//    };
//
// union with discriminant i32
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum SpecEntryUdt {
    V0(SpecEntryUdtV0),
}

impl SpecEntryUdt {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0(_) => "V0",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> i32 {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0(_) => 0,
        }
    }
}

impl ReadXdr for SpecEntryUdt {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: i32 = <i32 as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            0 => Self::V0(SpecEntryUdtV0::read_xdr(r)?),
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for SpecEntryUdt {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}

// SpecEntryKind is an XDR Enum defines as:
//
//   enum SpecEntryKind
//    {
//        SPEC_ENTRY_FUNCTION = 0,
//        SPEC_ENTRY_UDT = 1
//    };
//
// enum
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum SpecEntryKind {
    Function = 0,
    Udt = 1,
}

impl SpecEntryKind {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Function => "Function",
            Self::Udt => "Udt",
        }
    }
}

impl fmt::Display for SpecEntryKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for SpecEntryKind {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self> {
        let e = match i {
            0 => SpecEntryKind::Function,
            1 => SpecEntryKind::Udt,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<SpecEntryKind> for i32 {
    #[must_use]
    fn from(e: SpecEntryKind) -> Self {
        e as Self
    }
}

impl ReadXdr for SpecEntryKind {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let e = i32::read_xdr(r)?;
        let v: Self = e.try_into()?;
        Ok(v)
    }
}

impl WriteXdr for SpecEntryKind {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        let i: i32 = (*self).into();
        i.write_xdr(w)
    }
}

// SpecEntry is an XDR Union defines as:
//
//   union SpecEntry switch (SpecEntryKind kind)
//    {
//    case SPEC_ENTRY_FUNCTION:
//        SpecEntryFunction function;
//    case SPEC_ENTRY_UDT:
//        SpecEntryUDT udt;
//    };
//
// union with discriminant SpecEntryKind
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum SpecEntry {
    Function(SpecEntryFunction),
    Udt(SpecEntryUdt),
}

impl SpecEntry {
    #[must_use]
    pub fn name(&self) -> &str {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Function(_) => "Function",
            Self::Udt(_) => "Udt",
        }
    }

    #[must_use]
    pub fn discriminant(&self) -> SpecEntryKind {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Function(_) => SpecEntryKind::Function,
            Self::Udt(_) => SpecEntryKind::Udt,
        }
    }
}

impl ReadXdr for SpecEntry {
    #[cfg(feature = "std")]
    fn read_xdr(r: &mut impl Read) -> Result<Self> {
        let dv: SpecEntryKind = <SpecEntryKind as ReadXdr>::read_xdr(r)?;
        #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
        let v = match dv {
            SpecEntryKind::Function => Self::Function(SpecEntryFunction::read_xdr(r)?),
            SpecEntryKind::Udt => Self::Udt(SpecEntryUdt::read_xdr(r)?),
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(v)
    }
}

impl WriteXdr for SpecEntry {
    #[cfg(feature = "std")]
    fn write_xdr(&self, w: &mut impl Write) -> Result<()> {
        self.discriminant().write_xdr(w)?;
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Function(v) => v.write_xdr(w)?,
            Self::Udt(v) => v.write_xdr(w)?,
        };
        Ok(())
    }
}
