#[allow(unused_imports, clippy::wildcard_imports)]
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

/// ClaimPredicateRef is a borrowing equivalent of [`ClaimPredicate`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(clippy::large_enum_variant)]
pub enum ClaimPredicateRef<'a> {
    Unconditional,
    And(VecMRef<'a, ClaimPredicateRef<'a>, 2>),
    Or(VecMRef<'a, ClaimPredicateRef<'a>, 2>),
    Not(Option<&'a ClaimPredicateRef<'a>>),
    BeforeAbsoluteTime(i64),
    BeforeRelativeTime(i64),
}

#[cfg(feature = "alloc")]
impl IntoOwned for ClaimPredicateRef<'_> {
    type Owned = ClaimPredicate;
    fn into_owned(self) -> ClaimPredicate {
        #[allow(clippy::match_same_arms)]
        match self {
            ClaimPredicateRef::Unconditional => ClaimPredicate::Unconditional,
            ClaimPredicateRef::And(value) => ClaimPredicate::And(value.into_owned()),
            ClaimPredicateRef::Or(value) => ClaimPredicate::Or(value.into_owned()),
            ClaimPredicateRef::Not(value) => {
                ClaimPredicate::Not(value.map(|v| Box::new(v.into_owned())))
            }
            ClaimPredicateRef::BeforeAbsoluteTime(value) => {
                ClaimPredicate::BeforeAbsoluteTime(value.into_owned())
            }
            ClaimPredicateRef::BeforeRelativeTime(value) => {
                ClaimPredicate::BeforeRelativeTime(value.into_owned())
            }
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&ClaimPredicateRef<'_>> for ClaimPredicate {
    #[must_use]
    fn from(v: &ClaimPredicateRef<'_>) -> Self {
        v.into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<ClaimPredicateRef<'_>> for ClaimPredicate {
    #[must_use]
    fn from(v: ClaimPredicateRef<'_>) -> Self {
        v.into_owned()
    }
}

impl ClaimPredicateRef<'_> {
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
}

impl WriteXdr for ClaimPredicateRef<'_> {
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

#[cfg(feature = "const")]
impl ClaimPredicateRef<'_> {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_claim_predicate(self);
        w.len()
    }

    /// Serialize this value as XDR into a fixed-size `[u8; N]` using only const
    /// operations. This is the const counterpart to [`WriteXdr::to_xdr`].
    ///
    /// `N` must equal [`Self::const_xdr_len`]. It is intended for callers, such
    /// as a proc-macro, that compute the length with `const_xdr_len` and pass
    /// it as `N`; `const_to_xdr` itself does not need to call `const_xdr_len`.
    ///
    /// # Panics
    ///
    /// Panics if `N` does not equal the value's [`Self::const_xdr_len`].
    #[must_use]
    pub const fn const_to_xdr<const N: usize>(&self) -> [u8; N] {
        let mut buf = [0u8; N];
        let mut w = ConstWriter::new(&mut buf);
        w.write_type_claim_predicate(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`ClaimPredicate`], mirroring `<ClaimPredicate as WriteXdr>::write_xdr`.
    pub const fn write_type_claim_predicate(&mut self, v: &ClaimPredicateRef<'_>) {
        let d = v.discriminant();
        self.write_type_claim_predicate_type(&d);
        #[allow(clippy::match_same_arms)]
        match v {
            ClaimPredicateRef::Unconditional => {}
            ClaimPredicateRef::And(value) => {
                self.write_type_vec_claim_predicate(value);
            }
            ClaimPredicateRef::Or(value) => {
                self.write_type_vec_claim_predicate(value);
            }
            ClaimPredicateRef::Not(value) => {
                self.write_type_option_ref_claim_predicate(*value);
            }
            ClaimPredicateRef::BeforeAbsoluteTime(value) => {
                self.write_i64(*value);
            }
            ClaimPredicateRef::BeforeRelativeTime(value) => {
                self.write_i64(*value);
            }
        }
    }

    /// Serializes an optional [`ClaimPredicate`], mirroring `<Option<Box<ClaimPredicate>> as WriteXdr>::write_xdr`.
    pub const fn write_type_option_ref_claim_predicate(
        &mut self,
        v: Option<&'_ ClaimPredicateRef<'_>>,
    ) {
        match v {
            Some(v) => {
                self.write_u32(1);
                self.write_type_claim_predicate(v);
            }
            None => {
                self.write_u32(0);
            }
        }
    }

    /// Serializes a variable-length array of [`ClaimPredicate`], mirroring `<VecM<ClaimPredicate, MAX> as WriteXdr>::write_xdr`.
    pub const fn write_type_vec_claim_predicate<const MAX: u32>(
        &mut self,
        v: &VecMRef<'_, ClaimPredicateRef<'_>, MAX>,
    ) {
        let s = v.as_slice();
        let len = s.len();
        self.write_len(len);
        let mut i = 0usize;
        while i < len {
            self.write_type_claim_predicate(&s[i]);
            i += 1;
        }
    }
}
