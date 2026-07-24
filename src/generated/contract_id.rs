#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ContractId is an XDR Typedef defined as:
///
/// ```text
/// typedef Hash ContractID;
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
pub struct ContractId(pub Hash);

impl From<ContractId> for Hash {
    #[must_use]
    fn from(x: ContractId) -> Self {
        x.0
    }
}

impl From<Hash> for ContractId {
    #[must_use]
    fn from(x: Hash) -> Self {
        ContractId(x)
    }
}

impl AsRef<Hash> for ContractId {
    #[must_use]
    fn as_ref(&self) -> &Hash {
        &self.0
    }
}

impl ReadXdr for ContractId {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let i = Hash::read_xdr(r)?;
            let v = ContractId(i);
            Ok(v)
        })
    }
}

impl ContractId {
    /// Serialize this value as XDR into a [`ConstWriter`] using only const
    /// operations. This is the const implementation underlying `to_xdr`.
    #[cfg(feature = "std")]
    pub const fn const_write_xdr(&self, w: &mut ConstWriter) {
        w.enter_depth();
        self.0.const_write_xdr(w);
        w.leave_depth();
    }
}

impl WriteXdr for ContractId {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| self.0.write_xdr(w))
    }

    #[cfg(feature = "std")]
    fn to_xdr(&self, limits: Limits) -> Result<Vec<u8>, Error> {
        to_xdr_via_const(self, &limits, Self::const_write_xdr)
    }
}
