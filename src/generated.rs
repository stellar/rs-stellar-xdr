// Module is generated from:
//  xdr/Stellar-SCP.x
//  xdr/Stellar-contract-config-setting.x
//  xdr/Stellar-contract-env-meta.x
//  xdr/Stellar-contract-meta.x
//  xdr/Stellar-contract-spec.x
//  xdr/Stellar-contract.x
//  xdr/Stellar-exporter.x
//  xdr/Stellar-internal.x
//  xdr/Stellar-ledger-entries.x
//  xdr/Stellar-ledger.x
//  xdr/Stellar-overlay.x
//  xdr/Stellar-transaction.x
//  xdr/Stellar-types.x

#![allow(
    clippy::missing_errors_doc,
    clippy::unreadable_literal,
    clippy::wildcard_imports
)]

/// `XDR_FILES_SHA256` is a list of pairs of source files and their SHA256 hashes.
pub const XDR_FILES_SHA256: [(&str, &str); 13] = [
    (
        "xdr/Stellar-SCP.x",
        "6aed428fb6c2d000f5bc1eef0ba685d6108f3faa96208ffa588c0e2990813939",
    ),
    (
        "xdr/Stellar-contract-config-setting.x",
        "a034a3eb4d8b94f5c4c573fe14a1afc548aa316e1e897aa70e5a1688aada3c77",
    ),
    (
        "xdr/Stellar-contract-env-meta.x",
        "75a271414d852096fea3283c63b7f2a702f2905f78fc28eb60ec7d7bd366a780",
    ),
    (
        "xdr/Stellar-contract-meta.x",
        "f01532c11ca044e19d9f9f16fe373e9af64835da473be556b9a807ee3319ae0d",
    ),
    (
        "xdr/Stellar-contract-spec.x",
        "7d99679155f6ce029f4f2bd8e1bf09524ef2f3e4ca8973265085cfcfdbdae987",
    ),
    (
        "xdr/Stellar-contract.x",
        "dce61df115c93fef5bb352beac1b504a518cb11dcb8ee029b1bb1b5f8fe52982",
    ),
    (
        "xdr/Stellar-exporter.x",
        "a00c83d02e8c8382e06f79a191f1fb5abd097a4bbcab8481c67467e3270e0529",
    ),
    (
        "xdr/Stellar-internal.x",
        "227835866c1b2122d1eaf28839ba85ea7289d1cb681dda4ca619c2da3d71fe00",
    ),
    (
        "xdr/Stellar-ledger-entries.x",
        "5157cad76b008b3606fe5bc2cfe87596827d8e02d16cbec3cedc297bb571aa54",
    ),
    (
        "xdr/Stellar-ledger.x",
        "cf936606885dd265082e553aa433c2cf47b720b6d58839b154cf71096b885d1e",
    ),
    (
        "xdr/Stellar-overlay.x",
        "8c9b9c13c86fa4672f03d741705b41e7221be0fc48e1ea6eeb1ba07d31ec0723",
    ),
    (
        "xdr/Stellar-transaction.x",
        "30d03669fb29ca48fdda1c84258473fe6d798f3b881c0224b34df1a1f9e21e80",
    ),
    (
        "xdr/Stellar-types.x",
        "f3a360cbb07f24637ead188e885a0b4a47b903ca475f9798be5fa12453075e28",
    ),
];

use core::{array::TryFromSliceError, fmt, fmt::Debug, marker::Sized, ops::Deref, slice};

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
#[cfg(feature = "std")]
use std::string::FromUtf8Error;

#[cfg(feature = "arbitrary")]
use arbitrary::Arbitrary;

#[cfg(all(feature = "schemars", feature = "alloc", not(feature = "std")))]
use alloc::borrow::Cow;
#[cfg(all(feature = "schemars", feature = "alloc", feature = "std"))]
use std::borrow::Cow;

// TODO: Add support for read/write xdr fns when std not available.

#[cfg(feature = "std")]
use std::{
    error, io,
    io::{BufRead, BufReader, Cursor, Read, Write},
};

/// Error contains all errors returned by functions in this crate. It can be
/// compared via `PartialEq`, however any contained IO errors will only be
/// compared on their `ErrorKind`.
#[derive(Debug)]
pub enum Error {
    Invalid,
    Unsupported,
    LengthExceedsMax,
    LengthMismatch,
    NonZeroPadding,
    Utf8Error(core::str::Utf8Error),
    #[cfg(feature = "alloc")]
    InvalidHex,
    #[cfg(feature = "std")]
    Io(io::Error),
    DepthLimitExceeded,
    #[cfg(feature = "serde_json")]
    Json(serde_json::Error),
    LengthLimitExceeded,
    #[cfg(feature = "arbitrary")]
    Arbitrary(arbitrary::Error),
}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Invalid, Self::Invalid)
            | (Self::Unsupported, Self::Unsupported)
            | (Self::LengthExceedsMax, Self::LengthExceedsMax)
            | (Self::LengthMismatch, Self::LengthMismatch)
            | (Self::NonZeroPadding, Self::NonZeroPadding) => true,

            (Self::Utf8Error(l), Self::Utf8Error(r)) => l == r,

            #[cfg(feature = "alloc")]
            (Self::InvalidHex, Self::InvalidHex) => true,

            // IO errors cannot be compared, but in the absence of any more
            // meaningful way to compare the errors we compare the kind of error
            // and ignore the embedded source error or OS error. The main use
            // case for comparing errors outputted by the XDR library is for
            // error case testing, and a lack of the ability to compare has a
            // detrimental affect on failure testing, so this is a tradeoff.
            #[cfg(feature = "std")]
            (Self::Io(l), Self::Io(r)) => l.kind() == r.kind(),

            (Self::DepthLimitExceeded, Self::DepthLimitExceeded) => true,

            #[cfg(feature = "serde_json")]
            (Self::Json(l), Self::Json(r)) => l.classify() == r.classify(),

            (Self::LengthLimitExceeded, Self::LengthLimitExceeded) => true,

            #[cfg(feature = "arbitrary")]
            (Self::Arbitrary(l), Self::Arbitrary(r)) => l == r,

            _ => false,
        }
    }
}

#[cfg(feature = "std")]
impl error::Error for Error {
    #[must_use]
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::Invalid
            | Error::Unsupported
            | Error::LengthExceedsMax
            | Error::LengthMismatch
            | Error::NonZeroPadding => None,

            Error::Utf8Error(e) => Some(e),

            Self::InvalidHex => None,

            Self::Io(e) => Some(e),

            Self::DepthLimitExceeded => None,

            #[cfg(feature = "serde_json")]
            Self::Json(e) => Some(e),

            Self::LengthLimitExceeded => None,

            #[cfg(feature = "arbitrary")]
            Self::Arbitrary(e) => Some(e),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Invalid => write!(f, "xdr value invalid"),
            Error::Unsupported => write!(f, "xdr value unsupported"),
            Error::LengthExceedsMax => write!(f, "xdr value max length exceeded"),
            Error::LengthMismatch => write!(f, "xdr value length does not match"),
            Error::NonZeroPadding => write!(f, "xdr padding contains non-zero bytes"),
            Error::Utf8Error(e) => write!(f, "{e}"),

            #[cfg(feature = "alloc")]
            Error::InvalidHex => write!(f, "hex invalid"),

            #[cfg(feature = "std")]
            Error::Io(e) => write!(f, "{e}"),

            Error::DepthLimitExceeded => write!(f, "depth limit exceeded"),

            #[cfg(feature = "serde_json")]
            Error::Json(e) => write!(f, "{e}"),

            Error::LengthLimitExceeded => write!(f, "length limit exceeded"),

            #[cfg(feature = "arbitrary")]
            Error::Arbitrary(e) => write!(f, "{e}"),
        }
    }
}

impl From<TryFromSliceError> for Error {
    fn from(_: TryFromSliceError) -> Error {
        Error::LengthMismatch
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

#[cfg(feature = "serde_json")]
impl From<serde_json::Error> for Error {
    #[must_use]
    fn from(e: serde_json::Error) -> Self {
        Error::Json(e)
    }
}

#[cfg(feature = "arbitrary")]
impl From<arbitrary::Error> for Error {
    #[must_use]
    fn from(e: arbitrary::Error) -> Self {
        Error::Arbitrary(e)
    }
}

impl From<Error> for () {
    fn from(_: Error) {}
}

/// Name defines types that assign a static name to their value, such as the
/// name given to an identifier in an XDR enum, or the name given to the case in
/// a union.
pub trait Name {
    fn name(&self) -> &'static str;
}

/// Discriminant defines types that may contain a one-of value determined
/// according to the discriminant, and exposes the value of the discriminant for
/// that type, such as in an XDR union.
pub trait Discriminant<D> {
    fn discriminant(&self) -> D;
}

/// Iter defines types that have variants that can be iterated.
pub trait Variants<V> {
    fn variants() -> slice::Iter<'static, V>
    where
        V: Sized;
}

// Enum defines a type that is represented as an XDR enumeration when encoded.
pub trait Enum: Name + Variants<Self> + Sized {}

// Union defines a type that is represented as an XDR union when encoded.
pub trait Union<D>: Name + Discriminant<D> + Variants<D>
where
    D: Sized,
{
}

/// `Limits` contains the limits that a limited reader or writer will be
/// constrained to.
#[cfg(feature = "std")]
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Limits {
    /// Defines the maximum depth for recursive calls in `Read/WriteXdr` to
    /// prevent stack overflow.
    ///
    /// The depth limit is akin to limiting stack depth. Its purpose is to
    /// prevent the program from hitting the maximum stack size allowed by Rust,
    /// which would result in an unrecoverable `SIGABRT`.  For more information
    /// about Rust's stack size limit, refer to the [Rust
    /// documentation](https://doc.rust-lang.org/std/thread/#stack-size).
    pub depth: u32,

    /// Defines the maximum number of bytes that will be read or written.
    pub len: usize,
}

#[cfg(feature = "std")]
impl Limits {
    #[must_use]
    pub fn none() -> Self {
        Self {
            depth: u32::MAX,
            len: usize::MAX,
        }
    }

    #[must_use]
    pub fn depth(depth: u32) -> Self {
        Limits {
            depth,
            ..Limits::none()
        }
    }

    #[must_use]
    pub fn len(len: usize) -> Self {
        Limits {
            len,
            ..Limits::none()
        }
    }
}

/// `Limited` wraps an object and provides functions for enforcing limits.
///
/// Intended for use with readers and writers and limiting their reads and
/// writes.
#[cfg(feature = "std")]
pub struct Limited<L> {
    pub inner: L,
    pub(crate) limits: Limits,
}

#[cfg(feature = "std")]
impl<L> Limited<L> {
    /// Constructs a new `Limited`.
    ///
    /// - `inner`: The value being limited.
    /// - `limits`: The limits to enforce.
    pub fn new(inner: L, limits: Limits) -> Self {
        Limited { inner, limits }
    }

    /// Consume the given length from the internal remaining length limit.
    ///
    /// ### Errors
    ///
    /// If the length would consume more length than the remaining length limit
    /// allows.
    pub(crate) fn consume_len(&mut self, len: usize) -> Result<(), Error> {
        if let Some(len) = self.limits.len.checked_sub(len) {
            self.limits.len = len;
            Ok(())
        } else {
            Err(Error::LengthLimitExceeded)
        }
    }

    /// Consumes a single depth for the duration of the given function.
    ///
    /// ### Errors
    ///
    /// If the depth limit is already exhausted.
    pub(crate) fn with_limited_depth<T, F>(&mut self, f: F) -> Result<T, Error>
    where
        F: FnOnce(&mut Self) -> Result<T, Error>,
    {
        if let Some(depth) = self.limits.depth.checked_sub(1) {
            self.limits.depth = depth;
            let res = f(self);
            self.limits.depth = self.limits.depth.saturating_add(1);
            res
        } else {
            Err(Error::DepthLimitExceeded)
        }
    }
}

#[cfg(feature = "std")]
impl<R: Read> Read for Limited<R> {
    /// Forwards the read operation to the wrapped object.
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.inner.read(buf)
    }
}

#[cfg(feature = "std")]
impl<R: BufRead> BufRead for Limited<R> {
    /// Forwards the read operation to the wrapped object.
    fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
        self.inner.fill_buf()
    }

    /// Forwards the read operation to the wrapped object.
    fn consume(&mut self, amt: usize) {
        self.inner.consume(amt);
    }
}

#[cfg(feature = "std")]
impl<W: Write> Write for Limited<W> {
    /// Forwards the write operation to the wrapped object.
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.inner.write(buf)
    }

    /// Forwards the flush operation to the wrapped object.
    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }
}

#[cfg(feature = "std")]
pub struct ReadXdrIter<R: Read, S: ReadXdr> {
    reader: Limited<BufReader<R>>,
    _s: PhantomData<S>,
}

#[cfg(feature = "std")]
impl<R: Read, S: ReadXdr> ReadXdrIter<R, S> {
    fn new(r: R, limits: Limits) -> Self {
        Self {
            reader: Limited {
                inner: BufReader::new(r),
                limits,
            },
            _s: PhantomData,
        }
    }
}

#[cfg(feature = "std")]
impl<R: Read, S: ReadXdr> Iterator for ReadXdrIter<R, S> {
    type Item = Result<S, Error>;

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
        let r = self.reader.with_limited_depth(|dlr| S::read_xdr(dlr));
        match r {
            Ok(s) => Some(Ok(s)),
            Err(e) => Some(Err(e)),
        }
    }
}

pub trait ReadXdr
where
    Self: Sized,
{
    /// Read the XDR and construct the type.
    ///
    /// Read bytes from the given read implementation, decoding the bytes as
    /// XDR, and construct the type implementing this interface from those
    /// bytes.
    ///
    /// Just enough bytes are read from the read implementation to construct the
    /// type. Any residual bytes remain in the read implementation.
    ///
    /// All implementations should continue if the read implementation returns
    /// [`ErrorKind::Interrupted`](std::io::ErrorKind::Interrupted).
    ///
    /// Use [`ReadXdR: Read_xdr_to_end`] when the intent is for all bytes in the
    /// read implementation to be consumed by the read.
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error>;

    /// Construct the type from the XDR bytes base64 encoded.
    ///
    /// An error is returned if the bytes are not completely consumed by the
    /// deserialization.
    #[cfg(feature = "base64")]
    fn read_xdr_base64<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        let mut dec = Limited::new(
            base64::read::DecoderReader::new(
                SkipWhitespace::new(&mut r.inner),
                &base64::engine::general_purpose::STANDARD,
            ),
            r.limits.clone(),
        );
        let t = Self::read_xdr(&mut dec)?;
        Ok(t)
    }

    /// Read the XDR and construct the type, and consider it an error if the
    /// read does not completely consume the read implementation.
    ///
    /// Read bytes from the given read implementation, decoding the bytes as
    /// XDR, and construct the type implementing this interface from those
    /// bytes.
    ///
    /// Just enough bytes are read from the read implementation to construct the
    /// type, and then confirm that no further bytes remain. To confirm no
    /// further bytes remain additional bytes are attempted to be read from the
    /// read implementation. If it is possible to read any residual bytes from
    /// the read implementation an error is returned. The read implementation
    /// may not be exhaustively read if there are residual bytes, and it is
    /// considered undefined how many residual bytes or how much of the residual
    /// buffer are consumed in this case.
    ///
    /// All implementations should continue if the read implementation returns
    /// [`ErrorKind::Interrupted`](std::io::ErrorKind::Interrupted).
    #[cfg(feature = "std")]
    fn read_xdr_to_end<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        let s = Self::read_xdr(r)?;
        // Check that any further reads, such as this read of one byte, read no
        // data, indicating EOF. If a byte is read the data is invalid.
        if r.read(&mut [0u8; 1])? == 0 {
            Ok(s)
        } else {
            Err(Error::Invalid)
        }
    }

    /// Construct the type from the XDR bytes base64 encoded.
    ///
    /// An error is returned if the bytes are not completely consumed by the
    /// deserialization.
    #[cfg(feature = "base64")]
    fn read_xdr_base64_to_end<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        let mut dec = Limited::new(
            base64::read::DecoderReader::new(
                SkipWhitespace::new(&mut r.inner),
                &base64::engine::general_purpose::STANDARD,
            ),
            r.limits.clone(),
        );
        let t = Self::read_xdr_to_end(&mut dec)?;
        Ok(t)
    }

    /// Read the XDR and construct the type.
    ///
    /// Read bytes from the given read implementation, decoding the bytes as
    /// XDR, and construct the type implementing this interface from those
    /// bytes.
    ///
    /// Just enough bytes are read from the read implementation to construct the
    /// type. Any residual bytes remain in the read implementation.
    ///
    /// All implementations should continue if the read implementation returns
    /// [`ErrorKind::Interrupted`](std::io::ErrorKind::Interrupted).
    ///
    /// Use [`ReadXdR: Read_xdr_into_to_end`] when the intent is for all bytes
    /// in the read implementation to be consumed by the read.
    #[cfg(feature = "std")]
    fn read_xdr_into<R: Read>(&mut self, r: &mut Limited<R>) -> Result<(), Error> {
        *self = Self::read_xdr(r)?;
        Ok(())
    }

    /// Read the XDR into the existing value, and consider it an error if the
    /// read does not completely consume the read implementation.
    ///
    /// Read bytes from the given read implementation, decoding the bytes as
    /// XDR, and construct the type implementing this interface from those
    /// bytes.
    ///
    /// Just enough bytes are read from the read implementation to construct the
    /// type, and then confirm that no further bytes remain. To confirm no
    /// further bytes remain additional bytes are attempted to be read from the
    /// read implementation. If it is possible to read any residual bytes from
    /// the read implementation an error is returned. The read implementation
    /// may not be exhaustively read if there are residual bytes, and it is
    /// considered undefined how many residual bytes or how much of the residual
    /// buffer are consumed in this case.
    ///
    /// All implementations should continue if the read implementation returns
    /// [`ErrorKind::Interrupted`](std::io::ErrorKind::Interrupted).
    #[cfg(feature = "std")]
    fn read_xdr_into_to_end<R: Read>(&mut self, r: &mut Limited<R>) -> Result<(), Error> {
        Self::read_xdr_into(self, r)?;
        // Check that any further reads, such as this read of one byte, read no
        // data, indicating EOF. If a byte is read the data is invalid.
        if r.read(&mut [0u8; 1])? == 0 {
            Ok(())
        } else {
            Err(Error::Invalid)
        }
    }

    /// Create an iterator that reads the read implementation as a stream of
    /// values that are read into the implementing type.
    ///
    /// Read bytes from the given read implementation, decoding the bytes as
    /// XDR, and construct the type implementing this interface from those
    /// bytes.
    ///
    /// Just enough bytes are read from the read implementation to construct the
    /// type, and then confirm that no further bytes remain. To confirm no
    /// further bytes remain additional bytes are attempted to be read from the
    /// read implementation. If it is possible to read any residual bytes from
    /// the read implementation an error is returned. The read implementation
    /// may not be exhaustively read if there are residual bytes, and it is
    /// considered undefined how many residual bytes or how much of the residual
    /// buffer are consumed in this case.
    ///
    /// All implementations should continue if the read implementation returns
    /// [`ErrorKind::Interrupted`](std::io::ErrorKind::Interrupted).
    #[cfg(feature = "std")]
    fn read_xdr_iter<R: Read>(r: &mut Limited<R>) -> ReadXdrIter<&mut R, Self> {
        ReadXdrIter::new(&mut r.inner, r.limits.clone())
    }

    /// Create an iterator that reads the read implementation as a stream of
    /// values that are read into the implementing type.
    #[cfg(feature = "base64")]
    fn read_xdr_base64_iter<R: Read>(
        r: &mut Limited<R>,
    ) -> ReadXdrIter<
        base64::read::DecoderReader<
            '_,
            base64::engine::general_purpose::GeneralPurpose,
            SkipWhitespace<&mut R>,
        >,
        Self,
    > {
        let dec = base64::read::DecoderReader::new(
            SkipWhitespace::new(&mut r.inner),
            &base64::engine::general_purpose::STANDARD,
        );
        ReadXdrIter::new(dec, r.limits.clone())
    }

    /// Construct the type from the XDR bytes.
    ///
    /// An error is returned if the bytes are not completely consumed by the
    /// deserialization.
    #[cfg(feature = "std")]
    fn from_xdr(bytes: impl AsRef<[u8]>, limits: Limits) -> Result<Self, Error> {
        let mut cursor = Limited::new(Cursor::new(bytes.as_ref()), limits);
        let t = Self::read_xdr_to_end(&mut cursor)?;
        Ok(t)
    }

    /// Construct the type from the XDR bytes base64 encoded.
    ///
    /// An error is returned if the bytes are not completely consumed by the
    /// deserialization.
    #[cfg(feature = "base64")]
    fn from_xdr_base64(b64: impl AsRef<[u8]>, limits: Limits) -> Result<Self, Error> {
        let b64_reader = Cursor::new(b64);
        let mut dec = Limited::new(
            base64::read::DecoderReader::new(
                SkipWhitespace::new(b64_reader),
                &base64::engine::general_purpose::STANDARD,
            ),
            limits,
        );
        let t = Self::read_xdr_to_end(&mut dec)?;
        Ok(t)
    }
}

