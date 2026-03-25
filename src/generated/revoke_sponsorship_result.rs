#[allow(unused_imports)]
use super::*;
/// RevokeSponsorshipResult is an XDR Union defined as:
///
/// ```text
/// union RevokeSponsorshipResult switch (RevokeSponsorshipResultCode code)
/// {
/// case REVOKE_SPONSORSHIP_SUCCESS:
///     void;
/// case REVOKE_SPONSORSHIP_DOES_NOT_EXIST:
/// case REVOKE_SPONSORSHIP_NOT_SPONSOR:
/// case REVOKE_SPONSORSHIP_LOW_RESERVE:
/// case REVOKE_SPONSORSHIP_ONLY_TRANSFERABLE:
/// case REVOKE_SPONSORSHIP_MALFORMED:
///     void;
/// };
/// ```
///
// union with discriminant RevokeSponsorshipResultCode
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
pub enum RevokeSponsorshipResult {
    Success,
    DoesNotExist,
    NotSponsor,
    LowReserve,
    OnlyTransferable,
    Malformed,
}

#[cfg(feature = "alloc")]
impl Default for RevokeSponsorshipResult {
    fn default() -> Self {
        Self::Success
    }
}

impl RevokeSponsorshipResult {
    const _VARIANTS: &[RevokeSponsorshipResultCode] = &[
        RevokeSponsorshipResultCode::Success,
        RevokeSponsorshipResultCode::DoesNotExist,
        RevokeSponsorshipResultCode::NotSponsor,
        RevokeSponsorshipResultCode::LowReserve,
        RevokeSponsorshipResultCode::OnlyTransferable,
        RevokeSponsorshipResultCode::Malformed,
    ];
    pub const VARIANTS: [RevokeSponsorshipResultCode; Self::_VARIANTS.len()] = {
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
        "NotSponsor",
        "LowReserve",
        "OnlyTransferable",
        "Malformed",
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
            Self::NotSponsor => "NotSponsor",
            Self::LowReserve => "LowReserve",
            Self::OnlyTransferable => "OnlyTransferable",
            Self::Malformed => "Malformed",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> RevokeSponsorshipResultCode {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => RevokeSponsorshipResultCode::Success,
            Self::DoesNotExist => RevokeSponsorshipResultCode::DoesNotExist,
            Self::NotSponsor => RevokeSponsorshipResultCode::NotSponsor,
            Self::LowReserve => RevokeSponsorshipResultCode::LowReserve,
            Self::OnlyTransferable => RevokeSponsorshipResultCode::OnlyTransferable,
            Self::Malformed => RevokeSponsorshipResultCode::Malformed,
        }
    }

    #[must_use]
    pub const fn variants() -> [RevokeSponsorshipResultCode; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for RevokeSponsorshipResult {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<RevokeSponsorshipResultCode> for RevokeSponsorshipResult {
    #[must_use]
    fn discriminant(&self) -> RevokeSponsorshipResultCode {
        Self::discriminant(self)
    }
}

impl Variants<RevokeSponsorshipResultCode> for RevokeSponsorshipResult {
    fn variants() -> slice::Iter<'static, RevokeSponsorshipResultCode> {
        Self::VARIANTS.iter()
    }
}

impl Union<RevokeSponsorshipResultCode> for RevokeSponsorshipResult {}

impl ReadXdr for RevokeSponsorshipResult {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: RevokeSponsorshipResultCode =
                <RevokeSponsorshipResultCode as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                RevokeSponsorshipResultCode::Success => Self::Success,
                RevokeSponsorshipResultCode::DoesNotExist => Self::DoesNotExist,
                RevokeSponsorshipResultCode::NotSponsor => Self::NotSponsor,
                RevokeSponsorshipResultCode::LowReserve => Self::LowReserve,
                RevokeSponsorshipResultCode::OnlyTransferable => Self::OnlyTransferable,
                RevokeSponsorshipResultCode::Malformed => Self::Malformed,
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for RevokeSponsorshipResult {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::Success => ().write_xdr(w)?,
                Self::DoesNotExist => ().write_xdr(w)?,
                Self::NotSponsor => ().write_xdr(w)?,
                Self::LowReserve => ().write_xdr(w)?,
                Self::OnlyTransferable => ().write_xdr(w)?,
                Self::Malformed => ().write_xdr(w)?,
            };
            Ok(())
        })
    }
}
