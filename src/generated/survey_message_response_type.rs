#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// SurveyMessageResponseType is an XDR Enum defined as:
///
/// ```text
/// enum SurveyMessageResponseType
/// {
///     SURVEY_TOPOLOGY_RESPONSE_V2 = 2
/// };
/// ```
///
// enum
#[cfg_attr(feature = "alloc", derive(Default))]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[repr(i32)]
pub enum SurveyMessageResponseType {
    #[cfg_attr(feature = "alloc", default)]
    SurveyTopologyResponseV2 = 2,
}

impl SurveyMessageResponseType {
    const _VARIANTS: &[SurveyMessageResponseType] =
        &[SurveyMessageResponseType::SurveyTopologyResponseV2];
    pub const VARIANTS: [SurveyMessageResponseType; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &["SurveyTopologyResponseV2"];
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
            Self::SurveyTopologyResponseV2 => "SurveyTopologyResponseV2",
        }
    }

    #[must_use]
    pub const fn variants() -> [SurveyMessageResponseType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for SurveyMessageResponseType {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Variants<SurveyMessageResponseType> for SurveyMessageResponseType {
    fn variants() -> slice::Iter<'static, SurveyMessageResponseType> {
        Self::VARIANTS.iter()
    }
}

impl Enum for SurveyMessageResponseType {}

impl fmt::Display for SurveyMessageResponseType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for SurveyMessageResponseType {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self, Error> {
        let e = match i {
            2 => SurveyMessageResponseType::SurveyTopologyResponseV2,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<SurveyMessageResponseType> for i32 {
    #[must_use]
    fn from(e: SurveyMessageResponseType) -> Self {
        e as Self
    }
}

impl ReadXdr for SurveyMessageResponseType {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let e = i32::read_xdr(r)?;
            let v: Self = e.try_into()?;
            Ok(v)
        })
    }
}

impl WriteXdr for SurveyMessageResponseType {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            let i: i32 = (*self).into();
            i.write_xdr(w)
        })
    }
}
