#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// Preconditions is an XDR Union defined as:
///
/// ```text
/// union Preconditions switch (PreconditionType type)
/// {
/// case PRECOND_NONE:
///     void;
/// case PRECOND_TIME:
///     TimeBounds timeBounds;
/// case PRECOND_V2:
///     PreconditionsV2 v2;
/// };
/// ```
///
// union with discriminant PreconditionType
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
pub enum Preconditions {
    None,
    Time(TimeBounds),
    V2(PreconditionsV2),
}

#[cfg(feature = "alloc")]
impl Default for Preconditions {
    fn default() -> Self {
        Self::None
    }
}

impl Preconditions {
    const _VARIANTS: &[PreconditionType] = &[
        PreconditionType::None,
        PreconditionType::Time,
        PreconditionType::V2,
    ];
    pub const VARIANTS: [PreconditionType; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &["None", "Time", "V2"];
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
            Self::None => "None",
            Self::Time(_) => "Time",
            Self::V2(_) => "V2",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> PreconditionType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::None => PreconditionType::None,
            Self::Time(_) => PreconditionType::Time,
            Self::V2(_) => PreconditionType::V2,
        }
    }

    #[must_use]
    pub const fn variants() -> [PreconditionType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for Preconditions {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<PreconditionType> for Preconditions {
    #[must_use]
    fn discriminant(&self) -> PreconditionType {
        Self::discriminant(self)
    }
}

impl Variants<PreconditionType> for Preconditions {
    fn variants() -> slice::Iter<'static, PreconditionType> {
        Self::VARIANTS.iter()
    }
}

impl Union<PreconditionType> for Preconditions {}

impl ReadXdr for Preconditions {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: PreconditionType = <PreconditionType as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                PreconditionType::None => Self::None,
                PreconditionType::Time => Self::Time(TimeBounds::read_xdr(r)?),
                PreconditionType::V2 => Self::V2(PreconditionsV2::read_xdr(r)?),
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for Preconditions {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::None => ().write_xdr(w)?,
                Self::Time(v) => v.write_xdr(w)?,
                Self::V2(v) => v.write_xdr(w)?,
            };
            Ok(())
        })
    }
}
