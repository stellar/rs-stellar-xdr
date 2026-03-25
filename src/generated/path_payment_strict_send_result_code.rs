#[allow(unused_imports)]
use super::*;
/// PathPaymentStrictSendResultCode is an XDR Enum defined as:
///
/// ```text
/// enum PathPaymentStrictSendResultCode
/// {
///     // codes considered as "success" for the operation
///     PATH_PAYMENT_STRICT_SEND_SUCCESS = 0, // success
///
///     // codes considered as "failure" for the operation
///     PATH_PAYMENT_STRICT_SEND_MALFORMED = -1, // bad input
///     PATH_PAYMENT_STRICT_SEND_UNDERFUNDED =
///         -2, // not enough funds in source account
///     PATH_PAYMENT_STRICT_SEND_SRC_NO_TRUST =
///         -3, // no trust line on source account
///     PATH_PAYMENT_STRICT_SEND_SRC_NOT_AUTHORIZED =
///         -4, // source not authorized to transfer
///     PATH_PAYMENT_STRICT_SEND_NO_DESTINATION =
///         -5, // destination account does not exist
///     PATH_PAYMENT_STRICT_SEND_NO_TRUST =
///         -6, // dest missing a trust line for asset
///     PATH_PAYMENT_STRICT_SEND_NOT_AUTHORIZED =
///         -7, // dest not authorized to hold asset
///     PATH_PAYMENT_STRICT_SEND_LINE_FULL = -8, // dest would go above their limit
///     PATH_PAYMENT_STRICT_SEND_NO_ISSUER = -9, // missing issuer on one asset
///     PATH_PAYMENT_STRICT_SEND_TOO_FEW_OFFERS =
///         -10, // not enough offers to satisfy path
///     PATH_PAYMENT_STRICT_SEND_OFFER_CROSS_SELF =
///         -11, // would cross one of its own offers
///     PATH_PAYMENT_STRICT_SEND_UNDER_DESTMIN = -12 // could not satisfy destMin
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
pub enum PathPaymentStrictSendResultCode {
    #[cfg_attr(feature = "alloc", default)]
    Success = 0,
    Malformed = -1,
    Underfunded = -2,
    SrcNoTrust = -3,
    SrcNotAuthorized = -4,
    NoDestination = -5,
    NoTrust = -6,
    NotAuthorized = -7,
    LineFull = -8,
    NoIssuer = -9,
    TooFewOffers = -10,
    OfferCrossSelf = -11,
    UnderDestmin = -12,
}

impl PathPaymentStrictSendResultCode {
    const _VARIANTS: &[PathPaymentStrictSendResultCode] = &[
        PathPaymentStrictSendResultCode::Success,
        PathPaymentStrictSendResultCode::Malformed,
        PathPaymentStrictSendResultCode::Underfunded,
        PathPaymentStrictSendResultCode::SrcNoTrust,
        PathPaymentStrictSendResultCode::SrcNotAuthorized,
        PathPaymentStrictSendResultCode::NoDestination,
        PathPaymentStrictSendResultCode::NoTrust,
        PathPaymentStrictSendResultCode::NotAuthorized,
        PathPaymentStrictSendResultCode::LineFull,
        PathPaymentStrictSendResultCode::NoIssuer,
        PathPaymentStrictSendResultCode::TooFewOffers,
        PathPaymentStrictSendResultCode::OfferCrossSelf,
        PathPaymentStrictSendResultCode::UnderDestmin,
    ];
    pub const VARIANTS: [PathPaymentStrictSendResultCode; Self::_VARIANTS.len()] = {
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
        "Underfunded",
        "SrcNoTrust",
        "SrcNotAuthorized",
        "NoDestination",
        "NoTrust",
        "NotAuthorized",
        "LineFull",
        "NoIssuer",
        "TooFewOffers",
        "OfferCrossSelf",
        "UnderDestmin",
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
            Self::Underfunded => "Underfunded",
            Self::SrcNoTrust => "SrcNoTrust",
            Self::SrcNotAuthorized => "SrcNotAuthorized",
            Self::NoDestination => "NoDestination",
            Self::NoTrust => "NoTrust",
            Self::NotAuthorized => "NotAuthorized",
            Self::LineFull => "LineFull",
            Self::NoIssuer => "NoIssuer",
            Self::TooFewOffers => "TooFewOffers",
            Self::OfferCrossSelf => "OfferCrossSelf",
            Self::UnderDestmin => "UnderDestmin",
        }
    }

    #[must_use]
    pub const fn variants() -> [PathPaymentStrictSendResultCode; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for PathPaymentStrictSendResultCode {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Variants<PathPaymentStrictSendResultCode> for PathPaymentStrictSendResultCode {
    fn variants() -> slice::Iter<'static, PathPaymentStrictSendResultCode> {
        Self::VARIANTS.iter()
    }
}

impl Enum for PathPaymentStrictSendResultCode {}

impl fmt::Display for PathPaymentStrictSendResultCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for PathPaymentStrictSendResultCode {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self, Error> {
        let e = match i {
            0 => PathPaymentStrictSendResultCode::Success,
            -1 => PathPaymentStrictSendResultCode::Malformed,
            -2 => PathPaymentStrictSendResultCode::Underfunded,
            -3 => PathPaymentStrictSendResultCode::SrcNoTrust,
            -4 => PathPaymentStrictSendResultCode::SrcNotAuthorized,
            -5 => PathPaymentStrictSendResultCode::NoDestination,
            -6 => PathPaymentStrictSendResultCode::NoTrust,
            -7 => PathPaymentStrictSendResultCode::NotAuthorized,
            -8 => PathPaymentStrictSendResultCode::LineFull,
            -9 => PathPaymentStrictSendResultCode::NoIssuer,
            -10 => PathPaymentStrictSendResultCode::TooFewOffers,
            -11 => PathPaymentStrictSendResultCode::OfferCrossSelf,
            -12 => PathPaymentStrictSendResultCode::UnderDestmin,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<PathPaymentStrictSendResultCode> for i32 {
    #[must_use]
    fn from(e: PathPaymentStrictSendResultCode) -> Self {
        e as Self
    }
}

impl ReadXdr for PathPaymentStrictSendResultCode {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let e = i32::read_xdr(r)?;
            let v: Self = e.try_into()?;
            Ok(v)
        })
    }
}

impl WriteXdr for PathPaymentStrictSendResultCode {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            let i: i32 = (*self).into();
            i.write_xdr(w)
        })
    }
}
