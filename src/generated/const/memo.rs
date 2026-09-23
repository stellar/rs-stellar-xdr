#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// Memo is a borrowing equivalent of [`Memo`](super::super::Memo)
/// over `'static` data, for const XDR encoding.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[allow(clippy::large_enum_variant)]
pub enum Memo {
    None,
    Text(StringM<28>),
    Id(u64),
    Hash(Hash),
    Return(Hash),
}

impl Memo {
    #[must_use]
    pub const fn discriminant(&self) -> MemoType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::None => MemoType::None,
            Self::Text(_) => MemoType::Text,
            Self::Id(_) => MemoType::Id,
            Self::Hash(_) => MemoType::Hash,
            Self::Return(_) => MemoType::Return,
        }
    }
}

impl Memo {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_memo(self);
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
        w.write_type_memo(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

impl ConstWriter<'_> {
    /// Serializes a [`Memo`], mirroring `<Memo as WriteXdr>::write_xdr`.
    pub const fn write_type_memo(&mut self, v: &Memo) {
        let d = v.discriminant();
        self.write_type_memo_type(&d);
        #[allow(clippy::match_same_arms)]
        match v {
            Memo::None => {}
            Memo::Text(value) => {
                self.write_var_opaque(value.as_slice());
            }
            Memo::Id(value) => {
                self.write_u64(*value);
            }
            Memo::Hash(value) => {
                self.write_type_hash(value);
            }
            Memo::Return(value) => {
                self.write_type_hash(value);
            }
        }
    }
}
