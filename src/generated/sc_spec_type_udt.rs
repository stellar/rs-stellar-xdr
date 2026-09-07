#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ScSpecTypeUdt is an XDR Struct defined as:
///
/// ```text
/// struct SCSpecTypeUDT
/// {
///     string name<60>;
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
pub struct ScSpecTypeUdt {
    pub name: StringM<60>,
}

impl ReadXdr for ScSpecTypeUdt {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                name: StringM::<60>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ScSpecTypeUdt {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.name.write_xdr(w)?;
            Ok(())
        })
    }
}

/// ScSpecTypeUdtView is a borrowing equivalent of [`ScSpecTypeUdt`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScSpecTypeUdtView<'a> {
    pub name: StringMView<'a, 60>,
}

#[cfg(feature = "alloc")]
impl IntoOwned for ScSpecTypeUdtView<'_> {
    type Owned = ScSpecTypeUdt;
    fn into_owned(&self) -> ScSpecTypeUdt {
        ScSpecTypeUdt {
            name: IntoOwned::into_owned(&self.name),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&ScSpecTypeUdtView<'_>> for ScSpecTypeUdt {
    #[must_use]
    fn from(v: &ScSpecTypeUdtView<'_>) -> Self {
        IntoOwned::into_owned(v)
    }
}

#[cfg(feature = "alloc")]
impl From<ScSpecTypeUdtView<'_>> for ScSpecTypeUdt {
    #[must_use]
    fn from(v: ScSpecTypeUdtView<'_>) -> Self {
        IntoOwned::into_owned(&v)
    }
}

impl WriteXdr for ScSpecTypeUdtView<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.name.write_xdr(w)?;
            Ok(())
        })
    }
}