pub trait WriteXdr {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error>;

    #[cfg(feature = "std")]
    fn to_xdr(&self, limits: Limits) -> Result<Vec<u8>, Error> {
        let mut cursor = Limited::new(Cursor::new(vec![]), limits);
        self.write_xdr(&mut cursor)?;
        let bytes = cursor.inner.into_inner();
        Ok(bytes)
    }

    #[cfg(feature = "base64")]
    fn to_xdr_base64(&self, limits: Limits) -> Result<String, Error> {
        let mut enc = Limited::new(
            base64::write::EncoderStringWriter::new(&base64::engine::general_purpose::STANDARD),
            limits,
        );
        self.write_xdr(&mut enc)?;
        let b64 = enc.inner.into_inner();
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
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        let mut b = [0u8; 4];
        r.with_limited_depth(|r| {
            r.consume_len(b.len())?;
            r.read_exact(&mut b)?;
            Ok(i32::from_be_bytes(b))
        })
    }
}

impl WriteXdr for i32 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        let b: [u8; 4] = self.to_be_bytes();
        w.with_limited_depth(|w| {
            w.consume_len(b.len())?;
            Ok(w.write_all(&b)?)
        })
    }
}

impl ReadXdr for u32 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        let mut b = [0u8; 4];
        r.with_limited_depth(|r| {
            r.consume_len(b.len())?;
            r.read_exact(&mut b)?;
            Ok(u32::from_be_bytes(b))
        })
    }
}

impl WriteXdr for u32 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        let b: [u8; 4] = self.to_be_bytes();
        w.with_limited_depth(|w| {
            w.consume_len(b.len())?;
            Ok(w.write_all(&b)?)
        })
    }
}

impl ReadXdr for i64 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        let mut b = [0u8; 8];
        r.with_limited_depth(|r| {
            r.consume_len(b.len())?;
            r.read_exact(&mut b)?;
            Ok(i64::from_be_bytes(b))
        })
    }
}

impl WriteXdr for i64 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        let b: [u8; 8] = self.to_be_bytes();
        w.with_limited_depth(|w| {
            w.consume_len(b.len())?;
            Ok(w.write_all(&b)?)
        })
    }
}

impl ReadXdr for u64 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        let mut b = [0u8; 8];
        r.with_limited_depth(|r| {
            r.consume_len(b.len())?;
            r.read_exact(&mut b)?;
            Ok(u64::from_be_bytes(b))
        })
    }
}

impl WriteXdr for u64 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        let b: [u8; 8] = self.to_be_bytes();
        w.with_limited_depth(|w| {
            w.consume_len(b.len())?;
            Ok(w.write_all(&b)?)
        })
    }
}

impl ReadXdr for f32 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(_r: &mut Limited<R>) -> Result<Self, Error> {
        todo!()
    }
}

impl WriteXdr for f32 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, _w: &mut Limited<W>) -> Result<(), Error> {
        todo!()
    }
}

impl ReadXdr for f64 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(_r: &mut Limited<R>) -> Result<Self, Error> {
        todo!()
    }
}

impl WriteXdr for f64 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, _w: &mut Limited<W>) -> Result<(), Error> {
        todo!()
    }
}

impl ReadXdr for bool {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let i = u32::read_xdr(r)?;
            let b = i == 1;
            Ok(b)
        })
    }
}

impl WriteXdr for bool {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            let i = u32::from(*self); // true = 1, false = 0
            i.write_xdr(w)
        })
    }
}

impl<T: ReadXdr> ReadXdr for Option<T> {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let i = u32::read_xdr(r)?;
            match i {
                0 => Ok(None),
                1 => {
                    let t = T::read_xdr(r)?;
                    Ok(Some(t))
                }
                _ => Err(Error::Invalid),
            }
        })
    }
}

impl<T: WriteXdr> WriteXdr for Option<T> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            if let Some(t) = self {
                1u32.write_xdr(w)?;
                t.write_xdr(w)?;
            } else {
                0u32.write_xdr(w)?;
            }
            Ok(())
        })
    }
}

impl<T: ReadXdr> ReadXdr for Box<T> {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| Ok(Box::new(T::read_xdr(r)?)))
    }
}

impl<T: WriteXdr> WriteXdr for Box<T> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| T::write_xdr(self, w))
    }
}

impl ReadXdr for () {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(_r: &mut Limited<R>) -> Result<Self, Error> {
        Ok(())
    }
}

impl WriteXdr for () {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, _w: &mut Limited<W>) -> Result<(), Error> {
        Ok(())
    }
}

impl<const N: usize> ReadXdr for [u8; N] {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            r.consume_len(N)?;
            let padding = pad_len(N);
            r.consume_len(padding)?;
            let mut arr = [0u8; N];
            r.read_exact(&mut arr)?;
            let pad = &mut [0u8; 3][..padding];
            r.read_exact(pad)?;
            if pad.iter().any(|b| *b != 0) {
                return Err(Error::NonZeroPadding);
            }
            Ok(arr)
        })
    }
}

impl<const N: usize> WriteXdr for [u8; N] {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            w.consume_len(N)?;
            let padding = pad_len(N);
            w.consume_len(padding)?;
            w.write_all(self)?;
            w.write_all(&[0u8; 3][..padding])?;
            Ok(())
        })
    }
}

impl<T: ReadXdr, const N: usize> ReadXdr for [T; N] {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let mut vec = Vec::with_capacity(N);
            for _ in 0..N {
                let t = T::read_xdr(r)?;
                vec.push(t);
            }
            let arr: [T; N] = vec.try_into().unwrap_or_else(|_: Vec<T>| unreachable!());
            Ok(arr)
        })
    }
}

impl<T: WriteXdr, const N: usize> WriteXdr for [T; N] {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            for t in self {
                t.write_xdr(w)?;
            }
            Ok(())
        })
    }
}

// VecM ------------------------------------------------------------------------

#[cfg(feature = "alloc")]
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    feature = "serde",
    serde_with::serde_as,
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
pub struct VecM<T, const MAX: u32 = { u32::MAX }>(Vec<T>);

#[cfg(not(feature = "alloc"))]
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
pub struct VecM<T, const MAX: u32 = { u32::MAX }>(Vec<T>)
where
    T: 'static;

impl<T, const MAX: u32> Deref for VecM<T, MAX> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, const MAX: u32> Default for VecM<T, MAX> {
    fn default() -> Self {
        Self(Vec::default())
    }
}

#[cfg(feature = "schemars")]
impl<T: schemars::JsonSchema, const MAX: u32> schemars::JsonSchema for VecM<T, MAX> {
    fn schema_name() -> String {
        format!("VecM<{}, {}>", T::schema_name(), MAX)
    }

    fn is_referenceable() -> bool {
        false
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        let schema = Vec::<T>::json_schema(gen);
        if let schemars::schema::Schema::Object(mut schema) = schema {
            if let Some(array) = schema.array.clone() {
                schema.array = Some(Box::new(schemars::schema::ArrayValidation {
                    max_items: Some(MAX),
                    ..*array
                }));
            }
            schema.into()
        } else {
            schema
        }
    }
}

#[cfg(feature = "schemars")]
impl<T, TA, const MAX: u32> serde_with::schemars_0_8::JsonSchemaAs<VecM<T, MAX>> for VecM<TA, MAX>
where
    TA: serde_with::schemars_0_8::JsonSchemaAs<T>,
{
    fn schema_name() -> String {
        <VecM<serde_with::Schema<T, TA>, MAX> as schemars::JsonSchema>::schema_name()
    }

    fn schema_id() -> Cow<'static, str> {
        <VecM<serde_with::Schema<T, TA>, MAX> as schemars::JsonSchema>::schema_id()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        <VecM<serde_with::Schema<T, TA>, MAX> as schemars::JsonSchema>::json_schema(gen)
    }

    fn is_referenceable() -> bool {
        <VecM<serde_with::Schema<T, TA>, MAX> as schemars::JsonSchema>::is_referenceable()
    }
}

#[cfg(feature = "serde")]
impl<T, U, const MAX: u32> serde_with::SerializeAs<VecM<T, MAX>> for VecM<U, MAX>
where
    U: serde_with::SerializeAs<T>,
{
    fn serialize_as<S>(source: &VecM<T, MAX>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_seq(
            source
                .iter()
                .map(|item| serde_with::ser::SerializeAsWrap::<T, U>::new(item)),
        )
    }
}

#[cfg(feature = "serde")]
impl<'de, T, U, const MAX: u32> serde_with::DeserializeAs<'de, VecM<T, MAX>> for VecM<U, MAX>
where
    U: serde_with::DeserializeAs<'de, T>,
{
    fn deserialize_as<D>(deserializer: D) -> Result<VecM<T, MAX>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let vec = <Vec<U> as serde_with::DeserializeAs<Vec<T>>>::deserialize_as(deserializer)?;
        vec.try_into().map_err(serde::de::Error::custom)
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

#[cfg(feature = "alloc")]
impl<T, const MAX: u32> VecM<T, MAX> {
    pub fn iter_mut(&mut self) -> core::slice::IterMut<'_, T> {
        self.0.iter_mut()
    }
}

#[cfg(feature = "alloc")]
impl<'a, T, const MAX: u32> core::iter::IntoIterator for &'a mut VecM<T, MAX> {
    type Item = &'a mut T;
    type IntoIter = core::slice::IterMut<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

#[cfg(feature = "alloc")]
impl<T, const MAX: u32> core::iter::IntoIterator for VecM<T, MAX> {
    type Item = T;
    type IntoIter = <Vec<T> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a, T, const MAX: u32> core::iter::IntoIterator for &'a VecM<T, MAX> {
    type Item = &'a T;
    type IntoIter = core::slice::Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
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
    pub fn to_string(&self) -> Result<String, Error> {
        self.try_into()
    }

    #[cfg(feature = "alloc")]
    pub fn into_string(self) -> Result<String, Error> {
        self.try_into()
    }

    #[cfg(feature = "alloc")]
    #[must_use]
    pub fn to_string_lossy(&self) -> String {
        String::from_utf8_lossy(&self.0).into_owned()
    }

    #[cfg(feature = "alloc")]
    #[must_use]
    pub fn into_string_lossy(self) -> String {
        String::from_utf8_lossy(&self.0).into_owned()
    }
}

impl<T: Clone> VecM<T, 1> {
    #[must_use]
    pub fn to_option(&self) -> Option<T> {
        if self.len() > 0 {
            Some(self.0[0].clone())
        } else {
            None
        }
    }
}

#[cfg(not(feature = "alloc"))]
impl<T: Clone> From<VecM<T, 1>> for Option<T> {
    #[must_use]
    fn from(v: VecM<T, 1>) -> Self {
        v.to_option()
    }
}

#[cfg(feature = "alloc")]
impl<T> VecM<T, 1> {
    #[must_use]
    pub fn into_option(mut self) -> Option<T> {
        self.0.drain(..).next()
    }
}

#[cfg(feature = "alloc")]
impl<T> From<VecM<T, 1>> for Option<T> {
    #[must_use]
    fn from(v: VecM<T, 1>) -> Self {
        v.into_option()
    }
}

impl<T, const MAX: u32> TryFrom<Vec<T>> for VecM<T, MAX> {
    type Error = Error;

    fn try_from(v: Vec<T>) -> Result<Self, Error> {
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
impl<T: Clone, const MAX: u32> TryFrom<&Vec<T>> for VecM<T, MAX> {
    type Error = Error;

    fn try_from(v: &Vec<T>) -> Result<Self, Error> {
        v.as_slice().try_into()
    }
}

#[cfg(feature = "alloc")]
impl<T: Clone, const MAX: u32> TryFrom<&[T]> for VecM<T, MAX> {
    type Error = Error;

    fn try_from(v: &[T]) -> Result<Self, Error> {
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

    fn try_from(v: [T; N]) -> Result<Self, Error> {
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

    fn try_from(v: &[T; N]) -> Result<Self, Error> {
        let len: u32 = v.len().try_into().map_err(|_| Error::LengthExceedsMax)?;
        if len <= MAX {
            Ok(VecM(v.to_vec()))
        } else {
            Err(Error::LengthExceedsMax)
        }
    }
}

#[cfg(not(feature = "alloc"))]
impl<T: Clone, const N: usize, const MAX: u32> TryFrom<&'static [T; N]> for VecM<T, MAX> {
    type Error = Error;

    fn try_from(v: &'static [T; N]) -> Result<Self, Error> {
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

    fn try_from(v: &String) -> Result<Self, Error> {
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

    fn try_from(v: String) -> Result<Self, Error> {
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

    fn try_from(v: VecM<u8, MAX>) -> Result<Self, Error> {
        Ok(String::from_utf8(v.0)?)
    }
}

#[cfg(feature = "alloc")]
impl<const MAX: u32> TryFrom<&VecM<u8, MAX>> for String {
    type Error = Error;

    fn try_from(v: &VecM<u8, MAX>) -> Result<Self, Error> {
        Ok(core::str::from_utf8(v.as_ref())?.to_owned())
    }
}

#[cfg(feature = "alloc")]
impl<const MAX: u32> TryFrom<&str> for VecM<u8, MAX> {
    type Error = Error;

    fn try_from(v: &str) -> Result<Self, Error> {
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

    fn try_from(v: &'static str) -> Result<Self, Error> {
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

    fn try_from(v: &'a VecM<u8, MAX>) -> Result<Self, Error> {
        Ok(core::str::from_utf8(v.as_ref())?)
    }
}

impl<const MAX: u32> ReadXdr for VecM<u8, MAX> {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let len: u32 = u32::read_xdr(r)?;
            if len > MAX {
                return Err(Error::LengthExceedsMax);
            }

            r.consume_len(len as usize)?;
            let padding = pad_len(len as usize);
            r.consume_len(padding)?;

            let mut vec = vec![0u8; len as usize];
            r.read_exact(&mut vec)?;

            let pad = &mut [0u8; 3][..padding];
            r.read_exact(pad)?;
            if pad.iter().any(|b| *b != 0) {
                return Err(Error::NonZeroPadding);
            }

            Ok(VecM(vec))
        })
    }
}

impl<const MAX: u32> WriteXdr for VecM<u8, MAX> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            let len: u32 = self.len().try_into().map_err(|_| Error::LengthExceedsMax)?;
            len.write_xdr(w)?;

            w.consume_len(self.len())?;
            let padding = pad_len(self.len());
            w.consume_len(padding)?;

            w.write_all(&self.0)?;

            w.write_all(&[0u8; 3][..padding])?;

            Ok(())
        })
    }
}

impl<T: ReadXdr, const MAX: u32> ReadXdr for VecM<T, MAX> {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let len = u32::read_xdr(r)?;
            if len > MAX {
                return Err(Error::LengthExceedsMax);
            }

            let mut vec = Vec::new();
            for _ in 0..len {
                let t = T::read_xdr(r)?;
                vec.push(t);
            }

            Ok(VecM(vec))
        })
    }
}

impl<T: WriteXdr, const MAX: u32> WriteXdr for VecM<T, MAX> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            let len: u32 = self.len().try_into().map_err(|_| Error::LengthExceedsMax)?;
            len.write_xdr(w)?;

            for t in &self.0 {
                t.write_xdr(w)?;
            }

            Ok(())
        })
    }
}

// BytesM ------------------------------------------------------------------------

#[cfg(feature = "alloc")]
#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    feature = "serde",
    derive(serde_with::SerializeDisplay, serde_with::DeserializeFromStr)
)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
pub struct BytesM<const MAX: u32 = { u32::MAX }>(Vec<u8>);

#[cfg(not(feature = "alloc"))]
#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
pub struct BytesM<const MAX: u32 = { u32::MAX }>(Vec<u8>);

impl<const MAX: u32> core::fmt::Display for BytesM<MAX> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        #[cfg(feature = "alloc")]
        let v = &self.0;
        #[cfg(not(feature = "alloc"))]
        let v = self.0;
        for b in v {
            write!(f, "{b:02x}")?;
        }
        Ok(())
    }
}

impl<const MAX: u32> core::fmt::Debug for BytesM<MAX> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        #[cfg(feature = "alloc")]
        let v = &self.0;
        #[cfg(not(feature = "alloc"))]
        let v = self.0;
        write!(f, "BytesM(")?;
        for b in v {
            write!(f, "{b:02x}")?;
        }
        write!(f, ")")?;
        Ok(())
    }
}

#[cfg(feature = "alloc")]
impl<const MAX: u32> core::str::FromStr for BytesM<MAX> {
    type Err = Error;
    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        hex::decode(s).map_err(|_| Error::InvalidHex)?.try_into()
    }
}

impl<const MAX: u32> Deref for BytesM<MAX> {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(feature = "schemars")]
impl<const MAX: u32> schemars::JsonSchema for BytesM<MAX> {
    fn schema_name() -> String {
        format!("BytesM<{MAX}>")
    }

    fn is_referenceable() -> bool {
        false
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        let schema = String::json_schema(gen);
        if let schemars::schema::Schema::Object(mut schema) = schema {
            schema.extensions.insert(
                "contentEncoding".to_owned(),
                serde_json::Value::String("hex".to_string()),
            );
            schema.extensions.insert(
                "contentMediaType".to_owned(),
                serde_json::Value::String("application/binary".to_string()),
            );
            let string = *schema.string.unwrap_or_default().clone();
            schema.string = Some(Box::new(schemars::schema::StringValidation {
                max_length: MAX.checked_mul(2).map(Some).unwrap_or_default(),
                min_length: None,
                ..string
            }));
            schema.into()
        } else {
            schema
        }
    }
}

impl<const MAX: u32> Default for BytesM<MAX> {
    fn default() -> Self {
        Self(Vec::default())
    }
}

impl<const MAX: u32> BytesM<MAX> {
    pub const MAX_LEN: usize = { MAX as usize };

    #[must_use]
    #[allow(clippy::unused_self)]
    pub fn max_len(&self) -> usize {
        Self::MAX_LEN
    }

