#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ScSpecTypeVec is an XDR Struct defined as:
///
/// ```text
/// struct SCSpecTypeVec
/// {
///     SCSpecTypeDef elementType;
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
pub struct ScSpecTypeVec {
    pub element_type: Box<ScSpecTypeDef>,
}

impl ReadXdr for ScSpecTypeVec {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                element_type: Box::<ScSpecTypeDef>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ScSpecTypeVec {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.element_type.write_xdr(w)?;
            Ok(())
        })
    }
}

/// ScSpecTypeVecRef is a borrowing equivalent of [`ScSpecTypeVec`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScSpecTypeVecRef<'a> {
    pub element_type: &'a ScSpecTypeDefRef<'a>,
}

#[cfg(feature = "alloc")]
impl From<&ScSpecTypeVecRef<'_>> for ScSpecTypeVec {
    #[must_use]
    fn from(v: &ScSpecTypeVecRef<'_>) -> Self {
        Self {
            element_type: Box::new(v.element_type.into()),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<ScSpecTypeVecRef<'_>> for ScSpecTypeVec {
    #[must_use]
    fn from(v: ScSpecTypeVecRef<'_>) -> Self {
        Self::from(&v)
    }
}

impl WriteXdr for ScSpecTypeVecRef<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.element_type.write_xdr(w)?;
            Ok(())
        })
    }
}
