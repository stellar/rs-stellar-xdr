#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// LedgerFootprint is an XDR Struct defined as:
///
/// ```text
/// struct LedgerFootprint
/// {
///     LedgerKey readOnly<>;
///     LedgerKey readWrite<>;
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
pub struct LedgerFootprint {
    pub read_only: VecM<LedgerKey>,
    pub read_write: VecM<LedgerKey>,
}

impl ReadXdr for LedgerFootprint {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                read_only: VecM::<LedgerKey>::read_xdr(r)?,
                read_write: VecM::<LedgerKey>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for LedgerFootprint {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.read_only.write_xdr(w)?;
            self.read_write.write_xdr(w)?;
            Ok(())
        })
    }
}

/// LedgerFootprintRef is a borrowing equivalent of [`LedgerFootprint`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct LedgerFootprintRef<'a> {
    pub read_only: VecMRef<'a, LedgerKeyRef<'a>>,
    pub read_write: VecMRef<'a, LedgerKeyRef<'a>>,
}

#[cfg(feature = "alloc")]
impl From<&LedgerFootprintRef<'_>> for LedgerFootprint {
    #[must_use]
    fn from(v: &LedgerFootprintRef<'_>) -> Self {
        Self {
            read_only: v.read_only.to_vecm_from(),
            read_write: v.read_write.to_vecm_from(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<LedgerFootprintRef<'_>> for LedgerFootprint {
    #[must_use]
    fn from(v: LedgerFootprintRef<'_>) -> Self {
        Self::from(&v)
    }
}
