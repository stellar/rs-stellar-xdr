#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// PeerStats is a borrowing equivalent of [`PeerStats`](super::super::PeerStats)
/// over `'static` data, for const XDR encoding.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
pub struct PeerStats {
    pub id: NodeId,
    pub version_str: StringM<100>,
    pub messages_read: u64,
    pub messages_written: u64,
    pub bytes_read: u64,
    pub bytes_written: u64,
    pub seconds_connected: u64,
    pub unique_flood_bytes_recv: u64,
    pub duplicate_flood_bytes_recv: u64,
    pub unique_fetch_bytes_recv: u64,
    pub duplicate_fetch_bytes_recv: u64,
    pub unique_flood_message_recv: u64,
    pub duplicate_flood_message_recv: u64,
    pub unique_fetch_message_recv: u64,
    pub duplicate_fetch_message_recv: u64,
}

impl PeerStats {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_peer_stats(self);
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
        w.write_type_peer_stats(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

impl ConstWriter<'_> {
    /// Serializes a [`PeerStats`], mirroring `<PeerStats as WriteXdr>::write_xdr`.
    pub const fn write_type_peer_stats(&mut self, v: &PeerStats) {
        self.write_type_node_id(&v.id);
        self.write_var_opaque(v.version_str.as_slice());
        self.write_u64(v.messages_read);
        self.write_u64(v.messages_written);
        self.write_u64(v.bytes_read);
        self.write_u64(v.bytes_written);
        self.write_u64(v.seconds_connected);
        self.write_u64(v.unique_flood_bytes_recv);
        self.write_u64(v.duplicate_flood_bytes_recv);
        self.write_u64(v.unique_fetch_bytes_recv);
        self.write_u64(v.duplicate_fetch_bytes_recv);
        self.write_u64(v.unique_flood_message_recv);
        self.write_u64(v.duplicate_flood_message_recv);
        self.write_u64(v.unique_fetch_message_recv);
        self.write_u64(v.duplicate_fetch_message_recv);
    }
}
