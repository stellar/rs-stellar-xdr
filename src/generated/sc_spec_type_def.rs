#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ScSpecTypeDef is an XDR Union defined as:
///
/// ```text
/// union SCSpecTypeDef switch (SCSpecType type)
/// {
/// case SC_SPEC_TYPE_VAL:
/// case SC_SPEC_TYPE_BOOL:
/// case SC_SPEC_TYPE_VOID:
/// case SC_SPEC_TYPE_ERROR:
/// case SC_SPEC_TYPE_U32:
/// case SC_SPEC_TYPE_I32:
/// case SC_SPEC_TYPE_U64:
/// case SC_SPEC_TYPE_I64:
/// case SC_SPEC_TYPE_TIMEPOINT:
/// case SC_SPEC_TYPE_DURATION:
/// case SC_SPEC_TYPE_U128:
/// case SC_SPEC_TYPE_I128:
/// case SC_SPEC_TYPE_U256:
/// case SC_SPEC_TYPE_I256:
/// case SC_SPEC_TYPE_BYTES:
/// case SC_SPEC_TYPE_STRING:
/// case SC_SPEC_TYPE_SYMBOL:
/// case SC_SPEC_TYPE_ADDRESS:
/// case SC_SPEC_TYPE_MUXED_ADDRESS:
///     void;
/// case SC_SPEC_TYPE_OPTION:
///     SCSpecTypeOption option;
/// case SC_SPEC_TYPE_RESULT:
///     SCSpecTypeResult result;
/// case SC_SPEC_TYPE_VEC:
///     SCSpecTypeVec vec;
/// case SC_SPEC_TYPE_MAP:
///     SCSpecTypeMap map;
/// case SC_SPEC_TYPE_TUPLE:
///     SCSpecTypeTuple tuple;
/// case SC_SPEC_TYPE_BYTES_N:
///     SCSpecTypeBytesN bytesN;
/// case SC_SPEC_TYPE_UDT:
///     SCSpecTypeUDT udt;
/// };
/// ```
///
// union with discriminant ScSpecType
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
pub enum ScSpecTypeDef {
    Val,
    Bool,
    Void,
    Error,
    U32,
    I32,
    U64,
    I64,
    Timepoint,
    Duration,
    U128,
    I128,
    U256,
    I256,
    Bytes,
    String,
    Symbol,
    Address,
    MuxedAddress,
    Option(
        Box<ScSpecTypeOption>,
    ),
    Result(
        Box<ScSpecTypeResult>,
    ),
    Vec(
        Box<ScSpecTypeVec>,
    ),
    Map(
        Box<ScSpecTypeMap>,
    ),
    Tuple(
        Box<ScSpecTypeTuple>,
    ),
    BytesN(
        ScSpecTypeBytesN,
    ),
    Udt(
        ScSpecTypeUdt,
    ),
}


#[cfg(feature = "alloc")]
impl Default for ScSpecTypeDef {
    fn default() -> Self {
        Self::Val
    }
}

