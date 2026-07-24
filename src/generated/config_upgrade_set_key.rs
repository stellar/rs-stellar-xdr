#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ConfigUpgradeSetKey is an XDR Struct defined as:
///
/// ```text
/// struct ConfigUpgradeSetKey {
///     ContractID contractID;
///     Hash contentHash;
/// };
/// ```
///
#[cfg_attr(feature = "alloc", derive(Default))]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    serde_with::serde_as,
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct ConfigUpgradeSetKey {
    pub contract_id: ContractId,
    pub content_hash: Hash,
}

impl ReadXdr for ConfigUpgradeSetKey {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                contract_id: ContractId::read_xdr(r)?,
                content_hash: Hash::read_xdr(r)?,
            })
        })
    }
}

impl ConfigUpgradeSetKey {
    /// Serialize this value as XDR into a [`ConstWriter`] using only const
    /// operations. This is the const implementation underlying `to_xdr`.
    #[cfg(feature = "std")]
    pub const fn const_to_xdr(&self, w: &mut ConstWriter) {
        w.enter_depth();
        self.contract_id.const_to_xdr(w);
        self.content_hash.const_to_xdr(w);
        w.leave_depth();
    }
}

impl WriteXdr for ConfigUpgradeSetKey {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.contract_id.write_xdr(w)?;
            self.content_hash.write_xdr(w)?;
            Ok(())
        })
    }

    #[cfg(feature = "std")]
    fn to_xdr(&self, limits: Limits) -> Result<Vec<u8>, Error> {
        to_xdr_via_const(self, &limits, Self::const_to_xdr)
    }
}
