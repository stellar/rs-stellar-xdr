#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ManageSellOfferResult is a borrowing equivalent of [`ManageSellOfferResult`](super::super::ManageSellOfferResult)
/// over `'static` data, for const XDR encoding.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
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

impl ManageSellOfferResult {
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
}

impl ManageSellOfferResult {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_manage_sell_offer_result(self);
        w.len()
    }

    /// Serialize this value as XDR into a fixed-size `[u8; N]` using only const
    /// operations. This is the const counterpart to
    /// [`WriteXdr::to_xdr`](super::super::WriteXdr::to_xdr).
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
        w.write_type_manage_sell_offer_result(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

impl ConstWriter<'_> {
    /// Serializes a [`ManageSellOfferResult`], mirroring `<ManageSellOfferResult as WriteXdr>::write_xdr`.
    pub const fn write_type_manage_sell_offer_result(&mut self, v: &ManageSellOfferResult) {
        let d = v.discriminant();
        self.write_type_manage_sell_offer_result_code(&d);
        #[allow(clippy::match_same_arms)]
        match v {
            ManageSellOfferResult::Success(value) => {
                self.write_type_manage_offer_success_result(value);
            }
            ManageSellOfferResult::Malformed => {}
            ManageSellOfferResult::SellNoTrust => {}
            ManageSellOfferResult::BuyNoTrust => {}
            ManageSellOfferResult::SellNotAuthorized => {}
            ManageSellOfferResult::BuyNotAuthorized => {}
            ManageSellOfferResult::LineFull => {}
            ManageSellOfferResult::Underfunded => {}
            ManageSellOfferResult::CrossSelf => {}
            ManageSellOfferResult::SellNoIssuer => {}
            ManageSellOfferResult::BuyNoIssuer => {}
            ManageSellOfferResult::NotFound => {}
            ManageSellOfferResult::LowReserve => {}
        }
    }
}
