#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
/// InflationResultCode is an XDR Enum defined as:
///
/// ```text
/// enum InflationResultCode
/// {
///     // codes considered as "success" for the operation
///     INFLATION_SUCCESS = 0,
///     // codes considered as "failure" for the operation
///     INFLATION_NOT_TIME = -1
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
pub enum InflationResultCode {
    #[cfg_attr(feature = "alloc", default)]
    Success = 0,
    NotTime = -1,
}

impl InflationResultCode {
    const _VARIANTS: &[InflationResultCode] =
        &[InflationResultCode::Success, InflationResultCode::NotTime];
    pub const VARIANTS: [InflationResultCode; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &["Success", "NotTime"];
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
            Self::NotTime => "NotTime",
        }
    }

    #[must_use]
    pub const fn variants() -> [InflationResultCode; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for InflationResultCode {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Variants<InflationResultCode> for InflationResultCode {
    fn variants() -> slice::Iter<'static, InflationResultCode> {
        Self::VARIANTS.iter()
    }
}

impl Enum for InflationResultCode {}

impl fmt::Display for InflationResultCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for InflationResultCode {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self, Error> {
        let e = match i {
            0 => InflationResultCode::Success,
            -1 => InflationResultCode::NotTime,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<InflationResultCode> for i32 {
    #[must_use]
    fn from(e: InflationResultCode) -> Self {
        e as Self
    }
}

impl ReadXdr for InflationResultCode {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let e = i32::read_xdr(r)?;
            let v: Self = e.try_into()?;
            Ok(v)
        })
    }
}

impl WriteXdr for InflationResultCode {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            let i: i32 = (*self).into();
            i.write_xdr(w)
        })
    }
}
