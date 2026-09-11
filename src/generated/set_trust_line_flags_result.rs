#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// SetTrustLineFlagsResult is an XDR Union defined as:
///
/// ```text
/// union SetTrustLineFlagsResult switch (SetTrustLineFlagsResultCode code)
/// {
/// case SET_TRUST_LINE_FLAGS_SUCCESS:
///     void;
/// case SET_TRUST_LINE_FLAGS_MALFORMED:
/// case SET_TRUST_LINE_FLAGS_NO_TRUST_LINE:
/// case SET_TRUST_LINE_FLAGS_CANT_REVOKE:
/// case SET_TRUST_LINE_FLAGS_INVALID_STATE:
/// case SET_TRUST_LINE_FLAGS_LOW_RESERVE:
///     void;
/// };
/// ```
///
// union with discriminant SetTrustLineFlagsResultCode
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
pub enum SetTrustLineFlagsResult {
    Success,
    Malformed,
    NoTrustLine,
    CantRevoke,
    InvalidState,
    LowReserve,
}

#[cfg(feature = "alloc")]
impl Default for SetTrustLineFlagsResult {
    fn default() -> Self {
        Self::Success
    }
}

impl SetTrustLineFlagsResult {
    const _VARIANTS: &[SetTrustLineFlagsResultCode] = &[
        SetTrustLineFlagsResultCode::Success,
        SetTrustLineFlagsResultCode::Malformed,
        SetTrustLineFlagsResultCode::NoTrustLine,
        SetTrustLineFlagsResultCode::CantRevoke,
        SetTrustLineFlagsResultCode::InvalidState,
        SetTrustLineFlagsResultCode::LowReserve,
    ];
    pub const VARIANTS: [SetTrustLineFlagsResultCode; Self::_VARIANTS.len()] = {
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
        "NoTrustLine",
        "CantRevoke",
        "InvalidState",
        "LowReserve",
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
            Self::NoTrustLine => "NoTrustLine",
            Self::CantRevoke => "CantRevoke",
            Self::InvalidState => "InvalidState",
            Self::LowReserve => "LowReserve",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> SetTrustLineFlagsResultCode {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => SetTrustLineFlagsResultCode::Success,
            Self::Malformed => SetTrustLineFlagsResultCode::Malformed,
            Self::NoTrustLine => SetTrustLineFlagsResultCode::NoTrustLine,
            Self::CantRevoke => SetTrustLineFlagsResultCode::CantRevoke,
            Self::InvalidState => SetTrustLineFlagsResultCode::InvalidState,
            Self::LowReserve => SetTrustLineFlagsResultCode::LowReserve,
        }
    }

    #[must_use]
    pub const fn variants() -> [SetTrustLineFlagsResultCode; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for SetTrustLineFlagsResult {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<SetTrustLineFlagsResultCode> for SetTrustLineFlagsResult {
    #[must_use]
    fn discriminant(&self) -> SetTrustLineFlagsResultCode {
        Self::discriminant(self)
    }
}

impl Variants<SetTrustLineFlagsResultCode> for SetTrustLineFlagsResult {
    fn variants() -> slice::Iter<'static, SetTrustLineFlagsResultCode> {
        Self::VARIANTS.iter()
    }
}

impl Union<SetTrustLineFlagsResultCode> for SetTrustLineFlagsResult {}

impl ReadXdr for SetTrustLineFlagsResult {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: SetTrustLineFlagsResultCode =
                <SetTrustLineFlagsResultCode as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                SetTrustLineFlagsResultCode::Success => Self::Success,
                SetTrustLineFlagsResultCode::Malformed => Self::Malformed,
                SetTrustLineFlagsResultCode::NoTrustLine => Self::NoTrustLine,
                SetTrustLineFlagsResultCode::CantRevoke => Self::CantRevoke,
                SetTrustLineFlagsResultCode::InvalidState => Self::InvalidState,
                SetTrustLineFlagsResultCode::LowReserve => Self::LowReserve,
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for SetTrustLineFlagsResult {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::Success => ().write_xdr(w)?,
                Self::Malformed => ().write_xdr(w)?,
                Self::NoTrustLine => ().write_xdr(w)?,
                Self::CantRevoke => ().write_xdr(w)?,
                Self::InvalidState => ().write_xdr(w)?,
                Self::LowReserve => ().write_xdr(w)?,
            };
            Ok(())
        })
    }
}
