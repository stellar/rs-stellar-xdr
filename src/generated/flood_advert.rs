#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// FloodAdvert is an XDR Struct defined as:
///
/// ```text
/// struct FloodAdvert
/// {
///     TxAdvertVector txHashes;
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
pub struct FloodAdvert {
    pub tx_hashes: TxAdvertVector,
}

impl ReadXdr for FloodAdvert {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                tx_hashes: TxAdvertVector::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for FloodAdvert {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.tx_hashes.write_xdr(w)?;
            Ok(())
        })
    }
}

/// FloodAdvertRef is a borrowing equivalent of [`FloodAdvert`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct FloodAdvertRef<'a> {
    pub tx_hashes: TxAdvertVectorRef<'a>,
}

#[cfg(feature = "alloc")]
impl From<&FloodAdvertRef<'_>> for FloodAdvert {
    #[must_use]
    fn from(v: &FloodAdvertRef<'_>) -> Self {
        Self {
            tx_hashes: (&v.tx_hashes).into(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<FloodAdvertRef<'_>> for FloodAdvert {
    #[must_use]
    fn from(v: FloodAdvertRef<'_>) -> Self {
        Self::from(&v)
    }
}
