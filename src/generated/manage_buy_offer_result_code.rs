#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
/// ManageBuyOfferResultCode is an XDR Enum defined as:
///
/// ```text
/// enum ManageBuyOfferResultCode
/// {
///     // codes considered as "success" for the operation
///     MANAGE_BUY_OFFER_SUCCESS = 0,
///
///     // codes considered as "failure" for the operation
///     MANAGE_BUY_OFFER_MALFORMED = -1,     // generated offer would be invalid
///     MANAGE_BUY_OFFER_SELL_NO_TRUST = -2, // no trust line for what we're selling
///     MANAGE_BUY_OFFER_BUY_NO_TRUST = -3,  // no trust line for what we're buying
///     MANAGE_BUY_OFFER_SELL_NOT_AUTHORIZED = -4, // not authorized to sell
///     MANAGE_BUY_OFFER_BUY_NOT_AUTHORIZED = -5,  // not authorized to buy
///     MANAGE_BUY_OFFER_LINE_FULL = -6,   // can't receive more of what it's buying
///     MANAGE_BUY_OFFER_UNDERFUNDED = -7, // doesn't hold what it's trying to sell
///     MANAGE_BUY_OFFER_CROSS_SELF = -8, // would cross an offer from the same user
///     MANAGE_BUY_OFFER_SELL_NO_ISSUER = -9, // no issuer for what we're selling
///     MANAGE_BUY_OFFER_BUY_NO_ISSUER = -10, // no issuer for what we're buying
///
///     // update errors
///     MANAGE_BUY_OFFER_NOT_FOUND =
///         -11, // offerID does not match an existing offer
///
///     MANAGE_BUY_OFFER_LOW_RESERVE = -12 // not enough funds to create a new Offer
/// };
/// ```
///
// enum
#[cfg_attr(feature = "alloc", derive(Default))]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[repr(i32)]
pub enum ManageBuyOfferResultCode {
    #[cfg_attr(feature = "alloc", default)]
    Success = 0,
    Malformed = -1,
    SellNoTrust = -2,
    BuyNoTrust = -3,
    SellNotAuthorized = -4,
    BuyNotAuthorized = -5,
    LineFull = -6,
    Underfunded = -7,
    CrossSelf = -8,
    SellNoIssuer = -9,
    BuyNoIssuer = -10,
    NotFound = -11,
    LowReserve = -12,
}

impl ManageBuyOfferResultCode {
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
            Self::Success => "Success",
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
    pub const fn variants() -> [ManageBuyOfferResultCode; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for ManageBuyOfferResultCode {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Variants<ManageBuyOfferResultCode> for ManageBuyOfferResultCode {
    fn variants() -> slice::Iter<'static, ManageBuyOfferResultCode> {
        Self::VARIANTS.iter()
    }
}

impl Enum for ManageBuyOfferResultCode {}

impl fmt::Display for ManageBuyOfferResultCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for ManageBuyOfferResultCode {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self, Error> {
        let e = match i {
            0 => ManageBuyOfferResultCode::Success,
            -1 => ManageBuyOfferResultCode::Malformed,
            -2 => ManageBuyOfferResultCode::SellNoTrust,
            -3 => ManageBuyOfferResultCode::BuyNoTrust,
            -4 => ManageBuyOfferResultCode::SellNotAuthorized,
            -5 => ManageBuyOfferResultCode::BuyNotAuthorized,
            -6 => ManageBuyOfferResultCode::LineFull,
            -7 => ManageBuyOfferResultCode::Underfunded,
            -8 => ManageBuyOfferResultCode::CrossSelf,
            -9 => ManageBuyOfferResultCode::SellNoIssuer,
            -10 => ManageBuyOfferResultCode::BuyNoIssuer,
            -11 => ManageBuyOfferResultCode::NotFound,
            -12 => ManageBuyOfferResultCode::LowReserve,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<ManageBuyOfferResultCode> for i32 {
    #[must_use]
    fn from(e: ManageBuyOfferResultCode) -> Self {
        e as Self
    }
}

impl ReadXdr for ManageBuyOfferResultCode {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let e = i32::read_xdr(r)?;
            let v: Self = e.try_into()?;
            Ok(v)
        })
    }
}

impl WriteXdr for ManageBuyOfferResultCode {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            let i: i32 = (*self).into();
            i.write_xdr(w)
        })
    }
}
