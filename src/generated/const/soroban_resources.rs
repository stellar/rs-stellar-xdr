#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// SorobanResources is a borrowing equivalent of [`SorobanResources`](super::super::SorobanResources)
/// over `'static` data, for const XDR encoding.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
pub struct SorobanResources {
    pub footprint: LedgerFootprint,
    pub instructions: u32,
    pub disk_read_bytes: u32,
    pub write_bytes: u32,
}

impl SorobanResources {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_soroban_resources(self);
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
        w.write_type_soroban_resources(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

impl ConstWriter<'_> {
    /// Serializes a [`SorobanResources`], mirroring `<SorobanResources as WriteXdr>::write_xdr`.
    pub const fn write_type_soroban_resources(&mut self, v: &SorobanResources) {
        self.write_type_ledger_footprint(&v.footprint);
        self.write_u32(v.instructions);
        self.write_u32(v.disk_read_bytes);
        self.write_u32(v.write_bytes);
    }
}
