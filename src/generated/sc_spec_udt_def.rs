#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ScSpecUdtDef is an XDR Union defined as:
///
/// ```text
/// union SCSpecUDTDef switch (SCSpecUDTDefKind kind)
/// {
/// case SC_SPEC_UDT_DEF_LIB:
///     string lib<80>;
/// case SC_SPEC_UDT_DEF_HASH:
///     opaque hash[8];
/// };
/// ```
///
// union with discriminant ScSpecUdtDefKind
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval)]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    serde_with::serde_as,
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[allow(clippy::large_enum_variant)]
pub enum ScSpecUdtDef {
    Lib(StringM<80>),
    Hash([u8; 8]),
}

#[cfg(feature = "alloc")]
impl Default for ScSpecUdtDef {
    fn default() -> Self {
        Self::Lib(StringM::<80>::default())
    }
}

impl ScSpecUdtDef {
    const _VARIANTS: &[ScSpecUdtDefKind] = &[ScSpecUdtDefKind::Lib, ScSpecUdtDefKind::Hash];
    pub const VARIANTS: [ScSpecUdtDefKind; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &["Lib", "Hash"];
    pub const VARIANTS_STR: [&'static str; Self::_VARIANTS_STR.len()] = {
        let mut arr = [Self::_VARIANTS_STR[0]; Self::_VARIANTS_STR.len()];
        let mut i = 1;
        while i < Self::_VARIANTS_STR.len() {
            arr[i] = Self::_VARIANTS_STR[i];
            i += 1;
        }
        arr
    };

    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Lib(_) => "Lib",
            Self::Hash(_) => "Hash",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> ScSpecUdtDefKind {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Lib(_) => ScSpecUdtDefKind::Lib,
            Self::Hash(_) => ScSpecUdtDefKind::Hash,
        }
    }

    #[must_use]
    pub const fn variants() -> [ScSpecUdtDefKind; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for ScSpecUdtDef {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<ScSpecUdtDefKind> for ScSpecUdtDef {
    #[must_use]
    fn discriminant(&self) -> ScSpecUdtDefKind {
        Self::discriminant(self)
    }
}

impl Variants<ScSpecUdtDefKind> for ScSpecUdtDef {
    fn variants() -> slice::Iter<'static, ScSpecUdtDefKind> {
        Self::VARIANTS.iter()
    }
}

impl Union<ScSpecUdtDefKind> for ScSpecUdtDef {}

impl ReadXdr for ScSpecUdtDef {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: ScSpecUdtDefKind = <ScSpecUdtDefKind as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                ScSpecUdtDefKind::Lib => Self::Lib(StringM::<80>::read_xdr(r)?),
                ScSpecUdtDefKind::Hash => Self::Hash(<[u8; 8]>::read_xdr(r)?),
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for ScSpecUdtDef {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::Lib(v) => v.write_xdr(w)?,
                Self::Hash(v) => v.write_xdr(w)?,
            };
            Ok(())
        })
    }
}
