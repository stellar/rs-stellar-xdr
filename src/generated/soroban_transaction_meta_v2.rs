#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// SorobanTransactionMetaV2 is an XDR Struct defined as:
///
/// ```text
/// struct SorobanTransactionMetaV2
/// {
///     SorobanTransactionMetaExt ext;
///
///     SCVal* returnValue;
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
pub struct SorobanTransactionMetaV2 {
    pub ext: SorobanTransactionMetaExt,
    pub return_value: Option<ScVal>,
}

impl ReadXdr for SorobanTransactionMetaV2 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                ext: SorobanTransactionMetaExt::read_xdr(r)?,
                return_value: Option::<ScVal>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for SorobanTransactionMetaV2 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.ext.write_xdr(w)?;
            self.return_value.write_xdr(w)?;
            Ok(())
        })
    }
}

/// SorobanTransactionMetaV2View is a borrowing equivalent of [`SorobanTransactionMetaV2`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SorobanTransactionMetaV2View<'a> {
    pub ext: SorobanTransactionMetaExt,
    pub return_value: Option<ScValView<'a>>,
}

#[cfg(feature = "alloc")]
impl IntoOwned for SorobanTransactionMetaV2View<'_> {
    type Owned = SorobanTransactionMetaV2;
    fn into_owned(self) -> SorobanTransactionMetaV2 {
        SorobanTransactionMetaV2 {
            ext: self.ext.into_owned(),
            return_value: self.return_value.into_owned(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&SorobanTransactionMetaV2View<'_>> for SorobanTransactionMetaV2 {
    #[must_use]
    fn from(v: &SorobanTransactionMetaV2View<'_>) -> Self {
        v.clone().into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<SorobanTransactionMetaV2View<'_>> for SorobanTransactionMetaV2 {
    #[must_use]
    fn from(v: SorobanTransactionMetaV2View<'_>) -> Self {
        v.into_owned()
    }
}

impl WriteXdr for SorobanTransactionMetaV2View<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.ext.write_xdr(w)?;
            self.return_value.write_xdr(w)?;
            Ok(())
        })
    }
}
