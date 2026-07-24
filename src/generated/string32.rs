#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// String32 is an XDR Typedef defined as:
///
/// ```text
/// typedef string string32<32>;
/// ```
///
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval)]
#[derive(Default, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    serde_with::serde_as,
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Debug)]
pub struct String32(pub StringM<32>);

impl From<String32> for StringM<32> {
    #[must_use]
    fn from(x: String32) -> Self {
        x.0
    }
}

impl From<StringM<32>> for String32 {
    #[must_use]
    fn from(x: StringM<32>) -> Self {
        String32(x)
    }
}

impl AsRef<StringM<32>> for String32 {
    #[must_use]
    fn as_ref(&self) -> &StringM<32> {
        &self.0
    }
}

impl ReadXdr for String32 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let i = StringM::<32>::read_xdr(r)?;
            let v = String32(i);
            Ok(v)
        })
    }
}

impl String32 {
    /// Serialize this value as XDR into a [`ConstWriter`] using only const
    /// operations. This is the const implementation underlying `to_xdr`.
    #[cfg(feature = "std")]
    pub const fn const_write_xdr(&self, w: &mut ConstWriter) {
        w.enter_depth();
        w.write_len_prefixed(self.0 .0.as_slice());
        w.leave_depth();
    }
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::to_xdr_array`] at compile time.
    #[cfg(feature = "std")]
    #[must_use]
    pub const fn xdr_len(&self) -> usize {
        let limits = Limits {
            depth: u32::MAX,
            len: usize::MAX,
        };
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty, &limits);
        self.const_write_xdr(&mut w);
        w.position()
    }

    /// Serialize this value as XDR into a fixed-size `[u8; N]` using only const
    /// operations.
    ///
    /// `N` must equal [`Self::xdr_len`]. It is intended for callers, such as a
    /// proc-macro, that compute the length with `xdr_len` and pass it as `N`;
    /// `to_xdr_array` itself does not need to call `xdr_len`.
    ///
    /// # Panics
    ///
    /// Panics if `N` does not equal the value's [`Self::xdr_len`].
    #[cfg(feature = "std")]
    #[must_use]
    pub const fn to_xdr_array<const N: usize>(&self) -> [u8; N] {
        let limits = Limits {
            depth: u32::MAX,
            len: usize::MAX,
        };
        let mut buf = [0u8; N];
        let mut w = ConstWriter::new(&mut buf, &limits);
        self.const_write_xdr(&mut w);
        assert!(
            w.position() == N,
            "to_xdr_array: N does not equal the XDR-encoded length"
        );
        buf
    }
}

impl WriteXdr for String32 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| self.0.write_xdr(w))
    }

    #[cfg(feature = "std")]
    fn to_xdr(&self, limits: Limits) -> Result<Vec<u8>, Error> {
        to_xdr_via_const(self, &limits, Self::const_write_xdr)
    }
}

impl Deref for String32 {
    type Target = StringM<32>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<String32> for Vec<u8> {
    #[must_use]
    fn from(x: String32) -> Self {
        x.0 .0
    }
}

impl TryFrom<Vec<u8>> for String32 {
    type Error = Error;
    fn try_from(x: Vec<u8>) -> Result<Self, Error> {
        Ok(String32(x.try_into()?))
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<&Vec<u8>> for String32 {
    type Error = Error;
    fn try_from(x: &Vec<u8>) -> Result<Self, Error> {
        Ok(String32(x.try_into()?))
    }
}

impl AsRef<Vec<u8>> for String32 {
    #[must_use]
    fn as_ref(&self) -> &Vec<u8> {
        &self.0 .0
    }
}

impl AsRef<[u8]> for String32 {
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
