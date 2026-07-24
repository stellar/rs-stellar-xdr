#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// PeerStats is an XDR Struct defined as:
///
/// ```text
/// struct PeerStats
/// {
///     NodeID id;
///     string versionStr<100>;
///     uint64 messagesRead;
///     uint64 messagesWritten;
///     uint64 bytesRead;
///     uint64 bytesWritten;
///     uint64 secondsConnected;
///
///     uint64 uniqueFloodBytesRecv;
///     uint64 duplicateFloodBytesRecv;
///     uint64 uniqueFetchBytesRecv;
///     uint64 duplicateFetchBytesRecv;
///
///     uint64 uniqueFloodMessageRecv;
///     uint64 duplicateFloodMessageRecv;
///     uint64 uniqueFetchMessageRecv;
///     uint64 duplicateFetchMessageRecv;
/// };
/// ```
///
#[cfg_attr(feature = "alloc", derive(Default))]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    serde_with::serde_as,
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct PeerStats {
    pub id: NodeId,
    pub version_str: StringM<100>,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub messages_read: u64,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub messages_written: u64,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub bytes_read: u64,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub bytes_written: u64,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub seconds_connected: u64,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub unique_flood_bytes_recv: u64,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub duplicate_flood_bytes_recv: u64,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub unique_fetch_bytes_recv: u64,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub duplicate_fetch_bytes_recv: u64,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub unique_flood_message_recv: u64,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub duplicate_flood_message_recv: u64,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub unique_fetch_message_recv: u64,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub duplicate_fetch_message_recv: u64,
}

impl ReadXdr for PeerStats {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                id: NodeId::read_xdr(r)?,
                version_str: StringM::<100>::read_xdr(r)?,
                messages_read: u64::read_xdr(r)?,
                messages_written: u64::read_xdr(r)?,
                bytes_read: u64::read_xdr(r)?,
                bytes_written: u64::read_xdr(r)?,
                seconds_connected: u64::read_xdr(r)?,
                unique_flood_bytes_recv: u64::read_xdr(r)?,
                duplicate_flood_bytes_recv: u64::read_xdr(r)?,
                unique_fetch_bytes_recv: u64::read_xdr(r)?,
                duplicate_fetch_bytes_recv: u64::read_xdr(r)?,
                unique_flood_message_recv: u64::read_xdr(r)?,
                duplicate_flood_message_recv: u64::read_xdr(r)?,
                unique_fetch_message_recv: u64::read_xdr(r)?,
                duplicate_fetch_message_recv: u64::read_xdr(r)?,
            })
        })
    }
}

impl PeerStats {
    /// Serialize this value as XDR into a [`ConstWriter`] using only const
    /// operations. This is the const counterpart to [`WriteXdr::write_xdr`].
    #[cfg(feature = "const")]
    pub const fn const_write_xdr(&self, w: &mut ConstWriter) {
        w.enter_depth();
        self.id.const_write_xdr(w);
        w.write_len_prefixed(self.version_str.0.as_slice());
        w.write_u64(self.messages_read);
        w.write_u64(self.messages_written);
        w.write_u64(self.bytes_read);
        w.write_u64(self.bytes_written);
        w.write_u64(self.seconds_connected);
        w.write_u64(self.unique_flood_bytes_recv);
        w.write_u64(self.duplicate_flood_bytes_recv);
        w.write_u64(self.unique_fetch_bytes_recv);
        w.write_u64(self.duplicate_fetch_bytes_recv);
        w.write_u64(self.unique_flood_message_recv);
        w.write_u64(self.duplicate_flood_message_recv);
        w.write_u64(self.unique_fetch_message_recv);
        w.write_u64(self.duplicate_fetch_message_recv);
        w.leave_depth();
    }
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[cfg(feature = "const")]
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let limits = Limits {
            depth: u32::MAX,
            len: usize::MAX,
        };
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty, &limits);
        self.const_write_xdr(&mut w);
        w.position()
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
    #[cfg(feature = "const")]
    #[must_use]
    pub const fn const_to_xdr<const N: usize>(&self) -> [u8; N] {
        let limits = Limits {
            depth: u32::MAX,
            len: usize::MAX,
        };
        let mut buf = [0u8; N];
        let mut w = ConstWriter::new(&mut buf, &limits);
        self.const_write_xdr(&mut w);
        assert!(
            w.position() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

impl WriteXdr for PeerStats {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.id.write_xdr(w)?;
            self.version_str.write_xdr(w)?;
            self.messages_read.write_xdr(w)?;
            self.messages_written.write_xdr(w)?;
            self.bytes_read.write_xdr(w)?;
            self.bytes_written.write_xdr(w)?;
            self.seconds_connected.write_xdr(w)?;
            self.unique_flood_bytes_recv.write_xdr(w)?;
            self.duplicate_flood_bytes_recv.write_xdr(w)?;
            self.unique_fetch_bytes_recv.write_xdr(w)?;
            self.duplicate_fetch_bytes_recv.write_xdr(w)?;
            self.unique_flood_message_recv.write_xdr(w)?;
            self.duplicate_flood_message_recv.write_xdr(w)?;
            self.unique_fetch_message_recv.write_xdr(w)?;
            self.duplicate_fetch_message_recv.write_xdr(w)?;
            Ok(())
        })
    }
}
