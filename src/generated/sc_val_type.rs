#[allow(unused_imports)]
use super::*;
/// ScValType is an XDR Enum defined as:
///
/// ```text
/// enum SCValType
/// {
///     SCV_BOOL = 0,
///     SCV_VOID = 1,
///     SCV_ERROR = 2,
///
///     // 32 bits is the smallest type in WASM or XDR; no need for u8/u16.
///     SCV_U32 = 3,
///     SCV_I32 = 4,
///
///     // 64 bits is naturally supported by both WASM and XDR also.
///     SCV_U64 = 5,
///     SCV_I64 = 6,
///
///     // Time-related u64 subtypes with their own functions and formatting.
///     SCV_TIMEPOINT = 7,
///     SCV_DURATION = 8,
///
///     // 128 bits is naturally supported by Rust and we use it for Soroban
///     // fixed-point arithmetic prices / balances / similar "quantities". These
///     // are represented in XDR as a pair of 2 u64s.
///     SCV_U128 = 9,
///     SCV_I128 = 10,
///
///     // 256 bits is the size of sha256 output, ed25519 keys, and the EVM machine
///     // word, so for interop use we include this even though it requires a small
///     // amount of Rust guest and/or host library code.
///     SCV_U256 = 11,
///     SCV_I256 = 12,
///
///     // Bytes come in 3 flavors, 2 of which have meaningfully different
///     // formatting and validity-checking / domain-restriction.
///     SCV_BYTES = 13,
///     SCV_STRING = 14,
///     SCV_SYMBOL = 15,
///
///     // Vecs and maps are just polymorphic containers of other ScVals.
///     SCV_VEC = 16,
///     SCV_MAP = 17,
///
///     // Address is the universal identifier for contracts and classic
///     // accounts.
///     SCV_ADDRESS = 18,
///
///     // The following are the internal SCVal variants that are not
///     // exposed to the contracts.
///     SCV_CONTRACT_INSTANCE = 19,
///
///     // SCV_LEDGER_KEY_CONTRACT_INSTANCE and SCV_LEDGER_KEY_NONCE are unique
///     // symbolic SCVals used as the key for ledger entries for a contract's
///     // instance and an address' nonce, respectively.
///     SCV_LEDGER_KEY_CONTRACT_INSTANCE = 20,
///     SCV_LEDGER_KEY_NONCE = 21
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
pub enum ScValType {
    #[cfg_attr(feature = "alloc", default)]
    Bool = 0,
    Void = 1,
    Error = 2,
    U32 = 3,
    I32 = 4,
    U64 = 5,
    I64 = 6,
    Timepoint = 7,
    Duration = 8,
    U128 = 9,
    I128 = 10,
    U256 = 11,
    I256 = 12,
    Bytes = 13,
    String = 14,
    Symbol = 15,
    Vec = 16,
    Map = 17,
    Address = 18,
    ContractInstance = 19,
    LedgerKeyContractInstance = 20,
    LedgerKeyNonce = 21,
}

impl ScValType {
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
            Self::Vec => "Vec",
            Self::Map => "Map",
            Self::Address => "Address",
            Self::ContractInstance => "ContractInstance",
            Self::LedgerKeyContractInstance => "LedgerKeyContractInstance",
            Self::LedgerKeyNonce => "LedgerKeyNonce",
        }
    }

    #[must_use]
    pub const fn variants() -> [ScValType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for ScValType {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Variants<ScValType> for ScValType {
    fn variants() -> slice::Iter<'static, ScValType> {
        Self::VARIANTS.iter()
    }
}

impl Enum for ScValType {}

impl fmt::Display for ScValType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for ScValType {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self, Error> {
        let e = match i {
            0 => ScValType::Bool,
            1 => ScValType::Void,
            2 => ScValType::Error,
            3 => ScValType::U32,
            4 => ScValType::I32,
            5 => ScValType::U64,
            6 => ScValType::I64,
            7 => ScValType::Timepoint,
            8 => ScValType::Duration,
            9 => ScValType::U128,
            10 => ScValType::I128,
            11 => ScValType::U256,
            12 => ScValType::I256,
            13 => ScValType::Bytes,
            14 => ScValType::String,
            15 => ScValType::Symbol,
            16 => ScValType::Vec,
            17 => ScValType::Map,
            18 => ScValType::Address,
            19 => ScValType::ContractInstance,
            20 => ScValType::LedgerKeyContractInstance,
            21 => ScValType::LedgerKeyNonce,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<ScValType> for i32 {
    #[must_use]
    fn from(e: ScValType) -> Self {
        e as Self
    }
}

impl ReadXdr for ScValType {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let e = i32::read_xdr(r)?;
            let v: Self = e.try_into()?;
            Ok(v)
        })
    }
}

impl WriteXdr for ScValType {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            let i: i32 = (*self).into();
            i.write_xdr(w)
        })
    }
}
