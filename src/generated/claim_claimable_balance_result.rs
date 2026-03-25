#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
/// ClaimClaimableBalanceResult is an XDR Union defined as:
///
/// ```text
/// union ClaimClaimableBalanceResult switch (ClaimClaimableBalanceResultCode code)
/// {
/// case CLAIM_CLAIMABLE_BALANCE_SUCCESS:
///     void;
/// case CLAIM_CLAIMABLE_BALANCE_DOES_NOT_EXIST:
/// case CLAIM_CLAIMABLE_BALANCE_CANNOT_CLAIM:
/// case CLAIM_CLAIMABLE_BALANCE_LINE_FULL:
/// case CLAIM_CLAIMABLE_BALANCE_NO_TRUST:
/// case CLAIM_CLAIMABLE_BALANCE_NOT_AUTHORIZED:
/// case CLAIM_CLAIMABLE_BALANCE_TRUSTLINE_FROZEN:
///     void;
/// };
/// ```
///
// union with discriminant ClaimClaimableBalanceResultCode
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
pub enum ClaimClaimableBalanceResult {
    Success,
    DoesNotExist,
    CannotClaim,
    LineFull,
    NoTrust,
    NotAuthorized,
    TrustlineFrozen,
}

#[cfg(feature = "alloc")]
impl Default for ClaimClaimableBalanceResult {
    fn default() -> Self {
        Self::Success
    }
}

impl ClaimClaimableBalanceResult {
    const _VARIANTS: &[ClaimClaimableBalanceResultCode] = &[
        ClaimClaimableBalanceResultCode::Success,
        ClaimClaimableBalanceResultCode::DoesNotExist,
        ClaimClaimableBalanceResultCode::CannotClaim,
        ClaimClaimableBalanceResultCode::LineFull,
        ClaimClaimableBalanceResultCode::NoTrust,
        ClaimClaimableBalanceResultCode::NotAuthorized,
        ClaimClaimableBalanceResultCode::TrustlineFrozen,
    ];
    pub const VARIANTS: [ClaimClaimableBalanceResultCode; Self::_VARIANTS.len()] = {
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
        "DoesNotExist",
        "CannotClaim",
        "LineFull",
        "NoTrust",
        "NotAuthorized",
        "TrustlineFrozen",
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
            Self::DoesNotExist => "DoesNotExist",
            Self::CannotClaim => "CannotClaim",
            Self::LineFull => "LineFull",
            Self::NoTrust => "NoTrust",
            Self::NotAuthorized => "NotAuthorized",
            Self::TrustlineFrozen => "TrustlineFrozen",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> ClaimClaimableBalanceResultCode {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => ClaimClaimableBalanceResultCode::Success,
            Self::DoesNotExist => ClaimClaimableBalanceResultCode::DoesNotExist,
            Self::CannotClaim => ClaimClaimableBalanceResultCode::CannotClaim,
            Self::LineFull => ClaimClaimableBalanceResultCode::LineFull,
            Self::NoTrust => ClaimClaimableBalanceResultCode::NoTrust,
            Self::NotAuthorized => ClaimClaimableBalanceResultCode::NotAuthorized,
            Self::TrustlineFrozen => ClaimClaimableBalanceResultCode::TrustlineFrozen,
        }
    }

    #[must_use]
    pub const fn variants() -> [ClaimClaimableBalanceResultCode; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for ClaimClaimableBalanceResult {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<ClaimClaimableBalanceResultCode> for ClaimClaimableBalanceResult {
    #[must_use]
    fn discriminant(&self) -> ClaimClaimableBalanceResultCode {
        Self::discriminant(self)
    }
}

impl Variants<ClaimClaimableBalanceResultCode> for ClaimClaimableBalanceResult {
    fn variants() -> slice::Iter<'static, ClaimClaimableBalanceResultCode> {
        Self::VARIANTS.iter()
    }
}

impl Union<ClaimClaimableBalanceResultCode> for ClaimClaimableBalanceResult {}

impl ReadXdr for ClaimClaimableBalanceResult {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: ClaimClaimableBalanceResultCode =
                <ClaimClaimableBalanceResultCode as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                ClaimClaimableBalanceResultCode::Success => Self::Success,
                ClaimClaimableBalanceResultCode::DoesNotExist => Self::DoesNotExist,
                ClaimClaimableBalanceResultCode::CannotClaim => Self::CannotClaim,
                ClaimClaimableBalanceResultCode::LineFull => Self::LineFull,
                ClaimClaimableBalanceResultCode::NoTrust => Self::NoTrust,
                ClaimClaimableBalanceResultCode::NotAuthorized => Self::NotAuthorized,
                ClaimClaimableBalanceResultCode::TrustlineFrozen => Self::TrustlineFrozen,
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for ClaimClaimableBalanceResult {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::Success => ().write_xdr(w)?,
                Self::DoesNotExist => ().write_xdr(w)?,
                Self::CannotClaim => ().write_xdr(w)?,
                Self::LineFull => ().write_xdr(w)?,
                Self::NoTrust => ().write_xdr(w)?,
                Self::NotAuthorized => ().write_xdr(w)?,
                Self::TrustlineFrozen => ().write_xdr(w)?,
            };
            Ok(())
        })
    }
}
