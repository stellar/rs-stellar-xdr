#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ManageOfferSuccessResultOffer is an XDR NestedUnion defined as:
///
/// ```text
/// union switch (ManageOfferEffect effect)
///     {
///     case MANAGE_OFFER_CREATED:
///     case MANAGE_OFFER_UPDATED:
///         OfferEntry offer;
///     case MANAGE_OFFER_DELETED:
///         void;
///     }
/// ```
///
// union with discriminant ManageOfferEffect
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
pub enum ManageOfferSuccessResultOffer {
    Created(OfferEntry),
    Updated(OfferEntry),
    Deleted,
}

#[cfg(feature = "alloc")]
impl Default for ManageOfferSuccessResultOffer {
    fn default() -> Self {
        Self::Created(OfferEntry::default())
    }
}

impl ManageOfferSuccessResultOffer {
    const _VARIANTS: &[ManageOfferEffect] = &[
        ManageOfferEffect::Created,
        ManageOfferEffect::Updated,
        ManageOfferEffect::Deleted,
    ];
    pub const VARIANTS: [ManageOfferEffect; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &["Created", "Updated", "Deleted"];
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
            Self::Created(_) => "Created",
            Self::Updated(_) => "Updated",
            Self::Deleted => "Deleted",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> ManageOfferEffect {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Created(_) => ManageOfferEffect::Created,
            Self::Updated(_) => ManageOfferEffect::Updated,
            Self::Deleted => ManageOfferEffect::Deleted,
        }
    }

    #[must_use]
    pub const fn variants() -> [ManageOfferEffect; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for ManageOfferSuccessResultOffer {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<ManageOfferEffect> for ManageOfferSuccessResultOffer {
    #[must_use]
    fn discriminant(&self) -> ManageOfferEffect {
        Self::discriminant(self)
    }
}

impl Variants<ManageOfferEffect> for ManageOfferSuccessResultOffer {
    fn variants() -> slice::Iter<'static, ManageOfferEffect> {
        Self::VARIANTS.iter()
    }
}

impl Union<ManageOfferEffect> for ManageOfferSuccessResultOffer {}

impl ReadXdr for ManageOfferSuccessResultOffer {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: ManageOfferEffect = <ManageOfferEffect as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                ManageOfferEffect::Created => Self::Created(OfferEntry::read_xdr(r)?),
                ManageOfferEffect::Updated => Self::Updated(OfferEntry::read_xdr(r)?),
                ManageOfferEffect::Deleted => Self::Deleted,
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for ManageOfferSuccessResultOffer {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::Created(v) => v.write_xdr(w)?,
                Self::Updated(v) => v.write_xdr(w)?,
                Self::Deleted => ().write_xdr(w)?,
            };
            Ok(())
        })
    }
}
