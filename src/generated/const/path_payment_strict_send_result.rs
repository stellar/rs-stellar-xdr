#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// PathPaymentStrictSendResult is a borrowing equivalent of [`PathPaymentStrictSendResult`](super::super::PathPaymentStrictSendResult)
/// over `'static` data, for const XDR encoding.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(clippy::large_enum_variant)]
pub enum PathPaymentStrictSendResult {
    Success(PathPaymentStrictSendResultSuccess),
    Malformed,
    Underfunded,
    SrcNoTrust,
    SrcNotAuthorized,
    NoDestination,
    NoTrust,
    NotAuthorized,
    LineFull,
    NoIssuer(Asset),
    TooFewOffers,
    OfferCrossSelf,
    UnderDestmin,
}

impl PathPaymentStrictSendResult {
    #[must_use]
    pub const fn discriminant(&self) -> PathPaymentStrictSendResultCode {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success(_) => PathPaymentStrictSendResultCode::Success,
            Self::Malformed => PathPaymentStrictSendResultCode::Malformed,
            Self::Underfunded => PathPaymentStrictSendResultCode::Underfunded,
            Self::SrcNoTrust => PathPaymentStrictSendResultCode::SrcNoTrust,
            Self::SrcNotAuthorized => PathPaymentStrictSendResultCode::SrcNotAuthorized,
            Self::NoDestination => PathPaymentStrictSendResultCode::NoDestination,
            Self::NoTrust => PathPaymentStrictSendResultCode::NoTrust,
            Self::NotAuthorized => PathPaymentStrictSendResultCode::NotAuthorized,
            Self::LineFull => PathPaymentStrictSendResultCode::LineFull,
            Self::NoIssuer(_) => PathPaymentStrictSendResultCode::NoIssuer,
            Self::TooFewOffers => PathPaymentStrictSendResultCode::TooFewOffers,
            Self::OfferCrossSelf => PathPaymentStrictSendResultCode::OfferCrossSelf,
            Self::UnderDestmin => PathPaymentStrictSendResultCode::UnderDestmin,
        }
    }
}

impl PathPaymentStrictSendResult {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_path_payment_strict_send_result(self);
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
        w.write_type_path_payment_strict_send_result(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

impl ConstWriter<'_> {
    /// Serializes a [`PathPaymentStrictSendResult`], mirroring `<PathPaymentStrictSendResult as WriteXdr>::write_xdr`.
    pub const fn write_type_path_payment_strict_send_result(
        &mut self,
        v: &PathPaymentStrictSendResult,
    ) {
        let d = v.discriminant();
        self.write_type_path_payment_strict_send_result_code(&d);
        #[allow(clippy::match_same_arms)]
        match v {
            PathPaymentStrictSendResult::Success(value) => {
                self.write_type_path_payment_strict_send_result_success(value);
            }
            PathPaymentStrictSendResult::Malformed => {}
            PathPaymentStrictSendResult::Underfunded => {}
            PathPaymentStrictSendResult::SrcNoTrust => {}
            PathPaymentStrictSendResult::SrcNotAuthorized => {}
            PathPaymentStrictSendResult::NoDestination => {}
            PathPaymentStrictSendResult::NoTrust => {}
            PathPaymentStrictSendResult::NotAuthorized => {}
            PathPaymentStrictSendResult::LineFull => {}
            PathPaymentStrictSendResult::NoIssuer(value) => {
                self.write_type_asset(value);
            }
            PathPaymentStrictSendResult::TooFewOffers => {}
            PathPaymentStrictSendResult::OfferCrossSelf => {}
            PathPaymentStrictSendResult::UnderDestmin => {}
        }
    }
}
