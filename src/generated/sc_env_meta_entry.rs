#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ScEnvMetaEntry is an XDR Union defined as:
///
/// ```text
/// union SCEnvMetaEntry switch (SCEnvMetaKind kind)
/// {
/// case SC_ENV_META_KIND_INTERFACE_VERSION:
///     struct {
///         uint32 protocol;
///         uint32 preRelease;
///     } interfaceVersion;
/// };
/// ```
///
// union with discriminant ScEnvMetaKind
#[cfg_eval::cfg_eval]
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
pub enum ScEnvMetaEntry {
    ScEnvMetaKindInterfaceVersion(
        ScEnvMetaEntryInterfaceVersion,
    ),
}


#[cfg(feature = "alloc")]
impl Default for ScEnvMetaEntry {
    fn default() -> Self {
        Self::ScEnvMetaKindInterfaceVersion(ScEnvMetaEntryInterfaceVersion::default())
    }
}

impl ScEnvMetaEntry {
    const _VARIANTS: &[ScEnvMetaKind] = &[
        ScEnvMetaKind::ScEnvMetaKindInterfaceVersion,
    ];
    pub const VARIANTS: [ScEnvMetaKind; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &[
        "ScEnvMetaKindInterfaceVersion",
    ];
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
            Self::ScEnvMetaKindInterfaceVersion(_) => "ScEnvMetaKindInterfaceVersion",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> ScEnvMetaKind {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::ScEnvMetaKindInterfaceVersion(_) => ScEnvMetaKind::ScEnvMetaKindInterfaceVersion,
        }
    }

    #[must_use]
    pub const fn variants() -> [ScEnvMetaKind; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for ScEnvMetaEntry {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<ScEnvMetaKind> for ScEnvMetaEntry {
    #[must_use]
    fn discriminant(&self) -> ScEnvMetaKind {
        Self::discriminant(self)
    }
}

impl Variants<ScEnvMetaKind> for ScEnvMetaEntry {
    fn variants() -> slice::Iter<'static, ScEnvMetaKind> {
        Self::VARIANTS.iter()
    }
}

impl Union<ScEnvMetaKind> for ScEnvMetaEntry {}

impl ReadXdr for ScEnvMetaEntry {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: ScEnvMetaKind = <ScEnvMetaKind as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                ScEnvMetaKind::ScEnvMetaKindInterfaceVersion => Self::ScEnvMetaKindInterfaceVersion(ScEnvMetaEntryInterfaceVersion::read_xdr(r)?),
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for ScEnvMetaEntry {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::ScEnvMetaKindInterfaceVersion(v) => v.write_xdr(w)?,
            };
            Ok(())
        })
    }
}
