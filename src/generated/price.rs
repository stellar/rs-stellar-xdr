#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// Price is an XDR Struct defined as:
///
/// ```text
/// struct Price
/// {
///     int32 n; // numerator
///     int32 d; // denominator
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
pub struct Price {
    pub n: i32,
    pub d: i32,
}

impl ReadXdr for Price {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                n: i32::read_xdr(r)?,
                d: i32::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for Price {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.n.write_xdr(w)?;
            self.d.write_xdr(w)?;
            Ok(())
        })
    }
}
