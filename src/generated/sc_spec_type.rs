#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ScSpecType is an XDR Enum defined as:
///
/// ```text
/// enum SCSpecType
/// {
///     SC_SPEC_TYPE_VAL = 0,
///
///     // Types with no parameters.
///     SC_SPEC_TYPE_BOOL = 1,
///     SC_SPEC_TYPE_VOID = 2,
///     SC_SPEC_TYPE_ERROR = 3,
///     SC_SPEC_TYPE_U32 = 4,
///     SC_SPEC_TYPE_I32 = 5,
///     SC_SPEC_TYPE_U64 = 6,
///     SC_SPEC_TYPE_I64 = 7,
///     SC_SPEC_TYPE_TIMEPOINT = 8,
///     SC_SPEC_TYPE_DURATION = 9,
///     SC_SPEC_TYPE_U128 = 10,
///     SC_SPEC_TYPE_I128 = 11,
///     SC_SPEC_TYPE_U256 = 12,
///     SC_SPEC_TYPE_I256 = 13,
///     SC_SPEC_TYPE_BYTES = 14,
///     SC_SPEC_TYPE_STRING = 16,
///     SC_SPEC_TYPE_SYMBOL = 17,
///     SC_SPEC_TYPE_ADDRESS = 19,
///     SC_SPEC_TYPE_MUXED_ADDRESS = 20,
///
///     // Types with parameters.
///     SC_SPEC_TYPE_OPTION = 1000,
///     SC_SPEC_TYPE_RESULT = 1001,
///     SC_SPEC_TYPE_VEC = 1002,
///     SC_SPEC_TYPE_MAP = 1004,
///     SC_SPEC_TYPE_TUPLE = 1005,
///     SC_SPEC_TYPE_BYTES_N = 1006,
///
///     // User defined types.
///     SC_SPEC_TYPE_UDT = 2000
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
pub enum ScSpecType {
    #[cfg_attr(feature = "alloc", default)]
    Val = 0,
    Bool = 1,
    Void = 2,
    Error = 3,
    U32 = 4,
    I32 = 5,
    U64 = 6,
    I64 = 7,
    Timepoint = 8,
    Duration = 9,
    U128 = 10,
    I128 = 11,
    U256 = 12,
    I256 = 13,
    Bytes = 14,
    String = 16,
    Symbol = 17,
    Address = 19,
    MuxedAddress = 20,
    Option = 1000,
    Result = 1001,
    Vec = 1002,
    Map = 1004,
    Tuple = 1005,
    BytesN = 1006,
    Udt = 2000,
}

impl ScSpecType {
    const _VARIANTS: &[ScSpecType] = &[
        ScSpecType::Val,
        ScSpecType::Bool,
        ScSpecType::Void,
        ScSpecType::Error,
        ScSpecType::U32,
        ScSpecType::I32,
        ScSpecType::U64,
        ScSpecType::I64,
        ScSpecType::Timepoint,
        ScSpecType::Duration,
        ScSpecType::U128,
        ScSpecType::I128,
        ScSpecType::U256,
        ScSpecType::I256,
        ScSpecType::Bytes,
        ScSpecType::String,
        ScSpecType::Symbol,
        ScSpecType::Address,
        ScSpecType::MuxedAddress,
        ScSpecType::Option,
        ScSpecType::Result,
        ScSpecType::Vec,
        ScSpecType::Map,
        ScSpecType::Tuple,
        ScSpecType::BytesN,
        ScSpecType::Udt,
    ];
    pub const VARIANTS: [ScSpecType; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &[
        "Val",
        "Bool",
        "Void",
        "Error",
        "U32",
        "I32",
        "U64",
        "I64",
        "Timepoint",
        "Duration",
        "U128",
        "I128",
        "U256",
        "I256",
        "Bytes",
        "String",
        "Symbol",
        "Address",
        "MuxedAddress",
        "Option",
        "Result",
        "Vec",
        "Map",
        "Tuple",
        "BytesN",
        "Udt",
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
            Self::Val => "Val",
            Self::Bool => "Bool",
            Self::Void => "Void",
            Self::Error => "Error",
            Self::U32 => "U32",
            Self::I32 => "I32",
            Self::U64 => "U64",
            Self::I64 => "I64",
            Self::Timepoint => "Timepoint",
            Self::Duration => "Duration",
            Self::U128 => "U128",
            Self::I128 => "I128",
            Self::U256 => "U256",
            Self::I256 => "I256",
            Self::Bytes => "Bytes",
            Self::String => "String",
            Self::Symbol => "Symbol",
            Self::Address => "Address",
            Self::MuxedAddress => "MuxedAddress",
            Self::Option => "Option",
            Self::Result => "Result",
            Self::Vec => "Vec",
            Self::Map => "Map",
            Self::Tuple => "Tuple",
            Self::BytesN => "BytesN",
            Self::Udt => "Udt",
        }
    }

    #[must_use]
    pub const fn variants() -> [ScSpecType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for ScSpecType {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Variants<ScSpecType> for ScSpecType {
    fn variants() -> slice::Iter<'static, ScSpecType> {
        Self::VARIANTS.iter()
    }
}

impl Enum for ScSpecType {}

impl fmt::Display for ScSpecType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for ScSpecType {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self, Error> {
        let e = match i {
            0 => ScSpecType::Val,
            1 => ScSpecType::Bool,
            2 => ScSpecType::Void,
            3 => ScSpecType::Error,
            4 => ScSpecType::U32,
            5 => ScSpecType::I32,
            6 => ScSpecType::U64,
            7 => ScSpecType::I64,
            8 => ScSpecType::Timepoint,
            9 => ScSpecType::Duration,
            10 => ScSpecType::U128,
            11 => ScSpecType::I128,
            12 => ScSpecType::U256,
            13 => ScSpecType::I256,
            14 => ScSpecType::Bytes,
            16 => ScSpecType::String,
            17 => ScSpecType::Symbol,
            19 => ScSpecType::Address,
            20 => ScSpecType::MuxedAddress,
            1000 => ScSpecType::Option,
            1001 => ScSpecType::Result,
            1002 => ScSpecType::Vec,
            1004 => ScSpecType::Map,
            1005 => ScSpecType::Tuple,
            1006 => ScSpecType::BytesN,
            2000 => ScSpecType::Udt,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<ScSpecType> for i32 {
    #[must_use]
    fn from(e: ScSpecType) -> Self {
        e as Self
    }
}

impl ReadXdr for ScSpecType {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let e = i32::read_xdr(r)?;
            let v: Self = e.try_into()?;
            Ok(v)
        })
    }
}

impl WriteXdr for ScSpecType {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            let i: i32 = (*self).into();
            i.write_xdr(w)
        })
    }
}
