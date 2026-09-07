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

/// ScSpecTypeUdtRef is a borrowing equivalent of [`ScSpecTypeUdt`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScSpecTypeUdtRef<'a> {
    pub name: StringMRef<'a, 60>,
}

#[cfg(feature = "alloc")]
impl IntoOwned for ScSpecTypeUdtRef<'_> {
    type Owned = ScSpecTypeUdt;
    fn into_owned(self) -> ScSpecTypeUdt {
        ScSpecTypeUdt {
            name: self.name.into_owned(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&ScSpecTypeUdtRef<'_>> for ScSpecTypeUdt {
    #[must_use]
    fn from(v: &ScSpecTypeUdtRef<'_>) -> Self {
        v.into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<ScSpecTypeUdtRef<'_>> for ScSpecTypeUdt {
    #[must_use]
    fn from(v: ScSpecTypeUdtRef<'_>) -> Self {
        v.into_owned()
    }
}

impl WriteXdr for ScSpecTypeUdtRef<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.name.write_xdr(w)?;
            Ok(())
        })
    }
}
