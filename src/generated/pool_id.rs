#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// PoolId is an XDR Typedef defined as:
///
/// ```text
/// typedef Hash PoolID;
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
#[derive(Debug)]
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
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let i = Hash::read_xdr(r)?;
            let v = PoolId(i);
            Ok(v)
        })
    }
}

impl PoolId {
    /// Serialize this value as XDR into a [`ConstWriter`] using only const
    /// operations. This is the const implementation underlying `to_xdr`.
    #[cfg(feature = "std")]
    pub const fn const_to_xdr(&self, w: &mut ConstWriter) {
        w.enter_depth();
        self.0.const_to_xdr(w);
        w.leave_depth();
    }
}

impl WriteXdr for PoolId {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| self.0.write_xdr(w))
    }

    #[cfg(feature = "std")]
    fn to_xdr(&self, limits: Limits) -> Result<Vec<u8>, Error> {
        to_xdr_via_const(self, &limits, Self::const_to_xdr)
    }
}