    #[must_use]
    pub fn as_vec(&self) -> &Vec<u8> {
        self.as_ref()
    }
}

impl<const MAX: u32> BytesM<MAX> {
    #[must_use]
    #[cfg(feature = "alloc")]
    pub fn to_vec(&self) -> Vec<u8> {
        self.into()
    }

    #[must_use]
    pub fn into_vec(self) -> Vec<u8> {
        self.into()
    }
}

impl<const MAX: u32> BytesM<MAX> {
    #[cfg(feature = "alloc")]
    pub fn to_string(&self) -> Result<String, Error> {
        self.try_into()
    }

    #[cfg(feature = "alloc")]
    pub fn into_string(self) -> Result<String, Error> {
        self.try_into()
    }

    #[cfg(feature = "alloc")]
    #[must_use]
    pub fn to_string_lossy(&self) -> String {
        String::from_utf8_lossy(&self.0).into_owned()
    }

    #[cfg(feature = "alloc")]
    #[must_use]
    pub fn into_string_lossy(self) -> String {
        String::from_utf8_lossy(&self.0).into_owned()
    }
}

impl<const MAX: u32> TryFrom<Vec<u8>> for BytesM<MAX> {
    type Error = Error;

    fn try_from(v: Vec<u8>) -> Result<Self, Error> {
        let len: u32 = v.len().try_into().map_err(|_| Error::LengthExceedsMax)?;
        if len <= MAX {
            Ok(BytesM(v))
        } else {
            Err(Error::LengthExceedsMax)
        }
    }
}

impl<const MAX: u32> From<BytesM<MAX>> for Vec<u8> {
    #[must_use]
    fn from(v: BytesM<MAX>) -> Self {
        v.0
    }
}

#[cfg(feature = "alloc")]
impl<const MAX: u32> From<&BytesM<MAX>> for Vec<u8> {
    #[must_use]
    fn from(v: &BytesM<MAX>) -> Self {
        v.0.clone()
    }
}

impl<const MAX: u32> AsRef<Vec<u8>> for BytesM<MAX> {
    #[must_use]
    fn as_ref(&self) -> &Vec<u8> {
        &self.0
    }
}

#[cfg(feature = "alloc")]
impl<const MAX: u32> TryFrom<&Vec<u8>> for BytesM<MAX> {
    type Error = Error;

    fn try_from(v: &Vec<u8>) -> Result<Self, Error> {
        v.as_slice().try_into()
    }
}

#[cfg(feature = "alloc")]
impl<const MAX: u32> TryFrom<&[u8]> for BytesM<MAX> {
    type Error = Error;

    fn try_from(v: &[u8]) -> Result<Self, Error> {
        let len: u32 = v.len().try_into().map_err(|_| Error::LengthExceedsMax)?;
        if len <= MAX {
            Ok(BytesM(v.to_vec()))
        } else {
            Err(Error::LengthExceedsMax)
        }
    }
}

impl<const MAX: u32> AsRef<[u8]> for BytesM<MAX> {
    #[cfg(feature = "alloc")]
    #[must_use]
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
    #[cfg(not(feature = "alloc"))]
    #[must_use]
    fn as_ref(&self) -> &[u8] {
        self.0
    }
}

#[cfg(feature = "alloc")]
impl<const N: usize, const MAX: u32> TryFrom<[u8; N]> for BytesM<MAX> {
    type Error = Error;

    fn try_from(v: [u8; N]) -> Result<Self, Error> {
        let len: u32 = v.len().try_into().map_err(|_| Error::LengthExceedsMax)?;
        if len <= MAX {
            Ok(BytesM(v.to_vec()))
        } else {
            Err(Error::LengthExceedsMax)
        }
    }
}

#[cfg(feature = "alloc")]
impl<const N: usize, const MAX: u32> TryFrom<BytesM<MAX>> for [u8; N] {
    type Error = BytesM<MAX>;

    fn try_from(v: BytesM<MAX>) -> core::result::Result<Self, Self::Error> {
        let s: [u8; N] = v.0.try_into().map_err(BytesM::<MAX>)?;
        Ok(s)
    }
}

#[cfg(feature = "alloc")]
impl<const N: usize, const MAX: u32> TryFrom<&[u8; N]> for BytesM<MAX> {
    type Error = Error;

    fn try_from(v: &[u8; N]) -> Result<Self, Error> {
        let len: u32 = v.len().try_into().map_err(|_| Error::LengthExceedsMax)?;
        if len <= MAX {
            Ok(BytesM(v.to_vec()))
        } else {
            Err(Error::LengthExceedsMax)
        }
    }
}

#[cfg(not(feature = "alloc"))]
impl<const N: usize, const MAX: u32> TryFrom<&'static [u8; N]> for BytesM<MAX> {
    type Error = Error;

    fn try_from(v: &'static [u8; N]) -> Result<Self, Error> {
        let len: u32 = v.len().try_into().map_err(|_| Error::LengthExceedsMax)?;
        if len <= MAX {
            Ok(BytesM(v))
        } else {
            Err(Error::LengthExceedsMax)
        }
    }
}

#[cfg(feature = "alloc")]
impl<const MAX: u32> TryFrom<&String> for BytesM<MAX> {
    type Error = Error;

    fn try_from(v: &String) -> Result<Self, Error> {
        let len: u32 = v.len().try_into().map_err(|_| Error::LengthExceedsMax)?;
        if len <= MAX {
            Ok(BytesM(v.as_bytes().to_vec()))
        } else {
            Err(Error::LengthExceedsMax)
        }
    }
}

#[cfg(feature = "alloc")]
impl<const MAX: u32> TryFrom<String> for BytesM<MAX> {
    type Error = Error;

    fn try_from(v: String) -> Result<Self, Error> {
        let len: u32 = v.len().try_into().map_err(|_| Error::LengthExceedsMax)?;
        if len <= MAX {
            Ok(BytesM(v.into()))
        } else {
            Err(Error::LengthExceedsMax)
        }
    }
}

#[cfg(feature = "alloc")]
impl<const MAX: u32> TryFrom<BytesM<MAX>> for String {
    type Error = Error;

    fn try_from(v: BytesM<MAX>) -> Result<Self, Error> {
        Ok(String::from_utf8(v.0)?)
    }
}

#[cfg(feature = "alloc")]
impl<const MAX: u32> TryFrom<&BytesM<MAX>> for String {
    type Error = Error;

    fn try_from(v: &BytesM<MAX>) -> Result<Self, Error> {
        Ok(core::str::from_utf8(v.as_ref())?.to_owned())
    }
}

#[cfg(feature = "alloc")]
impl<const MAX: u32> TryFrom<&str> for BytesM<MAX> {
    type Error = Error;

    fn try_from(v: &str) -> Result<Self, Error> {
        let len: u32 = v.len().try_into().map_err(|_| Error::LengthExceedsMax)?;
        if len <= MAX {
            Ok(BytesM(v.into()))
        } else {
            Err(Error::LengthExceedsMax)
        }
    }
}

#[cfg(not(feature = "alloc"))]
impl<const MAX: u32> TryFrom<&'static str> for BytesM<MAX> {
    type Error = Error;

    fn try_from(v: &'static str) -> Result<Self, Error> {
        let len: u32 = v.len().try_into().map_err(|_| Error::LengthExceedsMax)?;
        if len <= MAX {
            Ok(BytesM(v.as_bytes()))
        } else {
            Err(Error::LengthExceedsMax)
        }
    }
}

impl<'a, const MAX: u32> TryFrom<&'a BytesM<MAX>> for &'a str {
    type Error = Error;

    fn try_from(v: &'a BytesM<MAX>) -> Result<Self, Error> {
        Ok(core::str::from_utf8(v.as_ref())?)
    }
}

impl<const MAX: u32> ReadXdr for BytesM<MAX> {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let len: u32 = u32::read_xdr(r)?;
            if len > MAX {
                return Err(Error::LengthExceedsMax);
            }

            r.consume_len(len as usize)?;
            let padding = pad_len(len as usize);
            r.consume_len(padding)?;

            let mut vec = vec![0u8; len as usize];
            r.read_exact(&mut vec)?;

            let pad = &mut [0u8; 3][..padding];
            r.read_exact(pad)?;
            if pad.iter().any(|b| *b != 0) {
                return Err(Error::NonZeroPadding);
            }

            Ok(BytesM(vec))
        })
    }
}

impl<const MAX: u32> WriteXdr for BytesM<MAX> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            let len: u32 = self.len().try_into().map_err(|_| Error::LengthExceedsMax)?;
            len.write_xdr(w)?;

            w.consume_len(self.len())?;
            let padding = pad_len(self.len());
            w.consume_len(padding)?;

            w.write_all(&self.0)?;

            w.write_all(&[0u8; 3][..pad_len(len as usize)])?;

            Ok(())
        })
    }
}

// StringM ------------------------------------------------------------------------

/// A string type that contains arbitrary bytes.
///
/// Convertible, fallibly, to/from a Rust UTF-8 String using
/// [`TryFrom`]/[`TryInto`]/[`StringM::to_utf8_string`].
///
/// Convertible, lossyly, to a Rust UTF-8 String using
/// [`StringM::to_utf8_string_lossy`].
///
/// Convertible to/from escaped printable-ASCII using
/// [`Display`]/[`ToString`]/[`FromStr`].

#[cfg(feature = "alloc")]
#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    feature = "serde",
    derive(serde_with::SerializeDisplay, serde_with::DeserializeFromStr)
)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
pub struct StringM<const MAX: u32 = { u32::MAX }>(Vec<u8>);

#[cfg(not(feature = "alloc"))]
#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
pub struct StringM<const MAX: u32 = { u32::MAX }>(Vec<u8>);

impl<const MAX: u32> core::fmt::Display for StringM<MAX> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        #[cfg(feature = "alloc")]
        let v = &self.0;
        #[cfg(not(feature = "alloc"))]
        let v = self.0;
        for b in escape_bytes::Escape::new(v) {
            write!(f, "{}", b as char)?;
        }
        Ok(())
    }
}

impl<const MAX: u32> core::fmt::Debug for StringM<MAX> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        #[cfg(feature = "alloc")]
        let v = &self.0;
        #[cfg(not(feature = "alloc"))]
        let v = self.0;
        write!(f, "StringM(")?;
        for b in escape_bytes::Escape::new(v) {
            write!(f, "{}", b as char)?;
        }
        write!(f, ")")?;
        Ok(())
    }
}

#[cfg(feature = "alloc")]
impl<const MAX: u32> core::str::FromStr for StringM<MAX> {
    type Err = Error;
    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        let b = escape_bytes::unescape(s.as_bytes()).map_err(|_| Error::Invalid)?;
        b.try_into()
    }
}

impl<const MAX: u32> Deref for StringM<MAX> {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const MAX: u32> Default for StringM<MAX> {
    fn default() -> Self {
        Self(Vec::default())
    }
}

#[cfg(feature = "schemars")]
impl<const MAX: u32> schemars::JsonSchema for StringM<MAX> {
    fn schema_name() -> String {
        format!("StringM<{MAX}>")
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        let schema = String::json_schema(gen);
        if let schemars::schema::Schema::Object(mut schema) = schema {
            let string = *schema.string.unwrap_or_default().clone();
            schema.string = Some(Box::new(schemars::schema::StringValidation {
                max_length: Some(MAX),
                ..string
            }));
            schema.into()
        } else {
            schema
        }
    }
}

impl<const MAX: u32> StringM<MAX> {
    pub const MAX_LEN: usize = { MAX as usize };

    #[must_use]
    #[allow(clippy::unused_self)]
    pub fn max_len(&self) -> usize {
        Self::MAX_LEN
    }

    #[must_use]
    pub fn as_vec(&self) -> &Vec<u8> {
        self.as_ref()
    }
}

impl<const MAX: u32> StringM<MAX> {
    #[must_use]
    #[cfg(feature = "alloc")]
    pub fn to_vec(&self) -> Vec<u8> {
        self.into()
    }

    #[must_use]
    pub fn into_vec(self) -> Vec<u8> {
        self.into()
    }
}

impl<const MAX: u32> StringM<MAX> {
    #[cfg(feature = "alloc")]
    pub fn to_utf8_string(&self) -> Result<String, Error> {
        self.try_into()
    }

    #[cfg(feature = "alloc")]
    pub fn into_utf8_string(self) -> Result<String, Error> {
        self.try_into()
    }

    #[cfg(feature = "alloc")]
    #[must_use]
    pub fn to_utf8_string_lossy(&self) -> String {
        String::from_utf8_lossy(&self.0).into_owned()
    }

    #[cfg(feature = "alloc")]
    #[must_use]
    pub fn into_utf8_string_lossy(self) -> String {
        String::from_utf8_lossy(&self.0).into_owned()
    }
}

impl<const MAX: u32> TryFrom<Vec<u8>> for StringM<MAX> {
    type Error = Error;

    fn try_from(v: Vec<u8>) -> Result<Self, Error> {
        let len: u32 = v.len().try_into().map_err(|_| Error::LengthExceedsMax)?;
        if len <= MAX {
            Ok(StringM(v))
        } else {
            Err(Error::LengthExceedsMax)
        }
    }
}

impl<const MAX: u32> From<StringM<MAX>> for Vec<u8> {
    #[must_use]
    fn from(v: StringM<MAX>) -> Self {
        v.0
    }
}

#[cfg(feature = "alloc")]
impl<const MAX: u32> From<&StringM<MAX>> for Vec<u8> {
    #[must_use]
    fn from(v: &StringM<MAX>) -> Self {
        v.0.clone()
    }
}

impl<const MAX: u32> AsRef<Vec<u8>> for StringM<MAX> {
    #[must_use]
    fn as_ref(&self) -> &Vec<u8> {
        &self.0
    }
}

#[cfg(feature = "alloc")]
impl<const MAX: u32> TryFrom<&Vec<u8>> for StringM<MAX> {
    type Error = Error;

    fn try_from(v: &Vec<u8>) -> Result<Self, Error> {
        v.as_slice().try_into()
    }
}

#[cfg(feature = "alloc")]
impl<const MAX: u32> TryFrom<&[u8]> for StringM<MAX> {
    type Error = Error;

    fn try_from(v: &[u8]) -> Result<Self, Error> {
        let len: u32 = v.len().try_into().map_err(|_| Error::LengthExceedsMax)?;
        if len <= MAX {
            Ok(StringM(v.to_vec()))
        } else {
            Err(Error::LengthExceedsMax)
        }
    }
}

impl<const MAX: u32> AsRef<[u8]> for StringM<MAX> {
    #[cfg(feature = "alloc")]
    #[must_use]
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
    #[cfg(not(feature = "alloc"))]
    #[must_use]
    fn as_ref(&self) -> &[u8] {
        self.0
    }
}

#[cfg(feature = "alloc")]
impl<const N: usize, const MAX: u32> TryFrom<[u8; N]> for StringM<MAX> {
    type Error = Error;

    fn try_from(v: [u8; N]) -> Result<Self, Error> {
        let len: u32 = v.len().try_into().map_err(|_| Error::LengthExceedsMax)?;
        if len <= MAX {
            Ok(StringM(v.to_vec()))
        } else {
            Err(Error::LengthExceedsMax)
        }
    }
}

#[cfg(feature = "alloc")]
impl<const N: usize, const MAX: u32> TryFrom<StringM<MAX>> for [u8; N] {
    type Error = StringM<MAX>;

    fn try_from(v: StringM<MAX>) -> core::result::Result<Self, Self::Error> {
        let s: [u8; N] = v.0.try_into().map_err(StringM::<MAX>)?;
        Ok(s)
    }
}

#[cfg(feature = "alloc")]
impl<const N: usize, const MAX: u32> TryFrom<&[u8; N]> for StringM<MAX> {
    type Error = Error;

    fn try_from(v: &[u8; N]) -> Result<Self, Error> {
        let len: u32 = v.len().try_into().map_err(|_| Error::LengthExceedsMax)?;
        if len <= MAX {
            Ok(StringM(v.to_vec()))
        } else {
            Err(Error::LengthExceedsMax)
        }
    }
}

#[cfg(not(feature = "alloc"))]
impl<const N: usize, const MAX: u32> TryFrom<&'static [u8; N]> for StringM<MAX> {
    type Error = Error;

    fn try_from(v: &'static [u8; N]) -> Result<Self, Error> {
        let len: u32 = v.len().try_into().map_err(|_| Error::LengthExceedsMax)?;
        if len <= MAX {
            Ok(StringM(v))
        } else {
            Err(Error::LengthExceedsMax)
        }
    }
}

#[cfg(feature = "alloc")]
impl<const MAX: u32> TryFrom<&String> for StringM<MAX> {
    type Error = Error;

    fn try_from(v: &String) -> Result<Self, Error> {
        let len: u32 = v.len().try_into().map_err(|_| Error::LengthExceedsMax)?;
        if len <= MAX {
            Ok(StringM(v.as_bytes().to_vec()))
        } else {
            Err(Error::LengthExceedsMax)
        }
    }
}

#[cfg(feature = "alloc")]
impl<const MAX: u32> TryFrom<String> for StringM<MAX> {
    type Error = Error;

    fn try_from(v: String) -> Result<Self, Error> {
        let len: u32 = v.len().try_into().map_err(|_| Error::LengthExceedsMax)?;
        if len <= MAX {
            Ok(StringM(v.into()))
        } else {
            Err(Error::LengthExceedsMax)
        }
    }
}

#[cfg(feature = "alloc")]
impl<const MAX: u32> TryFrom<StringM<MAX>> for String {
    type Error = Error;

    fn try_from(v: StringM<MAX>) -> Result<Self, Error> {
        Ok(String::from_utf8(v.0)?)
    }
}

#[cfg(feature = "alloc")]
impl<const MAX: u32> TryFrom<&StringM<MAX>> for String {
    type Error = Error;

    fn try_from(v: &StringM<MAX>) -> Result<Self, Error> {
        Ok(core::str::from_utf8(v.as_ref())?.to_owned())
    }
}

#[cfg(feature = "alloc")]
impl<const MAX: u32> TryFrom<&str> for StringM<MAX> {
    type Error = Error;

    fn try_from(v: &str) -> Result<Self, Error> {
        let len: u32 = v.len().try_into().map_err(|_| Error::LengthExceedsMax)?;
        if len <= MAX {
            Ok(StringM(v.into()))
        } else {
            Err(Error::LengthExceedsMax)
        }
    }
}

#[cfg(not(feature = "alloc"))]
impl<const MAX: u32> TryFrom<&'static str> for StringM<MAX> {
    type Error = Error;

    fn try_from(v: &'static str) -> Result<Self, Error> {
        let len: u32 = v.len().try_into().map_err(|_| Error::LengthExceedsMax)?;
        if len <= MAX {
            Ok(StringM(v.as_bytes()))
        } else {
            Err(Error::LengthExceedsMax)
        }
    }
}

impl<'a, const MAX: u32> TryFrom<&'a StringM<MAX>> for &'a str {
    type Error = Error;

    fn try_from(v: &'a StringM<MAX>) -> Result<Self, Error> {
        Ok(core::str::from_utf8(v.as_ref())?)
    }
}

impl<const MAX: u32> ReadXdr for StringM<MAX> {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let len: u32 = u32::read_xdr(r)?;
            if len > MAX {
                return Err(Error::LengthExceedsMax);
            }

            r.consume_len(len as usize)?;
            let padding = pad_len(len as usize);
            r.consume_len(padding)?;

            let mut vec = vec![0u8; len as usize];
            r.read_exact(&mut vec)?;

            let pad = &mut [0u8; 3][..padding];
            r.read_exact(pad)?;
            if pad.iter().any(|b| *b != 0) {
                return Err(Error::NonZeroPadding);
            }

