#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ManageBuyOfferResult is a borrowing equivalent of [`ManageBuyOfferResult`](super::super::ManageBuyOfferResult)
/// over `'static` data, for const XDR encoding.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(clippy::large_enum_variant)]
pub enum ManageBuyOfferResult {
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

impl ManageBuyOfferResult {
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
}

impl ManageBuyOfferResult {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_manage_buy_offer_result(self);
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
        w.write_type_manage_buy_offer_result(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

impl ConstWriter<'_> {
    /// Serializes a [`ManageBuyOfferResult`], mirroring `<ManageBuyOfferResult as WriteXdr>::write_xdr`.
    pub const fn write_type_manage_buy_offer_result(&mut self, v: &ManageBuyOfferResult) {
        let d = v.discriminant();
        self.write_type_manage_buy_offer_result_code(&d);
        #[allow(clippy::match_same_arms)]
        match v {
            ManageBuyOfferResult::Success(value) => {
                self.write_type_manage_offer_success_result(value);
            }
            ManageBuyOfferResult::Malformed => {}
            ManageBuyOfferResult::SellNoTrust => {}
            ManageBuyOfferResult::BuyNoTrust => {}
            ManageBuyOfferResult::SellNotAuthorized => {}
            ManageBuyOfferResult::BuyNotAuthorized => {}
            ManageBuyOfferResult::LineFull => {}
            ManageBuyOfferResult::Underfunded => {}
            ManageBuyOfferResult::CrossSelf => {}
            ManageBuyOfferResult::SellNoIssuer => {}
            ManageBuyOfferResult::BuyNoIssuer => {}
            ManageBuyOfferResult::NotFound => {}
            ManageBuyOfferResult::LowReserve => {}
        }
    }
}
