#[allow(unused_imports)]
use super::*;
/// ManageSellOfferResult is an XDR Union defined as:
///
/// ```text
/// union ManageSellOfferResult switch (ManageSellOfferResultCode code)
/// {
/// case MANAGE_SELL_OFFER_SUCCESS:
///     ManageOfferSuccessResult success;
/// case MANAGE_SELL_OFFER_MALFORMED:
/// case MANAGE_SELL_OFFER_SELL_NO_TRUST:
/// case MANAGE_SELL_OFFER_BUY_NO_TRUST:
/// case MANAGE_SELL_OFFER_SELL_NOT_AUTHORIZED:
/// case MANAGE_SELL_OFFER_BUY_NOT_AUTHORIZED:
/// case MANAGE_SELL_OFFER_LINE_FULL:
/// case MANAGE_SELL_OFFER_UNDERFUNDED:
/// case MANAGE_SELL_OFFER_CROSS_SELF:
/// case MANAGE_SELL_OFFER_SELL_NO_ISSUER:
/// case MANAGE_SELL_OFFER_BUY_NO_ISSUER:
/// case MANAGE_SELL_OFFER_NOT_FOUND:
/// case MANAGE_SELL_OFFER_LOW_RESERVE:
///     void;
/// };
/// ```
///
// union with discriminant ManageSellOfferResultCode
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
pub enum ManageSellOfferResult {
    Success(ManageOfferSuccessResult),
    Malformed,
    SellNoTrust,
    BuyNoTrust,
    SellNotAuthorized,
    BuyNotAuthorized,
    LineFull,
    Underfunded,
    CrossSelf,
    SellNoIssuer,
    BuyNoIssuer,
    NotFound,
    LowReserve,
}

#[cfg(feature = "alloc")]
impl Default for ManageSellOfferResult {
    fn default() -> Self {
        Self::Success(ManageOfferSuccessResult::default())
    }
}

impl ManageSellOfferResult {
    const _VARIANTS: &[ManageSellOfferResultCode] = &[
        ManageSellOfferResultCode::Success,
        ManageSellOfferResultCode::Malformed,
        ManageSellOfferResultCode::SellNoTrust,
        ManageSellOfferResultCode::BuyNoTrust,
        ManageSellOfferResultCode::SellNotAuthorized,
        ManageSellOfferResultCode::BuyNotAuthorized,
        ManageSellOfferResultCode::LineFull,
        ManageSellOfferResultCode::Underfunded,
        ManageSellOfferResultCode::CrossSelf,
        ManageSellOfferResultCode::SellNoIssuer,
        ManageSellOfferResultCode::BuyNoIssuer,
        ManageSellOfferResultCode::NotFound,
        ManageSellOfferResultCode::LowReserve,
    ];
    pub const VARIANTS: [ManageSellOfferResultCode; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &[
        "Success",
        "Malformed",
        "SellNoTrust",
        "BuyNoTrust",
        "SellNotAuthorized",
        "BuyNotAuthorized",
        "LineFull",
        "Underfunded",
        "CrossSelf",
        "SellNoIssuer",
        "BuyNoIssuer",
        "NotFound",
        "LowReserve",
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
            Self::Success(_) => "Success",
            Self::Malformed => "Malformed",
            Self::SellNoTrust => "SellNoTrust",
            Self::BuyNoTrust => "BuyNoTrust",
            Self::SellNotAuthorized => "SellNotAuthorized",
            Self::BuyNotAuthorized => "BuyNotAuthorized",
            Self::LineFull => "LineFull",
            Self::Underfunded => "Underfunded",
            Self::CrossSelf => "CrossSelf",
            Self::SellNoIssuer => "SellNoIssuer",
            Self::BuyNoIssuer => "BuyNoIssuer",
            Self::NotFound => "NotFound",
            Self::LowReserve => "LowReserve",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> ManageSellOfferResultCode {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success(_) => ManageSellOfferResultCode::Success,
            Self::Malformed => ManageSellOfferResultCode::Malformed,
            Self::SellNoTrust => ManageSellOfferResultCode::SellNoTrust,
            Self::BuyNoTrust => ManageSellOfferResultCode::BuyNoTrust,
            Self::SellNotAuthorized => ManageSellOfferResultCode::SellNotAuthorized,
            Self::BuyNotAuthorized => ManageSellOfferResultCode::BuyNotAuthorized,
            Self::LineFull => ManageSellOfferResultCode::LineFull,
            Self::Underfunded => ManageSellOfferResultCode::Underfunded,
            Self::CrossSelf => ManageSellOfferResultCode::CrossSelf,
            Self::SellNoIssuer => ManageSellOfferResultCode::SellNoIssuer,
            Self::BuyNoIssuer => ManageSellOfferResultCode::BuyNoIssuer,
            Self::NotFound => ManageSellOfferResultCode::NotFound,
            Self::LowReserve => ManageSellOfferResultCode::LowReserve,
        }
    }

    #[must_use]
    pub const fn variants() -> [ManageSellOfferResultCode; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for ManageSellOfferResult {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<ManageSellOfferResultCode> for ManageSellOfferResult {
    #[must_use]
    fn discriminant(&self) -> ManageSellOfferResultCode {
        Self::discriminant(self)
    }
}

impl Variants<ManageSellOfferResultCode> for ManageSellOfferResult {
    fn variants() -> slice::Iter<'static, ManageSellOfferResultCode> {
        Self::VARIANTS.iter()
    }
}

impl Union<ManageSellOfferResultCode> for ManageSellOfferResult {}

impl ReadXdr for ManageSellOfferResult {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: ManageSellOfferResultCode =
                <ManageSellOfferResultCode as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                ManageSellOfferResultCode::Success => {
                    Self::Success(ManageOfferSuccessResult::read_xdr(r)?)
                }
                ManageSellOfferResultCode::Malformed => Self::Malformed,
                ManageSellOfferResultCode::SellNoTrust => Self::SellNoTrust,
                ManageSellOfferResultCode::BuyNoTrust => Self::BuyNoTrust,
                ManageSellOfferResultCode::SellNotAuthorized => Self::SellNotAuthorized,
                ManageSellOfferResultCode::BuyNotAuthorized => Self::BuyNotAuthorized,
                ManageSellOfferResultCode::LineFull => Self::LineFull,
                ManageSellOfferResultCode::Underfunded => Self::Underfunded,
                ManageSellOfferResultCode::CrossSelf => Self::CrossSelf,
                ManageSellOfferResultCode::SellNoIssuer => Self::SellNoIssuer,
                ManageSellOfferResultCode::BuyNoIssuer => Self::BuyNoIssuer,
                ManageSellOfferResultCode::NotFound => Self::NotFound,
                ManageSellOfferResultCode::LowReserve => Self::LowReserve,
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for ManageSellOfferResult {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::Success(v) => v.write_xdr(w)?,
                Self::Malformed => ().write_xdr(w)?,
                Self::SellNoTrust => ().write_xdr(w)?,
                Self::BuyNoTrust => ().write_xdr(w)?,
                Self::SellNotAuthorized => ().write_xdr(w)?,
                Self::BuyNotAuthorized => ().write_xdr(w)?,
                Self::LineFull => ().write_xdr(w)?,
                Self::Underfunded => ().write_xdr(w)?,
                Self::CrossSelf => ().write_xdr(w)?,
                Self::SellNoIssuer => ().write_xdr(w)?,
                Self::BuyNoIssuer => ().write_xdr(w)?,
                Self::NotFound => ().write_xdr(w)?,
                Self::LowReserve => ().write_xdr(w)?,
            };
            Ok(())
        })
    }
}
