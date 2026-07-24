#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ScBytes is an XDR Typedef defined as:
///
/// ```text
/// typedef opaque SCBytes<>;
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
pub struct ScBytes(pub BytesM);

impl From<ScBytes> for BytesM {
    #[must_use]
    fn from(x: ScBytes) -> Self {
        x.0
    }
}

impl From<BytesM> for ScBytes {
    #[must_use]
    fn from(x: BytesM) -> Self {
        ScBytes(x)
    }
}

impl AsRef<BytesM> for ScBytes {
    #[must_use]
    fn as_ref(&self) -> &BytesM {
        &self.0
    }
}

impl ReadXdr for ScBytes {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let i = BytesM::read_xdr(r)?;
            let v = ScBytes(i);
            Ok(v)
        })
    }
}

impl ScBytes {
    /// Serialize this value as XDR into a [`ConstWriter`] using only const
    /// operations. This is the const implementation underlying `to_xdr`.
    #[cfg(feature = "std")]
    pub const fn const_to_xdr(&self, w: &mut ConstWriter) {
        w.enter_depth();
        w.write_len_prefixed(self.0 .0.as_slice());
        w.leave_depth();
    }
}

impl WriteXdr for ScBytes {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| self.0.write_xdr(w))
    }

    #[cfg(feature = "std")]
    fn to_xdr(&self, limits: Limits) -> Result<Vec<u8>, Error> {
        to_xdr_via_const(self, &limits, Self::const_to_xdr)
    }
}

impl Deref for ScBytes {
    type Target = BytesM;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<ScBytes> for Vec<u8> {
    #[must_use]
    fn from(x: ScBytes) -> Self {
        x.0 .0
    }
}

impl TryFrom<Vec<u8>> for ScBytes {
    type Error = Error;
    fn try_from(x: Vec<u8>) -> Result<Self, Error> {
        Ok(ScBytes(x.try_into()?))
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<&Vec<u8>> for ScBytes {
    type Error = Error;
    fn try_from(x: &Vec<u8>) -> Result<Self, Error> {
        Ok(ScBytes(x.try_into()?))
    }
}

impl AsRef<Vec<u8>> for ScBytes {
    #[must_use]
    fn as_ref(&self) -> &Vec<u8> {
        &self.0 .0
    }
}

impl AsRef<[u8]> for ScBytes {
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
