#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ScSpecTypeTuple is an XDR Struct defined as:
///
/// ```text
/// struct SCSpecTypeTuple
/// {
///     SCSpecTypeDef valueTypes<12>;
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
pub struct ScSpecTypeTuple {
    pub value_types: VecM<ScSpecTypeDef, 12>,
}

impl ReadXdr for ScSpecTypeTuple {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                value_types: VecM::<ScSpecTypeDef, 12>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ScSpecTypeTuple {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.value_types.write_xdr(w)?;
            Ok(())
        })
    }
}

/// ScSpecTypeTupleRef is a borrowing equivalent of [`ScSpecTypeTuple`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScSpecTypeTupleRef<'a> {
    pub value_types: VecMRef<'a, ScSpecTypeDefRef<'a>, 12>,
}

#[cfg(feature = "alloc")]
impl IntoOwned for ScSpecTypeTupleRef<'_> {
    type Owned = ScSpecTypeTuple;
    fn into_owned(self) -> ScSpecTypeTuple {
        ScSpecTypeTuple {
            value_types: self.value_types.into_owned(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&ScSpecTypeTupleRef<'_>> for ScSpecTypeTuple {
    #[must_use]
    fn from(v: &ScSpecTypeTupleRef<'_>) -> Self {
        v.into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<ScSpecTypeTupleRef<'_>> for ScSpecTypeTuple {
    #[must_use]
    fn from(v: ScSpecTypeTupleRef<'_>) -> Self {
        v.into_owned()
    }
}

impl WriteXdr for ScSpecTypeTupleRef<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.value_types.write_xdr(w)?;
            Ok(())
        })
    }
}
