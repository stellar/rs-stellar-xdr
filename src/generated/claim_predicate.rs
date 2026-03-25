#[allow(unused_imports)]
use super::*;
/// ClaimPredicate is an XDR Union defined as:
///
/// ```text
/// union ClaimPredicate switch (ClaimPredicateType type)
/// {
/// case CLAIM_PREDICATE_UNCONDITIONAL:
///     void;
/// case CLAIM_PREDICATE_AND:
///     ClaimPredicate andPredicates<2>;
/// case CLAIM_PREDICATE_OR:
///     ClaimPredicate orPredicates<2>;
/// case CLAIM_PREDICATE_NOT:
///     ClaimPredicate* notPredicate;
/// case CLAIM_PREDICATE_BEFORE_ABSOLUTE_TIME:
///     int64 absBefore; // Predicate will be true if closeTime < absBefore
/// case CLAIM_PREDICATE_BEFORE_RELATIVE_TIME:
///     int64 relBefore; // Seconds since closeTime of the ledger in which the
///                      // ClaimableBalanceEntry was created
/// };
/// ```
///
// union with discriminant ClaimPredicateType
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
pub enum ClaimPredicate {
    Unconditional,
    And(VecM<ClaimPredicate, 2>),
    Or(VecM<ClaimPredicate, 2>),
    Not(Option<Box<ClaimPredicate>>),
    BeforeAbsoluteTime(
        #[cfg_attr(
            all(feature = "serde", feature = "alloc"),
            serde_as(as = "NumberOrString")
        )]
        i64,
    ),
    BeforeRelativeTime(
        #[cfg_attr(
            all(feature = "serde", feature = "alloc"),
            serde_as(as = "NumberOrString")
        )]
        i64,
    ),
}

#[cfg(feature = "alloc")]
impl Default for ClaimPredicate {
    fn default() -> Self {
        Self::Unconditional
    }
}

impl ClaimPredicate {
    const _VARIANTS: &[ClaimPredicateType] = &[
        ClaimPredicateType::Unconditional,
        ClaimPredicateType::And,
        ClaimPredicateType::Or,
        ClaimPredicateType::Not,
        ClaimPredicateType::BeforeAbsoluteTime,
        ClaimPredicateType::BeforeRelativeTime,
    ];
    pub const VARIANTS: [ClaimPredicateType; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &[
        "Unconditional",
        "And",
        "Or",
        "Not",
        "BeforeAbsoluteTime",
        "BeforeRelativeTime",
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
            Self::Unconditional => "Unconditional",
            Self::And(_) => "And",
            Self::Or(_) => "Or",
            Self::Not(_) => "Not",
            Self::BeforeAbsoluteTime(_) => "BeforeAbsoluteTime",
            Self::BeforeRelativeTime(_) => "BeforeRelativeTime",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> ClaimPredicateType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Unconditional => ClaimPredicateType::Unconditional,
            Self::And(_) => ClaimPredicateType::And,
            Self::Or(_) => ClaimPredicateType::Or,
            Self::Not(_) => ClaimPredicateType::Not,
            Self::BeforeAbsoluteTime(_) => ClaimPredicateType::BeforeAbsoluteTime,
            Self::BeforeRelativeTime(_) => ClaimPredicateType::BeforeRelativeTime,
        }
    }

    #[must_use]
    pub const fn variants() -> [ClaimPredicateType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for ClaimPredicate {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<ClaimPredicateType> for ClaimPredicate {
    #[must_use]
    fn discriminant(&self) -> ClaimPredicateType {
        Self::discriminant(self)
    }
}

impl Variants<ClaimPredicateType> for ClaimPredicate {
    fn variants() -> slice::Iter<'static, ClaimPredicateType> {
        Self::VARIANTS.iter()
    }
}

impl Union<ClaimPredicateType> for ClaimPredicate {}

impl ReadXdr for ClaimPredicate {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: ClaimPredicateType = <ClaimPredicateType as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                ClaimPredicateType::Unconditional => Self::Unconditional,
                ClaimPredicateType::And => Self::And(VecM::<ClaimPredicate, 2>::read_xdr(r)?),
                ClaimPredicateType::Or => Self::Or(VecM::<ClaimPredicate, 2>::read_xdr(r)?),
                ClaimPredicateType::Not => Self::Not(Option::<Box<ClaimPredicate>>::read_xdr(r)?),
                ClaimPredicateType::BeforeAbsoluteTime => {
                    Self::BeforeAbsoluteTime(i64::read_xdr(r)?)
                }
                ClaimPredicateType::BeforeRelativeTime => {
                    Self::BeforeRelativeTime(i64::read_xdr(r)?)
                }
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for ClaimPredicate {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::Unconditional => ().write_xdr(w)?,
                Self::And(v) => v.write_xdr(w)?,
                Self::Or(v) => v.write_xdr(w)?,
                Self::Not(v) => v.write_xdr(w)?,
                Self::BeforeAbsoluteTime(v) => v.write_xdr(w)?,
                Self::BeforeRelativeTime(v) => v.write_xdr(w)?,
            };
            Ok(())
        })
    }
}
