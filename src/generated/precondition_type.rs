#[allow(unused_imports)]
use super::*;
/// PreconditionType is an XDR Enum defined as:
///
/// ```text
/// enum PreconditionType
/// {
///     PRECOND_NONE = 0,
///     PRECOND_TIME = 1,
///     PRECOND_V2 = 2
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
pub enum PreconditionType {
    #[cfg_attr(feature = "alloc", default)]
    None = 0,
    Time = 1,
    V2 = 2,
}

impl PreconditionType {
    const _VARIANTS: &[PreconditionType] = &[
        PreconditionType::None,
        PreconditionType::Time,
        PreconditionType::V2,
    ];
    pub const VARIANTS: [PreconditionType; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &["None", "Time", "V2"];
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
            Self::None => "None",
            Self::Time => "Time",
            Self::V2 => "V2",
        }
    }

    #[must_use]
    pub const fn variants() -> [PreconditionType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for PreconditionType {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Variants<PreconditionType> for PreconditionType {
    fn variants() -> slice::Iter<'static, PreconditionType> {
        Self::VARIANTS.iter()
    }
}

impl Enum for PreconditionType {}

impl fmt::Display for PreconditionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for PreconditionType {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self, Error> {
        let e = match i {
            0 => PreconditionType::None,
            1 => PreconditionType::Time,
            2 => PreconditionType::V2,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<PreconditionType> for i32 {
    #[must_use]
    fn from(e: PreconditionType) -> Self {
        e as Self
    }
}

impl ReadXdr for PreconditionType {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let e = i32::read_xdr(r)?;
            let v: Self = e.try_into()?;
            Ok(v)
        })
    }
}

impl WriteXdr for PreconditionType {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            let i: i32 = (*self).into();
            i.write_xdr(w)
        })
    }
}
