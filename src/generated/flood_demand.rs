#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// FloodDemand is an XDR Struct defined as:
///
/// ```text
/// struct FloodDemand
/// {
///     TxDemandVector txHashes;
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
pub struct FloodDemand {
    pub tx_hashes: TxDemandVector,
}

impl ReadXdr for FloodDemand {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                tx_hashes: TxDemandVector::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for FloodDemand {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.tx_hashes.write_xdr(w)?;
            Ok(())
        })
    }
}

/// FloodDemandView is a borrowing equivalent of [`FloodDemand`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct FloodDemandView<'a> {
    pub tx_hashes: TxDemandVectorView<'a>,
}

#[cfg(feature = "alloc")]
impl From<&FloodDemandView<'_>> for FloodDemand {
    #[must_use]
    fn from(v: &FloodDemandView<'_>) -> Self {
        Self {
            tx_hashes: (&v.tx_hashes).into(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<FloodDemandView<'_>> for FloodDemand {
    #[must_use]
    fn from(v: FloodDemandView<'_>) -> Self {
        Self::from(&v)
    }
}

impl WriteXdr for FloodDemandView<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.tx_hashes.write_xdr(w)?;
            Ok(())
        })
    }
}
