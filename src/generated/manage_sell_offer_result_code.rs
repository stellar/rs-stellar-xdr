#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ManageSellOfferResultCode is an XDR Enum defined as:
///
/// ```text
/// enum ManageSellOfferResultCode
/// {
///     // codes considered as "success" for the operation
///     MANAGE_SELL_OFFER_SUCCESS = 0,
///
///     // codes considered as "failure" for the operation
///     MANAGE_SELL_OFFER_MALFORMED = -1, // generated offer would be invalid
///     MANAGE_SELL_OFFER_SELL_NO_TRUST =
///         -2,                              // no trust line for what we're selling
///     MANAGE_SELL_OFFER_BUY_NO_TRUST = -3, // no trust line for what we're buying
///     MANAGE_SELL_OFFER_SELL_NOT_AUTHORIZED = -4, // not authorized to sell
///     MANAGE_SELL_OFFER_BUY_NOT_AUTHORIZED = -5,  // not authorized to buy
///     MANAGE_SELL_OFFER_LINE_FULL = -6, // can't receive more of what it's buying
///     MANAGE_SELL_OFFER_UNDERFUNDED = -7, // doesn't hold what it's trying to sell
///     MANAGE_SELL_OFFER_CROSS_SELF =
///         -8, // would cross an offer from the same user
///     MANAGE_SELL_OFFER_SELL_NO_ISSUER = -9, // no issuer for what we're selling
///     MANAGE_SELL_OFFER_BUY_NO_ISSUER = -10, // no issuer for what we're buying
///
///     // update errors
///     MANAGE_SELL_OFFER_NOT_FOUND =
///         -11, // offerID does not match an existing offer
///
///     MANAGE_SELL_OFFER_LOW_RESERVE =
///         -12 // not enough funds to create a new Offer
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
pub enum ManageSellOfferResultCode {
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

impl ManageSellOfferResultCode {
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
    pub const fn variants() -> [ManageSellOfferResultCode; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for ManageSellOfferResultCode {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Variants<ManageSellOfferResultCode> for ManageSellOfferResultCode {
    fn variants() -> slice::Iter<'static, ManageSellOfferResultCode> {
        Self::VARIANTS.iter()
    }
}

impl Enum for ManageSellOfferResultCode {}

impl fmt::Display for ManageSellOfferResultCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for ManageSellOfferResultCode {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self, Error> {
        let e = match i {
            0 => ManageSellOfferResultCode::Success,
            -1 => ManageSellOfferResultCode::Malformed,
            -2 => ManageSellOfferResultCode::SellNoTrust,
            -3 => ManageSellOfferResultCode::BuyNoTrust,
            -4 => ManageSellOfferResultCode::SellNotAuthorized,
            -5 => ManageSellOfferResultCode::BuyNotAuthorized,
            -6 => ManageSellOfferResultCode::LineFull,
            -7 => ManageSellOfferResultCode::Underfunded,
            -8 => ManageSellOfferResultCode::CrossSelf,
            -9 => ManageSellOfferResultCode::SellNoIssuer,
            -10 => ManageSellOfferResultCode::BuyNoIssuer,
            -11 => ManageSellOfferResultCode::NotFound,
            -12 => ManageSellOfferResultCode::LowReserve,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<ManageSellOfferResultCode> for i32 {
    #[must_use]
    fn from(e: ManageSellOfferResultCode) -> Self {
        e as Self
    }
}

impl ReadXdr for ManageSellOfferResultCode {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let e = i32::read_xdr(r)?;
            let v: Self = e.try_into()?;
            Ok(v)
        })
    }
}

impl ManageSellOfferResultCode {
    /// Serialize this value as XDR into a [`ConstWriter`] using only const
    /// operations. This is the const counterpart to [`WriteXdr::write_xdr`].
    #[cfg(feature = "const")]
    pub const fn const_write_xdr(&self, w: &mut ConstWriter) {
        w.enter_depth();
        w.write_i32(*self as i32);
        w.leave_depth();
    }
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[cfg(feature = "const")]
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let limits = Limits {
            depth: u32::MAX,
            len: usize::MAX,
        };
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty, &limits);
        self.const_write_xdr(&mut w);
        w.position()
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
    #[cfg(feature = "const")]
    #[must_use]
    pub const fn const_to_xdr<const N: usize>(&self) -> [u8; N] {
        let limits = Limits {
            depth: u32::MAX,
            len: usize::MAX,
        };
        let mut buf = [0u8; N];
        let mut w = ConstWriter::new(&mut buf, &limits);
        self.const_write_xdr(&mut w);
        assert!(
            w.position() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

impl WriteXdr for ManageSellOfferResultCode {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            let i: i32 = (*self).into();
            i.write_xdr(w)
        })
    }
}
