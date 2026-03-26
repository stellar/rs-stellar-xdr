#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// Liabilities is an XDR Struct defined as:
///
/// ```text
/// struct Liabilities
/// {
///     int64 buying;
///     int64 selling;
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
pub struct Liabilities {
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub buying: i64,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub selling: i64,
}

impl ReadXdr for Liabilities {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                buying: i64::read_xdr(r)?,
                selling: i64::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for Liabilities {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.buying.write_xdr(w)?;
            self.selling.write_xdr(w)?;
            Ok(())
        })
    }
}
