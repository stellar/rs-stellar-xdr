#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// MuxedContract is an XDR Struct defined as:
///
/// ```text
/// struct MuxedContract
/// {
///     uint64 id;
///     ContractID contractId;
/// };
/// ```
///
#[cfg(feature = "cap_0084")]
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
pub struct MuxedContract {
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub id: u64,
    pub contract_id: ContractId,
}

#[cfg(feature = "cap_0084")]
impl ReadXdr for MuxedContract {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                id: u64::read_xdr(r)?,
                contract_id: ContractId::read_xdr(r)?,
            })
        })
    }
}

#[cfg(feature = "cap_0084")]
impl WriteXdr for MuxedContract {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.id.write_xdr(w)?;
            self.contract_id.write_xdr(w)?;
            Ok(())
        })
    }
}