impl ScSpecTypeDef {
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
            Self::Option(_) => "Option",
            Self::Result(_) => "Result",
            Self::Vec(_) => "Vec",
            Self::Map(_) => "Map",
            Self::Tuple(_) => "Tuple",
            Self::BytesN(_) => "BytesN",
            Self::Udt(_) => "Udt",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> ScSpecType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Val => ScSpecType::Val,
            Self::Bool => ScSpecType::Bool,
            Self::Void => ScSpecType::Void,
            Self::Error => ScSpecType::Error,
            Self::U32 => ScSpecType::U32,
            Self::I32 => ScSpecType::I32,
            Self::U64 => ScSpecType::U64,
            Self::I64 => ScSpecType::I64,
            Self::Timepoint => ScSpecType::Timepoint,
            Self::Duration => ScSpecType::Duration,
            Self::U128 => ScSpecType::U128,
            Self::I128 => ScSpecType::I128,
            Self::U256 => ScSpecType::U256,
            Self::I256 => ScSpecType::I256,
            Self::Bytes => ScSpecType::Bytes,
            Self::String => ScSpecType::String,
            Self::Symbol => ScSpecType::Symbol,
            Self::Address => ScSpecType::Address,
            Self::MuxedAddress => ScSpecType::MuxedAddress,
            Self::Option(_) => ScSpecType::Option,
            Self::Result(_) => ScSpecType::Result,
            Self::Vec(_) => ScSpecType::Vec,
            Self::Map(_) => ScSpecType::Map,
            Self::Tuple(_) => ScSpecType::Tuple,
            Self::BytesN(_) => ScSpecType::BytesN,
            Self::Udt(_) => ScSpecType::Udt,
        }
    }

    #[must_use]
    pub const fn variants() -> [ScSpecType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for ScSpecTypeDef {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<ScSpecType> for ScSpecTypeDef {
    #[must_use]
    fn discriminant(&self) -> ScSpecType {
        Self::discriminant(self)
    }
}

impl Variants<ScSpecType> for ScSpecTypeDef {
    fn variants() -> slice::Iter<'static, ScSpecType> {
        Self::VARIANTS.iter()
    }
}

impl Union<ScSpecType> for ScSpecTypeDef {}

impl ReadXdr for ScSpecTypeDef {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: ScSpecType = <ScSpecType as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                ScSpecType::Val => Self::Val,
                ScSpecType::Bool => Self::Bool,
                ScSpecType::Void => Self::Void,
                ScSpecType::Error => Self::Error,
                ScSpecType::U32 => Self::U32,
                ScSpecType::I32 => Self::I32,
                ScSpecType::U64 => Self::U64,
                ScSpecType::I64 => Self::I64,
                ScSpecType::Timepoint => Self::Timepoint,
                ScSpecType::Duration => Self::Duration,
                ScSpecType::U128 => Self::U128,
                ScSpecType::I128 => Self::I128,
                ScSpecType::U256 => Self::U256,
                ScSpecType::I256 => Self::I256,
                ScSpecType::Bytes => Self::Bytes,
                ScSpecType::String => Self::String,
                ScSpecType::Symbol => Self::Symbol,
                ScSpecType::Address => Self::Address,
                ScSpecType::MuxedAddress => Self::MuxedAddress,
                ScSpecType::Option => Self::Option(Box::<ScSpecTypeOption>::read_xdr(r)?),
                ScSpecType::Result => Self::Result(Box::<ScSpecTypeResult>::read_xdr(r)?),
                ScSpecType::Vec => Self::Vec(Box::<ScSpecTypeVec>::read_xdr(r)?),
                ScSpecType::Map => Self::Map(Box::<ScSpecTypeMap>::read_xdr(r)?),
                ScSpecType::Tuple => Self::Tuple(Box::<ScSpecTypeTuple>::read_xdr(r)?),
                ScSpecType::BytesN => Self::BytesN(ScSpecTypeBytesN::read_xdr(r)?),
                ScSpecType::Udt => Self::Udt(ScSpecTypeUdt::read_xdr(r)?),
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for ScSpecTypeDef {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::Val => ().write_xdr(w)?,
                Self::Bool => ().write_xdr(w)?,
                Self::Void => ().write_xdr(w)?,
                Self::Error => ().write_xdr(w)?,
                Self::U32 => ().write_xdr(w)?,
                Self::I32 => ().write_xdr(w)?,
                Self::U64 => ().write_xdr(w)?,
                Self::I64 => ().write_xdr(w)?,
                Self::Timepoint => ().write_xdr(w)?,
                Self::Duration => ().write_xdr(w)?,
                Self::U128 => ().write_xdr(w)?,
                Self::I128 => ().write_xdr(w)?,
                Self::U256 => ().write_xdr(w)?,
                Self::I256 => ().write_xdr(w)?,
                Self::Bytes => ().write_xdr(w)?,
                Self::String => ().write_xdr(w)?,
                Self::Symbol => ().write_xdr(w)?,
                Self::Address => ().write_xdr(w)?,
                Self::MuxedAddress => ().write_xdr(w)?,
                Self::Option(v) => v.write_xdr(w)?,
                Self::Result(v) => v.write_xdr(w)?,
                Self::Vec(v) => v.write_xdr(w)?,
                Self::Map(v) => v.write_xdr(w)?,
                Self::Tuple(v) => v.write_xdr(w)?,
                Self::BytesN(v) => v.write_xdr(w)?,
                Self::Udt(v) => v.write_xdr(w)?,
            };
            Ok(())
        })
    }
}
