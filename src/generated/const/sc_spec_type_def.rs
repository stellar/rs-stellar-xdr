#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ScSpecTypeDef is a borrowing equivalent of [`ScSpecTypeDef`](super::super::ScSpecTypeDef)
/// over `'static` data, for const XDR encoding.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
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
    Option(&'static ScSpecTypeOption),
    Result(&'static ScSpecTypeResult),
    Vec(&'static ScSpecTypeVec),
    Map(&'static ScSpecTypeMap),
    Tuple(&'static ScSpecTypeTuple),
    BytesN(ScSpecTypeBytesN),
    Udt(ScSpecTypeUdt),
}

impl ScSpecTypeDef {
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
}

impl ScSpecTypeDef {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_sc_spec_type_def(self);
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
        w.write_type_sc_spec_type_def(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

impl ConstWriter<'_> {
    /// Serializes a [`ScSpecTypeDef`], mirroring `<ScSpecTypeDef as WriteXdr>::write_xdr`.
    pub const fn write_type_sc_spec_type_def(&mut self, v: &ScSpecTypeDef) {
        let d = v.discriminant();
        self.write_type_sc_spec_type(&d);
        #[allow(clippy::match_same_arms)]
        match v {
            ScSpecTypeDef::Val => {}
            ScSpecTypeDef::Bool => {}
            ScSpecTypeDef::Void => {}
            ScSpecTypeDef::Error => {}
            ScSpecTypeDef::U32 => {}
            ScSpecTypeDef::I32 => {}
            ScSpecTypeDef::U64 => {}
            ScSpecTypeDef::I64 => {}
            ScSpecTypeDef::Timepoint => {}
            ScSpecTypeDef::Duration => {}
            ScSpecTypeDef::U128 => {}
            ScSpecTypeDef::I128 => {}
            ScSpecTypeDef::U256 => {}
            ScSpecTypeDef::I256 => {}
            ScSpecTypeDef::Bytes => {}
            ScSpecTypeDef::String => {}
            ScSpecTypeDef::Symbol => {}
            ScSpecTypeDef::Address => {}
            ScSpecTypeDef::MuxedAddress => {}
            ScSpecTypeDef::Option(value) => {
                self.write_type_sc_spec_type_option(value);
            }
            ScSpecTypeDef::Result(value) => {
                self.write_type_sc_spec_type_result(value);
            }
            ScSpecTypeDef::Vec(value) => {
                self.write_type_sc_spec_type_vec(value);
            }
            ScSpecTypeDef::Map(value) => {
                self.write_type_sc_spec_type_map(value);
            }
            ScSpecTypeDef::Tuple(value) => {
                self.write_type_sc_spec_type_tuple(value);
            }
            ScSpecTypeDef::BytesN(value) => {
                self.write_type_sc_spec_type_bytes_n(value);
            }
            ScSpecTypeDef::Udt(value) => {
                self.write_type_sc_spec_type_udt(value);
            }
        }
    }

    /// Serializes a variable-length array of [`ScSpecTypeDef`], mirroring `<VecM<ScSpecTypeDef, MAX> as WriteXdr>::write_xdr`.
    pub const fn write_type_vec_sc_spec_type_def<const MAX: u32>(
        &mut self,
        v: &VecM<ScSpecTypeDef, MAX>,
    ) {
        let s = v.as_slice();
        let len = s.len();
        self.write_len(len);
        let mut i = 0usize;
        while i < len {
            self.write_type_sc_spec_type_def(&s[i]);
            i += 1;
        }
    }
}
