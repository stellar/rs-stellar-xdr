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
#[cfg(feature = "cap_0084_muxed_contract")]
#[cfg_attr(feature = "alloc", derive(Default))]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde_with::SerializeDisplay)
)]
pub struct MuxedContract {
    pub id: u64,
    pub contract_id: ContractId,
}

#[cfg(feature = "cap_0084_muxed_contract")]
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

#[cfg(feature = "cap_0084_muxed_contract")]
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
#[cfg(feature = "cap_0084_muxed_contract")]
#[cfg(all(feature = "serde", feature = "alloc"))]
impl<'de> serde::Deserialize<'de> for MuxedContract {
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::Deserialize;
        #[derive(Deserialize)]
        struct MuxedContract {
            id: u64,
            contract_id: ContractId,
        }
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum MuxedContractOrString<'a> {
            Str(&'a str),
            String(String),
            MuxedContract(MuxedContract),
        }
        match MuxedContractOrString::deserialize(deserializer)? {
            MuxedContractOrString::Str(s) => s.parse().map_err(serde::de::Error::custom),
            MuxedContractOrString::String(s) => s.parse().map_err(serde::de::Error::custom),
            MuxedContractOrString::MuxedContract(MuxedContract { id, contract_id }) => {
                Ok(self::MuxedContract { id, contract_id })
            }
        }
    }
}
