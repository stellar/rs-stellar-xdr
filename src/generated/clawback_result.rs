#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ClawbackResult is an XDR Union defined as:
///
/// ```text
/// union ClawbackResult switch (ClawbackResultCode code)
/// {
/// case CLAWBACK_SUCCESS:
///     void;
/// case CLAWBACK_MALFORMED:
/// case CLAWBACK_NOT_CLAWBACK_ENABLED:
/// case CLAWBACK_NO_TRUST:
/// case CLAWBACK_UNDERFUNDED:
///     void;
/// };
/// ```
///
// union with discriminant ClawbackResultCode
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
pub enum ClawbackResult {
    Success,
    Malformed,
    NotClawbackEnabled,
    NoTrust,
    Underfunded,
}

#[cfg(feature = "alloc")]
impl Default for ClawbackResult {
    fn default() -> Self {
        Self::Success
    }
}

impl ClawbackResult {
    const _VARIANTS: &[ClawbackResultCode] = &[
        ClawbackResultCode::Success,
        ClawbackResultCode::Malformed,
        ClawbackResultCode::NotClawbackEnabled,
        ClawbackResultCode::NoTrust,
        ClawbackResultCode::Underfunded,
    ];
    pub const VARIANTS: [ClawbackResultCode; Self::_VARIANTS.len()] = {
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
        "NotClawbackEnabled",
        "NoTrust",
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
            Self::Success => "Success",
            Self::Malformed => "Malformed",
            Self::NotClawbackEnabled => "NotClawbackEnabled",
            Self::NoTrust => "NoTrust",
            Self::Underfunded => "Underfunded",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> ClawbackResultCode {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => ClawbackResultCode::Success,
            Self::Malformed => ClawbackResultCode::Malformed,
            Self::NotClawbackEnabled => ClawbackResultCode::NotClawbackEnabled,
            Self::NoTrust => ClawbackResultCode::NoTrust,
            Self::Underfunded => ClawbackResultCode::Underfunded,
        }
    }

    #[must_use]
    pub const fn variants() -> [ClawbackResultCode; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for ClawbackResult {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<ClawbackResultCode> for ClawbackResult {
    #[must_use]
    fn discriminant(&self) -> ClawbackResultCode {
        Self::discriminant(self)
    }
}

impl Variants<ClawbackResultCode> for ClawbackResult {
    fn variants() -> slice::Iter<'static, ClawbackResultCode> {
        Self::VARIANTS.iter()
    }
}

impl Union<ClawbackResultCode> for ClawbackResult {}

impl ReadXdr for ClawbackResult {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: ClawbackResultCode = <ClawbackResultCode as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                ClawbackResultCode::Success => Self::Success,
                ClawbackResultCode::Malformed => Self::Malformed,
                ClawbackResultCode::NotClawbackEnabled => Self::NotClawbackEnabled,
                ClawbackResultCode::NoTrust => Self::NoTrust,
                ClawbackResultCode::Underfunded => Self::Underfunded,
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for ClawbackResult {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::Success => ().write_xdr(w)?,
                Self::Malformed => ().write_xdr(w)?,
                Self::NotClawbackEnabled => ().write_xdr(w)?,
                Self::NoTrust => ().write_xdr(w)?,
                Self::Underfunded => ().write_xdr(w)?,
            };
            Ok(())
        })
    }
}
