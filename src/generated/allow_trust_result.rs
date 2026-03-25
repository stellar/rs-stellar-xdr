#[allow(unused_imports)]
use super::*;
/// AllowTrustResult is an XDR Union defined as:
///
/// ```text
/// union AllowTrustResult switch (AllowTrustResultCode code)
/// {
/// case ALLOW_TRUST_SUCCESS:
///     void;
/// case ALLOW_TRUST_MALFORMED:
/// case ALLOW_TRUST_NO_TRUST_LINE:
/// case ALLOW_TRUST_TRUST_NOT_REQUIRED:
/// case ALLOW_TRUST_CANT_REVOKE:
/// case ALLOW_TRUST_SELF_NOT_ALLOWED:
/// case ALLOW_TRUST_LOW_RESERVE:
///     void;
/// };
/// ```
///
// union with discriminant AllowTrustResultCode
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
pub enum AllowTrustResult {
    Success,
    Malformed,
    NoTrustLine,
    TrustNotRequired,
    CantRevoke,
    SelfNotAllowed,
    LowReserve,
}

#[cfg(feature = "alloc")]
impl Default for AllowTrustResult {
    fn default() -> Self {
        Self::Success
    }
}

impl AllowTrustResult {
    const _VARIANTS: &[AllowTrustResultCode] = &[
        AllowTrustResultCode::Success,
        AllowTrustResultCode::Malformed,
        AllowTrustResultCode::NoTrustLine,
        AllowTrustResultCode::TrustNotRequired,
        AllowTrustResultCode::CantRevoke,
        AllowTrustResultCode::SelfNotAllowed,
        AllowTrustResultCode::LowReserve,
    ];
    pub const VARIANTS: [AllowTrustResultCode; Self::_VARIANTS.len()] = {
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
        "TrustNotRequired",
        "CantRevoke",
        "SelfNotAllowed",
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
            Self::TrustNotRequired => "TrustNotRequired",
            Self::CantRevoke => "CantRevoke",
            Self::SelfNotAllowed => "SelfNotAllowed",
            Self::LowReserve => "LowReserve",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> AllowTrustResultCode {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => AllowTrustResultCode::Success,
            Self::Malformed => AllowTrustResultCode::Malformed,
            Self::NoTrustLine => AllowTrustResultCode::NoTrustLine,
            Self::TrustNotRequired => AllowTrustResultCode::TrustNotRequired,
            Self::CantRevoke => AllowTrustResultCode::CantRevoke,
            Self::SelfNotAllowed => AllowTrustResultCode::SelfNotAllowed,
            Self::LowReserve => AllowTrustResultCode::LowReserve,
        }
    }

    #[must_use]
    pub const fn variants() -> [AllowTrustResultCode; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for AllowTrustResult {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<AllowTrustResultCode> for AllowTrustResult {
    #[must_use]
    fn discriminant(&self) -> AllowTrustResultCode {
        Self::discriminant(self)
    }
}

impl Variants<AllowTrustResultCode> for AllowTrustResult {
    fn variants() -> slice::Iter<'static, AllowTrustResultCode> {
        Self::VARIANTS.iter()
    }
}

impl Union<AllowTrustResultCode> for AllowTrustResult {}

impl ReadXdr for AllowTrustResult {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: AllowTrustResultCode = <AllowTrustResultCode as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                AllowTrustResultCode::Success => Self::Success,
                AllowTrustResultCode::Malformed => Self::Malformed,
                AllowTrustResultCode::NoTrustLine => Self::NoTrustLine,
                AllowTrustResultCode::TrustNotRequired => Self::TrustNotRequired,
                AllowTrustResultCode::CantRevoke => Self::CantRevoke,
                AllowTrustResultCode::SelfNotAllowed => Self::SelfNotAllowed,
                AllowTrustResultCode::LowReserve => Self::LowReserve,
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for AllowTrustResult {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::Success => ().write_xdr(w)?,
                Self::Malformed => ().write_xdr(w)?,
                Self::NoTrustLine => ().write_xdr(w)?,
                Self::TrustNotRequired => ().write_xdr(w)?,
                Self::CantRevoke => ().write_xdr(w)?,
                Self::SelfNotAllowed => ().write_xdr(w)?,
                Self::LowReserve => ().write_xdr(w)?,
            };
            Ok(())
        })
    }
}
