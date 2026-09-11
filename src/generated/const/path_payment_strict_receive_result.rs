#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// PathPaymentStrictReceiveResult is a borrowing equivalent of [`PathPaymentStrictReceiveResult`](super::super::PathPaymentStrictReceiveResult)
/// over `'static` data, for const XDR encoding.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[allow(clippy::large_enum_variant)]
pub enum PathPaymentStrictReceiveResult {
    Success(PathPaymentStrictReceiveResultSuccess),
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
    OverSendmax,
}

impl PathPaymentStrictReceiveResult {
    #[must_use]
    pub const fn discriminant(&self) -> PathPaymentStrictReceiveResultCode {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success(_) => PathPaymentStrictReceiveResultCode::Success,
            Self::Malformed => PathPaymentStrictReceiveResultCode::Malformed,
            Self::Underfunded => PathPaymentStrictReceiveResultCode::Underfunded,
            Self::SrcNoTrust => PathPaymentStrictReceiveResultCode::SrcNoTrust,
            Self::SrcNotAuthorized => PathPaymentStrictReceiveResultCode::SrcNotAuthorized,
            Self::NoDestination => PathPaymentStrictReceiveResultCode::NoDestination,
            Self::NoTrust => PathPaymentStrictReceiveResultCode::NoTrust,
            Self::NotAuthorized => PathPaymentStrictReceiveResultCode::NotAuthorized,
            Self::LineFull => PathPaymentStrictReceiveResultCode::LineFull,
            Self::NoIssuer(_) => PathPaymentStrictReceiveResultCode::NoIssuer,
            Self::TooFewOffers => PathPaymentStrictReceiveResultCode::TooFewOffers,
            Self::OfferCrossSelf => PathPaymentStrictReceiveResultCode::OfferCrossSelf,
            Self::OverSendmax => PathPaymentStrictReceiveResultCode::OverSendmax,
        }
    }
}

impl PathPaymentStrictReceiveResult {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_path_payment_strict_receive_result(self);
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
        w.write_type_path_payment_strict_receive_result(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

impl ConstWriter<'_> {
    /// Serializes a [`PathPaymentStrictReceiveResult`], mirroring `<PathPaymentStrictReceiveResult as WriteXdr>::write_xdr`.
    pub const fn write_type_path_payment_strict_receive_result(
        &mut self,
        v: &PathPaymentStrictReceiveResult,
    ) {
        let d = v.discriminant();
        self.write_type_path_payment_strict_receive_result_code(&d);
        #[allow(clippy::match_same_arms)]
        match v {
            PathPaymentStrictReceiveResult::Success(value) => {
                self.write_type_path_payment_strict_receive_result_success(value);
            }
            PathPaymentStrictReceiveResult::Malformed => {}
            PathPaymentStrictReceiveResult::Underfunded => {}
            PathPaymentStrictReceiveResult::SrcNoTrust => {}
            PathPaymentStrictReceiveResult::SrcNotAuthorized => {}
            PathPaymentStrictReceiveResult::NoDestination => {}
            PathPaymentStrictReceiveResult::NoTrust => {}
            PathPaymentStrictReceiveResult::NotAuthorized => {}
            PathPaymentStrictReceiveResult::LineFull => {}
            PathPaymentStrictReceiveResult::NoIssuer(value) => {
                self.write_type_asset(value);
            }
            PathPaymentStrictReceiveResult::TooFewOffers => {}
            PathPaymentStrictReceiveResult::OfferCrossSelf => {}
            PathPaymentStrictReceiveResult::OverSendmax => {}
        }
    }
}
