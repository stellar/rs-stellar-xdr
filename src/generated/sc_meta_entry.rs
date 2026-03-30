#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ScMetaEntry is an XDR Union defined as:
///
/// ```text
/// union SCMetaEntry switch (SCMetaKind kind)
/// {
/// case SC_META_V0:
///     SCMetaV0 v0;
/// };
/// ```
///
// union with discriminant ScMetaKind
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
pub enum ScMetaEntry {
    ScMetaV0(
        ScMetaV0,
    ),
}


#[cfg(feature = "alloc")]
impl Default for ScMetaEntry {
    fn default() -> Self {
        Self::ScMetaV0(ScMetaV0::default())
    }
}

impl ScMetaEntry {
    const _VARIANTS: &[ScMetaKind] = &[
        ScMetaKind::ScMetaV0,
    ];
    pub const VARIANTS: [ScMetaKind; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &[
        "ScMetaV0",
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
            Self::ScMetaV0(_) => "ScMetaV0",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> ScMetaKind {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::ScMetaV0(_) => ScMetaKind::ScMetaV0,
        }
    }

    #[must_use]
    pub const fn variants() -> [ScMetaKind; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for ScMetaEntry {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<ScMetaKind> for ScMetaEntry {
    #[must_use]
    fn discriminant(&self) -> ScMetaKind {
        Self::discriminant(self)
    }
}

impl Variants<ScMetaKind> for ScMetaEntry {
    fn variants() -> slice::Iter<'static, ScMetaKind> {
        Self::VARIANTS.iter()
    }
}

impl Union<ScMetaKind> for ScMetaEntry {}

impl ReadXdr for ScMetaEntry {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: ScMetaKind = <ScMetaKind as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                ScMetaKind::ScMetaV0 => Self::ScMetaV0(ScMetaV0::read_xdr(r)?),
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for ScMetaEntry {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::ScMetaV0(v) => v.write_xdr(w)?,
            };
            Ok(())
        })
    }
}
