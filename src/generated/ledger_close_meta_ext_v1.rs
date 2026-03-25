#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
/// LedgerCloseMetaExtV1 is an XDR Struct defined as:
///
/// ```text
/// struct LedgerCloseMetaExtV1
/// {
///     ExtensionPoint ext;
///     int64 sorobanFeeWrite1KB;
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
pub struct LedgerCloseMetaExtV1 {
    pub ext: ExtensionPoint,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub soroban_fee_write1_kb: i64,
}

impl ReadXdr for LedgerCloseMetaExtV1 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                ext: ExtensionPoint::read_xdr(r)?,
                soroban_fee_write1_kb: i64::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for LedgerCloseMetaExtV1 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.ext.write_xdr(w)?;
            self.soroban_fee_write1_kb.write_xdr(w)?;
            Ok(())
        })
    }
}
