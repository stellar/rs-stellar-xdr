#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// AssetCode4 is an XDR Typedef defined as:
///
/// ```text
/// typedef opaque AssetCode4[4];
/// ```
///
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval)]
#[cfg_attr(feature = "alloc", derive(Default))]
#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde_with::SerializeDisplay, serde_with::DeserializeFromStr)
)]
pub struct AssetCode4(pub [u8; 4]);

impl core::fmt::Debug for AssetCode4 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let v = &self.0;
        write!(f, "AssetCode4(")?;
        for b in v {
            write!(f, "{b:02x}")?;
        }
        write!(f, ")")?;
        Ok(())
    }
}
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
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let i = <[u8; 4]>::read_xdr(r)?;
            let v = AssetCode4(i);
            Ok(v)
        })
    }
}

impl AssetCode4 {
    /// Serialize this value as XDR into a [`ConstWriter`] using only const
    /// operations. This is the const implementation underlying `to_xdr`.
    #[cfg(feature = "std")]
    pub const fn const_write_xdr(&self, w: &mut ConstWriter) {
        w.enter_depth();
        w.write_fixed_opaque(&self.0);
        w.leave_depth();
    }
}

impl WriteXdr for AssetCode4 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| self.0.write_xdr(w))
    }

    #[cfg(feature = "std")]
    fn to_xdr(&self, limits: Limits) -> Result<Vec<u8>, Error> {
        to_xdr_via_const(self, &limits, Self::const_write_xdr)
    }
}

impl AssetCode4 {
    #[must_use]
    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<Vec<u8>> for AssetCode4 {
    type Error = Error;
    fn try_from(x: Vec<u8>) -> Result<Self, Error> {
        x.as_slice().try_into()
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<&Vec<u8>> for AssetCode4 {
    type Error = Error;
    fn try_from(x: &Vec<u8>) -> Result<Self, Error> {
        x.as_slice().try_into()
    }
}

impl TryFrom<&[u8]> for AssetCode4 {
    type Error = Error;
    fn try_from(x: &[u8]) -> Result<Self, Error> {
        Ok(AssetCode4(x.try_into()?))
    }
}

impl AsRef<[u8]> for AssetCode4 {
    #[must_use]
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}
