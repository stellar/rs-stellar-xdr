#[allow(unused_imports)]
use super::*;
/// ScVal is an XDR Union defined as:
///
/// ```text
/// union SCVal switch (SCValType type)
/// {
///
/// case SCV_BOOL:
///     bool b;
/// case SCV_VOID:
///     void;
/// case SCV_ERROR:
///     SCError error;
///
/// case SCV_U32:
///     uint32 u32;
/// case SCV_I32:
///     int32 i32;
///
/// case SCV_U64:
///     uint64 u64;
/// case SCV_I64:
///     int64 i64;
/// case SCV_TIMEPOINT:
///     TimePoint timepoint;
/// case SCV_DURATION:
///     Duration duration;
///
/// case SCV_U128:
///     UInt128Parts u128;
/// case SCV_I128:
///     Int128Parts i128;
///
/// case SCV_U256:
///     UInt256Parts u256;
/// case SCV_I256:
///     Int256Parts i256;
///
/// case SCV_BYTES:
///     SCBytes bytes;
/// case SCV_STRING:
///     SCString str;
/// case SCV_SYMBOL:
///     SCSymbol sym;
///
/// // Vec and Map are recursive so need to live
/// // behind an option, due to xdrpp limitations.
/// case SCV_VEC:
///     SCVec *vec;
/// case SCV_MAP:
///     SCMap *map;
///
/// case SCV_ADDRESS:
///     SCAddress address;
///
/// // Special SCVals reserved for system-constructed contract-data
/// // ledger keys, not generally usable elsewhere.
/// case SCV_CONTRACT_INSTANCE:
///     SCContractInstance instance;
/// case SCV_LEDGER_KEY_CONTRACT_INSTANCE:
///     void;
/// case SCV_LEDGER_KEY_NONCE:
///     SCNonceKey nonce_key;
/// };
/// ```
///
// union with discriminant ScValType
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
pub enum ScVal {
    Bool(bool),
    Void,
    Error(ScError),
    U32(u32),
    I32(i32),
    U64(
        #[cfg_attr(
            all(feature = "serde", feature = "alloc"),
            serde_as(as = "NumberOrString")
        )]
        u64,
    ),
    I64(
        #[cfg_attr(
            all(feature = "serde", feature = "alloc"),
            serde_as(as = "NumberOrString")
        )]
        i64,
    ),
    Timepoint(TimePoint),
    Duration(Duration),
    U128(UInt128Parts),
    I128(Int128Parts),
    U256(UInt256Parts),
    I256(Int256Parts),
    Bytes(ScBytes),
    String(ScString),
    Symbol(ScSymbol),
    Vec(Option<ScVec>),
    Map(Option<ScMap>),
    Address(ScAddress),
    ContractInstance(ScContractInstance),
    LedgerKeyContractInstance,
    LedgerKeyNonce(ScNonceKey),
}

#[cfg(feature = "alloc")]
impl Default for ScVal {
    fn default() -> Self {
        Self::Bool(bool::default())
    }
}

impl ScVal {
    const _VARIANTS: &[ScValType] = &[
        ScValType::Bool,
        ScValType::Void,
        ScValType::Error,
        ScValType::U32,
        ScValType::I32,
        ScValType::U64,
        ScValType::I64,
        ScValType::Timepoint,
        ScValType::Duration,
        ScValType::U128,
        ScValType::I128,
        ScValType::U256,
        ScValType::I256,
        ScValType::Bytes,
        ScValType::String,
        ScValType::Symbol,
        ScValType::Vec,
        ScValType::Map,
        ScValType::Address,
        ScValType::ContractInstance,
        ScValType::LedgerKeyContractInstance,
        ScValType::LedgerKeyNonce,
    ];
    pub const VARIANTS: [ScValType; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &[
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
        "Vec",
        "Map",
        "Address",
        "ContractInstance",
        "LedgerKeyContractInstance",
        "LedgerKeyNonce",
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
            Self::Bool(_) => "Bool",
            Self::Void => "Void",
            Self::Error(_) => "Error",
            Self::U32(_) => "U32",
            Self::I32(_) => "I32",
            Self::U64(_) => "U64",
            Self::I64(_) => "I64",
            Self::Timepoint(_) => "Timepoint",
            Self::Duration(_) => "Duration",
            Self::U128(_) => "U128",
            Self::I128(_) => "I128",
            Self::U256(_) => "U256",
            Self::I256(_) => "I256",
            Self::Bytes(_) => "Bytes",
            Self::String(_) => "String",
            Self::Symbol(_) => "Symbol",
            Self::Vec(_) => "Vec",
            Self::Map(_) => "Map",
            Self::Address(_) => "Address",
            Self::ContractInstance(_) => "ContractInstance",
            Self::LedgerKeyContractInstance => "LedgerKeyContractInstance",
            Self::LedgerKeyNonce(_) => "LedgerKeyNonce",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> ScValType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Bool(_) => ScValType::Bool,
            Self::Void => ScValType::Void,
            Self::Error(_) => ScValType::Error,
            Self::U32(_) => ScValType::U32,
            Self::I32(_) => ScValType::I32,
            Self::U64(_) => ScValType::U64,
            Self::I64(_) => ScValType::I64,
            Self::Timepoint(_) => ScValType::Timepoint,
            Self::Duration(_) => ScValType::Duration,
            Self::U128(_) => ScValType::U128,
            Self::I128(_) => ScValType::I128,
            Self::U256(_) => ScValType::U256,
            Self::I256(_) => ScValType::I256,
            Self::Bytes(_) => ScValType::Bytes,
            Self::String(_) => ScValType::String,
            Self::Symbol(_) => ScValType::Symbol,
            Self::Vec(_) => ScValType::Vec,
            Self::Map(_) => ScValType::Map,
            Self::Address(_) => ScValType::Address,
            Self::ContractInstance(_) => ScValType::ContractInstance,
            Self::LedgerKeyContractInstance => ScValType::LedgerKeyContractInstance,
            Self::LedgerKeyNonce(_) => ScValType::LedgerKeyNonce,
        }
    }

