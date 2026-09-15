#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ScVal is a borrowing equivalent of [`ScVal`](super::super::ScVal)
/// over `'static` data, for const XDR encoding.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[allow(clippy::large_enum_variant)]
pub enum ScVal {
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

impl ScVal {
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

impl ScVal {
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
    /// operations. This is the const counterpart to
    /// [`WriteXdr::to_xdr`](super::super::WriteXdr::to_xdr).
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

impl ConstWriter<'_> {
    /// Serializes a [`ScVal`], mirroring `<ScVal as WriteXdr>::write_xdr`.
    pub const fn write_type_sc_val(&mut self, v: &ScVal) {
        let d = v.discriminant();
        self.write_type_sc_val_type(&d);
        #[allow(clippy::match_same_arms)]
        match v {
            ScVal::Bool(value) => {
                self.write_bool(*value);
            }
            ScVal::Void => {}
            ScVal::Error(value) => {
                self.write_type_sc_error(value);
            }
            ScVal::U32(value) => {
                self.write_u32(*value);
            }
            ScVal::I32(value) => {
                self.write_i32(*value);
            }
            ScVal::U64(value) => {
                self.write_u64(*value);
            }
            ScVal::I64(value) => {
                self.write_i64(*value);
            }
            ScVal::Timepoint(value) => {
                self.write_type_time_point(value);
            }
            ScVal::Duration(value) => {
                self.write_type_duration(value);
            }
            ScVal::U128(value) => {
                self.write_type_u_int128_parts(value);
            }
            ScVal::I128(value) => {
                self.write_type_int128_parts(value);
            }
            ScVal::U256(value) => {
                self.write_type_u_int256_parts(value);
            }
            ScVal::I256(value) => {
                self.write_type_int256_parts(value);
            }
            ScVal::Bytes(value) => {
                self.write_type_sc_bytes(value);
            }
            ScVal::String(value) => {
                self.write_type_sc_string(value);
            }
            ScVal::Symbol(value) => {
                self.write_type_sc_symbol(value);
            }
            ScVal::Vec(value) => {
                self.write_type_option_sc_vec(value);
            }
            ScVal::Map(value) => {
                self.write_type_option_sc_map(value);
            }
            ScVal::Address(value) => {
                self.write_type_sc_address(value);
            }
            ScVal::ContractInstance(value) => {
                self.write_type_sc_contract_instance(value);
            }
            ScVal::LedgerKeyContractInstance => {}
            ScVal::LedgerKeyNonce(value) => {
                self.write_type_sc_nonce_key(value);
            }
            ScVal::ExecutableTag(value) => {
                self.write_type_sc_string(value);
            }
        }
    }

    /// Serializes an optional [`ScVal`], mirroring `<Option<ScVal> as WriteXdr>::write_xdr`.
    pub const fn write_type_option_sc_val(&mut self, v: &Option<ScVal>) {
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
    pub const fn write_type_vec_sc_val<const MAX: u32>(&mut self, v: &VecM<ScVal, MAX>) {
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
