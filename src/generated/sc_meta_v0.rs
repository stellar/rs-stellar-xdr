#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ScMetaV0 is an XDR Struct defined as:
///
/// ```text
/// struct SCMetaV0
/// {
///     string key<>;
///     string val<>;
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
pub struct ScMetaV0 {
    pub key: StringM,
    pub val: StringM,
}

impl ReadXdr for ScMetaV0 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                key: StringM::read_xdr(r)?,
                val: StringM::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ScMetaV0 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.key.write_xdr(w)?;
            self.val.write_xdr(w)?;
            Ok(())
        })
    }
}

/// ScMetaV0View is a borrowing equivalent of [`ScMetaV0`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScMetaV0View<'a> {
    pub key: StringMView<'a>,
    pub val: StringMView<'a>,
}

#[cfg(feature = "alloc")]
impl IntoOwned for ScMetaV0View<'_> {
    type Owned = ScMetaV0;
    fn into_owned(self) -> ScMetaV0 {
        ScMetaV0 {
            key: self.key.into_owned(),
            val: self.val.into_owned(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&ScMetaV0View<'_>> for ScMetaV0 {
    #[must_use]
    fn from(v: &ScMetaV0View<'_>) -> Self {
        v.clone().into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<ScMetaV0View<'_>> for ScMetaV0 {
    #[must_use]
    fn from(v: ScMetaV0View<'_>) -> Self {
        v.into_owned()
    }
}

impl WriteXdr for ScMetaV0View<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.key.write_xdr(w)?;
            self.val.write_xdr(w)?;
            Ok(())
        })
    }
}
