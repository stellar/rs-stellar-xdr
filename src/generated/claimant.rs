#[allow(unused_imports, clippy::wildcard_imports)]
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

/// ClaimantRef is a borrowing equivalent of [`Claimant`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(clippy::large_enum_variant)]
pub enum ClaimantRef<'a> {
    ClaimantTypeV0(ClaimantV0Ref<'a>),
}

#[cfg(feature = "alloc")]
impl IntoOwned for ClaimantRef<'_> {
    type Owned = Claimant;
    fn into_owned(self) -> Claimant {
        #[allow(clippy::match_same_arms)]
        match self {
            ClaimantRef::ClaimantTypeV0(value) => Claimant::ClaimantTypeV0(value.into_owned()),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&ClaimantRef<'_>> for Claimant {
    #[must_use]
    fn from(v: &ClaimantRef<'_>) -> Self {
        v.into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<ClaimantRef<'_>> for Claimant {
    #[must_use]
    fn from(v: ClaimantRef<'_>) -> Self {
        v.into_owned()
    }
}

impl ClaimantRef<'_> {
    #[must_use]
    pub const fn discriminant(&self) -> ClaimantType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::ClaimantTypeV0(_) => ClaimantType::ClaimantTypeV0,
        }
    }
}

impl WriteXdr for ClaimantRef<'_> {
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

#[cfg(feature = "const")]
impl ClaimantView<'_> {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_claimant(self);
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
        w.write_type_claimant(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`Claimant`], mirroring `<Claimant as WriteXdr>::write_xdr`.
    pub const fn write_type_claimant(&mut self, v: &ClaimantView<'_>) {
        let d = v.discriminant();
        self.write_type_claimant_type(&d);
        #[allow(clippy::match_same_arms)]
        match v {
            ClaimantView::ClaimantTypeV0(value) => {
                self.write_type_claimant_v0(value);
            }
        }
    }

    /// Serializes a variable-length array of [`Claimant`], mirroring `<VecM<Claimant, MAX> as WriteXdr>::write_xdr`.
    pub const fn write_type_vec_claimant<const MAX: u32>(
        &mut self,
        v: &VecMView<'_, ClaimantView<'_>, MAX>,
    ) {
        let s = v.as_slice();
        let len = s.len();
        self.write_len(len);
        let mut i = 0usize;
        while i < len {
            self.write_type_claimant(&s[i]);
            i += 1;
        }
    }
}