    #[must_use]
    pub const fn variants() -> [ScValType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for ScVal {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<ScValType> for ScVal {
    #[must_use]
    fn discriminant(&self) -> ScValType {
        Self::discriminant(self)
    }
}

impl Variants<ScValType> for ScVal {
    fn variants() -> slice::Iter<'static, ScValType> {
        Self::VARIANTS.iter()
    }
}

impl Union<ScValType> for ScVal {}

impl ReadXdr for ScVal {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: ScValType = <ScValType as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                ScValType::Bool => Self::Bool(bool::read_xdr(r)?),
                ScValType::Void => Self::Void,
                ScValType::Error => Self::Error(ScError::read_xdr(r)?),
                ScValType::U32 => Self::U32(u32::read_xdr(r)?),
                ScValType::I32 => Self::I32(i32::read_xdr(r)?),
                ScValType::U64 => Self::U64(u64::read_xdr(r)?),
                ScValType::I64 => Self::I64(i64::read_xdr(r)?),
                ScValType::Timepoint => Self::Timepoint(TimePoint::read_xdr(r)?),
                ScValType::Duration => Self::Duration(Duration::read_xdr(r)?),
                ScValType::U128 => Self::U128(UInt128Parts::read_xdr(r)?),
                ScValType::I128 => Self::I128(Int128Parts::read_xdr(r)?),
                ScValType::U256 => Self::U256(UInt256Parts::read_xdr(r)?),
                ScValType::I256 => Self::I256(Int256Parts::read_xdr(r)?),
                ScValType::Bytes => Self::Bytes(ScBytes::read_xdr(r)?),
                ScValType::String => Self::String(ScString::read_xdr(r)?),
                ScValType::Symbol => Self::Symbol(ScSymbol::read_xdr(r)?),
                ScValType::Vec => Self::Vec(Option::<ScVec>::read_xdr(r)?),
                ScValType::Map => Self::Map(Option::<ScMap>::read_xdr(r)?),
                ScValType::Address => Self::Address(ScAddress::read_xdr(r)?),
                ScValType::ContractInstance => {
                    Self::ContractInstance(ScContractInstance::read_xdr(r)?)
                }
                ScValType::LedgerKeyContractInstance => Self::LedgerKeyContractInstance,
                ScValType::LedgerKeyNonce => Self::LedgerKeyNonce(ScNonceKey::read_xdr(r)?),
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for ScVal {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::Bool(v) => v.write_xdr(w)?,
                Self::Void => ().write_xdr(w)?,
                Self::Error(v) => v.write_xdr(w)?,
                Self::U32(v) => v.write_xdr(w)?,
                Self::I32(v) => v.write_xdr(w)?,
                Self::U64(v) => v.write_xdr(w)?,
                Self::I64(v) => v.write_xdr(w)?,
                Self::Timepoint(v) => v.write_xdr(w)?,
                Self::Duration(v) => v.write_xdr(w)?,
                Self::U128(v) => v.write_xdr(w)?,
                Self::I128(v) => v.write_xdr(w)?,
                Self::U256(v) => v.write_xdr(w)?,
                Self::I256(v) => v.write_xdr(w)?,
                Self::Bytes(v) => v.write_xdr(w)?,
                Self::String(v) => v.write_xdr(w)?,
                Self::Symbol(v) => v.write_xdr(w)?,
                Self::Vec(v) => v.write_xdr(w)?,
                Self::Map(v) => v.write_xdr(w)?,
                Self::Address(v) => v.write_xdr(w)?,
                Self::ContractInstance(v) => v.write_xdr(w)?,
                Self::LedgerKeyContractInstance => ().write_xdr(w)?,
                Self::LedgerKeyNonce(v) => v.write_xdr(w)?,
            };
            Ok(())
        })
    }
}
