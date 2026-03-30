#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ChangeTrustResult is an XDR Union defined as:
///
/// ```text
/// union ChangeTrustResult switch (ChangeTrustResultCode code)
/// {
/// case CHANGE_TRUST_SUCCESS:
///     void;
/// case CHANGE_TRUST_MALFORMED:
/// case CHANGE_TRUST_NO_ISSUER:
/// case CHANGE_TRUST_INVALID_LIMIT:
/// case CHANGE_TRUST_LOW_RESERVE:
/// case CHANGE_TRUST_SELF_NOT_ALLOWED:
/// case CHANGE_TRUST_TRUST_LINE_MISSING:
/// case CHANGE_TRUST_CANNOT_DELETE:
/// case CHANGE_TRUST_NOT_AUTH_MAINTAIN_LIABILITIES:
///     void;
/// };
/// ```
///
// union with discriminant ChangeTrustResultCode
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
pub enum ChangeTrustResult {
    Success,
    Malformed,
    NoIssuer,
    InvalidLimit,
    LowReserve,
    SelfNotAllowed,
    TrustLineMissing,
    CannotDelete,
    NotAuthMaintainLiabilities,
}

#[cfg(feature = "alloc")]
impl Default for ChangeTrustResult {
    fn default() -> Self {
        Self::Success
    }
}

impl ChangeTrustResult {
    const _VARIANTS: &[ChangeTrustResultCode] = &[
        ChangeTrustResultCode::Success,
        ChangeTrustResultCode::Malformed,
        ChangeTrustResultCode::NoIssuer,
        ChangeTrustResultCode::InvalidLimit,
        ChangeTrustResultCode::LowReserve,
        ChangeTrustResultCode::SelfNotAllowed,
        ChangeTrustResultCode::TrustLineMissing,
        ChangeTrustResultCode::CannotDelete,
        ChangeTrustResultCode::NotAuthMaintainLiabilities,
    ];
    pub const VARIANTS: [ChangeTrustResultCode; Self::_VARIANTS.len()] = {
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
        "NoIssuer",
        "InvalidLimit",
        "LowReserve",
        "SelfNotAllowed",
        "TrustLineMissing",
        "CannotDelete",
        "NotAuthMaintainLiabilities",
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
            Self::NoIssuer => "NoIssuer",
            Self::InvalidLimit => "InvalidLimit",
            Self::LowReserve => "LowReserve",
            Self::SelfNotAllowed => "SelfNotAllowed",
            Self::TrustLineMissing => "TrustLineMissing",
            Self::CannotDelete => "CannotDelete",
            Self::NotAuthMaintainLiabilities => "NotAuthMaintainLiabilities",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> ChangeTrustResultCode {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => ChangeTrustResultCode::Success,
            Self::Malformed => ChangeTrustResultCode::Malformed,
            Self::NoIssuer => ChangeTrustResultCode::NoIssuer,
            Self::InvalidLimit => ChangeTrustResultCode::InvalidLimit,
            Self::LowReserve => ChangeTrustResultCode::LowReserve,
            Self::SelfNotAllowed => ChangeTrustResultCode::SelfNotAllowed,
            Self::TrustLineMissing => ChangeTrustResultCode::TrustLineMissing,
            Self::CannotDelete => ChangeTrustResultCode::CannotDelete,
            Self::NotAuthMaintainLiabilities => ChangeTrustResultCode::NotAuthMaintainLiabilities,
        }
    }

    #[must_use]
    pub const fn variants() -> [ChangeTrustResultCode; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for ChangeTrustResult {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<ChangeTrustResultCode> for ChangeTrustResult {
    #[must_use]
    fn discriminant(&self) -> ChangeTrustResultCode {
        Self::discriminant(self)
    }
}

impl Variants<ChangeTrustResultCode> for ChangeTrustResult {
    fn variants() -> slice::Iter<'static, ChangeTrustResultCode> {
        Self::VARIANTS.iter()
    }
}

impl Union<ChangeTrustResultCode> for ChangeTrustResult {}

impl ReadXdr for ChangeTrustResult {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: ChangeTrustResultCode = <ChangeTrustResultCode as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                ChangeTrustResultCode::Success => Self::Success,
                ChangeTrustResultCode::Malformed => Self::Malformed,
                ChangeTrustResultCode::NoIssuer => Self::NoIssuer,
                ChangeTrustResultCode::InvalidLimit => Self::InvalidLimit,
                ChangeTrustResultCode::LowReserve => Self::LowReserve,
                ChangeTrustResultCode::SelfNotAllowed => Self::SelfNotAllowed,
                ChangeTrustResultCode::TrustLineMissing => Self::TrustLineMissing,
                ChangeTrustResultCode::CannotDelete => Self::CannotDelete,
                ChangeTrustResultCode::NotAuthMaintainLiabilities => {
                    Self::NotAuthMaintainLiabilities
                }
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for ChangeTrustResult {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::Success => ().write_xdr(w)?,
                Self::Malformed => ().write_xdr(w)?,
                Self::NoIssuer => ().write_xdr(w)?,
                Self::InvalidLimit => ().write_xdr(w)?,
                Self::LowReserve => ().write_xdr(w)?,
                Self::SelfNotAllowed => ().write_xdr(w)?,
                Self::TrustLineMissing => ().write_xdr(w)?,
                Self::CannotDelete => ().write_xdr(w)?,
                Self::NotAuthMaintainLiabilities => ().write_xdr(w)?,
            };
            Ok(())
        })
    }
}
