#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// PaymentResult is an XDR Union defined as:
///
/// ```text
/// union PaymentResult switch (PaymentResultCode code)
/// {
/// case PAYMENT_SUCCESS:
///     void;
/// case PAYMENT_MALFORMED:
/// case PAYMENT_UNDERFUNDED:
/// case PAYMENT_SRC_NO_TRUST:
/// case PAYMENT_SRC_NOT_AUTHORIZED:
/// case PAYMENT_NO_DESTINATION:
/// case PAYMENT_NO_TRUST:
/// case PAYMENT_NOT_AUTHORIZED:
/// case PAYMENT_LINE_FULL:
/// case PAYMENT_NO_ISSUER:
///     void;
/// };
/// ```
///
// union with discriminant PaymentResultCode
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
pub enum PaymentResult {
    Success,
    Malformed,
    Underfunded,
    SrcNoTrust,
    SrcNotAuthorized,
    NoDestination,
    NoTrust,
    NotAuthorized,
    LineFull,
    NoIssuer,
}

#[cfg(feature = "alloc")]
impl Default for PaymentResult {
    fn default() -> Self {
        Self::Success
    }
}

impl PaymentResult {
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
    pub const fn discriminant(&self) -> PaymentResultCode {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => PaymentResultCode::Success,
            Self::Malformed => PaymentResultCode::Malformed,
            Self::Underfunded => PaymentResultCode::Underfunded,
            Self::SrcNoTrust => PaymentResultCode::SrcNoTrust,
            Self::SrcNotAuthorized => PaymentResultCode::SrcNotAuthorized,
            Self::NoDestination => PaymentResultCode::NoDestination,
            Self::NoTrust => PaymentResultCode::NoTrust,
            Self::NotAuthorized => PaymentResultCode::NotAuthorized,
            Self::LineFull => PaymentResultCode::LineFull,
            Self::NoIssuer => PaymentResultCode::NoIssuer,
        }
    }

    #[must_use]
    pub const fn variants() -> [PaymentResultCode; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for PaymentResult {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<PaymentResultCode> for PaymentResult {
    #[must_use]
    fn discriminant(&self) -> PaymentResultCode {
        Self::discriminant(self)
    }
}

impl Variants<PaymentResultCode> for PaymentResult {
    fn variants() -> slice::Iter<'static, PaymentResultCode> {
        Self::VARIANTS.iter()
    }
}

impl Union<PaymentResultCode> for PaymentResult {}

impl ReadXdr for PaymentResult {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: PaymentResultCode = <PaymentResultCode as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                PaymentResultCode::Success => Self::Success,
                PaymentResultCode::Malformed => Self::Malformed,
                PaymentResultCode::Underfunded => Self::Underfunded,
                PaymentResultCode::SrcNoTrust => Self::SrcNoTrust,
                PaymentResultCode::SrcNotAuthorized => Self::SrcNotAuthorized,
                PaymentResultCode::NoDestination => Self::NoDestination,
                PaymentResultCode::NoTrust => Self::NoTrust,
                PaymentResultCode::NotAuthorized => Self::NotAuthorized,
                PaymentResultCode::LineFull => Self::LineFull,
                PaymentResultCode::NoIssuer => Self::NoIssuer,
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl PaymentResult {
    /// Serialize this value as XDR into a [`ConstWriter`] using only const
    /// operations. This is the const counterpart to [`WriteXdr::write_xdr`].
    #[cfg(feature = "const")]
    pub const fn const_write_xdr(&self, w: &mut ConstWriter) {
        w.enter_depth();
        let d = self.discriminant();
        d.const_write_xdr(w);
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => {}
            Self::Malformed => {}
            Self::Underfunded => {}
            Self::SrcNoTrust => {}
            Self::SrcNotAuthorized => {}
            Self::NoDestination => {}
            Self::NoTrust => {}
            Self::NotAuthorized => {}
            Self::LineFull => {}
            Self::NoIssuer => {}
        }
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

impl WriteXdr for PaymentResult {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::Success => ().write_xdr(w)?,
                Self::Malformed => ().write_xdr(w)?,
                Self::Underfunded => ().write_xdr(w)?,
                Self::SrcNoTrust => ().write_xdr(w)?,
                Self::SrcNotAuthorized => ().write_xdr(w)?,
                Self::NoDestination => ().write_xdr(w)?,
                Self::NoTrust => ().write_xdr(w)?,
                Self::NotAuthorized => ().write_xdr(w)?,
                Self::LineFull => ().write_xdr(w)?,
                Self::NoIssuer => ().write_xdr(w)?,
            };
            Ok(())
        })
    }
}
