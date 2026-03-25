#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
/// RestoreFootprintResult is an XDR Union defined as:
///
/// ```text
/// union RestoreFootprintResult switch (RestoreFootprintResultCode code)
/// {
/// case RESTORE_FOOTPRINT_SUCCESS:
///     void;
/// case RESTORE_FOOTPRINT_MALFORMED:
/// case RESTORE_FOOTPRINT_RESOURCE_LIMIT_EXCEEDED:
/// case RESTORE_FOOTPRINT_INSUFFICIENT_REFUNDABLE_FEE:
///     void;
/// };
/// ```
///
// union with discriminant RestoreFootprintResultCode
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
pub enum RestoreFootprintResult {
    Success,
    Malformed,
    ResourceLimitExceeded,
    InsufficientRefundableFee,
}

#[cfg(feature = "alloc")]
impl Default for RestoreFootprintResult {
    fn default() -> Self {
        Self::Success
    }
}

impl RestoreFootprintResult {
    const _VARIANTS: &[RestoreFootprintResultCode] = &[
        RestoreFootprintResultCode::Success,
        RestoreFootprintResultCode::Malformed,
        RestoreFootprintResultCode::ResourceLimitExceeded,
        RestoreFootprintResultCode::InsufficientRefundableFee,
    ];
    pub const VARIANTS: [RestoreFootprintResultCode; Self::_VARIANTS.len()] = {
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
        "ResourceLimitExceeded",
        "InsufficientRefundableFee",
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
            Self::ResourceLimitExceeded => "ResourceLimitExceeded",
            Self::InsufficientRefundableFee => "InsufficientRefundableFee",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> RestoreFootprintResultCode {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => RestoreFootprintResultCode::Success,
            Self::Malformed => RestoreFootprintResultCode::Malformed,
            Self::ResourceLimitExceeded => RestoreFootprintResultCode::ResourceLimitExceeded,
            Self::InsufficientRefundableFee => {
                RestoreFootprintResultCode::InsufficientRefundableFee
            }
        }
    }

    #[must_use]
    pub const fn variants() -> [RestoreFootprintResultCode; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for RestoreFootprintResult {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<RestoreFootprintResultCode> for RestoreFootprintResult {
    #[must_use]
    fn discriminant(&self) -> RestoreFootprintResultCode {
        Self::discriminant(self)
    }
}

impl Variants<RestoreFootprintResultCode> for RestoreFootprintResult {
    fn variants() -> slice::Iter<'static, RestoreFootprintResultCode> {
        Self::VARIANTS.iter()
    }
}

impl Union<RestoreFootprintResultCode> for RestoreFootprintResult {}

impl ReadXdr for RestoreFootprintResult {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: RestoreFootprintResultCode =
                <RestoreFootprintResultCode as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                RestoreFootprintResultCode::Success => Self::Success,
                RestoreFootprintResultCode::Malformed => Self::Malformed,
                RestoreFootprintResultCode::ResourceLimitExceeded => Self::ResourceLimitExceeded,
                RestoreFootprintResultCode::InsufficientRefundableFee => {
                    Self::InsufficientRefundableFee
                }
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for RestoreFootprintResult {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::Success => ().write_xdr(w)?,
                Self::Malformed => ().write_xdr(w)?,
                Self::ResourceLimitExceeded => ().write_xdr(w)?,
                Self::InsufficientRefundableFee => ().write_xdr(w)?,
            };
            Ok(())
        })
    }
}
