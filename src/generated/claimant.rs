#[allow(unused_imports)]
use super::*;
/// Claimant is an XDR Union defined as:
///
/// ```text
/// union Claimant switch (ClaimantType type)
/// {
/// case CLAIMANT_TYPE_V0:
///     struct
///     {
///         AccountID destination;    // The account that can use this condition
///         ClaimPredicate predicate; // Claimable if predicate is true
///     } v0;
/// };
/// ```
///
// union with discriminant ClaimantType
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
pub enum Claimant {
    ClaimantTypeV0(ClaimantV0),
}

#[cfg(feature = "alloc")]
impl Default for Claimant {
    fn default() -> Self {
        Self::ClaimantTypeV0(ClaimantV0::default())
    }
}

impl Claimant {
    const _VARIANTS: &[ClaimantType] = &[ClaimantType::ClaimantTypeV0];
    pub const VARIANTS: [ClaimantType; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &["ClaimantTypeV0"];
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
            Self::ClaimantTypeV0(_) => "ClaimantTypeV0",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> ClaimantType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::ClaimantTypeV0(_) => ClaimantType::ClaimantTypeV0,
        }
    }

    #[must_use]
    pub const fn variants() -> [ClaimantType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for Claimant {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<ClaimantType> for Claimant {
    #[must_use]
    fn discriminant(&self) -> ClaimantType {
        Self::discriminant(self)
    }
}

impl Variants<ClaimantType> for Claimant {
    fn variants() -> slice::Iter<'static, ClaimantType> {
        Self::VARIANTS.iter()
    }
}

impl Union<ClaimantType> for Claimant {}

impl ReadXdr for Claimant {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: ClaimantType = <ClaimantType as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                ClaimantType::ClaimantTypeV0 => Self::ClaimantTypeV0(ClaimantV0::read_xdr(r)?),
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for Claimant {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::ClaimantTypeV0(v) => v.write_xdr(w)?,
            };
            Ok(())
        })
    }
}
