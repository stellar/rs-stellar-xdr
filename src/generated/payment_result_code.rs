#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// PaymentResultCode is an XDR Enum defined as:
///
/// ```text
/// enum PaymentResultCode
/// {
///     // codes considered as "success" for the operation
///     PAYMENT_SUCCESS = 0, // payment successfully completed
///
///     // codes considered as "failure" for the operation
///     PAYMENT_MALFORMED = -1,          // bad input
///     PAYMENT_UNDERFUNDED = -2,        // not enough funds in source account
///     PAYMENT_SRC_NO_TRUST = -3,       // no trust line on source account
///     PAYMENT_SRC_NOT_AUTHORIZED = -4, // source not authorized to transfer
///     PAYMENT_NO_DESTINATION = -5,     // destination account does not exist
///     PAYMENT_NO_TRUST = -6,       // destination missing a trust line for asset
///     PAYMENT_NOT_AUTHORIZED = -7, // destination not authorized to hold asset
///     PAYMENT_LINE_FULL = -8,      // destination would go above their limit
///     PAYMENT_NO_ISSUER = -9       // missing issuer on asset
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
pub enum PaymentResultCode {
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
}

impl PaymentResultCode {
    const _VARIANTS: &[PaymentResultCode] = &[
        PaymentResultCode::Success,
        PaymentResultCode::Malformed,
        PaymentResultCode::Underfunded,
        PaymentResultCode::SrcNoTrust,
        PaymentResultCode::SrcNotAuthorized,
        PaymentResultCode::NoDestination,
        PaymentResultCode::NoTrust,
        PaymentResultCode::NotAuthorized,
        PaymentResultCode::LineFull,
        PaymentResultCode::NoIssuer,
    ];
    pub const VARIANTS: [PaymentResultCode; Self::_VARIANTS.len()] = {
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
        }
    }

    #[must_use]
    pub const fn variants() -> [PaymentResultCode; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for PaymentResultCode {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Variants<PaymentResultCode> for PaymentResultCode {
    fn variants() -> slice::Iter<'static, PaymentResultCode> {
        Self::VARIANTS.iter()
    }
}

impl Enum for PaymentResultCode {}

impl fmt::Display for PaymentResultCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for PaymentResultCode {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self, Error> {
        let e = match i {
            0 => PaymentResultCode::Success,
            -1 => PaymentResultCode::Malformed,
            -2 => PaymentResultCode::Underfunded,
            -3 => PaymentResultCode::SrcNoTrust,
            -4 => PaymentResultCode::SrcNotAuthorized,
            -5 => PaymentResultCode::NoDestination,
            -6 => PaymentResultCode::NoTrust,
            -7 => PaymentResultCode::NotAuthorized,
            -8 => PaymentResultCode::LineFull,
            -9 => PaymentResultCode::NoIssuer,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<PaymentResultCode> for i32 {
    #[must_use]
    fn from(e: PaymentResultCode) -> Self {
        e as Self
    }
}

impl ReadXdr for PaymentResultCode {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let e = i32::read_xdr(r)?;
            let v: Self = e.try_into()?;
            Ok(v)
        })
    }
}

impl WriteXdr for PaymentResultCode {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            let i: i32 = (*self).into();
            i.write_xdr(w)
        })
    }
}
