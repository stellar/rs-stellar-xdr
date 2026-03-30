#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ExtendFootprintTtlResultCode is an XDR Enum defined as:
///
/// ```text
/// enum ExtendFootprintTTLResultCode
/// {
///     // codes considered as "success" for the operation
///     EXTEND_FOOTPRINT_TTL_SUCCESS = 0,
/// 
///     // codes considered as "failure" for the operation
///     EXTEND_FOOTPRINT_TTL_MALFORMED = -1,
///     EXTEND_FOOTPRINT_TTL_RESOURCE_LIMIT_EXCEEDED = -2,
///     EXTEND_FOOTPRINT_TTL_INSUFFICIENT_REFUNDABLE_FEE = -3
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
pub enum ExtendFootprintTtlResultCode {
    #[cfg_attr(feature = "alloc", default)]
    Success = 0,
    Malformed = -1,
    ResourceLimitExceeded = -2,
    InsufficientRefundableFee = -3,
}

impl ExtendFootprintTtlResultCode {
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
    pub const fn variants() -> [ExtendFootprintTtlResultCode; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for ExtendFootprintTtlResultCode {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Variants<ExtendFootprintTtlResultCode> for ExtendFootprintTtlResultCode {
    fn variants() -> slice::Iter<'static, ExtendFootprintTtlResultCode> {
        Self::VARIANTS.iter()
    }
}

impl Enum for ExtendFootprintTtlResultCode {}

impl fmt::Display for ExtendFootprintTtlResultCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for ExtendFootprintTtlResultCode {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self, Error> {
        let e = match i {
            0 => ExtendFootprintTtlResultCode::Success,
            -1 => ExtendFootprintTtlResultCode::Malformed,
            -2 => ExtendFootprintTtlResultCode::ResourceLimitExceeded,
            -3 => ExtendFootprintTtlResultCode::InsufficientRefundableFee,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<ExtendFootprintTtlResultCode> for i32 {
    #[must_use]
    fn from(e: ExtendFootprintTtlResultCode) -> Self {
        e as Self
    }
}

impl ReadXdr for ExtendFootprintTtlResultCode {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let e = i32::read_xdr(r)?;
            let v: Self = e.try_into()?;
            Ok(v)
        })
    }
}

impl WriteXdr for ExtendFootprintTtlResultCode {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            let i: i32 = (*self).into();
            i.write_xdr(w)
        })
    }
}
