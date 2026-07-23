#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// DontHave is an XDR Struct defined as:
///
/// ```text
/// struct DontHave
/// {
///     MessageType type;
///     uint256 reqHash;
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
pub struct DontHave {
    #[cfg_attr(all(feature = "serde", feature = "alloc"), serde(alias = "type_"))]
    pub r#type: MessageType,
    pub req_hash: Uint256,
}

impl ReadXdr for DontHave {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                r#type: MessageType::read_xdr(r)?,
                req_hash: Uint256::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for DontHave {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.r#type.write_xdr(w)?;
            self.req_hash.write_xdr(w)?;
            Ok(())
        })
    }
}