            Ok(StringM(vec))
        })
    }
}

impl<const MAX: u32> WriteXdr for StringM<MAX> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            let len: u32 = self.len().try_into().map_err(|_| Error::LengthExceedsMax)?;
            len.write_xdr(w)?;

            w.consume_len(self.len())?;
            let padding = pad_len(self.len());
            w.consume_len(padding)?;

            w.write_all(&self.0)?;

            w.write_all(&[0u8; 3][..padding])?;

            Ok(())
        })
    }
}

// Frame ------------------------------------------------------------------------

/// Frame wraps an XDR object with the framing defined by the Record Marking
/// Standard in [RFC 5531 Section 11].
///
/// Each frame begins with a 4-byte big-endian header where:
///  - Bit 31 (high bit) is the last-fragment flag (`1` = last fragment).
///  - Bits 0-30 contain the byte length of the fragment data that follows.
///
/// A record is composed of one or more fragments. In Stellar's usage each
/// record contains exactly one XDR object encoded as a single fragment with
/// the last-fragment bit set.
///
/// [RFC 5531 Section 11]: https://www.rfc-editor.org/rfc/rfc5531#section-11
#[derive(Default, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
pub struct Frame<T>(pub T)
where
    T: ReadXdr;

#[cfg(feature = "schemars")]
impl<T: schemars::JsonSchema + ReadXdr> schemars::JsonSchema for Frame<T> {
    fn schema_name() -> String {
        format!("Frame<{}>", T::schema_name())
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        T::json_schema(gen)
    }
}

impl<T> ReadXdr for Frame<T>
where
    T: ReadXdr,
{
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        // Read the 4-byte fragment header defined by the Record Marking
        // Standard in RFC 5531 Section 11
        // (https://www.rfc-editor.org/rfc/rfc5531#section-11).
        //  - Bit 31 (high bit) is the last-fragment flag: 1 if this is the
        //    last fragment of the record, 0 if more fragments follow.
        //  - Bits 0-30 contain the byte length of the fragment data that
        //    follows the header.
        let header = u32::read_xdr(r)?;
        // TODO: Use the length and cap the length we'll read from `r`.
        let last_record = header >> 31 == 1;
        if last_record {
            // Read the record in the frame.
            Ok(Self(T::read_xdr(r)?))
        } else {
            // TODO: Support reading those additional frames for the same
            // record.
            Err(Error::Unsupported)
        }
    }
}

/// Forwards read operations to the wrapped object, skipping over any
/// whitespace.
#[cfg(feature = "std")]
pub struct SkipWhitespace<R: Read> {
    pub inner: R,
}

#[cfg(feature = "std")]
impl<R: Read> SkipWhitespace<R> {
    pub fn new(inner: R) -> Self {
        SkipWhitespace { inner }
    }
}

#[cfg(feature = "std")]
impl<R: Read> Read for SkipWhitespace<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let n = self.inner.read(buf)?;

        let mut written = 0;
        for read in 0..n {
            if !buf[read].is_ascii_whitespace() {
                buf[written] = buf[read];
                written += 1;
            }
        }

        Ok(written)
    }
}

#[cfg(all(test, feature = "std"))]
mod test_skip_whitespace {
    use super::*;

    #[test]
    fn test() {
        struct Test {
            input: &'static [u8],
            output: &'static [u8],
        }
        let tests = [
            Test {
                input: b"",
                output: b"",
            },
            Test {
                input: b" \n\t\r",
                output: b"",
            },
            Test {
                input: b"a c",
                output: b"ac",
            },
            Test {
                input: b"ab cd",
                output: b"abcd",
            },
            Test {
                input: b" ab \n cd ",
                output: b"abcd",
            },
        ];
        for (i, t) in tests.iter().enumerate() {
            let mut skip = SkipWhitespace::new(t.input);
            let mut output = Vec::new();
            skip.read_to_end(&mut output).unwrap();
            assert_eq!(output, t.output, "#{i}");
        }
    }
}

// NumberOrString ---------------------------------------------------------------

/// NumberOrString is a serde_as serializer/deserializer.
///
/// It deserializers any integer that fits into a 64-bit value into an i64 or u64 field from either
/// a JSON Number or JSON String value.
///
/// It serializes always to a string.
///
/// It has a JsonSchema implementation that only advertises that the allowed format is a String.
/// This is because the type is intended to soften the changing of fields from JSON Number to JSON
/// String by permitting deserialization, but discourage new uses of JSON Number.
#[cfg(feature = "serde")]
struct NumberOrString;

#[cfg(feature = "serde")]
impl<'de> serde_with::DeserializeAs<'de, i64> for NumberOrString {
    fn deserialize_as<D>(deserializer: D) -> Result<i64, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::Deserialize;
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum I64OrString<'a> {
            Str(&'a str),
            String(String),
            I64(i64),
        }
        match I64OrString::deserialize(deserializer)? {
            I64OrString::Str(s) => s.parse().map_err(serde::de::Error::custom),
            I64OrString::String(s) => s.parse().map_err(serde::de::Error::custom),
            I64OrString::I64(v) => Ok(v),
        }
    }
}

#[cfg(feature = "serde")]
impl<'de> serde_with::DeserializeAs<'de, u64> for NumberOrString {
    fn deserialize_as<D>(deserializer: D) -> Result<u64, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::Deserialize;
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum U64OrString<'a> {
            Str(&'a str),
            String(String),
            U64(u64),
        }
        match U64OrString::deserialize(deserializer)? {
            U64OrString::Str(s) => s.parse().map_err(serde::de::Error::custom),
            U64OrString::String(s) => s.parse().map_err(serde::de::Error::custom),
            U64OrString::U64(v) => Ok(v),
        }
    }
}

#[cfg(feature = "serde")]
impl serde_with::SerializeAs<i64> for NumberOrString {
    fn serialize_as<S>(source: &i64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(source)
    }
}

#[cfg(feature = "serde")]
impl serde_with::SerializeAs<u64> for NumberOrString {
    fn serialize_as<S>(source: &u64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(source)
    }
}

#[cfg(feature = "schemars")]
impl<T> serde_with::schemars_0_8::JsonSchemaAs<T> for NumberOrString {
    fn schema_name() -> String {
        <String as schemars::JsonSchema>::schema_name()
    }

    fn schema_id() -> std::borrow::Cow<'static, str> {
        <String as schemars::JsonSchema>::schema_id()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        <String as schemars::JsonSchema>::json_schema(gen)
    }

    fn is_referenceable() -> bool {
        <String as schemars::JsonSchema>::is_referenceable()
    }
}

// Tests ------------------------------------------------------------------------

#[cfg(all(test, feature = "std"))]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    pub fn vec_u8_read_without_padding() {
        let buf = Cursor::new(vec![0, 0, 0, 4, 2, 2, 2, 2]);
        let v = VecM::<u8, 8>::read_xdr(&mut Limited::new(buf, Limits::none())).unwrap();
        assert_eq!(v.to_vec(), vec![2, 2, 2, 2]);
    }

    #[test]
    pub fn vec_u8_read_with_padding() {
        let buf = Cursor::new(vec![0, 0, 0, 1, 2, 0, 0, 0]);
        let v = VecM::<u8, 8>::read_xdr(&mut Limited::new(buf, Limits::none())).unwrap();
        assert_eq!(v.to_vec(), vec![2]);
    }

    #[test]
    pub fn vec_u8_read_with_insufficient_padding() {
        let buf = Cursor::new(vec![0, 0, 0, 1, 2, 0, 0]);
        let res = VecM::<u8, 8>::read_xdr(&mut Limited::new(buf, Limits::none()));
        match res {
            Err(Error::Io(_)) => (),
            _ => panic!("expected IO error got {res:?}"),
        }
    }

    #[test]
    pub fn vec_u8_read_with_non_zero_padding() {
        let buf = Cursor::new(vec![0, 0, 0, 1, 2, 3, 0, 0]);
        let res = VecM::<u8, 8>::read_xdr(&mut Limited::new(buf, Limits::none()));
        match res {
            Err(Error::NonZeroPadding) => (),
            _ => panic!("expected NonZeroPadding got {res:?}"),
        }
    }

    #[test]
    pub fn vec_u8_write_without_padding() {
        let mut buf = vec![];
        let v: VecM<u8, 8> = vec![2, 2, 2, 2].try_into().unwrap();

        v.write_xdr(&mut Limited::new(Cursor::new(&mut buf), Limits::none()))
            .unwrap();
        assert_eq!(buf, vec![0, 0, 0, 4, 2, 2, 2, 2]);
    }

    #[test]
    pub fn vec_u8_write_with_padding() {
        let mut buf = vec![];
        let v: VecM<u8, 8> = vec![2].try_into().unwrap();
        v.write_xdr(&mut Limited::new(Cursor::new(&mut buf), Limits::none()))
            .unwrap();
        assert_eq!(buf, vec![0, 0, 0, 1, 2, 0, 0, 0]);
    }

    #[test]
    pub fn arr_u8_read_without_padding() {
        let buf = Cursor::new(vec![2, 2, 2, 2]);
        let v = <[u8; 4]>::read_xdr(&mut Limited::new(buf, Limits::none())).unwrap();
        assert_eq!(v, [2, 2, 2, 2]);
    }

    #[test]
    pub fn arr_u8_read_with_padding() {
        let buf = Cursor::new(vec![2, 0, 0, 0]);
        let v = <[u8; 1]>::read_xdr(&mut Limited::new(buf, Limits::none())).unwrap();
        assert_eq!(v, [2]);
    }

    #[test]
    pub fn arr_u8_read_with_insufficient_padding() {
        let buf = Cursor::new(vec![2, 0, 0]);
        let res = <[u8; 1]>::read_xdr(&mut Limited::new(buf, Limits::none()));
        match res {
            Err(Error::Io(_)) => (),
            _ => panic!("expected IO error got {res:?}"),
        }
    }

    #[test]
    pub fn arr_u8_read_with_non_zero_padding() {
        let buf = Cursor::new(vec![2, 3, 0, 0]);
        let res = <[u8; 1]>::read_xdr(&mut Limited::new(buf, Limits::none()));
        match res {
            Err(Error::NonZeroPadding) => (),
            _ => panic!("expected NonZeroPadding got {res:?}"),
        }
    }

    #[test]
    pub fn arr_u8_write_without_padding() {
        let mut buf = vec![];
        [2u8, 2, 2, 2]
            .write_xdr(&mut Limited::new(Cursor::new(&mut buf), Limits::none()))
            .unwrap();
        assert_eq!(buf, vec![2, 2, 2, 2]);
    }

    #[test]
    pub fn arr_u8_write_with_padding() {
        let mut buf = vec![];
        [2u8]
            .write_xdr(&mut Limited::new(Cursor::new(&mut buf), Limits::none()))
            .unwrap();
        assert_eq!(buf, vec![2, 0, 0, 0]);
    }
}

#[cfg(all(test, feature = "std"))]
mod test {
    use super::*;

    #[test]
    fn into_option_none() {
        let v: VecM<u32, 1> = vec![].try_into().unwrap();
        assert_eq!(v.into_option(), None);
    }

    #[test]
    fn into_option_some() {
        let v: VecM<_, 1> = vec![1].try_into().unwrap();
        assert_eq!(v.into_option(), Some(1));
    }

    #[test]
    fn to_option_none() {
        let v: VecM<u32, 1> = vec![].try_into().unwrap();
        assert_eq!(v.to_option(), None);
    }

    #[test]
    fn to_option_some() {
        let v: VecM<_, 1> = vec![1].try_into().unwrap();
        assert_eq!(v.to_option(), Some(1));
    }

    #[test]
    fn depth_limited_read_write_under_the_limit_success() {
        let a: Option<Option<Option<u32>>> = Some(Some(Some(5)));
        let mut buf = Limited::new(Vec::new(), Limits::depth(4));
        a.write_xdr(&mut buf).unwrap();

        let mut dlr = Limited::new(Cursor::new(buf.inner.as_slice()), Limits::depth(4));
        let a_back: Option<Option<Option<u32>>> = ReadXdr::read_xdr(&mut dlr).unwrap();
        assert_eq!(a, a_back);
    }

    #[test]
    fn write_over_depth_limit_fail() {
        let a: Option<Option<Option<u32>>> = Some(Some(Some(5)));
        let mut buf = Limited::new(Vec::new(), Limits::depth(3));
        let res = a.write_xdr(&mut buf);
        match res {
            Err(Error::DepthLimitExceeded) => (),
            _ => panic!("expected DepthLimitExceeded got {res:?}"),
        }
    }

    #[test]
    fn read_over_depth_limit_fail() {
        let read_limits = Limits::depth(3);
        let write_limits = Limits::depth(5);
        let a: Option<Option<Option<u32>>> = Some(Some(Some(5)));
        let mut buf = Limited::new(Vec::new(), write_limits);
        a.write_xdr(&mut buf).unwrap();

        let mut dlr = Limited::new(Cursor::new(buf.inner.as_slice()), read_limits);
        let res: Result<Option<Option<Option<u32>>>, _> = ReadXdr::read_xdr(&mut dlr);
        match res {
            Err(Error::DepthLimitExceeded) => (),
            _ => panic!("expected DepthLimitExceeded got {res:?}"),
        }
    }

    #[test]
    fn length_limited_read_write_i32() {
        // Exact limit, success
        let v = 123i32;
        let mut buf = Limited::new(Vec::new(), Limits::len(4));
        v.write_xdr(&mut buf).unwrap();
        assert_eq!(buf.limits.len, 0);
        let mut lr = Limited::new(Cursor::new(buf.inner.as_slice()), Limits::len(4));
        let v_back: i32 = ReadXdr::read_xdr(&mut lr).unwrap();
        assert_eq!(buf.limits.len, 0);
        assert_eq!(v, v_back);

        // Over limit, success
        let v = 123i32;
        let mut buf = Limited::new(Vec::new(), Limits::len(5));
        v.write_xdr(&mut buf).unwrap();
        assert_eq!(buf.limits.len, 1);
        let mut lr = Limited::new(Cursor::new(buf.inner.as_slice()), Limits::len(5));
        let v_back: i32 = ReadXdr::read_xdr(&mut lr).unwrap();
        assert_eq!(buf.limits.len, 1);
        assert_eq!(v, v_back);

        // Write under limit, failure
        let v = 123i32;
        let mut buf = Limited::new(Vec::new(), Limits::len(3));
        assert_eq!(v.write_xdr(&mut buf), Err(Error::LengthLimitExceeded));

        // Read under limit, failure
        let v = 123i32;
        let mut buf = Limited::new(Vec::new(), Limits::len(4));
        v.write_xdr(&mut buf).unwrap();
        assert_eq!(buf.limits.len, 0);
        let mut lr = Limited::new(Cursor::new(buf.inner.as_slice()), Limits::len(3));
        assert_eq!(
            <i32 as ReadXdr>::read_xdr(&mut lr),
            Err(Error::LengthLimitExceeded)
        );
    }

    #[test]
    fn length_limited_read_write_u32() {
        // Exact limit, success
        let v = 123u32;
        let mut buf = Limited::new(Vec::new(), Limits::len(4));
        v.write_xdr(&mut buf).unwrap();
        assert_eq!(buf.limits.len, 0);
        let mut lr = Limited::new(Cursor::new(buf.inner.as_slice()), Limits::len(4));
        let v_back: u32 = ReadXdr::read_xdr(&mut lr).unwrap();
        assert_eq!(buf.limits.len, 0);
        assert_eq!(v, v_back);

        // Over limit, success
        let v = 123u32;
        let mut buf = Limited::new(Vec::new(), Limits::len(5));
        v.write_xdr(&mut buf).unwrap();
        assert_eq!(buf.limits.len, 1);
        let mut lr = Limited::new(Cursor::new(buf.inner.as_slice()), Limits::len(5));
        let v_back: u32 = ReadXdr::read_xdr(&mut lr).unwrap();
        assert_eq!(buf.limits.len, 1);
        assert_eq!(v, v_back);

        // Write under limit, failure
        let v = 123u32;
        let mut buf = Limited::new(Vec::new(), Limits::len(3));
        assert_eq!(v.write_xdr(&mut buf), Err(Error::LengthLimitExceeded));

        // Read under limit, failure
        let v = 123u32;
        let mut buf = Limited::new(Vec::new(), Limits::len(4));
        v.write_xdr(&mut buf).unwrap();
        assert_eq!(buf.limits.len, 0);
        let mut lr = Limited::new(Cursor::new(buf.inner.as_slice()), Limits::len(3));
        assert_eq!(
            <u32 as ReadXdr>::read_xdr(&mut lr),
            Err(Error::LengthLimitExceeded)
        );
    }

    #[test]
    fn length_limited_read_write_i64() {
        // Exact limit, success
        let v = 123i64;
        let mut buf = Limited::new(Vec::new(), Limits::len(8));
        v.write_xdr(&mut buf).unwrap();
        assert_eq!(buf.limits.len, 0);
        let mut lr = Limited::new(Cursor::new(buf.inner.as_slice()), Limits::len(8));
        let v_back: i64 = ReadXdr::read_xdr(&mut lr).unwrap();
        assert_eq!(buf.limits.len, 0);
        assert_eq!(v, v_back);

        // Over limit, success
        let v = 123i64;
        let mut buf = Limited::new(Vec::new(), Limits::len(9));
        v.write_xdr(&mut buf).unwrap();
        assert_eq!(buf.limits.len, 1);
        let mut lr = Limited::new(Cursor::new(buf.inner.as_slice()), Limits::len(9));
        let v_back: i64 = ReadXdr::read_xdr(&mut lr).unwrap();
        assert_eq!(buf.limits.len, 1);
        assert_eq!(v, v_back);

        // Write under limit, failure
        let v = 123i64;
        let mut buf = Limited::new(Vec::new(), Limits::len(7));
        assert_eq!(v.write_xdr(&mut buf), Err(Error::LengthLimitExceeded));

        // Read under limit, failure
        let v = 123i64;
        let mut buf = Limited::new(Vec::new(), Limits::len(8));
        v.write_xdr(&mut buf).unwrap();
        assert_eq!(buf.limits.len, 0);
        let mut lr = Limited::new(Cursor::new(buf.inner.as_slice()), Limits::len(7));
        assert_eq!(
            <i64 as ReadXdr>::read_xdr(&mut lr),
            Err(Error::LengthLimitExceeded)
        );
    }

    #[test]
    fn length_limited_read_write_u64() {
        // Exact limit, success
        let v = 123u64;
        let mut buf = Limited::new(Vec::new(), Limits::len(8));
        v.write_xdr(&mut buf).unwrap();
        assert_eq!(buf.limits.len, 0);
        let mut lr = Limited::new(Cursor::new(buf.inner.as_slice()), Limits::len(8));
        let v_back: u64 = ReadXdr::read_xdr(&mut lr).unwrap();
        assert_eq!(buf.limits.len, 0);
        assert_eq!(v, v_back);

        // Over limit, success
        let v = 123u64;
        let mut buf = Limited::new(Vec::new(), Limits::len(9));
        v.write_xdr(&mut buf).unwrap();
        assert_eq!(buf.limits.len, 1);
        let mut lr = Limited::new(Cursor::new(buf.inner.as_slice()), Limits::len(9));
        let v_back: u64 = ReadXdr::read_xdr(&mut lr).unwrap();
        assert_eq!(buf.limits.len, 1);
        assert_eq!(v, v_back);

        // Write under limit, failure
        let v = 123u64;
        let mut buf = Limited::new(Vec::new(), Limits::len(7));
        assert_eq!(v.write_xdr(&mut buf), Err(Error::LengthLimitExceeded));

        // Read under limit, failure
        let v = 123u64;
        let mut buf = Limited::new(Vec::new(), Limits::len(8));
        v.write_xdr(&mut buf).unwrap();
        assert_eq!(buf.limits.len, 0);
        let mut lr = Limited::new(Cursor::new(buf.inner.as_slice()), Limits::len(7));
        assert_eq!(
            <u64 as ReadXdr>::read_xdr(&mut lr),
            Err(Error::LengthLimitExceeded)
        );
    }

