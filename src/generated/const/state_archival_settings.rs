#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

impl StateArchivalSettings {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_state_archival_settings(self);
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
        w.write_type_state_archival_settings(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

impl ConstWriter<'_> {
    /// Serializes a [`StateArchivalSettings`], mirroring `<StateArchivalSettings as WriteXdr>::write_xdr`.
    pub const fn write_type_state_archival_settings(&mut self, v: &StateArchivalSettings) {
        self.write_u32(v.max_entry_ttl);
        self.write_u32(v.min_temporary_ttl);
        self.write_u32(v.min_persistent_ttl);
        self.write_i64(v.persistent_rent_rate_denominator);
        self.write_i64(v.temp_rent_rate_denominator);
        self.write_u32(v.max_entries_to_archive);
        self.write_u32(v.live_soroban_state_size_window_sample_size);
        self.write_u32(v.live_soroban_state_size_window_sample_period);
        self.write_u32(v.eviction_scan_size);
        self.write_u32(v.starting_eviction_scan_level);
    }
}
