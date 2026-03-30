#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ScpStatementType is an XDR Enum defined as:
///
/// ```text
/// enum SCPStatementType
/// {
///     SCP_ST_PREPARE = 0,
///     SCP_ST_CONFIRM = 1,
///     SCP_ST_EXTERNALIZE = 2,
///     SCP_ST_NOMINATE = 3
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
pub enum ScpStatementType {
    #[cfg_attr(feature = "alloc", default)]
    Prepare = 0,
    Confirm = 1,
    Externalize = 2,
    Nominate = 3,
}

impl ScpStatementType {
    const _VARIANTS: &[ScpStatementType] = &[
        ScpStatementType::Prepare,
        ScpStatementType::Confirm,
        ScpStatementType::Externalize,
        ScpStatementType::Nominate,
    ];
    pub const VARIANTS: [ScpStatementType; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &[
        "Prepare",
        "Confirm",
        "Externalize",
        "Nominate",
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
            Self::Prepare => "Prepare",
            Self::Confirm => "Confirm",
            Self::Externalize => "Externalize",
            Self::Nominate => "Nominate",
        }
    }

    #[must_use]
    pub const fn variants() -> [ScpStatementType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for ScpStatementType {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Variants<ScpStatementType> for ScpStatementType {
    fn variants() -> slice::Iter<'static, ScpStatementType> {
        Self::VARIANTS.iter()
    }
}

impl Enum for ScpStatementType {}

impl fmt::Display for ScpStatementType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for ScpStatementType {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self, Error> {
        let e = match i {
            0 => ScpStatementType::Prepare,
            1 => ScpStatementType::Confirm,
            2 => ScpStatementType::Externalize,
            3 => ScpStatementType::Nominate,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<ScpStatementType> for i32 {
    #[must_use]
    fn from(e: ScpStatementType) -> Self {
        e as Self
    }
}

impl ReadXdr for ScpStatementType {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let e = i32::read_xdr(r)?;
            let v: Self = e.try_into()?;
            Ok(v)
        })
    }
}

impl WriteXdr for ScpStatementType {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            let i: i32 = (*self).into();
            i.write_xdr(w)
        })
    }
}
