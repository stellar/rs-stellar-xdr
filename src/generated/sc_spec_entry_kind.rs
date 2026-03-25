#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
/// ScSpecEntryKind is an XDR Enum defined as:
///
/// ```text
/// enum SCSpecEntryKind
/// {
///     SC_SPEC_ENTRY_FUNCTION_V0 = 0,
///     SC_SPEC_ENTRY_UDT_STRUCT_V0 = 1,
///     SC_SPEC_ENTRY_UDT_UNION_V0 = 2,
///     SC_SPEC_ENTRY_UDT_ENUM_V0 = 3,
///     SC_SPEC_ENTRY_UDT_ERROR_ENUM_V0 = 4,
///     SC_SPEC_ENTRY_EVENT_V0 = 5
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
pub enum ScSpecEntryKind {
    #[cfg_attr(feature = "alloc", default)]
    FunctionV0 = 0,
    UdtStructV0 = 1,
    UdtUnionV0 = 2,
    UdtEnumV0 = 3,
    UdtErrorEnumV0 = 4,
    EventV0 = 5,
}

impl ScSpecEntryKind {
    const _VARIANTS: &[ScSpecEntryKind] = &[
        ScSpecEntryKind::FunctionV0,
        ScSpecEntryKind::UdtStructV0,
        ScSpecEntryKind::UdtUnionV0,
        ScSpecEntryKind::UdtEnumV0,
        ScSpecEntryKind::UdtErrorEnumV0,
        ScSpecEntryKind::EventV0,
    ];
    pub const VARIANTS: [ScSpecEntryKind; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &[
        "FunctionV0",
        "UdtStructV0",
        "UdtUnionV0",
        "UdtEnumV0",
        "UdtErrorEnumV0",
        "EventV0",
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
            Self::FunctionV0 => "FunctionV0",
            Self::UdtStructV0 => "UdtStructV0",
            Self::UdtUnionV0 => "UdtUnionV0",
            Self::UdtEnumV0 => "UdtEnumV0",
            Self::UdtErrorEnumV0 => "UdtErrorEnumV0",
            Self::EventV0 => "EventV0",
        }
    }

    #[must_use]
    pub const fn variants() -> [ScSpecEntryKind; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for ScSpecEntryKind {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Variants<ScSpecEntryKind> for ScSpecEntryKind {
    fn variants() -> slice::Iter<'static, ScSpecEntryKind> {
        Self::VARIANTS.iter()
    }
}

impl Enum for ScSpecEntryKind {}

impl fmt::Display for ScSpecEntryKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for ScSpecEntryKind {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self, Error> {
        let e = match i {
            0 => ScSpecEntryKind::FunctionV0,
            1 => ScSpecEntryKind::UdtStructV0,
            2 => ScSpecEntryKind::UdtUnionV0,
            3 => ScSpecEntryKind::UdtEnumV0,
            4 => ScSpecEntryKind::UdtErrorEnumV0,
            5 => ScSpecEntryKind::EventV0,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<ScSpecEntryKind> for i32 {
    #[must_use]
    fn from(e: ScSpecEntryKind) -> Self {
        e as Self
    }
}

impl ReadXdr for ScSpecEntryKind {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let e = i32::read_xdr(r)?;
            let v: Self = e.try_into()?;
            Ok(v)
        })
    }
}

impl WriteXdr for ScSpecEntryKind {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            let i: i32 = (*self).into();
            i.write_xdr(w)
        })
    }
}
