#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
/// ContractCostParamEntry is an XDR Struct defined as:
///
/// ```text
/// struct ContractCostParamEntry {
///     // use `ext` to add more terms (e.g. higher order polynomials) in the future
///     ExtensionPoint ext;
///
///     int64 constTerm;
///     int64 linearTerm;
/// };
/// ```
///
#[cfg_attr(feature = "alloc", derive(Default))]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_eval::cfg_eval]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    serde_with::serde_as,
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct ContractCostParamEntry {
    pub ext: ExtensionPoint,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub const_term: i64,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub linear_term: i64,
}

impl ReadXdr for ContractCostParamEntry {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                ext: ExtensionPoint::read_xdr(r)?,
                const_term: i64::read_xdr(r)?,
                linear_term: i64::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ContractCostParamEntry {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.ext.write_xdr(w)?;
            self.const_term.write_xdr(w)?;
            self.linear_term.write_xdr(w)?;
            Ok(())
        })
    }
}
