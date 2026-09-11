#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ManageDataResult is an XDR Union defined as:
///
/// ```text
/// union ManageDataResult switch (ManageDataResultCode code)
/// {
/// case MANAGE_DATA_SUCCESS:
///     void;
/// case MANAGE_DATA_NOT_SUPPORTED_YET:
/// case MANAGE_DATA_NAME_NOT_FOUND:
/// case MANAGE_DATA_LOW_RESERVE:
/// case MANAGE_DATA_INVALID_NAME:
///     void;
/// };
/// ```
///
// union with discriminant ManageDataResultCode
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
pub enum ManageDataResult {
    Success,
    NotSupportedYet,
    NameNotFound,
    LowReserve,
    InvalidName,
}

#[cfg(feature = "alloc")]
impl Default for ManageDataResult {
    fn default() -> Self {
        Self::Success
    }
}

impl ManageDataResult {
    const _VARIANTS: &[ManageDataResultCode] = &[
        ManageDataResultCode::Success,
        ManageDataResultCode::NotSupportedYet,
        ManageDataResultCode::NameNotFound,
        ManageDataResultCode::LowReserve,
        ManageDataResultCode::InvalidName,
    ];
    pub const VARIANTS: [ManageDataResultCode; Self::_VARIANTS.len()] = {
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
        "NotSupportedYet",
        "NameNotFound",
        "LowReserve",
        "InvalidName",
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
            Self::NotSupportedYet => "NotSupportedYet",
            Self::NameNotFound => "NameNotFound",
            Self::LowReserve => "LowReserve",
            Self::InvalidName => "InvalidName",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> ManageDataResultCode {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => ManageDataResultCode::Success,
            Self::NotSupportedYet => ManageDataResultCode::NotSupportedYet,
            Self::NameNotFound => ManageDataResultCode::NameNotFound,
            Self::LowReserve => ManageDataResultCode::LowReserve,
            Self::InvalidName => ManageDataResultCode::InvalidName,
        }
    }

    #[must_use]
    pub const fn variants() -> [ManageDataResultCode; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for ManageDataResult {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<ManageDataResultCode> for ManageDataResult {
    #[must_use]
    fn discriminant(&self) -> ManageDataResultCode {
        Self::discriminant(self)
    }
}

impl Variants<ManageDataResultCode> for ManageDataResult {
    fn variants() -> slice::Iter<'static, ManageDataResultCode> {
        Self::VARIANTS.iter()
    }
}

impl Union<ManageDataResultCode> for ManageDataResult {}

impl ReadXdr for ManageDataResult {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: ManageDataResultCode = <ManageDataResultCode as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                ManageDataResultCode::Success => Self::Success,
                ManageDataResultCode::NotSupportedYet => Self::NotSupportedYet,
                ManageDataResultCode::NameNotFound => Self::NameNotFound,
                ManageDataResultCode::LowReserve => Self::LowReserve,
                ManageDataResultCode::InvalidName => Self::InvalidName,
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for ManageDataResult {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::Success => ().write_xdr(w)?,
                Self::NotSupportedYet => ().write_xdr(w)?,
                Self::NameNotFound => ().write_xdr(w)?,
                Self::LowReserve => ().write_xdr(w)?,
                Self::InvalidName => ().write_xdr(w)?,
            };
            Ok(())
        })
    }
}
