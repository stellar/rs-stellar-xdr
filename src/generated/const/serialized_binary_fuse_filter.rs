#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// SerializedBinaryFuseFilter is a borrowing equivalent of [`SerializedBinaryFuseFilter`](super::super::SerializedBinaryFuseFilter)
/// over `'static` data, for const XDR encoding.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SerializedBinaryFuseFilter {
    pub type_: BinaryFuseFilterType,
    pub input_hash_seed: ShortHashSeed,
    pub filter_seed: ShortHashSeed,
    pub segment_length: u32,
    pub segement_length_mask: u32,
    pub segment_count: u32,
    pub segment_count_length: u32,
    pub fingerprint_length: u32,
    pub fingerprints: BytesM,
}

impl SerializedBinaryFuseFilter {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_serialized_binary_fuse_filter(self);
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
        w.write_type_serialized_binary_fuse_filter(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

impl ConstWriter<'_> {
    /// Serializes a [`SerializedBinaryFuseFilter`], mirroring `<SerializedBinaryFuseFilter as WriteXdr>::write_xdr`.
    pub const fn write_type_serialized_binary_fuse_filter(
        &mut self,
        v: &SerializedBinaryFuseFilter,
    ) {
        self.write_type_binary_fuse_filter_type(&v.type_);
        self.write_type_short_hash_seed(&v.input_hash_seed);
        self.write_type_short_hash_seed(&v.filter_seed);
        self.write_u32(v.segment_length);
        self.write_u32(v.segement_length_mask);
        self.write_u32(v.segment_count);
        self.write_u32(v.segment_count_length);
        self.write_u32(v.fingerprint_length);
        self.write_var_opaque(v.fingerprints.as_slice());
    }
}