    #[test]
    fn length_limited_read_write_bool() {
        // Exact limit, success
        let v = true;
        let mut buf = Limited::new(Vec::new(), Limits::len(4));
        v.write_xdr(&mut buf).unwrap();
        assert_eq!(buf.limits.len, 0);
        let mut lr = Limited::new(Cursor::new(buf.inner.as_slice()), Limits::len(4));
        let v_back: bool = ReadXdr::read_xdr(&mut lr).unwrap();
        assert_eq!(buf.limits.len, 0);
        assert_eq!(v, v_back);

        // Over limit, success
        let v = true;
        let mut buf = Limited::new(Vec::new(), Limits::len(5));
        v.write_xdr(&mut buf).unwrap();
        assert_eq!(buf.limits.len, 1);
        let mut lr = Limited::new(Cursor::new(buf.inner.as_slice()), Limits::len(5));
        let v_back: bool = ReadXdr::read_xdr(&mut lr).unwrap();
        assert_eq!(buf.limits.len, 1);
        assert_eq!(v, v_back);

        // Write under limit, failure
        let v = true;
        let mut buf = Limited::new(Vec::new(), Limits::len(3));
        assert_eq!(v.write_xdr(&mut buf), Err(Error::LengthLimitExceeded));

        // Read under limit, failure
        let v = true;
        let mut buf = Limited::new(Vec::new(), Limits::len(4));
        v.write_xdr(&mut buf).unwrap();
        assert_eq!(buf.limits.len, 0);
        let mut lr = Limited::new(Cursor::new(buf.inner.as_slice()), Limits::len(3));
        assert_eq!(
            <bool as ReadXdr>::read_xdr(&mut lr),
            Err(Error::LengthLimitExceeded)
        );
    }

    #[test]
    fn length_limited_read_write_option() {
        // Exact limit, success
        let v = Some(true);
        let mut buf = Limited::new(Vec::new(), Limits::len(8));
        v.write_xdr(&mut buf).unwrap();
        assert_eq!(buf.limits.len, 0);
        let mut lr = Limited::new(Cursor::new(buf.inner.as_slice()), Limits::len(8));
        let v_back: Option<bool> = ReadXdr::read_xdr(&mut lr).unwrap();
        assert_eq!(buf.limits.len, 0);
        assert_eq!(v, v_back);

        // Over limit, success
        let v = Some(true);
        let mut buf = Limited::new(Vec::new(), Limits::len(9));
        v.write_xdr(&mut buf).unwrap();
        assert_eq!(buf.limits.len, 1);
        let mut lr = Limited::new(Cursor::new(buf.inner.as_slice()), Limits::len(9));
        let v_back: Option<bool> = ReadXdr::read_xdr(&mut lr).unwrap();
        assert_eq!(buf.limits.len, 1);
        assert_eq!(v, v_back);

        // Write under limit, failure
        let v = Some(true);
        let mut buf = Limited::new(Vec::new(), Limits::len(7));
        assert_eq!(v.write_xdr(&mut buf), Err(Error::LengthLimitExceeded));

        // Read under limit, failure
        let v = Some(true);
        let mut buf = Limited::new(Vec::new(), Limits::len(8));
        v.write_xdr(&mut buf).unwrap();
        assert_eq!(buf.limits.len, 0);
        let mut lr = Limited::new(Cursor::new(buf.inner.as_slice()), Limits::len(7));
        assert_eq!(
            <Option<bool> as ReadXdr>::read_xdr(&mut lr),
            Err(Error::LengthLimitExceeded)
        );
    }

    #[test]
    fn length_limited_read_write_array_u8() {
        // Exact limit, success
        let v = [1u8, 2, 3];
        let mut buf = Limited::new(Vec::new(), Limits::len(4));
        v.write_xdr(&mut buf).unwrap();
        assert_eq!(buf.limits.len, 0);
        let mut lr = Limited::new(Cursor::new(buf.inner.as_slice()), Limits::len(4));
        let v_back: [u8; 3] = ReadXdr::read_xdr(&mut lr).unwrap();
        assert_eq!(buf.limits.len, 0);
        assert_eq!(v, v_back);

        // Over limit, success
        let v = [1u8, 2, 3];
        let mut buf = Limited::new(Vec::new(), Limits::len(5));
        v.write_xdr(&mut buf).unwrap();
        assert_eq!(buf.limits.len, 1);
        let mut lr = Limited::new(Cursor::new(buf.inner.as_slice()), Limits::len(5));
        let v_back: [u8; 3] = ReadXdr::read_xdr(&mut lr).unwrap();
        assert_eq!(buf.limits.len, 1);
        assert_eq!(v, v_back);

        // Write under limit, failure
        let v = [1u8, 2, 3];
        let mut buf = Limited::new(Vec::new(), Limits::len(3));
        assert_eq!(v.write_xdr(&mut buf), Err(Error::LengthLimitExceeded));

        // Read under limit, failure
        let v = [1u8, 2, 3];
        let mut buf = Limited::new(Vec::new(), Limits::len(4));
        v.write_xdr(&mut buf).unwrap();
        assert_eq!(buf.limits.len, 0);
        let mut lr = Limited::new(Cursor::new(buf.inner.as_slice()), Limits::len(3));
        assert_eq!(
            <[u8; 3] as ReadXdr>::read_xdr(&mut lr),
            Err(Error::LengthLimitExceeded)
        );
    }

    #[test]
    fn length_limited_read_write_array_type() {
        // Exact limit, success
        let v = [true, false, true];
        let mut buf = Limited::new(Vec::new(), Limits::len(12));
        v.write_xdr(&mut buf).unwrap();
        assert_eq!(buf.limits.len, 0);
        let mut lr = Limited::new(Cursor::new(buf.inner.as_slice()), Limits::len(12));
        let v_back: [bool; 3] = ReadXdr::read_xdr(&mut lr).unwrap();
        assert_eq!(buf.limits.len, 0);
        assert_eq!(v, v_back);

        // Over limit, success
        let v = [true, false, true];
        let mut buf = Limited::new(Vec::new(), Limits::len(13));
        v.write_xdr(&mut buf).unwrap();
        assert_eq!(buf.limits.len, 1);
        let mut lr = Limited::new(Cursor::new(buf.inner.as_slice()), Limits::len(13));
        let v_back: [bool; 3] = ReadXdr::read_xdr(&mut lr).unwrap();
        assert_eq!(buf.limits.len, 1);
        assert_eq!(v, v_back);

        // Write under limit, failure
        let v = [true, false, true];
        let mut buf = Limited::new(Vec::new(), Limits::len(11));
        assert_eq!(v.write_xdr(&mut buf), Err(Error::LengthLimitExceeded));

        // Read under limit, failure
        let v = [true, false, true];
        let mut buf = Limited::new(Vec::new(), Limits::len(12));
        v.write_xdr(&mut buf).unwrap();
        assert_eq!(buf.limits.len, 0);
        let mut lr = Limited::new(Cursor::new(buf.inner.as_slice()), Limits::len(11));
        assert_eq!(
            <[bool; 3] as ReadXdr>::read_xdr(&mut lr),
            Err(Error::LengthLimitExceeded)
        );
    }

    #[test]
    fn length_limited_read_write_vec() {
        // Exact limit, success
        let v = VecM::<i32, 3>::try_from([1i32, 2, 3]).unwrap();
        let mut buf = Limited::new(Vec::new(), Limits::len(16));
        v.write_xdr(&mut buf).unwrap();
        assert_eq!(buf.limits.len, 0);
        let mut lr = Limited::new(Cursor::new(buf.inner.as_slice()), Limits::len(16));
        let v_back: VecM<i32, 3> = ReadXdr::read_xdr(&mut lr).unwrap();
        assert_eq!(buf.limits.len, 0);
        assert_eq!(v, v_back);

        // Over limit, success
        let v = VecM::<i32, 3>::try_from([1i32, 2, 3]).unwrap();
        let mut buf = Limited::new(Vec::new(), Limits::len(17));
        v.write_xdr(&mut buf).unwrap();
        assert_eq!(buf.limits.len, 1);
        let mut lr = Limited::new(Cursor::new(buf.inner.as_slice()), Limits::len(17));
        let v_back: VecM<i32, 3> = ReadXdr::read_xdr(&mut lr).unwrap();
        assert_eq!(buf.limits.len, 1);
        assert_eq!(v, v_back);

        // Write under limit, failure
        let v = VecM::<i32, 3>::try_from([1i32, 2, 3]).unwrap();
        let mut buf = Limited::new(Vec::new(), Limits::len(15));
        assert_eq!(v.write_xdr(&mut buf), Err(Error::LengthLimitExceeded));

        // Read under limit, failure
        let v = VecM::<i32, 3>::try_from([1i32, 2, 3]).unwrap();
        let mut buf = Limited::new(Vec::new(), Limits::len(16));
        v.write_xdr(&mut buf).unwrap();
        assert_eq!(buf.limits.len, 0);
        let mut lr = Limited::new(Cursor::new(buf.inner.as_slice()), Limits::len(15));
        assert_eq!(
            <VecM<i32, 3> as ReadXdr>::read_xdr(&mut lr),
            Err(Error::LengthLimitExceeded)
        );
    }

    #[test]
    fn length_limited_read_write_bytes() {
        // Exact limit, success
        let v = BytesM::<3>::try_from([1u8, 2, 3]).unwrap();
        let mut buf = Limited::new(Vec::new(), Limits::len(8));
        v.write_xdr(&mut buf).unwrap();
        assert_eq!(buf.limits.len, 0);
        let mut lr = Limited::new(Cursor::new(buf.inner.as_slice()), Limits::len(8));
        let v_back: BytesM<3> = ReadXdr::read_xdr(&mut lr).unwrap();
        assert_eq!(buf.limits.len, 0);
        assert_eq!(v, v_back);

        // Over limit, success
        let v = BytesM::<3>::try_from([1u8, 2, 3]).unwrap();
        let mut buf = Limited::new(Vec::new(), Limits::len(9));
        v.write_xdr(&mut buf).unwrap();
        assert_eq!(buf.limits.len, 1);
        let mut lr = Limited::new(Cursor::new(buf.inner.as_slice()), Limits::len(9));
        let v_back: BytesM<3> = ReadXdr::read_xdr(&mut lr).unwrap();
        assert_eq!(buf.limits.len, 1);
        assert_eq!(v, v_back);

        // Write under limit, failure
        let v = BytesM::<3>::try_from([1u8, 2, 3]).unwrap();
        let mut buf = Limited::new(Vec::new(), Limits::len(7));
        assert_eq!(v.write_xdr(&mut buf), Err(Error::LengthLimitExceeded));

        // Read under limit, failure
        let v = BytesM::<3>::try_from([1u8, 2, 3]).unwrap();
        let mut buf = Limited::new(Vec::new(), Limits::len(8));
        v.write_xdr(&mut buf).unwrap();
        assert_eq!(buf.limits.len, 0);
        let mut lr = Limited::new(Cursor::new(buf.inner.as_slice()), Limits::len(7));
        assert_eq!(
            <BytesM<3> as ReadXdr>::read_xdr(&mut lr),
            Err(Error::LengthLimitExceeded)
        );
    }

    #[test]
    fn length_limited_read_write_string() {
        // Exact limit, success
        let v = StringM::<3>::try_from("123").unwrap();
        let mut buf = Limited::new(Vec::new(), Limits::len(8));
        v.write_xdr(&mut buf).unwrap();
        assert_eq!(buf.limits.len, 0);
        let mut lr = Limited::new(Cursor::new(buf.inner.as_slice()), Limits::len(8));
        let v_back: StringM<3> = ReadXdr::read_xdr(&mut lr).unwrap();
        assert_eq!(buf.limits.len, 0);
        assert_eq!(v, v_back);

        // Over limit, success
        let v = StringM::<3>::try_from("123").unwrap();
        let mut buf = Limited::new(Vec::new(), Limits::len(9));
        v.write_xdr(&mut buf).unwrap();
        assert_eq!(buf.limits.len, 1);
        let mut lr = Limited::new(Cursor::new(buf.inner.as_slice()), Limits::len(9));
        let v_back: StringM<3> = ReadXdr::read_xdr(&mut lr).unwrap();
        assert_eq!(buf.limits.len, 1);
        assert_eq!(v, v_back);

        // Write under limit, failure
        let v = StringM::<3>::try_from("123").unwrap();
        let mut buf = Limited::new(Vec::new(), Limits::len(7));
        assert_eq!(v.write_xdr(&mut buf), Err(Error::LengthLimitExceeded));

        // Read under limit, failure
        let v = StringM::<3>::try_from("123").unwrap();
        let mut buf = Limited::new(Vec::new(), Limits::len(8));
        v.write_xdr(&mut buf).unwrap();
        assert_eq!(buf.limits.len, 0);
        let mut lr = Limited::new(Cursor::new(buf.inner.as_slice()), Limits::len(7));
        assert_eq!(
            <StringM<3> as ReadXdr>::read_xdr(&mut lr),
            Err(Error::LengthLimitExceeded)
        );
    }
}

#[cfg(all(test, not(feature = "alloc")))]
mod test {
    use super::VecM;

    #[test]
    fn to_option_none() {
        let v: VecM<u32, 1> = (&[]).try_into().unwrap();
        assert_eq!(v.to_option(), None);
    }

    #[test]
    fn to_option_some() {
        let v: VecM<_, 1> = (&[1]).try_into().unwrap();
        assert_eq!(v.to_option(), Some(1));
    }
}

#[cfg(all(test, feature = "serde"))]
mod tests_for_number_or_string {
    use super::*;
    use serde::{Deserialize, Serialize};
    use serde_with::serde_as;

    // --- Helper Structs ---
    #[serde_as]
    #[derive(Debug, PartialEq, Deserialize, Serialize)]
    struct TestI64 {
        #[serde_as(as = "NumberOrString")]
        val: i64,
    }

    #[serde_as]
    #[derive(Debug, PartialEq, Deserialize, Serialize)]
    struct TestU64 {
        #[serde_as(as = "NumberOrString")]
        val: u64,
    }

    #[serde_as]
    #[derive(Debug, PartialEq, Deserialize, Serialize)]
    struct TestOptionI64 {
        #[serde_as(as = "Option<NumberOrString>")]
        val: Option<i64>,
    }

    #[serde_as]
    #[derive(Debug, PartialEq, Deserialize, Serialize)]
    struct TestOptionU64 {
        #[serde_as(as = "Option<NumberOrString>")]
        val: Option<u64>,
    }

    #[serde_as]
    #[derive(Debug, PartialEq, Deserialize, Serialize)]
    struct TestVecI64 {
        #[serde_as(as = "Vec<NumberOrString>")]
        val: Vec<i64>,
    }

    #[serde_as]
    #[derive(Debug, PartialEq, Deserialize, Serialize)]
    struct TestVecU64 {
        #[serde_as(as = "Vec<NumberOrString>")]
        val: Vec<u64>,
    }

    // Helper Enum for testing field access within variants
    #[serde_as]
    #[derive(Debug, PartialEq, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")] // Added to make JSON keys distinct for variants
    enum TestEnum {
        VariantA {
            #[serde(rename = "numVal")]
            #[serde_as(as = "NumberOrString")]
            num_val: i64,
            #[serde(rename = "otherData")]
            other_data: String,
        },
        VariantB {
            #[serde_as(as = "NumberOrString")]
            count: u64,
        },
        SimpleVariant,
    }

    // --- i64 Deserialization Tests ---
    #[test]
    fn deserialize_i64_from_json_reader() {
        let json = r#"{"val": "123"}"#;
        let expected = TestI64 { val: 123 };
        assert_eq!(
            serde_json::from_reader::<_, TestI64>(Cursor::new(json)).unwrap(),
            expected
        );
    }

    #[test]
    fn deserialize_i64_from_json_number_positive() {
        let json = r#"{"val": 123}"#;
        let expected = TestI64 { val: 123 };
        assert_eq!(serde_json::from_str::<TestI64>(json).unwrap(), expected);
    }

    #[test]
    fn deserialize_i64_from_json_number_negative() {
        let json = r#"{"val": -456}"#;
        let expected = TestI64 { val: -456 };
        assert_eq!(serde_json::from_str::<TestI64>(json).unwrap(), expected);
    }

    #[test]
    fn deserialize_i64_from_json_number_zero() {
        let json = r#"{"val": 0}"#;
        let expected = TestI64 { val: 0 };
        assert_eq!(serde_json::from_str::<TestI64>(json).unwrap(), expected);
    }

