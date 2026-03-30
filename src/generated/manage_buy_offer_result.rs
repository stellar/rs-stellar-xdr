#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ManageBuyOfferResult is an XDR Union defined as:
///
/// ```text
/// union ManageBuyOfferResult switch (ManageBuyOfferResultCode code)
/// {
/// case MANAGE_BUY_OFFER_SUCCESS:
///     ManageOfferSuccessResult success;
/// case MANAGE_BUY_OFFER_MALFORMED:
/// case MANAGE_BUY_OFFER_SELL_NO_TRUST:
/// case MANAGE_BUY_OFFER_BUY_NO_TRUST:
/// case MANAGE_BUY_OFFER_SELL_NOT_AUTHORIZED:
/// case MANAGE_BUY_OFFER_BUY_NOT_AUTHORIZED:
/// case MANAGE_BUY_OFFER_LINE_FULL:
/// case MANAGE_BUY_OFFER_UNDERFUNDED:
/// case MANAGE_BUY_OFFER_CROSS_SELF:
/// case MANAGE_BUY_OFFER_SELL_NO_ISSUER:
/// case MANAGE_BUY_OFFER_BUY_NO_ISSUER:
/// case MANAGE_BUY_OFFER_NOT_FOUND:
/// case MANAGE_BUY_OFFER_LOW_RESERVE:
///     void;
/// };
/// ```
///
// union with discriminant ManageBuyOfferResultCode
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
pub enum ManageBuyOfferResult {
    Success(
        ManageOfferSuccessResult,
    ),
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
impl Default for ManageBuyOfferResult {
    fn default() -> Self {
        Self::Success(ManageOfferSuccessResult::default())
    }
}

impl ManageBuyOfferResult {
    const _VARIANTS: &[ManageBuyOfferResultCode] = &[
        ManageBuyOfferResultCode::Success,
        ManageBuyOfferResultCode::Malformed,
        ManageBuyOfferResultCode::SellNoTrust,
        ManageBuyOfferResultCode::BuyNoTrust,
        ManageBuyOfferResultCode::SellNotAuthorized,
        ManageBuyOfferResultCode::BuyNotAuthorized,
        ManageBuyOfferResultCode::LineFull,
        ManageBuyOfferResultCode::Underfunded,
        ManageBuyOfferResultCode::CrossSelf,
        ManageBuyOfferResultCode::SellNoIssuer,
        ManageBuyOfferResultCode::BuyNoIssuer,
        ManageBuyOfferResultCode::NotFound,
        ManageBuyOfferResultCode::LowReserve,
    ];
    pub const VARIANTS: [ManageBuyOfferResultCode; Self::_VARIANTS.len()] = {
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
    pub const fn discriminant(&self) -> ManageBuyOfferResultCode {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success(_) => ManageBuyOfferResultCode::Success,
            Self::Malformed => ManageBuyOfferResultCode::Malformed,
            Self::SellNoTrust => ManageBuyOfferResultCode::SellNoTrust,
            Self::BuyNoTrust => ManageBuyOfferResultCode::BuyNoTrust,
            Self::SellNotAuthorized => ManageBuyOfferResultCode::SellNotAuthorized,
            Self::BuyNotAuthorized => ManageBuyOfferResultCode::BuyNotAuthorized,
            Self::LineFull => ManageBuyOfferResultCode::LineFull,
            Self::Underfunded => ManageBuyOfferResultCode::Underfunded,
            Self::CrossSelf => ManageBuyOfferResultCode::CrossSelf,
            Self::SellNoIssuer => ManageBuyOfferResultCode::SellNoIssuer,
            Self::BuyNoIssuer => ManageBuyOfferResultCode::BuyNoIssuer,
            Self::NotFound => ManageBuyOfferResultCode::NotFound,
            Self::LowReserve => ManageBuyOfferResultCode::LowReserve,
        }
    }

    #[must_use]
    pub const fn variants() -> [ManageBuyOfferResultCode; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for ManageBuyOfferResult {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<ManageBuyOfferResultCode> for ManageBuyOfferResult {
    #[must_use]
    fn discriminant(&self) -> ManageBuyOfferResultCode {
        Self::discriminant(self)
    }
}

impl Variants<ManageBuyOfferResultCode> for ManageBuyOfferResult {
    fn variants() -> slice::Iter<'static, ManageBuyOfferResultCode> {
        Self::VARIANTS.iter()
    }
}

impl Union<ManageBuyOfferResultCode> for ManageBuyOfferResult {}

impl ReadXdr for ManageBuyOfferResult {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: ManageBuyOfferResultCode = <ManageBuyOfferResultCode as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                ManageBuyOfferResultCode::Success => Self::Success(ManageOfferSuccessResult::read_xdr(r)?),
                ManageBuyOfferResultCode::Malformed => Self::Malformed,
                ManageBuyOfferResultCode::SellNoTrust => Self::SellNoTrust,
                ManageBuyOfferResultCode::BuyNoTrust => Self::BuyNoTrust,
                ManageBuyOfferResultCode::SellNotAuthorized => Self::SellNotAuthorized,
                ManageBuyOfferResultCode::BuyNotAuthorized => Self::BuyNotAuthorized,
                ManageBuyOfferResultCode::LineFull => Self::LineFull,
                ManageBuyOfferResultCode::Underfunded => Self::Underfunded,
                ManageBuyOfferResultCode::CrossSelf => Self::CrossSelf,
                ManageBuyOfferResultCode::SellNoIssuer => Self::SellNoIssuer,
                ManageBuyOfferResultCode::BuyNoIssuer => Self::BuyNoIssuer,
                ManageBuyOfferResultCode::NotFound => Self::NotFound,
                ManageBuyOfferResultCode::LowReserve => Self::LowReserve,
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for ManageBuyOfferResult {
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
