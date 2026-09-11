#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// String32 is a borrowing equivalent of [`String32`](super::super::String32)
/// over `'static` data, for const XDR encoding.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct String32(pub StringM<32>);

impl String32 {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_string32(self);
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
        w.write_type_string32(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

impl ConstWriter<'_> {
    /// Serializes a [`String32`], mirroring `<String32 as WriteXdr>::write_xdr`.
    pub const fn write_type_string32(&mut self, v: &String32) {
        self.write_var_opaque(v.0.as_slice());
    }

    /// Serializes an optional [`String32`], mirroring `<Option<String32> as WriteXdr>::write_xdr`.
    pub const fn write_type_option_string32(&mut self, v: &Option<String32>) {
        match v {
            Some(v) => {
                self.write_u32(1);
                self.write_type_string32(v);
            }
            None => {
                self.write_u32(0);
            }
        }
    }
}
