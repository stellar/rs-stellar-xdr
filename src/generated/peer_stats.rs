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

/// PeerStatsView is a borrowing equivalent of [`PeerStats`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct PeerStatsView<'a> {
    pub id: NodeId,
    pub version_str: StringMView<'a, 100>,
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

#[cfg(feature = "alloc")]
impl From<&PeerStatsView<'_>> for PeerStats {
    #[must_use]
    fn from(v: &PeerStatsView<'_>) -> Self {
        Self {
            id: v.id.clone(),
            version_str: v.version_str.to_stringm(),
            messages_read: v.messages_read,
            messages_written: v.messages_written,
            bytes_read: v.bytes_read,
            bytes_written: v.bytes_written,
            seconds_connected: v.seconds_connected,
            unique_flood_bytes_recv: v.unique_flood_bytes_recv,
            duplicate_flood_bytes_recv: v.duplicate_flood_bytes_recv,
            unique_fetch_bytes_recv: v.unique_fetch_bytes_recv,
            duplicate_fetch_bytes_recv: v.duplicate_fetch_bytes_recv,
            unique_flood_message_recv: v.unique_flood_message_recv,
            duplicate_flood_message_recv: v.duplicate_flood_message_recv,
            unique_fetch_message_recv: v.unique_fetch_message_recv,
            duplicate_fetch_message_recv: v.duplicate_fetch_message_recv,
        }
    }
}

#[cfg(feature = "alloc")]
impl From<PeerStatsView<'_>> for PeerStats {
    #[must_use]
    fn from(v: PeerStatsView<'_>) -> Self {
        Self::from(&v)
    }
}

impl WriteXdr for PeerStatsView<'_> {
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
