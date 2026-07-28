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

/// ScMetaV0Ref is a borrowing equivalent of [`ScMetaV0`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScMetaV0Ref<'a> {
    pub key: StringMRef<'a>,
    pub val: StringMRef<'a>,
}

#[cfg(feature = "alloc")]
impl From<&ScMetaV0Ref<'_>> for ScMetaV0 {
    #[must_use]
    fn from(v: &ScMetaV0Ref<'_>) -> Self {
        Self {
            key: v.key.to_stringm(),
            val: v.val.to_stringm(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<ScMetaV0Ref<'_>> for ScMetaV0 {
    #[must_use]
    fn from(v: ScMetaV0Ref<'_>) -> Self {
        Self::from(&v)
    }
}

impl WriteXdr for ScMetaV0Ref<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.key.write_xdr(w)?;
            self.val.write_xdr(w)?;
            Ok(())
        })
    }
}
