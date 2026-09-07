#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// PathPaymentStrictSendResult is an XDR Union defined as:
///
/// ```text
/// union PathPaymentStrictSendResult switch (PathPaymentStrictSendResultCode code)
/// {
/// case PATH_PAYMENT_STRICT_SEND_SUCCESS:
///     struct
///     {
///         ClaimAtom offers<>;
///         SimplePaymentResult last;
///     } success;
/// case PATH_PAYMENT_STRICT_SEND_MALFORMED:
/// case PATH_PAYMENT_STRICT_SEND_UNDERFUNDED:
/// case PATH_PAYMENT_STRICT_SEND_SRC_NO_TRUST:
/// case PATH_PAYMENT_STRICT_SEND_SRC_NOT_AUTHORIZED:
/// case PATH_PAYMENT_STRICT_SEND_NO_DESTINATION:
/// case PATH_PAYMENT_STRICT_SEND_NO_TRUST:
/// case PATH_PAYMENT_STRICT_SEND_NOT_AUTHORIZED:
/// case PATH_PAYMENT_STRICT_SEND_LINE_FULL:
///     void;
/// case PATH_PAYMENT_STRICT_SEND_NO_ISSUER:
///     Asset noIssuer; // the asset that caused the error
/// case PATH_PAYMENT_STRICT_SEND_TOO_FEW_OFFERS:
/// case PATH_PAYMENT_STRICT_SEND_OFFER_CROSS_SELF:
/// case PATH_PAYMENT_STRICT_SEND_UNDER_DESTMIN:
///     void;
/// };
/// ```
///
// union with discriminant PathPaymentStrictSendResultCode
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval)]
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

#[cfg(feature = "alloc")]
impl Default for PathPaymentStrictSendResult {
    fn default() -> Self {
        Self::Success(PathPaymentStrictSendResultSuccess::default())
    }
}

impl PathPaymentStrictSendResult {
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
            Self::UnderDestmin => "UnderDestmin",
        }
    }

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

    #[must_use]
    pub const fn variants() -> [PathPaymentStrictSendResultCode; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for PathPaymentStrictSendResult {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<PathPaymentStrictSendResultCode> for PathPaymentStrictSendResult {
    #[must_use]
    fn discriminant(&self) -> PathPaymentStrictSendResultCode {
        Self::discriminant(self)
    }
}

impl Variants<PathPaymentStrictSendResultCode> for PathPaymentStrictSendResult {
    fn variants() -> slice::Iter<'static, PathPaymentStrictSendResultCode> {
        Self::VARIANTS.iter()
    }
}

impl Union<PathPaymentStrictSendResultCode> for PathPaymentStrictSendResult {}

impl ReadXdr for PathPaymentStrictSendResult {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: PathPaymentStrictSendResultCode =
                <PathPaymentStrictSendResultCode as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                PathPaymentStrictSendResultCode::Success => {
                    Self::Success(PathPaymentStrictSendResultSuccess::read_xdr(r)?)
                }
                PathPaymentStrictSendResultCode::Malformed => Self::Malformed,
                PathPaymentStrictSendResultCode::Underfunded => Self::Underfunded,
                PathPaymentStrictSendResultCode::SrcNoTrust => Self::SrcNoTrust,
                PathPaymentStrictSendResultCode::SrcNotAuthorized => Self::SrcNotAuthorized,
                PathPaymentStrictSendResultCode::NoDestination => Self::NoDestination,
                PathPaymentStrictSendResultCode::NoTrust => Self::NoTrust,
                PathPaymentStrictSendResultCode::NotAuthorized => Self::NotAuthorized,
                PathPaymentStrictSendResultCode::LineFull => Self::LineFull,
                PathPaymentStrictSendResultCode::NoIssuer => Self::NoIssuer(Asset::read_xdr(r)?),
                PathPaymentStrictSendResultCode::TooFewOffers => Self::TooFewOffers,
                PathPaymentStrictSendResultCode::OfferCrossSelf => Self::OfferCrossSelf,
                PathPaymentStrictSendResultCode::UnderDestmin => Self::UnderDestmin,
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for PathPaymentStrictSendResult {
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
                Self::UnderDestmin => ().write_xdr(w)?,
            };
            Ok(())
        })
    }
}

