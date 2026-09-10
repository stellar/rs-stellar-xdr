#[allow(unused_imports, clippy::wildcard_imports)]
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
///
/// case SCV_EXECUTABLE_TAG:
///     SCString executable_tag;
/// };
/// ```
///
// union with discriminant ScValType
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval)]
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
    ExecutableTag(ScString),
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
        ScValType::ExecutableTag,
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
        "ExecutableTag",
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
            Self::ExecutableTag(_) => "ExecutableTag",
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
            Self::ExecutableTag(_) => ScValType::ExecutableTag,
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
                ScValType::ExecutableTag => Self::ExecutableTag(ScString::read_xdr(r)?),
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
                Self::ExecutableTag(v) => v.write_xdr(w)?,
            };
            Ok(())
        })
    }
}

/// ScValConst is a borrowing equivalent of [`ScVal`] over `'static`
/// data, for const XDR encoding.
#[cfg(feature = "const")]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(clippy::large_enum_variant)]
pub enum ScValConst {
    Bool(bool),
    Void,
    Error(ScError),
    U32(u32),
    I32(i32),
    U64(u64),
    I64(i64),
    Timepoint(TimePoint),
    Duration(Duration),
    U128(UInt128Parts),
    I128(Int128Parts),
    U256(UInt256Parts),
    I256(Int256Parts),
    Bytes(ScBytesConst),
    String(ScStringConst),
    Symbol(ScSymbolConst),
    Vec(Option<ScVecConst>),
    Map(Option<ScMapConst>),
    Address(ScAddress),
    ContractInstance(ScContractInstanceConst),
    LedgerKeyContractInstance,
    LedgerKeyNonce(ScNonceKey),
    ExecutableTag(ScStringConst),
}

#[cfg(feature = "const")]
impl ScValConst {
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
            Self::ExecutableTag(_) => ScValType::ExecutableTag,
        }
    }
}

#[cfg(feature = "const")]
impl ScValConst {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_sc_val(self);
        w.len()
    }

    /// Serialize this value as XDR into a fixed-size `[u8; N]` using only const
    /// operations. This is the const counterpart to [`WriteXdr::to_xdr`].
    ///
    /// `N` must equal [`Self::const_xdr_len`]. It is intended for callers, such
    /// as a proc-macro, that compute the length with `const_xdr_len` and pass
    /// it as `N`; `const_to_xdr` itself does not need to call `const_xdr_len`.
    ///
    /// # Panics
    ///
    /// Panics if `N` does not equal the value's [`Self::const_xdr_len`].
    #[must_use]
    pub const fn const_to_xdr<const N: usize>(&self) -> [u8; N] {
        let mut buf = [0u8; N];
        let mut w = ConstWriter::new(&mut buf);
        w.write_type_sc_val(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`ScVal`], mirroring `<ScVal as WriteXdr>::write_xdr`.
    pub const fn write_type_sc_val(&mut self, v: &ScValConst) {
        let d = v.discriminant();
        self.write_type_sc_val_type(&d);
        #[allow(clippy::match_same_arms)]
        match v {
            ScValConst::Bool(value) => {
                self.write_bool(*value);
            }
            ScValConst::Void => {}
            ScValConst::Error(value) => {
                self.write_type_sc_error(value);
            }
            ScValConst::U32(value) => {
                self.write_u32(*value);
            }
            ScValConst::I32(value) => {
                self.write_i32(*value);
            }
            ScValConst::U64(value) => {
                self.write_u64(*value);
            }
            ScValConst::I64(value) => {
                self.write_i64(*value);
            }
            ScValConst::Timepoint(value) => {
                self.write_type_time_point(value);
            }
            ScValConst::Duration(value) => {
                self.write_type_duration(value);
            }
            ScValConst::U128(value) => {
                self.write_type_u_int128_parts(value);
            }
            ScValConst::I128(value) => {
                self.write_type_int128_parts(value);
            }
            ScValConst::U256(value) => {
                self.write_type_u_int256_parts(value);
            }
            ScValConst::I256(value) => {
                self.write_type_int256_parts(value);
            }
            ScValConst::Bytes(value) => {
                self.write_type_sc_bytes(value);
            }
            ScValConst::String(value) => {
                self.write_type_sc_string(value);
            }
            ScValConst::Symbol(value) => {
                self.write_type_sc_symbol(value);
            }
            ScValConst::Vec(value) => {
                self.write_type_option_sc_vec(value);
            }
            ScValConst::Map(value) => {
                self.write_type_option_sc_map(value);
            }
            ScValConst::Address(value) => {
                self.write_type_sc_address(value);
            }
            ScValConst::ContractInstance(value) => {
                self.write_type_sc_contract_instance(value);
            }
            ScValConst::LedgerKeyContractInstance => {}
            ScValConst::LedgerKeyNonce(value) => {
                self.write_type_sc_nonce_key(value);
            }
            ScValConst::ExecutableTag(value) => {
                self.write_type_sc_string(value);
            }
        }
    }

    /// Serializes an optional [`ScVal`], mirroring `<Option<ScVal> as WriteXdr>::write_xdr`.
    pub const fn write_type_option_sc_val(&mut self, v: &Option<ScValConst>) {
        match v {
            Some(v) => {
                self.write_u32(1);
                self.write_type_sc_val(v);
            }
            None => {
                self.write_u32(0);
            }
        }
    }

    /// Serializes a variable-length array of [`ScVal`], mirroring `<VecM<ScVal, MAX> as WriteXdr>::write_xdr`.
    pub const fn write_type_vec_sc_val<const MAX: u32>(&mut self, v: &VecMConst<ScValConst, MAX>) {
        let s = v.as_slice();
        let len = s.len();
        self.write_len(len);
        let mut i = 0usize;
        while i < len {
            self.write_type_sc_val(&s[i]);
            i += 1;
        }
    }
}
