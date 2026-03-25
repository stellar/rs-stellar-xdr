#[allow(unused_imports)]
use super::*;
/// SurveyResponseBody is an XDR Union defined as:
///
/// ```text
/// union SurveyResponseBody switch (SurveyMessageResponseType type)
/// {
/// case SURVEY_TOPOLOGY_RESPONSE_V2:
///     TopologyResponseBodyV2 topologyResponseBodyV2;
/// };
/// ```
///
// union with discriminant SurveyMessageResponseType
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
pub enum SurveyResponseBody {
    SurveyTopologyResponseV2(TopologyResponseBodyV2),
}

#[cfg(feature = "alloc")]
impl Default for SurveyResponseBody {
    fn default() -> Self {
        Self::SurveyTopologyResponseV2(TopologyResponseBodyV2::default())
    }
}

impl SurveyResponseBody {
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
            Self::SurveyTopologyResponseV2(_) => "SurveyTopologyResponseV2",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> SurveyMessageResponseType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::SurveyTopologyResponseV2(_) => {
                SurveyMessageResponseType::SurveyTopologyResponseV2
            }
        }
    }

    #[must_use]
    pub const fn variants() -> [SurveyMessageResponseType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for SurveyResponseBody {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<SurveyMessageResponseType> for SurveyResponseBody {
    #[must_use]
    fn discriminant(&self) -> SurveyMessageResponseType {
        Self::discriminant(self)
    }
}

impl Variants<SurveyMessageResponseType> for SurveyResponseBody {
    fn variants() -> slice::Iter<'static, SurveyMessageResponseType> {
        Self::VARIANTS.iter()
    }
}

impl Union<SurveyMessageResponseType> for SurveyResponseBody {}

impl ReadXdr for SurveyResponseBody {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: SurveyMessageResponseType =
                <SurveyMessageResponseType as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                SurveyMessageResponseType::SurveyTopologyResponseV2 => {
                    Self::SurveyTopologyResponseV2(TopologyResponseBodyV2::read_xdr(r)?)
                }
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for SurveyResponseBody {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::SurveyTopologyResponseV2(v) => v.write_xdr(w)?,
            };
            Ok(())
        })
    }
}