/// PathPaymentStrictSendResultRef is a borrowing equivalent of [`PathPaymentStrictSendResult`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(clippy::large_enum_variant)]
pub enum PathPaymentStrictSendResultRef<'a> {
    Success(PathPaymentStrictSendResultSuccessRef<'a>),
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

#[cfg(feature = "alloc")]
impl IntoOwned for PathPaymentStrictSendResultRef<'_> {
    type Owned = PathPaymentStrictSendResult;
    fn into_owned(self) -> PathPaymentStrictSendResult {
        #[allow(clippy::match_same_arms)]
        match self {
            PathPaymentStrictSendResultRef::Success(value) => {
                PathPaymentStrictSendResult::Success(value.into_owned())
            }
            PathPaymentStrictSendResultRef::Malformed => PathPaymentStrictSendResult::Malformed,
            PathPaymentStrictSendResultRef::Underfunded => PathPaymentStrictSendResult::Underfunded,
            PathPaymentStrictSendResultRef::SrcNoTrust => PathPaymentStrictSendResult::SrcNoTrust,
            PathPaymentStrictSendResultRef::SrcNotAuthorized => {
                PathPaymentStrictSendResult::SrcNotAuthorized
            }
            PathPaymentStrictSendResultRef::NoDestination => {
                PathPaymentStrictSendResult::NoDestination
            }
            PathPaymentStrictSendResultRef::NoTrust => PathPaymentStrictSendResult::NoTrust,
            PathPaymentStrictSendResultRef::NotAuthorized => {
                PathPaymentStrictSendResult::NotAuthorized
            }
            PathPaymentStrictSendResultRef::LineFull => PathPaymentStrictSendResult::LineFull,
            PathPaymentStrictSendResultRef::NoIssuer(value) => {
                PathPaymentStrictSendResult::NoIssuer(value.into_owned())
            }
            PathPaymentStrictSendResultRef::TooFewOffers => {
                PathPaymentStrictSendResult::TooFewOffers
            }
            PathPaymentStrictSendResultRef::OfferCrossSelf => {
                PathPaymentStrictSendResult::OfferCrossSelf
            }
            PathPaymentStrictSendResultRef::UnderDestmin => {
                PathPaymentStrictSendResult::UnderDestmin
            }
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&PathPaymentStrictSendResultRef<'_>> for PathPaymentStrictSendResult {
    #[must_use]
    fn from(v: &PathPaymentStrictSendResultRef<'_>) -> Self {
        v.into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<PathPaymentStrictSendResultRef<'_>> for PathPaymentStrictSendResult {
    #[must_use]
    fn from(v: PathPaymentStrictSendResultRef<'_>) -> Self {
        v.into_owned()
    }
}

impl PathPaymentStrictSendResultRef<'_> {
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

impl WriteXdr for PathPaymentStrictSendResultRef<'_> {
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
                Self::UnderDestmin => ().write_xdr(w)?,
            };
            Ok(())
        })
    }
}

#[cfg(feature = "const")]
impl PathPaymentStrictSendResultRef<'_> {
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
    /// operations. This is the const counterpart to [`WriteXdr::to_xdr`].
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

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`PathPaymentStrictSendResult`], mirroring `<PathPaymentStrictSendResult as WriteXdr>::write_xdr`.
    pub const fn write_type_path_payment_strict_send_result(
        &mut self,
        v: &PathPaymentStrictSendResultRef<'_>,
    ) {
        let d = v.discriminant();
        self.write_type_path_payment_strict_send_result_code(&d);
        #[allow(clippy::match_same_arms)]
        match v {
            PathPaymentStrictSendResultRef::Success(value) => {
                self.write_type_path_payment_strict_send_result_success(value);
            }
            PathPaymentStrictSendResultRef::Malformed => {}
            PathPaymentStrictSendResultRef::Underfunded => {}
            PathPaymentStrictSendResultRef::SrcNoTrust => {}
            PathPaymentStrictSendResultRef::SrcNotAuthorized => {}
            PathPaymentStrictSendResultRef::NoDestination => {}
            PathPaymentStrictSendResultRef::NoTrust => {}
            PathPaymentStrictSendResultRef::NotAuthorized => {}
            PathPaymentStrictSendResultRef::LineFull => {}
            PathPaymentStrictSendResultRef::NoIssuer(value) => {
                self.write_type_asset(value);
            }
            PathPaymentStrictSendResultRef::TooFewOffers => {}
            PathPaymentStrictSendResultRef::OfferCrossSelf => {}
            PathPaymentStrictSendResultRef::UnderDestmin => {}
        }
    }
}
