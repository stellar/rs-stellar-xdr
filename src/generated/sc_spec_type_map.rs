#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ScSpecTypeMap is an XDR Struct defined as:
///
/// ```text
/// struct SCSpecTypeMap
/// {
///     SCSpecTypeDef keyType;
///     SCSpecTypeDef valueType;
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
pub struct ScSpecTypeMap {
    pub key_type: Box<ScSpecTypeDef>,
    pub value_type: Box<ScSpecTypeDef>,
}

impl ReadXdr for ScSpecTypeMap {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                key_type: Box::<ScSpecTypeDef>::read_xdr(r)?,
                value_type: Box::<ScSpecTypeDef>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ScSpecTypeMap {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.key_type.write_xdr(w)?;
            self.value_type.write_xdr(w)?;
            Ok(())
        })
    }
}

/// ScSpecTypeMapView is a borrowing equivalent of [`ScSpecTypeMap`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScSpecTypeMapView<'a> {
    pub key_type: &'a ScSpecTypeDefView<'a>,
    pub value_type: &'a ScSpecTypeDefView<'a>,
}

#[cfg(feature = "alloc")]
impl From<&ScSpecTypeMapView<'_>> for ScSpecTypeMap {
    #[must_use]
    fn from(v: &ScSpecTypeMapView<'_>) -> Self {
        Self {
            key_type: Box::new(v.key_type.into()),
            value_type: Box::new(v.value_type.into()),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<ScSpecTypeMapView<'_>> for ScSpecTypeMap {
    #[must_use]
    fn from(v: ScSpecTypeMapView<'_>) -> Self {
        Self::from(&v)
    }
}

impl WriteXdr for ScSpecTypeMapView<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.key_type.write_xdr(w)?;
            self.value_type.write_xdr(w)?;
            Ok(())
        })
    }
}
