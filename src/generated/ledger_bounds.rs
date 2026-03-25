#[allow(unused_imports)]
use super::*;
/// LedgerBounds is an XDR Struct defined as:
///
/// ```text
/// struct LedgerBounds
/// {
///     uint32 minLedger;
///     uint32 maxLedger; // 0 here means no maxLedger
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
pub struct LedgerBounds {
    pub min_ledger: u32,
    pub max_ledger: u32,
}

impl ReadXdr for LedgerBounds {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                min_ledger: u32::read_xdr(r)?,
                max_ledger: u32::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for LedgerBounds {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.min_ledger.write_xdr(w)?;
            self.max_ledger.write_xdr(w)?;
            Ok(())
        })
    }
}
