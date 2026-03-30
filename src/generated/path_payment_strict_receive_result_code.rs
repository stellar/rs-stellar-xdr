#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// PathPaymentStrictReceiveResultCode is an XDR Enum defined as:
///
/// ```text
/// enum PathPaymentStrictReceiveResultCode
/// {
///     // codes considered as "success" for the operation
///     PATH_PAYMENT_STRICT_RECEIVE_SUCCESS = 0, // success
/// 
///     // codes considered as "failure" for the operation
///     PATH_PAYMENT_STRICT_RECEIVE_MALFORMED = -1, // bad input
///     PATH_PAYMENT_STRICT_RECEIVE_UNDERFUNDED =
///         -2, // not enough funds in source account
///     PATH_PAYMENT_STRICT_RECEIVE_SRC_NO_TRUST =
///         -3, // no trust line on source account
///     PATH_PAYMENT_STRICT_RECEIVE_SRC_NOT_AUTHORIZED =
///         -4, // source not authorized to transfer
///     PATH_PAYMENT_STRICT_RECEIVE_NO_DESTINATION =
///         -5, // destination account does not exist
///     PATH_PAYMENT_STRICT_RECEIVE_NO_TRUST =
///         -6, // dest missing a trust line for asset
///     PATH_PAYMENT_STRICT_RECEIVE_NOT_AUTHORIZED =
///         -7, // dest not authorized to hold asset
///     PATH_PAYMENT_STRICT_RECEIVE_LINE_FULL =
///         -8, // dest would go above their limit
///     PATH_PAYMENT_STRICT_RECEIVE_NO_ISSUER = -9, // missing issuer on one asset
///     PATH_PAYMENT_STRICT_RECEIVE_TOO_FEW_OFFERS =
///         -10, // not enough offers to satisfy path
///     PATH_PAYMENT_STRICT_RECEIVE_OFFER_CROSS_SELF =
///         -11, // would cross one of its own offers
///     PATH_PAYMENT_STRICT_RECEIVE_OVER_SENDMAX = -12 // could not satisfy sendmax
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
pub enum PathPaymentStrictReceiveResultCode {
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
    OverSendmax = -12,
}

impl PathPaymentStrictReceiveResultCode {
    const _VARIANTS: &[PathPaymentStrictReceiveResultCode] = &[
        PathPaymentStrictReceiveResultCode::Success,
        PathPaymentStrictReceiveResultCode::Malformed,
        PathPaymentStrictReceiveResultCode::Underfunded,
        PathPaymentStrictReceiveResultCode::SrcNoTrust,
        PathPaymentStrictReceiveResultCode::SrcNotAuthorized,
        PathPaymentStrictReceiveResultCode::NoDestination,
        PathPaymentStrictReceiveResultCode::NoTrust,
        PathPaymentStrictReceiveResultCode::NotAuthorized,
        PathPaymentStrictReceiveResultCode::LineFull,
        PathPaymentStrictReceiveResultCode::NoIssuer,
        PathPaymentStrictReceiveResultCode::TooFewOffers,
        PathPaymentStrictReceiveResultCode::OfferCrossSelf,
        PathPaymentStrictReceiveResultCode::OverSendmax,
    ];
    pub const VARIANTS: [PathPaymentStrictReceiveResultCode; Self::_VARIANTS.len()] = {
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
        "OverSendmax",
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
            Self::OverSendmax => "OverSendmax",
        }
    }

    #[must_use]
    pub const fn variants() -> [PathPaymentStrictReceiveResultCode; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for PathPaymentStrictReceiveResultCode {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Variants<PathPaymentStrictReceiveResultCode> for PathPaymentStrictReceiveResultCode {
    fn variants() -> slice::Iter<'static, PathPaymentStrictReceiveResultCode> {
        Self::VARIANTS.iter()
    }
}

impl Enum for PathPaymentStrictReceiveResultCode {}

impl fmt::Display for PathPaymentStrictReceiveResultCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for PathPaymentStrictReceiveResultCode {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self, Error> {
        let e = match i {
            0 => PathPaymentStrictReceiveResultCode::Success,
            -1 => PathPaymentStrictReceiveResultCode::Malformed,
            -2 => PathPaymentStrictReceiveResultCode::Underfunded,
            -3 => PathPaymentStrictReceiveResultCode::SrcNoTrust,
            -4 => PathPaymentStrictReceiveResultCode::SrcNotAuthorized,
            -5 => PathPaymentStrictReceiveResultCode::NoDestination,
            -6 => PathPaymentStrictReceiveResultCode::NoTrust,
            -7 => PathPaymentStrictReceiveResultCode::NotAuthorized,
            -8 => PathPaymentStrictReceiveResultCode::LineFull,
            -9 => PathPaymentStrictReceiveResultCode::NoIssuer,
            -10 => PathPaymentStrictReceiveResultCode::TooFewOffers,
            -11 => PathPaymentStrictReceiveResultCode::OfferCrossSelf,
            -12 => PathPaymentStrictReceiveResultCode::OverSendmax,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<PathPaymentStrictReceiveResultCode> for i32 {
    #[must_use]
    fn from(e: PathPaymentStrictReceiveResultCode) -> Self {
        e as Self
    }
}

impl ReadXdr for PathPaymentStrictReceiveResultCode {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let e = i32::read_xdr(r)?;
            let v: Self = e.try_into()?;
            Ok(v)
        })
    }
}

impl WriteXdr for PathPaymentStrictReceiveResultCode {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            let i: i32 = (*self).into();
            i.write_xdr(w)
        })
    }
}
