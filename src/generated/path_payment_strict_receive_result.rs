#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
/// PathPaymentStrictReceiveResult is an XDR Union defined as:
///
/// ```text
/// union PathPaymentStrictReceiveResult switch (
///     PathPaymentStrictReceiveResultCode code)
/// {
/// case PATH_PAYMENT_STRICT_RECEIVE_SUCCESS:
///     struct
///     {
///         ClaimAtom offers<>;
///         SimplePaymentResult last;
///     } success;
/// case PATH_PAYMENT_STRICT_RECEIVE_MALFORMED:
/// case PATH_PAYMENT_STRICT_RECEIVE_UNDERFUNDED:
/// case PATH_PAYMENT_STRICT_RECEIVE_SRC_NO_TRUST:
/// case PATH_PAYMENT_STRICT_RECEIVE_SRC_NOT_AUTHORIZED:
/// case PATH_PAYMENT_STRICT_RECEIVE_NO_DESTINATION:
/// case PATH_PAYMENT_STRICT_RECEIVE_NO_TRUST:
/// case PATH_PAYMENT_STRICT_RECEIVE_NOT_AUTHORIZED:
/// case PATH_PAYMENT_STRICT_RECEIVE_LINE_FULL:
///     void;
/// case PATH_PAYMENT_STRICT_RECEIVE_NO_ISSUER:
///     Asset noIssuer; // the asset that caused the error
/// case PATH_PAYMENT_STRICT_RECEIVE_TOO_FEW_OFFERS:
/// case PATH_PAYMENT_STRICT_RECEIVE_OFFER_CROSS_SELF:
/// case PATH_PAYMENT_STRICT_RECEIVE_OVER_SENDMAX:
///     void;
/// };
/// ```
///
// union with discriminant PathPaymentStrictReceiveResultCode
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

#[cfg(feature = "alloc")]
impl Default for PathPaymentStrictReceiveResult {
    fn default() -> Self {
        Self::Success(PathPaymentStrictReceiveResultSuccess::default())
    }
}

impl PathPaymentStrictReceiveResult {
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
            Self::Success(_) => "Success",
            Self::Malformed => "Malformed",
            Self::Underfunded => "Underfunded",
            Self::SrcNoTrust => "SrcNoTrust",
            Self::SrcNotAuthorized => "SrcNotAuthorized",
            Self::NoDestination => "NoDestination",
            Self::NoTrust => "NoTrust",
            Self::NotAuthorized => "NotAuthorized",
            Self::LineFull => "LineFull",
            Self::NoIssuer(_) => "NoIssuer",
            Self::TooFewOffers => "TooFewOffers",
            Self::OfferCrossSelf => "OfferCrossSelf",
            Self::OverSendmax => "OverSendmax",
        }
    }

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

    #[must_use]
    pub const fn variants() -> [PathPaymentStrictReceiveResultCode; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for PathPaymentStrictReceiveResult {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<PathPaymentStrictReceiveResultCode> for PathPaymentStrictReceiveResult {
    #[must_use]
    fn discriminant(&self) -> PathPaymentStrictReceiveResultCode {
        Self::discriminant(self)
    }
}

impl Variants<PathPaymentStrictReceiveResultCode> for PathPaymentStrictReceiveResult {
    fn variants() -> slice::Iter<'static, PathPaymentStrictReceiveResultCode> {
        Self::VARIANTS.iter()
    }
}

impl Union<PathPaymentStrictReceiveResultCode> for PathPaymentStrictReceiveResult {}

impl ReadXdr for PathPaymentStrictReceiveResult {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: PathPaymentStrictReceiveResultCode =
                <PathPaymentStrictReceiveResultCode as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                PathPaymentStrictReceiveResultCode::Success => {
                    Self::Success(PathPaymentStrictReceiveResultSuccess::read_xdr(r)?)
                }
                PathPaymentStrictReceiveResultCode::Malformed => Self::Malformed,
                PathPaymentStrictReceiveResultCode::Underfunded => Self::Underfunded,
                PathPaymentStrictReceiveResultCode::SrcNoTrust => Self::SrcNoTrust,
                PathPaymentStrictReceiveResultCode::SrcNotAuthorized => Self::SrcNotAuthorized,
                PathPaymentStrictReceiveResultCode::NoDestination => Self::NoDestination,
                PathPaymentStrictReceiveResultCode::NoTrust => Self::NoTrust,
                PathPaymentStrictReceiveResultCode::NotAuthorized => Self::NotAuthorized,
                PathPaymentStrictReceiveResultCode::LineFull => Self::LineFull,
                PathPaymentStrictReceiveResultCode::NoIssuer => Self::NoIssuer(Asset::read_xdr(r)?),
                PathPaymentStrictReceiveResultCode::TooFewOffers => Self::TooFewOffers,
                PathPaymentStrictReceiveResultCode::OfferCrossSelf => Self::OfferCrossSelf,
                PathPaymentStrictReceiveResultCode::OverSendmax => Self::OverSendmax,
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for PathPaymentStrictReceiveResult {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::Success(v) => v.write_xdr(w)?,
                Self::Malformed => ().write_xdr(w)?,
                Self::Underfunded => ().write_xdr(w)?,
                Self::SrcNoTrust => ().write_xdr(w)?,
                Self::SrcNotAuthorized => ().write_xdr(w)?,
                Self::NoDestination => ().write_xdr(w)?,
                Self::NoTrust => ().write_xdr(w)?,
                Self::NotAuthorized => ().write_xdr(w)?,
                Self::LineFull => ().write_xdr(w)?,
                Self::NoIssuer(v) => v.write_xdr(w)?,
                Self::TooFewOffers => ().write_xdr(w)?,
                Self::OfferCrossSelf => ().write_xdr(w)?,
                Self::OverSendmax => ().write_xdr(w)?,
            };
            Ok(())
        })
    }
}