    #[test]
    fn deserialize_i64_from_json_number_max() {
        let json = format!(r#"{{"val": {}}}"#, i64::MAX);
        let expected = TestI64 { val: i64::MAX };
        assert_eq!(serde_json::from_str::<TestI64>(&json).unwrap(), expected);
    }

    #[test]
    fn deserialize_i64_from_json_number_min() {
        let json = format!(r#"{{"val": {}}}"#, i64::MIN);
        let expected = TestI64 { val: i64::MIN };
        assert_eq!(serde_json::from_str::<TestI64>(&json).unwrap(), expected);
    }

    #[test]
    fn deserialize_i64_from_json_string_positive() {
        let json = r#"{"val": "789"}"#;
        let expected = TestI64 { val: 789 };
        assert_eq!(serde_json::from_str::<TestI64>(json).unwrap(), expected);
    }

    #[test]
    fn deserialize_i64_from_json_string_negative() {
        let json = r#"{"val": "-101"}"#;
        let expected = TestI64 { val: -101 };
        assert_eq!(serde_json::from_str::<TestI64>(json).unwrap(), expected);
    }

    #[test]
    fn deserialize_i64_from_json_string_zero() {
        let json = r#"{"val": "0"}"#;
        let expected = TestI64 { val: 0 };
        assert_eq!(serde_json::from_str::<TestI64>(json).unwrap(), expected);
    }

    #[test]
    fn deserialize_i64_from_json_string_max() {
        let json = format!(r#"{{"val": "{}"}}"#, i64::MAX);
        let expected = TestI64 { val: i64::MAX };
        assert_eq!(serde_json::from_str::<TestI64>(&json).unwrap(), expected);
    }

    #[test]
    fn deserialize_i64_from_json_string_min() {
        let json = format!(r#"{{"val": "{}"}}"#, i64::MIN);
        let expected = TestI64 { val: i64::MIN };
        assert_eq!(serde_json::from_str::<TestI64>(&json).unwrap(), expected);
    }

    #[test]
    fn deserialize_i64_from_json_string_with_plus_prefix() {
        let json = r#"{"val": "+123"}"#;
        let expected = TestI64 { val: 123 };
        assert_eq!(serde_json::from_str::<TestI64>(json).unwrap(), expected);
    }

    #[test]
    fn deserialize_i64_from_json_string_with_plus_zero() {
        let json = r#"{"val": "+0"}"#;
        let expected = TestI64 { val: 0 };
        assert_eq!(serde_json::from_str::<TestI64>(json).unwrap(), expected);
    }

    #[test]
    fn deserialize_i64_from_json_string_with_minus_zero() {
        let json = r#"{"val": "-0"}"#;
        let expected = TestI64 { val: 0 };
        assert_eq!(serde_json::from_str::<TestI64>(json).unwrap(), expected);
    }

    #[test]
    fn deserialize_i64_error_from_json_string_with_leading_whitespace() {
        let json = r#"{"val": " 123"}"#;
        assert!(serde_json::from_str::<TestI64>(json).is_err());
    }

    #[test]
    fn deserialize_i64_error_from_json_string_with_trailing_whitespace() {
        let json = r#"{"val": "123 "}"#;
        assert!(serde_json::from_str::<TestI64>(json).is_err());
    }

    #[test]
    fn deserialize_i64_error_from_json_string_with_both_whitespace() {
        let json = r#"{"val": " 123 "}"#;
        assert!(serde_json::from_str::<TestI64>(json).is_err());
    }

    #[test]
    fn deserialize_i64_error_from_json_string_with_invalid_plus_prefix() {
        let json = r#"{"val": "++123"}"#;
        assert!(serde_json::from_str::<TestI64>(json).is_err());
    }

    #[test]
    fn deserialize_i64_error_from_json_string_with_invalid_minus_prefix() {
        let json = r#"{"val": "--123"}"#;
        assert!(serde_json::from_str::<TestI64>(json).is_err());
    }

    #[test]
    fn deserialize_i64_error_from_json_string_with_invalid_mixed_prefix() {
        let json = r#"{"val": "+-123"}"#;
        assert!(serde_json::from_str::<TestI64>(json).is_err());
    }

    #[test]
    fn deserialize_i64_error_from_string_not_a_number() {
        let json = r#"{"val": "abc"}"#;
        assert!(serde_json::from_str::<TestI64>(json).is_err());
    }

    #[test]
    fn deserialize_i64_error_from_string_float() {
        let json = r#"{"val": "123.45"}"#; // Not an integer
        assert!(serde_json::from_str::<TestI64>(json).is_err());
    }

    #[test]
    fn deserialize_i64_error_from_string_empty() {
        let json = r#"{"val": ""}"#;
        assert!(serde_json::from_str::<TestI64>(json).is_err());
    }

    #[test]
    fn deserialize_i64_error_from_string_overflow() {
        let overflow_val = i128::from(i64::MAX) + 1;
        let json = format!(r#"{{"val": "{overflow_val}"}}"#);
        assert!(serde_json::from_str::<TestI64>(&json).is_err());
    }

    #[test]
    fn deserialize_i64_error_from_string_underflow() {
        let underflow_val = i128::from(i64::MIN) - 1;
        let json = format!(r#"{{"val": "{underflow_val}"}}"#);
        assert!(serde_json::from_str::<TestI64>(&json).is_err());
    }

    #[test]
    fn deserialize_i64_error_from_json_float_number() {
        let json = r#"{"val": 123.45}"#;
        assert!(serde_json::from_str::<TestI64>(json).is_err());
    }

    #[test]
    fn deserialize_i64_error_from_json_bool_true() {
        let json = r#"{"val": true}"#;
        assert!(serde_json::from_str::<TestI64>(json).is_err());
    }

    #[test]
    fn deserialize_i64_error_from_json_array() {
        let json = r#"{"val": []}"#;
        assert!(serde_json::from_str::<TestI64>(json).is_err());
    }

    #[test]
    fn deserialize_i64_error_from_json_object() {
        let json = r#"{"val": {}}"#;
        assert!(serde_json::from_str::<TestI64>(json).is_err());
    }

    #[test]
    fn deserialize_i64_error_from_json_null() {
        let json = r#"{"val": null}"#;
        assert!(serde_json::from_str::<TestI64>(json).is_err());
    }

    // -- Additional i64 String Format Tests --
    #[test]
    fn deserialize_i64_error_from_hex_string() {
        let json = r#"{"val": "0x1A"}"#; // Hex "26"
                                         // std::primitive::i64.from_str() does not support "0x"
        assert!(
            serde_json::from_str::<TestI64>(json).is_err(),
            "Hex string should fail parsing to i64"
        );
    }

    #[test]
    fn deserialize_i64_error_from_octal_string() {
        let json = r#"{"val": "0o77"}"#; // Octal "63"
                                         // std::primitive::i64.from_str() does not support "0o"
        assert!(
            serde_json::from_str::<TestI64>(json).is_err(),
            "Octal string should fail parsing to i64"
        );
    }

    #[test]
    fn deserialize_i64_error_from_scientific_notation_string() {
        let json = r#"{"val": "1e3"}"#; // "1000" in scientific
                                        // std::primitive::i64.from_str() does not support scientific notation
        assert!(
            serde_json::from_str::<TestI64>(json).is_err(),
            "Scientific notation string should fail parsing to i64"
        );
    }

    #[test]
    fn deserialize_i64_error_from_invalid_scientific_notation_string() {
        let json = r#"{"val": "1e"}"#;
        assert!(
            serde_json::from_str::<TestI64>(json).is_err(),
            "Invalid scientific notation string should fail"
        );
    }

    #[test]
    fn deserialize_i64_error_from_string_with_underscores() {
        let json = r#"{"val": "1_000_000"}"#;
        // std::primitive::i64.from_str() does not support underscores
        assert!(
            serde_json::from_str::<TestI64>(json).is_err(),
            "String with underscores should fail parsing to i64"
        );
    }

    #[test]
    fn deserialize_i64_from_string_with_leading_zeros() {
        let json = r#"{"val": "000123"}"#;
        let expected = TestI64 { val: 123 };
        // std::primitive::i64.from_str() supports leading zeros
        assert_eq!(
            serde_json::from_str::<TestI64>(json).unwrap(),
            expected,
            "String with leading zeros should parse"
        );
    }

    #[test]
    fn deserialize_i64_from_string_with_leading_zeros_negative() {
        let json = r#"{"val": "-000123"}"#;
        let expected = TestI64 { val: -123 };
        assert_eq!(
            serde_json::from_str::<TestI64>(json).unwrap(),
            expected,
            "Negative string with leading zeros should parse"
        );
    }

    #[test]
    fn deserialize_i64_error_from_string_with_decimal_zeros() {
        let json = r#"{"val": "123.000"}"#;
        // std::primitive::i64.from_str() does not support decimals
        assert!(
            serde_json::from_str::<TestI64>(json).is_err(),
            "String with decimal part should fail parsing to i64"
        );
    }

    #[test]
    fn deserialize_i64_error_from_string_with_internal_decimal() {
        let json = r#"{"val": "12.345"}"#;
        assert!(
            serde_json::from_str::<TestI64>(json).is_err(),
            "String with internal decimal point should fail"
        );
    }

    #[test]
    fn deserialize_i64_error_from_localized_string_commas() {
        let json = r#"{"val": "1,234"}"#;
        // std::primitive::i64.from_str() does not support commas
        assert!(
            serde_json::from_str::<TestI64>(json).is_err(),
            "Localized string with commas should fail parsing to i64"
        );
    }

    // --- u64 Deserialization Tests ---
    #[test]
    fn deserialize_u64_from_json_reader() {
        let json = r#"{"val": "123"}"#;
        let expected = TestU64 { val: 123 };
        assert_eq!(
            serde_json::from_reader::<_, TestU64>(Cursor::new(json)).unwrap(),
            expected
        );
    }

    #[test]
    fn deserialize_u64_from_json_number() {
        let json = r#"{"val": 123}"#;
        let expected = TestU64 { val: 123 };
        assert_eq!(serde_json::from_str::<TestU64>(json).unwrap(), expected);
    }

    #[test]
    fn deserialize_u64_from_json_number_zero() {
        let json = r#"{"val": 0}"#;
        let expected = TestU64 { val: 0 };
        assert_eq!(serde_json::from_str::<TestU64>(json).unwrap(), expected);
    }

    #[test]
    fn deserialize_u64_from_json_number_max() {
        let json = format!(r#"{{"val": {}}}"#, u64::MAX);
        let expected = TestU64 { val: u64::MAX };
        assert_eq!(serde_json::from_str::<TestU64>(&json).unwrap(), expected);
    }

    #[test]
    fn deserialize_u64_from_json_string() {
        let json = r#"{"val": "789"}"#;
        let expected = TestU64 { val: 789 };
        assert_eq!(serde_json::from_str::<TestU64>(json).unwrap(), expected);
    }

    #[test]
    fn deserialize_u64_from_json_string_zero() {
        let json = r#"{"val": "0"}"#;
        let expected = TestU64 { val: 0 };
        assert_eq!(serde_json::from_str::<TestU64>(json).unwrap(), expected);
    }

    #[test]
    fn deserialize_u64_from_json_string_max() {
        let json = format!(r#"{{"val": "{}"}}"#, u64::MAX);
        let expected = TestU64 { val: u64::MAX };
        assert_eq!(serde_json::from_str::<TestU64>(&json).unwrap(), expected);
    }

    #[test]
    fn deserialize_u64_from_json_string_with_plus_prefix() {
        let json = r#"{"val": "+123"}"#;
        let expected = TestU64 { val: 123 };
        assert_eq!(serde_json::from_str::<TestU64>(json).unwrap(), expected);
    }

    #[test]
    fn deserialize_u64_from_json_string_with_plus_zero() {
        let json = r#"{"val": "+0"}"#;
        let expected = TestU64 { val: 0 };
        assert_eq!(serde_json::from_str::<TestU64>(json).unwrap(), expected);
    }

    #[test]
    fn deserialize_u64_error_from_json_string_with_leading_whitespace() {
        let json = r#"{"val": " 123"}"#;
        assert!(serde_json::from_str::<TestU64>(json).is_err());
    }

    #[test]
    fn deserialize_u64_error_from_json_string_with_trailing_whitespace() {
        let json = r#"{"val": "123 "}"#;
        assert!(serde_json::from_str::<TestU64>(json).is_err());
    }

    #[test]
    fn deserialize_u64_error_from_json_string_with_invalid_plus_prefix() {
        let json = r#"{"val": "++123"}"#;
        assert!(serde_json::from_str::<TestU64>(json).is_err());
    }

    #[test]
    fn deserialize_u64_error_from_string_negative() {
        let json = r#"{"val": "-123"}"#; // Negative not allowed for u64 string parse
        assert!(serde_json::from_str::<TestU64>(json).is_err());
    }

    #[test]
    fn deserialize_u64_error_from_json_number_negative() {
        let json = r#"{"val": -1}"#; // Negative not allowed for u64
        assert!(serde_json::from_str::<TestU64>(json).is_err());
    }

    #[test]
    fn deserialize_u64_error_from_string_not_a_number() {
        let json = r#"{"val": "abc"}"#;
        assert!(serde_json::from_str::<TestU64>(json).is_err());
    }

    #[test]
    fn deserialize_u64_error_from_string_float() {
        let json = r#"{"val": "123.45"}"#;
        assert!(serde_json::from_str::<TestU64>(json).is_err());
    }

    #[test]
    fn deserialize_u64_error_from_string_empty() {
        let json = r#"{"val": ""}"#;
        assert!(serde_json::from_str::<TestU64>(json).is_err());
    }

    #[test]
    fn deserialize_u64_error_from_string_overflow() {
        let overflow_val = u128::from(u64::MAX) + 1;
        let json = format!(r#"{{"val": "{overflow_val}"}}"#);
        assert!(serde_json::from_str::<TestU64>(&json).is_err());
    }

    #[test]
    fn deserialize_u64_error_from_json_float_number() {
        let json = r#"{"val": 123.45}"#;
        assert!(serde_json::from_str::<TestU64>(json).is_err());
    }

    #[test]
    fn deserialize_u64_error_from_json_bool_true() {
        let json = r#"{"val": true}"#;
        assert!(serde_json::from_str::<TestU64>(json).is_err());
    }

    #[test]
    fn deserialize_u64_error_from_json_array() {
        let json = r#"{"val": []}"#;
        assert!(serde_json::from_str::<TestU64>(json).is_err());
    }

    #[test]
    fn deserialize_u64_error_from_json_object() {
        let json = r#"{"val": {}}"#;
        assert!(serde_json::from_str::<TestU64>(json).is_err());
    }

    #[test]
    fn deserialize_u64_error_from_json_null() {
        let json = r#"{"val": null}"#;
        assert!(serde_json::from_str::<TestU64>(json).is_err());
    }

    // -- Additional u64 String Format Tests --
    #[test]
    fn deserialize_u64_error_from_hex_string() {
        let json = r#"{"val": "0x1A"}"#; // Hex "26"
        assert!(
            serde_json::from_str::<TestU64>(json).is_err(),
            "Hex string should fail parsing to u64"
        );
    }

    #[test]
    fn deserialize_u64_error_from_octal_string() {
        let json = r#"{"val": "0o77"}"#; // Octal "63"
        assert!(
            serde_json::from_str::<TestU64>(json).is_err(),
            "Octal string should fail parsing to u64"
        );
    }

    #[test]
    fn deserialize_u64_error_from_scientific_notation_string() {
        let json = r#"{"val": "1e3"}"#;
        assert!(
            serde_json::from_str::<TestU64>(json).is_err(),
            "Scientific notation string should fail parsing to u64"
        );
    }

    #[test]
    fn deserialize_u64_error_from_string_with_underscores() {
        let json = r#"{"val": "1_000_000"}"#;
        assert!(
            serde_json::from_str::<TestU64>(json).is_err(),
            "String with underscores should fail parsing to u64"
        );
    }

    #[test]
    fn deserialize_u64_from_string_with_leading_zeros() {
        let json = r#"{"val": "000123"}"#;
        let expected = TestU64 { val: 123 };
        assert_eq!(
            serde_json::from_str::<TestU64>(json).unwrap(),
            expected,
            "String with leading zeros should parse to u64"
        );
    }

    #[test]
    fn deserialize_u64_error_from_string_with_decimal_zeros() {
        let json = r#"{"val": "123.000"}"#;
        assert!(
            serde_json::from_str::<TestU64>(json).is_err(),
            "String with decimal part should fail parsing to u64"
        );
    }

    #[test]
    fn deserialize_u64_error_from_localized_string_commas() {
        let json = r#"{"val": "1,234"}"#;
        assert!(
            serde_json::from_str::<TestU64>(json).is_err(),
            "Localized string with commas should fail parsing to u64"
        );
    }

    // --- i64 Serialization Tests ---
    #[test]
    fn serialize_i64_positive() {
        let data = TestI64 { val: 123 };
        let expected_json = r#"{"val":"123"}"#;
        assert_eq!(serde_json::to_string(&data).unwrap(), expected_json);
    }

    #[test]
    fn serialize_i64_negative() {
        let data = TestI64 { val: -456 };
        let expected_json = r#"{"val":"-456"}"#;
        assert_eq!(serde_json::to_string(&data).unwrap(), expected_json);
    }

    #[test]
    fn serialize_i64_zero() {
        let data = TestI64 { val: 0 };
        let expected_json = r#"{"val":"0"}"#;
        assert_eq!(serde_json::to_string(&data).unwrap(), expected_json);
    }

    #[test]
    fn serialize_i64_max() {
        let data = TestI64 { val: i64::MAX };
        let expected_json = format!(r#"{{"val":"{}"}}"#, i64::MAX);
        assert_eq!(serde_json::to_string(&data).unwrap(), expected_json);
    }

    #[test]
    fn serialize_i64_min() {
        let data = TestI64 { val: i64::MIN };
        let expected_json = format!(r#"{{"val":"{}"}}"#, i64::MIN);
        assert_eq!(serde_json::to_string(&data).unwrap(), expected_json);
    }

    // --- u64 Serialization Tests ---
    #[test]
    fn serialize_u64_positive() {
        let data = TestU64 { val: 789 };
        let expected_json = r#"{"val":"789"}"#;
        assert_eq!(serde_json::to_string(&data).unwrap(), expected_json);
    }

    #[test]
    fn serialize_u64_zero() {
        let data = TestU64 { val: 0 };
        let expected_json = r#"{"val":"0"}"#;
        assert_eq!(serde_json::to_string(&data).unwrap(), expected_json);
    }

    #[test]
    fn serialize_u64_max() {
        let data = TestU64 { val: u64::MAX };
        let expected_json = format!(r#"{{"val":"{}"}}"#, u64::MAX);
        assert_eq!(serde_json::to_string(&data).unwrap(), expected_json);
    }

    // --- Option<i64> Tests ---
    #[test]
    fn deserialize_option_i64_some_from_json_number() {
        let json = r#"{"val": 123}"#;
        let expected = TestOptionI64 { val: Some(123) };
        assert_eq!(
            serde_json::from_str::<TestOptionI64>(json).unwrap(),
            expected
        );
    }

    #[test]
    fn deserialize_option_i64_some_from_json_string() {
        let json = r#"{"val": "456"}"#;
        let expected = TestOptionI64 { val: Some(456) };
        assert_eq!(
            serde_json::from_str::<TestOptionI64>(json).unwrap(),
            expected
        );
    }

    #[test]
    fn deserialize_option_i64_none_from_json_null() {
        let json = r#"{"val": null}"#;
        let expected = TestOptionI64 { val: None };
        assert_eq!(
            serde_json::from_str::<TestOptionI64>(json).unwrap(),
            expected
        );
    }

    #[test]
    fn deserialize_option_i64_error_from_invalid_string() {
        let json = r#"{"val": "abc"}"#;
        assert!(serde_json::from_str::<TestOptionI64>(json).is_err());
    }

    #[test]
    fn deserialize_option_i64_error_from_invalid_type() {
        let json = r#"{"val": true}"#;
        assert!(serde_json::from_str::<TestOptionI64>(json).is_err());
    }

    #[test]
    fn serialize_option_i64_some() {
        let data = TestOptionI64 { val: Some(123) };
        let expected_json = r#"{"val":"123"}"#;
        assert_eq!(serde_json::to_string(&data).unwrap(), expected_json);
    }

    #[test]
    fn serialize_option_i64_none() {
        let data = TestOptionI64 { val: None };
        let expected_json = r#"{"val":null}"#;
        assert_eq!(serde_json::to_string(&data).unwrap(), expected_json);
    }

    // --- Option<u64> Tests ---
    #[test]
    fn deserialize_option_u64_some_from_json_number() {
        let json = r#"{"val": 123}"#;
        let expected = TestOptionU64 { val: Some(123) };
        assert_eq!(
            serde_json::from_str::<TestOptionU64>(json).unwrap(),
            expected
        );
    }

    #[test]
    fn deserialize_option_u64_some_from_json_string() {
        let json = r#"{"val": "456"}"#;
        let expected = TestOptionU64 { val: Some(456) };
        assert_eq!(
            serde_json::from_str::<TestOptionU64>(json).unwrap(),
            expected
        );
    }

    #[test]
    fn deserialize_option_u64_none_from_json_null() {
        let json = r#"{"val": null}"#;
        let expected = TestOptionU64 { val: None };
        assert_eq!(
            serde_json::from_str::<TestOptionU64>(json).unwrap(),
            expected
        );
    }

    #[test]
    fn deserialize_option_u64_error_from_invalid_string() {
        let json = r#"{"val": "abc"}"#;
        assert!(serde_json::from_str::<TestOptionU64>(json).is_err());
    }

    #[test]
    fn deserialize_option_u64_error_from_negative_string() {
        let json = r#"{"val": "-1"}"#; // Invalid for u64
        assert!(serde_json::from_str::<TestOptionU64>(json).is_err());
    }

    #[test]
    fn serialize_option_u64_some() {
        let data = TestOptionU64 { val: Some(123) };
        let expected_json = r#"{"val":"123"}"#;
        assert_eq!(serde_json::to_string(&data).unwrap(), expected_json);
    }

    #[test]
    fn serialize_option_u64_none() {
        let data = TestOptionU64 { val: None };
        let expected_json = r#"{"val":null}"#;
        assert_eq!(serde_json::to_string(&data).unwrap(), expected_json);
    }

    // --- Vec<i64> Tests ---
    #[test]
    fn deserialize_vec_i64_empty() {
        let json = r#"{"val": []}"#;
        let expected = TestVecI64 { val: vec![] };
        assert_eq!(serde_json::from_str::<TestVecI64>(json).unwrap(), expected);
    }

    #[test]
    fn deserialize_vec_i64_from_numbers_and_strings() {
        let json = r#"{"val": [1, "2", -3, "-4"]}"#;
        let expected = TestVecI64 {
            val: vec![1, 2, -3, -4],
        };
        assert_eq!(serde_json::from_str::<TestVecI64>(json).unwrap(), expected);
    }

    #[test]
    fn deserialize_vec_i64_error_if_item_is_invalid_string() {
        let json = r#"{"val": [1, "abc", 3]}"#;
        let err = serde_json::from_str::<TestVecI64>(json).unwrap_err();
        // The error will point to the specific failing element
        assert!(err.to_string().contains("invalid digit found in string")); // From parse error
    }

    #[test]
    fn deserialize_vec_i64_error_if_item_is_invalid_type() {
        let json = r#"{"val": [1, true, 3]}"#;
        assert!(serde_json::from_str::<TestVecI64>(json).is_err());
    }

    #[test]
    fn serialize_vec_i64_empty() {
        let data = TestVecI64 { val: vec![] };
        let expected_json = r#"{"val":[]}"#;
        assert_eq!(serde_json::to_string(&data).unwrap(), expected_json);
    }

    #[test]
    fn serialize_vec_i64_with_values() {
        let data = TestVecI64 {
            val: vec![1, -2, 0],
        };
        let expected_json = r#"{"val":["1","-2","0"]}"#;
        assert_eq!(serde_json::to_string(&data).unwrap(), expected_json);
    }

    // --- Vec<u64> Tests ---
    #[test]
    fn deserialize_vec_u64_empty() {
        let json = r#"{"val": []}"#;
        let expected = TestVecU64 { val: vec![] };
        assert_eq!(serde_json::from_str::<TestVecU64>(json).unwrap(), expected);
    }

    #[test]
    fn deserialize_vec_u64_from_numbers_and_strings() {
        let json = r#"{"val": [1, "2", 3, "4"]}"#;
        let expected = TestVecU64 {
            val: vec![1, 2, 3, 4],
        };
        assert_eq!(serde_json::from_str::<TestVecU64>(json).unwrap(), expected);
    }

    #[test]
    fn deserialize_vec_u64_error_if_item_is_invalid_string() {
        let json = r#"{"val": [1, "abc", 3]}"#;
        let err = serde_json::from_str::<TestVecU64>(json).unwrap_err();
        assert!(err.to_string().contains("invalid digit found in string"));
    }

    #[test]
    fn deserialize_vec_u64_error_if_item_is_negative_string() {
        let json = r#"{"val": [1, "-2", 3]}"#;
        let err = serde_json::from_str::<TestVecU64>(json).unwrap_err();
        assert!(err.to_string().contains("invalid digit found in string")); // u64 parse error
    }

    #[test]
    fn deserialize_vec_u64_error_if_item_is_negative_number() {
        let json = r#"{"val": [1, -2, 3]}"#;
        assert!(serde_json::from_str::<TestVecU64>(json).is_err());
    }

    #[test]
    fn serialize_vec_u64_empty() {
        let data = TestVecU64 { val: vec![] };
        let expected_json = r#"{"val":[]}"#;
        assert_eq!(serde_json::to_string(&data).unwrap(), expected_json);
    }

    #[test]
    fn serialize_vec_u64_with_values() {
        let data = TestVecU64 { val: vec![1, 2, 0] };
        let expected_json = r#"{"val":["1","2","0"]}"#;
        assert_eq!(serde_json::to_string(&data).unwrap(), expected_json);
    }

    // --- Enum with NumberOrString field Tests ---
    #[test]
    fn deserialize_enum_variant_a_with_number() {
        let json = r#"{"variantA": {"numVal": 123, "otherData": "test"}}"#;
        let expected = TestEnum::VariantA {
            num_val: 123,
            other_data: "test".to_string(),
        };
        assert_eq!(serde_json::from_str::<TestEnum>(json).unwrap(), expected);
    }

    #[test]
    fn deserialize_enum_variant_a_with_string_number() {
        let json = r#"{"variantA": {"numVal": "-45", "otherData": "data"}}"#;
        let expected = TestEnum::VariantA {
            num_val: -45,
            other_data: "data".to_string(),
        };
        assert_eq!(serde_json::from_str::<TestEnum>(json).unwrap(), expected);
    }

    #[test]
    fn deserialize_enum_variant_b_with_number() {
        let json = r#"{"variantB": {"count": 7890}}"#;
        let expected = TestEnum::VariantB { count: 7890 };
        assert_eq!(serde_json::from_str::<TestEnum>(json).unwrap(), expected);
    }

    #[test]
    fn deserialize_enum_variant_b_with_string_number() {
        let json = r#"{"variantB": {"count": "1234567890"}}"#;
        let expected = TestEnum::VariantB { count: 1234567890 };
        assert_eq!(serde_json::from_str::<TestEnum>(json).unwrap(), expected);
    }

    #[test]
    fn deserialize_enum_variant_a_error_invalid_num_string() {
        let json = r#"{"variantA": {"numVal": "abc", "otherData": "test"}}"#;
        assert!(serde_json::from_str::<TestEnum>(json).is_err());
    }

    #[test]
    fn serialize_enum_variant_a() {
        let data = TestEnum::VariantA {
            num_val: 123,
            other_data: "test".to_string(),
        };
        // Note: num_val will be serialized as a string by NumberOrString
        let expected_json = r#"{"variantA":{"numVal":"123","otherData":"test"}}"#;
        assert_eq!(serde_json::to_string(&data).unwrap(), expected_json);
    }

    #[test]
    fn serialize_enum_variant_b() {
        let data = TestEnum::VariantB { count: 7890 };
        let expected_json = r#"{"variantB":{"count":"7890"}}"#;
        assert_eq!(serde_json::to_string(&data).unwrap(), expected_json);
    }
}

mod value;
#[allow(unused_imports)]
pub use value::*;
mod scp_ballot;
#[allow(unused_imports)]
pub use scp_ballot::*;
mod scp_statement_type;
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
#[allow(unused_imports)]
pub use config_setting_contract_execution_lanes_v0::*;
mod config_setting_contract_compute_v0;
#[allow(unused_imports)]
pub use config_setting_contract_compute_v0::*;
mod config_setting_contract_parallel_compute_v0;
#[allow(unused_imports)]
pub use config_setting_contract_parallel_compute_v0::*;
mod config_setting_contract_ledger_cost_v0;
#[allow(unused_imports)]
pub use config_setting_contract_ledger_cost_v0::*;
mod config_setting_contract_ledger_cost_ext_v0;
#[allow(unused_imports)]
pub use config_setting_contract_ledger_cost_ext_v0::*;
mod config_setting_contract_historical_data_v0;
#[allow(unused_imports)]
pub use config_setting_contract_historical_data_v0::*;
mod config_setting_contract_events_v0;
#[allow(unused_imports)]
pub use config_setting_contract_events_v0::*;
mod config_setting_contract_bandwidth_v0;
#[allow(unused_imports)]
pub use config_setting_contract_bandwidth_v0::*;
mod contract_cost_type;
#[allow(unused_imports)]
pub use contract_cost_type::*;
mod contract_cost_param_entry;
#[allow(unused_imports)]
pub use contract_cost_param_entry::*;
mod state_archival_settings;
#[allow(unused_imports)]
pub use state_archival_settings::*;
mod eviction_iterator;
#[allow(unused_imports)]
pub use eviction_iterator::*;
mod config_setting_scp_timing;
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
#[allow(unused_imports)]
pub use freeze_bypass_txs_delta::*;
mod contract_cost_count_limit;
#[allow(unused_imports)]
pub use contract_cost_count_limit::*;
mod contract_cost_params;
#[allow(unused_imports)]
pub use contract_cost_params::*;
mod config_setting_id;
#[allow(unused_imports)]
pub use config_setting_id::*;
mod config_setting_entry;
#[allow(unused_imports)]
pub use config_setting_entry::*;
mod sc_env_meta_kind;
#[allow(unused_imports)]
pub use sc_env_meta_kind::*;
mod sc_env_meta_entry_interface_version;
#[allow(unused_imports)]
pub use sc_env_meta_entry_interface_version::*;
mod sc_env_meta_entry;
#[allow(unused_imports)]
pub use sc_env_meta_entry::*;
mod sc_meta_v0;
#[allow(unused_imports)]
pub use sc_meta_v0::*;
mod sc_meta_kind;
#[allow(unused_imports)]
pub use sc_meta_kind::*;
mod sc_meta_entry;
#[allow(unused_imports)]
pub use sc_meta_entry::*;
mod sc_spec_doc_limit;
#[allow(unused_imports)]
pub use sc_spec_doc_limit::*;
mod sc_spec_type;
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
#[allow(unused_imports)]
pub use sc_spec_event_param_location_v0::*;
mod sc_spec_event_param_v0;
#[allow(unused_imports)]
pub use sc_spec_event_param_v0::*;
mod sc_spec_event_data_format;
#[allow(unused_imports)]
pub use sc_spec_event_data_format::*;
mod sc_spec_event_v0;
#[allow(unused_imports)]
pub use sc_spec_event_v0::*;
mod sc_spec_entry_kind;
#[allow(unused_imports)]
pub use sc_spec_entry_kind::*;
mod sc_spec_entry;
#[allow(unused_imports)]
pub use sc_spec_entry::*;
mod sc_val_type;
#[allow(unused_imports)]
pub use sc_val_type::*;
mod sc_error_type;
#[allow(unused_imports)]
pub use sc_error_type::*;
mod sc_error_code;
#[allow(unused_imports)]
pub use sc_error_code::*;
mod sc_error;
#[allow(unused_imports)]
pub use sc_error::*;
mod u_int128_parts;
#[allow(unused_imports)]
pub use u_int128_parts::*;
mod int128_parts;
#[allow(unused_imports)]
pub use int128_parts::*;
mod u_int256_parts;
#[allow(unused_imports)]
pub use u_int256_parts::*;
mod int256_parts;
#[allow(unused_imports)]
pub use int256_parts::*;
mod contract_executable_type;
#[allow(unused_imports)]
pub use contract_executable_type::*;
mod contract_executable;
#[allow(unused_imports)]
pub use contract_executable::*;
mod sc_address_type;
#[allow(unused_imports)]
pub use sc_address_type::*;
mod muxed_ed25519_account;
#[allow(unused_imports)]
pub use muxed_ed25519_account::*;
mod sc_address;
#[allow(unused_imports)]
pub use sc_address::*;
mod scsymbol_limit;
#[allow(unused_imports)]
pub use scsymbol_limit::*;
mod sc_vec;
#[allow(unused_imports)]
pub use sc_vec::*;
mod sc_map;
#[allow(unused_imports)]
pub use sc_map::*;
mod sc_bytes;
#[allow(unused_imports)]
pub use sc_bytes::*;
mod sc_string;
#[allow(unused_imports)]
pub use sc_string::*;
mod sc_symbol;
#[allow(unused_imports)]
pub use sc_symbol::*;
mod sc_nonce_key;
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
#[allow(unused_imports)]
pub use thresholds::*;
mod string32;
#[allow(unused_imports)]
pub use string32::*;
mod string64;
#[allow(unused_imports)]
pub use string64::*;
mod sequence_number;
#[allow(unused_imports)]
pub use sequence_number::*;
mod data_value;
#[allow(unused_imports)]
pub use data_value::*;
mod asset_code4;
#[allow(unused_imports)]
pub use asset_code4::*;
mod asset_code12;
#[allow(unused_imports)]
pub use asset_code12::*;
mod asset_type;
#[allow(unused_imports)]
pub use asset_type::*;
mod asset_code;
#[allow(unused_imports)]
pub use asset_code::*;
mod alpha_num4;
#[allow(unused_imports)]
pub use alpha_num4::*;
mod alpha_num12;
#[allow(unused_imports)]
pub use alpha_num12::*;
mod asset;
#[allow(unused_imports)]
pub use asset::*;
mod price;
#[allow(unused_imports)]
pub use price::*;
mod liabilities;
#[allow(unused_imports)]
pub use liabilities::*;
mod threshold_indexes;
#[allow(unused_imports)]
pub use threshold_indexes::*;
mod ledger_entry_type;
#[allow(unused_imports)]
pub use ledger_entry_type::*;
mod signer;
#[allow(unused_imports)]
pub use signer::*;
mod account_flags;
#[allow(unused_imports)]
pub use account_flags::*;
mod mask_account_flags;
#[allow(unused_imports)]
pub use mask_account_flags::*;
mod mask_account_flags_v17;
#[allow(unused_imports)]
pub use mask_account_flags_v17::*;
mod max_signers;
#[allow(unused_imports)]
pub use max_signers::*;
mod sponsorship_descriptor;
#[allow(unused_imports)]
pub use sponsorship_descriptor::*;
mod account_entry_extension_v3;
#[allow(unused_imports)]
pub use account_entry_extension_v3::*;
mod account_entry_extension_v2_ext;
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
#[allow(unused_imports)]
pub use trust_line_flags::*;
mod mask_trustline_flags;
#[allow(unused_imports)]
pub use mask_trustline_flags::*;
mod mask_trustline_flags_v13;
#[allow(unused_imports)]
pub use mask_trustline_flags_v13::*;
mod mask_trustline_flags_v17;
#[allow(unused_imports)]
pub use mask_trustline_flags_v17::*;
mod liquidity_pool_type;
#[allow(unused_imports)]
pub use liquidity_pool_type::*;
mod trust_line_asset;
#[allow(unused_imports)]
pub use trust_line_asset::*;
mod trust_line_entry_extension_v2_ext;
#[allow(unused_imports)]
pub use trust_line_entry_extension_v2_ext::*;
mod trust_line_entry_extension_v2;
#[allow(unused_imports)]
pub use trust_line_entry_extension_v2::*;
mod trust_line_entry_v1_ext;
#[allow(unused_imports)]
pub use trust_line_entry_v1_ext::*;
mod trust_line_entry_v1;
#[allow(unused_imports)]
pub use trust_line_entry_v1::*;
mod trust_line_entry_ext;
#[allow(unused_imports)]
pub use trust_line_entry_ext::*;
mod trust_line_entry;
#[allow(unused_imports)]
pub use trust_line_entry::*;
mod offer_entry_flags;
#[allow(unused_imports)]
pub use offer_entry_flags::*;
mod mask_offerentry_flags;
#[allow(unused_imports)]
pub use mask_offerentry_flags::*;
mod offer_entry_ext;
#[allow(unused_imports)]
pub use offer_entry_ext::*;
mod offer_entry;
#[allow(unused_imports)]
pub use offer_entry::*;
mod data_entry_ext;
#[allow(unused_imports)]
pub use data_entry_ext::*;
mod data_entry;
#[allow(unused_imports)]
pub use data_entry::*;
mod claim_predicate_type;
#[allow(unused_imports)]
pub use claim_predicate_type::*;
mod claim_predicate;
#[allow(unused_imports)]
pub use claim_predicate::*;
mod claimant_type;
#[allow(unused_imports)]
pub use claimant_type::*;
mod claimant_v0;
#[allow(unused_imports)]
pub use claimant_v0::*;
mod claimant;
#[allow(unused_imports)]
pub use claimant::*;
mod claimable_balance_flags;
#[allow(unused_imports)]
pub use claimable_balance_flags::*;
mod mask_claimable_balance_flags;
#[allow(unused_imports)]
pub use mask_claimable_balance_flags::*;
mod claimable_balance_entry_extension_v1_ext;
#[allow(unused_imports)]
pub use claimable_balance_entry_extension_v1_ext::*;
mod claimable_balance_entry_extension_v1;
#[allow(unused_imports)]
pub use claimable_balance_entry_extension_v1::*;
mod claimable_balance_entry_ext;
#[allow(unused_imports)]
pub use claimable_balance_entry_ext::*;
mod claimable_balance_entry;
#[allow(unused_imports)]
pub use claimable_balance_entry::*;
mod liquidity_pool_constant_product_parameters;
#[allow(unused_imports)]
pub use liquidity_pool_constant_product_parameters::*;
mod liquidity_pool_entry_constant_product;
#[allow(unused_imports)]
pub use liquidity_pool_entry_constant_product::*;
mod liquidity_pool_entry_body;
#[allow(unused_imports)]
pub use liquidity_pool_entry_body::*;
mod liquidity_pool_entry;
#[allow(unused_imports)]
pub use liquidity_pool_entry::*;
mod contract_data_durability;
#[allow(unused_imports)]
pub use contract_data_durability::*;
mod contract_data_entry;
#[allow(unused_imports)]
pub use contract_data_entry::*;
mod contract_code_cost_inputs;
#[allow(unused_imports)]
pub use contract_code_cost_inputs::*;
mod contract_code_entry_v1;
#[allow(unused_imports)]
pub use contract_code_entry_v1::*;
mod contract_code_entry_ext;
#[allow(unused_imports)]
pub use contract_code_entry_ext::*;
mod contract_code_entry;
#[allow(unused_imports)]
pub use contract_code_entry::*;
mod ttl_entry;
#[allow(unused_imports)]
pub use ttl_entry::*;
mod ledger_entry_extension_v1_ext;
#[allow(unused_imports)]
pub use ledger_entry_extension_v1_ext::*;
mod ledger_entry_extension_v1;
#[allow(unused_imports)]
pub use ledger_entry_extension_v1::*;
mod ledger_entry_data;
#[allow(unused_imports)]
pub use ledger_entry_data::*;
mod ledger_entry_ext;
#[allow(unused_imports)]
pub use ledger_entry_ext::*;
mod ledger_entry;
#[allow(unused_imports)]
pub use ledger_entry::*;
mod ledger_key_account;
#[allow(unused_imports)]
pub use ledger_key_account::*;
mod ledger_key_trust_line;
#[allow(unused_imports)]
pub use ledger_key_trust_line::*;
mod ledger_key_offer;
#[allow(unused_imports)]
pub use ledger_key_offer::*;
mod ledger_key_data;
#[allow(unused_imports)]
pub use ledger_key_data::*;
mod ledger_key_claimable_balance;
#[allow(unused_imports)]
pub use ledger_key_claimable_balance::*;
mod ledger_key_liquidity_pool;
#[allow(unused_imports)]
pub use ledger_key_liquidity_pool::*;
mod ledger_key_contract_data;
#[allow(unused_imports)]
pub use ledger_key_contract_data::*;
mod ledger_key_contract_code;
#[allow(unused_imports)]
pub use ledger_key_contract_code::*;
mod ledger_key_config_setting;
#[allow(unused_imports)]
pub use ledger_key_config_setting::*;
mod ledger_key_ttl;
#[allow(unused_imports)]
pub use ledger_key_ttl::*;
mod ledger_key;
#[allow(unused_imports)]
pub use ledger_key::*;
mod envelope_type;
#[allow(unused_imports)]
pub use envelope_type::*;
mod bucket_list_type;
#[allow(unused_imports)]
pub use bucket_list_type::*;
mod bucket_entry_type;
#[allow(unused_imports)]
pub use bucket_entry_type::*;
mod hot_archive_bucket_entry_type;
#[allow(unused_imports)]
pub use hot_archive_bucket_entry_type::*;
mod bucket_metadata_ext;
#[allow(unused_imports)]
pub use bucket_metadata_ext::*;
mod bucket_metadata;
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
#[allow(unused_imports)]
pub use stellar_value_type::*;
mod ledger_close_value_signature;
#[allow(unused_imports)]
pub use ledger_close_value_signature::*;
mod stellar_value_ext;
#[allow(unused_imports)]
pub use stellar_value_ext::*;
mod stellar_value;
#[allow(unused_imports)]
pub use stellar_value::*;
mod mask_ledger_header_flags;
#[allow(unused_imports)]
pub use mask_ledger_header_flags::*;
mod ledger_header_flags;
#[allow(unused_imports)]
pub use ledger_header_flags::*;
mod ledger_header_extension_v1_ext;
#[allow(unused_imports)]
pub use ledger_header_extension_v1_ext::*;
mod ledger_header_extension_v1;
#[allow(unused_imports)]
pub use ledger_header_extension_v1::*;
mod ledger_header_ext;
#[allow(unused_imports)]
pub use ledger_header_ext::*;
mod ledger_header;
#[allow(unused_imports)]
pub use ledger_header::*;
mod ledger_upgrade_type;
#[allow(unused_imports)]
pub use ledger_upgrade_type::*;
mod config_upgrade_set_key;
#[allow(unused_imports)]
pub use config_upgrade_set_key::*;
mod ledger_upgrade;
#[allow(unused_imports)]
pub use ledger_upgrade::*;
mod config_upgrade_set;
#[allow(unused_imports)]
pub use config_upgrade_set::*;
mod tx_set_component_type;
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
#[allow(unused_imports)]
pub use transaction_history_result_entry_ext::*;
mod transaction_history_result_entry;
#[allow(unused_imports)]
pub use transaction_history_result_entry::*;
mod ledger_header_history_entry_ext;
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
#[allow(unused_imports)]
pub use soroban_transaction_meta_ext_v1::*;
mod soroban_transaction_meta_ext;
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
#[allow(unused_imports)]
pub use ledger_close_meta_ext_v1::*;
mod ledger_close_meta_ext;
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
#[allow(unused_imports)]
pub use error_code::*;
mod s_error;
#[allow(unused_imports)]
pub use s_error::*;
mod send_more;
#[allow(unused_imports)]
pub use send_more::*;
mod send_more_extended;
#[allow(unused_imports)]
pub use send_more_extended::*;
mod auth_cert;
#[allow(unused_imports)]
pub use auth_cert::*;
mod hello;
#[allow(unused_imports)]
pub use hello::*;
mod auth_msg_flag_flow_control_bytes_requested;
#[allow(unused_imports)]
pub use auth_msg_flag_flow_control_bytes_requested::*;
mod auth;
#[allow(unused_imports)]
pub use auth::*;
mod ip_addr_type;
#[allow(unused_imports)]
pub use ip_addr_type::*;
mod peer_address_ip;
#[allow(unused_imports)]
pub use peer_address_ip::*;
mod peer_address;
#[allow(unused_imports)]
pub use peer_address::*;
mod message_type;
#[allow(unused_imports)]
pub use message_type::*;
mod dont_have;
#[allow(unused_imports)]
pub use dont_have::*;
mod survey_message_command_type;
#[allow(unused_imports)]
pub use survey_message_command_type::*;
mod survey_message_response_type;
#[allow(unused_imports)]
pub use survey_message_response_type::*;
mod time_sliced_survey_start_collecting_message;
#[allow(unused_imports)]
pub use time_sliced_survey_start_collecting_message::*;
mod signed_time_sliced_survey_start_collecting_message;
#[allow(unused_imports)]
pub use signed_time_sliced_survey_start_collecting_message::*;
mod time_sliced_survey_stop_collecting_message;
#[allow(unused_imports)]
pub use time_sliced_survey_stop_collecting_message::*;
mod signed_time_sliced_survey_stop_collecting_message;
#[allow(unused_imports)]
pub use signed_time_sliced_survey_stop_collecting_message::*;
mod survey_request_message;
#[allow(unused_imports)]
pub use survey_request_message::*;
mod time_sliced_survey_request_message;
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
#[allow(unused_imports)]
pub use survey_response_body::*;
mod tx_advert_vector_max_size;
#[allow(unused_imports)]
pub use tx_advert_vector_max_size::*;
mod tx_advert_vector;
#[allow(unused_imports)]
pub use tx_advert_vector::*;
mod flood_advert;
#[allow(unused_imports)]
pub use flood_advert::*;
mod tx_demand_vector_max_size;
#[allow(unused_imports)]
pub use tx_demand_vector_max_size::*;
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
#[allow(unused_imports)]
pub use authenticated_message::*;
mod max_ops_per_tx;
#[allow(unused_imports)]
pub use max_ops_per_tx::*;
mod liquidity_pool_parameters;
#[allow(unused_imports)]
pub use liquidity_pool_parameters::*;
mod muxed_account_med25519;
#[allow(unused_imports)]
pub use muxed_account_med25519::*;
mod muxed_account;
#[allow(unused_imports)]
pub use muxed_account::*;
mod decorated_signature;
#[allow(unused_imports)]
pub use decorated_signature::*;
mod operation_type;
#[allow(unused_imports)]
pub use operation_type::*;
mod create_account_op;
#[allow(unused_imports)]
pub use create_account_op::*;
mod payment_op;
#[allow(unused_imports)]
pub use payment_op::*;
mod path_payment_strict_receive_op;
#[allow(unused_imports)]
pub use path_payment_strict_receive_op::*;
mod path_payment_strict_send_op;
#[allow(unused_imports)]
pub use path_payment_strict_send_op::*;
mod manage_sell_offer_op;
#[allow(unused_imports)]
pub use manage_sell_offer_op::*;
mod manage_buy_offer_op;
#[allow(unused_imports)]
pub use manage_buy_offer_op::*;
mod create_passive_sell_offer_op;
#[allow(unused_imports)]
pub use create_passive_sell_offer_op::*;
mod set_options_op;
#[allow(unused_imports)]
pub use set_options_op::*;
mod change_trust_asset;
#[allow(unused_imports)]
pub use change_trust_asset::*;
mod change_trust_op;
#[allow(unused_imports)]
pub use change_trust_op::*;
mod allow_trust_op;
#[allow(unused_imports)]
pub use allow_trust_op::*;
mod manage_data_op;
#[allow(unused_imports)]
pub use manage_data_op::*;
mod bump_sequence_op;
#[allow(unused_imports)]
pub use bump_sequence_op::*;
mod create_claimable_balance_op;
#[allow(unused_imports)]
pub use create_claimable_balance_op::*;
mod claim_claimable_balance_op;
#[allow(unused_imports)]
pub use claim_claimable_balance_op::*;
mod begin_sponsoring_future_reserves_op;
#[allow(unused_imports)]
pub use begin_sponsoring_future_reserves_op::*;
mod revoke_sponsorship_type;
#[allow(unused_imports)]
pub use revoke_sponsorship_type::*;
mod revoke_sponsorship_op_signer;
#[allow(unused_imports)]
pub use revoke_sponsorship_op_signer::*;
mod revoke_sponsorship_op;
#[allow(unused_imports)]
pub use revoke_sponsorship_op::*;
mod clawback_op;
#[allow(unused_imports)]
pub use clawback_op::*;
mod clawback_claimable_balance_op;
#[allow(unused_imports)]
pub use clawback_claimable_balance_op::*;
mod set_trust_line_flags_op;
#[allow(unused_imports)]
pub use set_trust_line_flags_op::*;
mod liquidity_pool_fee_v18;
#[allow(unused_imports)]
pub use liquidity_pool_fee_v18::*;
mod liquidity_pool_deposit_op;
#[allow(unused_imports)]
pub use liquidity_pool_deposit_op::*;
mod liquidity_pool_withdraw_op;
#[allow(unused_imports)]
pub use liquidity_pool_withdraw_op::*;
mod host_function_type;
#[allow(unused_imports)]
pub use host_function_type::*;
mod contract_id_preimage_type;
#[allow(unused_imports)]
pub use contract_id_preimage_type::*;
mod contract_id_preimage_from_address;
#[allow(unused_imports)]
pub use contract_id_preimage_from_address::*;
mod contract_id_preimage;
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
mod soroban_credentials_type;
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
#[allow(unused_imports)]
pub use extend_footprint_ttl_op::*;
mod restore_footprint_op;
#[allow(unused_imports)]
pub use restore_footprint_op::*;
mod operation_body;
#[allow(unused_imports)]
pub use operation_body::*;
mod operation;
#[allow(unused_imports)]
pub use operation::*;
mod hash_id_preimage_operation_id;
#[allow(unused_imports)]
pub use hash_id_preimage_operation_id::*;
mod hash_id_preimage_revoke_id;
#[allow(unused_imports)]
pub use hash_id_preimage_revoke_id::*;
mod hash_id_preimage_contract_id;
#[allow(unused_imports)]
pub use hash_id_preimage_contract_id::*;
mod hash_id_preimage_soroban_authorization;
#[allow(unused_imports)]
pub use hash_id_preimage_soroban_authorization::*;
mod hash_id_preimage;
#[allow(unused_imports)]
pub use hash_id_preimage::*;
mod memo_type;
#[allow(unused_imports)]
pub use memo_type::*;
mod memo;
#[allow(unused_imports)]
pub use memo::*;
mod time_bounds;
#[allow(unused_imports)]
pub use time_bounds::*;
mod ledger_bounds;
#[allow(unused_imports)]
pub use ledger_bounds::*;
mod preconditions_v2;
#[allow(unused_imports)]
pub use preconditions_v2::*;
mod precondition_type;
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
#[allow(unused_imports)]
pub use claim_atom_type::*;
mod claim_offer_atom_v0;
#[allow(unused_imports)]
pub use claim_offer_atom_v0::*;
mod claim_offer_atom;
#[allow(unused_imports)]
pub use claim_offer_atom::*;
mod claim_liquidity_atom;
#[allow(unused_imports)]
pub use claim_liquidity_atom::*;
mod claim_atom;
#[allow(unused_imports)]
pub use claim_atom::*;
mod create_account_result_code;
#[allow(unused_imports)]
pub use create_account_result_code::*;
mod create_account_result;
#[allow(unused_imports)]
pub use create_account_result::*;
mod payment_result_code;
#[allow(unused_imports)]
pub use payment_result_code::*;
mod payment_result;
#[allow(unused_imports)]
pub use payment_result::*;
mod path_payment_strict_receive_result_code;
#[allow(unused_imports)]
pub use path_payment_strict_receive_result_code::*;
mod simple_payment_result;
#[allow(unused_imports)]
pub use simple_payment_result::*;
mod path_payment_strict_receive_result_success;
#[allow(unused_imports)]
pub use path_payment_strict_receive_result_success::*;
mod path_payment_strict_receive_result;
#[allow(unused_imports)]
pub use path_payment_strict_receive_result::*;
mod path_payment_strict_send_result_code;
#[allow(unused_imports)]
pub use path_payment_strict_send_result_code::*;
mod path_payment_strict_send_result_success;
#[allow(unused_imports)]
pub use path_payment_strict_send_result_success::*;
mod path_payment_strict_send_result;
#[allow(unused_imports)]
pub use path_payment_strict_send_result::*;
mod manage_sell_offer_result_code;
#[allow(unused_imports)]
pub use manage_sell_offer_result_code::*;
mod manage_offer_effect;
#[allow(unused_imports)]
pub use manage_offer_effect::*;
mod manage_offer_success_result_offer;
#[allow(unused_imports)]
pub use manage_offer_success_result_offer::*;
mod manage_offer_success_result;
#[allow(unused_imports)]
pub use manage_offer_success_result::*;
mod manage_sell_offer_result;
#[allow(unused_imports)]
pub use manage_sell_offer_result::*;
mod manage_buy_offer_result_code;
#[allow(unused_imports)]
pub use manage_buy_offer_result_code::*;
mod manage_buy_offer_result;
#[allow(unused_imports)]
pub use manage_buy_offer_result::*;
mod set_options_result_code;
#[allow(unused_imports)]
pub use set_options_result_code::*;
mod set_options_result;
#[allow(unused_imports)]
pub use set_options_result::*;
mod change_trust_result_code;
#[allow(unused_imports)]
pub use change_trust_result_code::*;
mod change_trust_result;
#[allow(unused_imports)]
pub use change_trust_result::*;
mod allow_trust_result_code;
#[allow(unused_imports)]
pub use allow_trust_result_code::*;
mod allow_trust_result;
#[allow(unused_imports)]
pub use allow_trust_result::*;
mod account_merge_result_code;
#[allow(unused_imports)]
pub use account_merge_result_code::*;
mod account_merge_result;
#[allow(unused_imports)]
pub use account_merge_result::*;
mod inflation_result_code;
#[allow(unused_imports)]
pub use inflation_result_code::*;
mod inflation_payout;
#[allow(unused_imports)]
pub use inflation_payout::*;
mod inflation_result;
#[allow(unused_imports)]
pub use inflation_result::*;
mod manage_data_result_code;
#[allow(unused_imports)]
pub use manage_data_result_code::*;
mod manage_data_result;
#[allow(unused_imports)]
pub use manage_data_result::*;
mod bump_sequence_result_code;
#[allow(unused_imports)]
pub use bump_sequence_result_code::*;
mod bump_sequence_result;
#[allow(unused_imports)]
pub use bump_sequence_result::*;
mod create_claimable_balance_result_code;
#[allow(unused_imports)]
pub use create_claimable_balance_result_code::*;
mod create_claimable_balance_result;
#[allow(unused_imports)]
pub use create_claimable_balance_result::*;
mod claim_claimable_balance_result_code;
#[allow(unused_imports)]
pub use claim_claimable_balance_result_code::*;
mod claim_claimable_balance_result;
#[allow(unused_imports)]
pub use claim_claimable_balance_result::*;
mod begin_sponsoring_future_reserves_result_code;
#[allow(unused_imports)]
pub use begin_sponsoring_future_reserves_result_code::*;
mod begin_sponsoring_future_reserves_result;
#[allow(unused_imports)]
pub use begin_sponsoring_future_reserves_result::*;
mod end_sponsoring_future_reserves_result_code;
#[allow(unused_imports)]
pub use end_sponsoring_future_reserves_result_code::*;
mod end_sponsoring_future_reserves_result;
#[allow(unused_imports)]
pub use end_sponsoring_future_reserves_result::*;
mod revoke_sponsorship_result_code;
#[allow(unused_imports)]
pub use revoke_sponsorship_result_code::*;
mod revoke_sponsorship_result;
#[allow(unused_imports)]
pub use revoke_sponsorship_result::*;
mod clawback_result_code;
#[allow(unused_imports)]
pub use clawback_result_code::*;
mod clawback_result;
#[allow(unused_imports)]
pub use clawback_result::*;
mod clawback_claimable_balance_result_code;
#[allow(unused_imports)]
pub use clawback_claimable_balance_result_code::*;
mod clawback_claimable_balance_result;
#[allow(unused_imports)]
pub use clawback_claimable_balance_result::*;
mod set_trust_line_flags_result_code;
#[allow(unused_imports)]
pub use set_trust_line_flags_result_code::*;
mod set_trust_line_flags_result;
#[allow(unused_imports)]
pub use set_trust_line_flags_result::*;
mod liquidity_pool_deposit_result_code;
#[allow(unused_imports)]
pub use liquidity_pool_deposit_result_code::*;
mod liquidity_pool_deposit_result;
#[allow(unused_imports)]
pub use liquidity_pool_deposit_result::*;
mod liquidity_pool_withdraw_result_code;
#[allow(unused_imports)]
pub use liquidity_pool_withdraw_result_code::*;
mod liquidity_pool_withdraw_result;
#[allow(unused_imports)]
pub use liquidity_pool_withdraw_result::*;
mod invoke_host_function_result_code;
#[allow(unused_imports)]
pub use invoke_host_function_result_code::*;
mod invoke_host_function_result;
#[allow(unused_imports)]
pub use invoke_host_function_result::*;
mod extend_footprint_ttl_result_code;
#[allow(unused_imports)]
pub use extend_footprint_ttl_result_code::*;
mod extend_footprint_ttl_result;
#[allow(unused_imports)]
pub use extend_footprint_ttl_result::*;
mod restore_footprint_result_code;
#[allow(unused_imports)]
pub use restore_footprint_result_code::*;
mod restore_footprint_result;
#[allow(unused_imports)]
pub use restore_footprint_result::*;
mod operation_result_code;
#[allow(unused_imports)]
pub use operation_result_code::*;
mod operation_result_tr;
#[allow(unused_imports)]
pub use operation_result_tr::*;
mod operation_result;
#[allow(unused_imports)]
pub use operation_result::*;
mod transaction_result_code;
#[allow(unused_imports)]
pub use transaction_result_code::*;
mod inner_transaction_result_result;
#[allow(unused_imports)]
pub use inner_transaction_result_result::*;
mod inner_transaction_result_ext;
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
#[allow(unused_imports)]
pub use transaction_result_ext::*;
mod transaction_result;
#[allow(unused_imports)]
pub use transaction_result::*;
mod hash;
#[allow(unused_imports)]
pub use hash::*;
mod uint256;
#[allow(unused_imports)]
pub use uint256::*;
mod uint32;
#[allow(unused_imports)]
pub use uint32::*;
mod int32;
#[allow(unused_imports)]
pub use int32::*;
mod uint64;
#[allow(unused_imports)]
pub use uint64::*;
mod int64;
#[allow(unused_imports)]
pub use int64::*;
mod time_point;
#[allow(unused_imports)]
pub use time_point::*;
mod duration;
#[allow(unused_imports)]
pub use duration::*;
mod extension_point;
#[allow(unused_imports)]
pub use extension_point::*;
mod crypto_key_type;
#[allow(unused_imports)]
pub use crypto_key_type::*;
mod public_key_type;
#[allow(unused_imports)]
pub use public_key_type::*;
mod signer_key_type;
#[allow(unused_imports)]
pub use signer_key_type::*;
mod public_key;
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
#[allow(unused_imports)]
pub use signature_hint::*;
mod node_id;
#[allow(unused_imports)]
pub use node_id::*;
mod account_id;
#[allow(unused_imports)]
pub use account_id::*;
mod contract_id;
#[allow(unused_imports)]
pub use contract_id::*;
mod curve25519_secret;
#[allow(unused_imports)]
pub use curve25519_secret::*;
mod curve25519_public;
#[allow(unused_imports)]
pub use curve25519_public::*;
mod hmac_sha256_key;
#[allow(unused_imports)]
pub use hmac_sha256_key::*;
mod hmac_sha256_mac;
#[allow(unused_imports)]
pub use hmac_sha256_mac::*;
mod short_hash_seed;
#[allow(unused_imports)]
pub use short_hash_seed::*;
mod binary_fuse_filter_type;
#[allow(unused_imports)]
pub use binary_fuse_filter_type::*;
mod serialized_binary_fuse_filter;
#[allow(unused_imports)]
pub use serialized_binary_fuse_filter::*;
mod pool_id;
#[allow(unused_imports)]
pub use pool_id::*;
mod claimable_balance_id_type;
#[allow(unused_imports)]
pub use claimable_balance_id_type::*;
mod claimable_balance_id;
#[allow(unused_imports)]
pub use claimable_balance_id::*;
mod test_next_type;
#[allow(unused_imports)]
pub use test_next_type::*;
mod type_enum;
#[allow(unused_imports)]
pub use type_enum::*;
