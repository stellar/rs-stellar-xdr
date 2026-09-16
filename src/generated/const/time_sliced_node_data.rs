#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

impl TimeSlicedNodeData {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_time_sliced_node_data(self);
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
        w.write_type_time_sliced_node_data(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

impl ConstWriter<'_> {
    /// Serializes a [`TimeSlicedNodeData`], mirroring `<TimeSlicedNodeData as WriteXdr>::write_xdr`.
    pub const fn write_type_time_sliced_node_data(&mut self, v: &TimeSlicedNodeData) {
        self.write_u32(v.added_authenticated_peers);
        self.write_u32(v.dropped_authenticated_peers);
        self.write_u32(v.total_inbound_peer_count);
        self.write_u32(v.total_outbound_peer_count);
        self.write_u32(v.p75_scp_first_to_self_latency_ms);
        self.write_u32(v.p75_scp_self_to_other_latency_ms);
        self.write_u32(v.lost_sync_count);
        self.write_bool(v.is_validator);
        self.write_u32(v.max_inbound_peer_count);
        self.write_u32(v.max_outbound_peer_count);
    }
}
