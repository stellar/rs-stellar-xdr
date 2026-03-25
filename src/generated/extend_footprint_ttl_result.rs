#[allow(unused_imports)]
use super::*;
/// ExtendFootprintTtlResult is an XDR Union defined as:
///
/// ```text
/// union ExtendFootprintTTLResult switch (ExtendFootprintTTLResultCode code)
/// {
/// case EXTEND_FOOTPRINT_TTL_SUCCESS:
///     void;
/// case EXTEND_FOOTPRINT_TTL_MALFORMED:
/// case EXTEND_FOOTPRINT_TTL_RESOURCE_LIMIT_EXCEEDED:
/// case EXTEND_FOOTPRINT_TTL_INSUFFICIENT_REFUNDABLE_FEE:
///     void;
/// };
/// ```
///
// union with discriminant ExtendFootprintTtlResultCode
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
pub enum ExtendFootprintTtlResult {
    Success,
    Malformed,
    ResourceLimitExceeded,
    InsufficientRefundableFee,
}

#[cfg(feature = "alloc")]
impl Default for ExtendFootprintTtlResult {
    fn default() -> Self {
        Self::Success
    }
}

impl ExtendFootprintTtlResult {
    const _VARIANTS: &[ExtendFootprintTtlResultCode] = &[
        ExtendFootprintTtlResultCode::Success,
        ExtendFootprintTtlResultCode::Malformed,
        ExtendFootprintTtlResultCode::ResourceLimitExceeded,
        ExtendFootprintTtlResultCode::InsufficientRefundableFee,
    ];
    pub const VARIANTS: [ExtendFootprintTtlResultCode; Self::_VARIANTS.len()] = {
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
    pub const fn discriminant(&self) -> ExtendFootprintTtlResultCode {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success => ExtendFootprintTtlResultCode::Success,
            Self::Malformed => ExtendFootprintTtlResultCode::Malformed,
            Self::ResourceLimitExceeded => ExtendFootprintTtlResultCode::ResourceLimitExceeded,
            Self::InsufficientRefundableFee => {
                ExtendFootprintTtlResultCode::InsufficientRefundableFee
            }
        }
    }

    #[must_use]
    pub const fn variants() -> [ExtendFootprintTtlResultCode; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for ExtendFootprintTtlResult {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<ExtendFootprintTtlResultCode> for ExtendFootprintTtlResult {
    #[must_use]
    fn discriminant(&self) -> ExtendFootprintTtlResultCode {
        Self::discriminant(self)
    }
}

impl Variants<ExtendFootprintTtlResultCode> for ExtendFootprintTtlResult {
    fn variants() -> slice::Iter<'static, ExtendFootprintTtlResultCode> {
        Self::VARIANTS.iter()
    }
}

impl Union<ExtendFootprintTtlResultCode> for ExtendFootprintTtlResult {}

impl ReadXdr for ExtendFootprintTtlResult {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: ExtendFootprintTtlResultCode =
                <ExtendFootprintTtlResultCode as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                ExtendFootprintTtlResultCode::Success => Self::Success,
                ExtendFootprintTtlResultCode::Malformed => Self::Malformed,
                ExtendFootprintTtlResultCode::ResourceLimitExceeded => Self::ResourceLimitExceeded,
                ExtendFootprintTtlResultCode::InsufficientRefundableFee => {
                    Self::InsufficientRefundableFee
                }
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for ExtendFootprintTtlResult {
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
