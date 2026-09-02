#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ScpHistoryEntryV0 is an XDR Struct defined as:
///
/// ```text
/// struct SCPHistoryEntryV0
/// {
///     SCPQuorumSet quorumSets<>; // additional quorum sets used by ledgerMessages
///     LedgerSCPMessages ledgerMessages;
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
pub struct ScpHistoryEntryV0 {
    pub quorum_sets: VecM<ScpQuorumSet>,
    pub ledger_messages: LedgerScpMessages,
}

impl ReadXdr for ScpHistoryEntryV0 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                quorum_sets: VecM::<ScpQuorumSet>::read_xdr(r)?,
                ledger_messages: LedgerScpMessages::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ScpHistoryEntryV0 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.quorum_sets.write_xdr(w)?;
            self.ledger_messages.write_xdr(w)?;
            Ok(())
        })
    }
}

/// ScpHistoryEntryV0View is a borrowing equivalent of [`ScpHistoryEntryV0`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScpHistoryEntryV0View<'a> {
    pub quorum_sets: VecMView<'a, ScpQuorumSetView<'a>>,
    pub ledger_messages: LedgerScpMessagesView<'a>,
}

#[cfg(feature = "alloc")]
impl From<&ScpHistoryEntryV0View<'_>> for ScpHistoryEntryV0 {
    #[must_use]
    fn from(v: &ScpHistoryEntryV0View<'_>) -> Self {
        Self {
            quorum_sets: v.quorum_sets.to_vecm(),
            ledger_messages: (&v.ledger_messages).into(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<ScpHistoryEntryV0View<'_>> for ScpHistoryEntryV0 {
    #[must_use]
    fn from(v: ScpHistoryEntryV0View<'_>) -> Self {
        Self::from(&v)
    }
}

impl WriteXdr for ScpHistoryEntryV0View<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.quorum_sets.write_xdr(w)?;
            self.ledger_messages.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "const")]
impl ScpHistoryEntryV0View<'_> {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_scp_history_entry_v0(self);
        w.len()
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
    #[must_use]
    pub const fn const_to_xdr<const N: usize>(&self) -> [u8; N] {
        let mut buf = [0u8; N];
        let mut w = ConstWriter::new(&mut buf);
        w.write_type_scp_history_entry_v0(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`ScpHistoryEntryV0`], mirroring `<ScpHistoryEntryV0 as WriteXdr>::write_xdr`.
    pub const fn write_type_scp_history_entry_v0(&mut self, v: &ScpHistoryEntryV0View<'_>) {
        self.write_type_vec_scp_quorum_set(&v.quorum_sets);
        self.write_type_ledger_scp_messages(&v.ledger_messages);
    }
}
