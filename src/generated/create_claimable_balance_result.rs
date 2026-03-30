#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// CreateClaimableBalanceResult is an XDR Union defined as:
///
/// ```text
/// union CreateClaimableBalanceResult switch (
///     CreateClaimableBalanceResultCode code)
/// {
/// case CREATE_CLAIMABLE_BALANCE_SUCCESS:
///     ClaimableBalanceID balanceID;
/// case CREATE_CLAIMABLE_BALANCE_MALFORMED:
/// case CREATE_CLAIMABLE_BALANCE_LOW_RESERVE:
/// case CREATE_CLAIMABLE_BALANCE_NO_TRUST:
/// case CREATE_CLAIMABLE_BALANCE_NOT_AUTHORIZED:
/// case CREATE_CLAIMABLE_BALANCE_UNDERFUNDED:
///     void;
/// };
/// ```
///
// union with discriminant CreateClaimableBalanceResultCode
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
pub enum CreateClaimableBalanceResult {
    Success(ClaimableBalanceId),
    Malformed,
    LowReserve,
    NoTrust,
    NotAuthorized,
    Underfunded,
}

#[cfg(feature = "alloc")]
impl Default for CreateClaimableBalanceResult {
    fn default() -> Self {
        Self::Success(ClaimableBalanceId::default())
    }
}

impl CreateClaimableBalanceResult {
    const _VARIANTS: &[CreateClaimableBalanceResultCode] = &[
        CreateClaimableBalanceResultCode::Success,
        CreateClaimableBalanceResultCode::Malformed,
        CreateClaimableBalanceResultCode::LowReserve,
        CreateClaimableBalanceResultCode::NoTrust,
        CreateClaimableBalanceResultCode::NotAuthorized,
        CreateClaimableBalanceResultCode::Underfunded,
    ];
    pub const VARIANTS: [CreateClaimableBalanceResultCode; Self::_VARIANTS.len()] = {
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
        "LowReserve",
        "NoTrust",
        "NotAuthorized",
        "Underfunded",
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
            Self::LowReserve => "LowReserve",
            Self::NoTrust => "NoTrust",
            Self::NotAuthorized => "NotAuthorized",
            Self::Underfunded => "Underfunded",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> CreateClaimableBalanceResultCode {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success(_) => CreateClaimableBalanceResultCode::Success,
            Self::Malformed => CreateClaimableBalanceResultCode::Malformed,
            Self::LowReserve => CreateClaimableBalanceResultCode::LowReserve,
            Self::NoTrust => CreateClaimableBalanceResultCode::NoTrust,
            Self::NotAuthorized => CreateClaimableBalanceResultCode::NotAuthorized,
            Self::Underfunded => CreateClaimableBalanceResultCode::Underfunded,
        }
    }

    #[must_use]
    pub const fn variants() -> [CreateClaimableBalanceResultCode; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for CreateClaimableBalanceResult {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<CreateClaimableBalanceResultCode> for CreateClaimableBalanceResult {
    #[must_use]
    fn discriminant(&self) -> CreateClaimableBalanceResultCode {
        Self::discriminant(self)
    }
}

impl Variants<CreateClaimableBalanceResultCode> for CreateClaimableBalanceResult {
    fn variants() -> slice::Iter<'static, CreateClaimableBalanceResultCode> {
        Self::VARIANTS.iter()
    }
}

impl Union<CreateClaimableBalanceResultCode> for CreateClaimableBalanceResult {}

impl ReadXdr for CreateClaimableBalanceResult {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: CreateClaimableBalanceResultCode =
                <CreateClaimableBalanceResultCode as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                CreateClaimableBalanceResultCode::Success => {
                    Self::Success(ClaimableBalanceId::read_xdr(r)?)
                }
                CreateClaimableBalanceResultCode::Malformed => Self::Malformed,
                CreateClaimableBalanceResultCode::LowReserve => Self::LowReserve,
                CreateClaimableBalanceResultCode::NoTrust => Self::NoTrust,
                CreateClaimableBalanceResultCode::NotAuthorized => Self::NotAuthorized,
                CreateClaimableBalanceResultCode::Underfunded => Self::Underfunded,
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for CreateClaimableBalanceResult {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::Success(v) => v.write_xdr(w)?,
                Self::Malformed => ().write_xdr(w)?,
                Self::LowReserve => ().write_xdr(w)?,
                Self::NoTrust => ().write_xdr(w)?,
                Self::NotAuthorized => ().write_xdr(w)?,
                Self::Underfunded => ().write_xdr(w)?,
            };
            Ok(())
        })
    }
}
