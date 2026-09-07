#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ScSpecTypeOption is an XDR Struct defined as:
///
/// ```text
/// struct SCSpecTypeOption
/// {
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
pub struct ScSpecTypeOption {
    pub value_type: Box<ScSpecTypeDef>,
}

impl ReadXdr for ScSpecTypeOption {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                value_type: Box::<ScSpecTypeDef>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ScSpecTypeOption {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.value_type.write_xdr(w)?;
            Ok(())
        })
    }
}

/// ScSpecTypeOptionView is a borrowing equivalent of [`ScSpecTypeOption`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScSpecTypeOptionView<'a> {
    pub value_type: &'a ScSpecTypeDefView<'a>,
}

#[cfg(feature = "alloc")]
impl IntoOwned for ScSpecTypeOptionView<'_> {
    type Owned = ScSpecTypeOption;
    fn into_owned(self) -> ScSpecTypeOption {
        ScSpecTypeOption {
            value_type: Box::new(self.value_type.into_owned()),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&ScSpecTypeOptionView<'_>> for ScSpecTypeOption {
    #[must_use]
    fn from(v: &ScSpecTypeOptionView<'_>) -> Self {
        v.into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<ScSpecTypeOptionView<'_>> for ScSpecTypeOption {
    #[must_use]
    fn from(v: ScSpecTypeOptionView<'_>) -> Self {
        v.into_owned()
    }
}

impl WriteXdr for ScSpecTypeOptionView<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.value_type.write_xdr(w)?;
            Ok(())
        })
    }
}
