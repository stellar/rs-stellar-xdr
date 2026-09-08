#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ScpStatement is an XDR Struct defined as:
///
/// ```text
/// struct SCPStatement
/// {
///     NodeID nodeID;    // v
///     uint64 slotIndex; // i
///
///     union switch (SCPStatementType type)
///     {
///     case SCP_ST_PREPARE:
///         struct
///         {
///             Hash quorumSetHash;       // D
///             SCPBallot ballot;         // b
///             SCPBallot* prepared;      // p
///             SCPBallot* preparedPrime; // p'
///             uint32 nC;                // c.n
///             uint32 nH;                // h.n
///         } prepare;
///     case SCP_ST_CONFIRM:
///         struct
///         {
///             SCPBallot ballot;   // b
///             uint32 nPrepared;   // p.n
///             uint32 nCommit;     // c.n
///             uint32 nH;          // h.n
///             Hash quorumSetHash; // D
///         } confirm;
///     case SCP_ST_EXTERNALIZE:
///         struct
///         {
///             SCPBallot commit;         // c
///             uint32 nH;                // h.n
///             Hash commitQuorumSetHash; // D used before EXTERNALIZE
///         } externalize;
///     case SCP_ST_NOMINATE:
///         SCPNomination nominate;
///     }
///     pledges;
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
pub struct ScpStatement {
    pub node_id: NodeId,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub slot_index: u64,
    pub pledges: ScpStatementPledges,
}

impl ReadXdr for ScpStatement {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                node_id: NodeId::read_xdr(r)?,
                slot_index: u64::read_xdr(r)?,
                pledges: ScpStatementPledges::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ScpStatement {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.node_id.write_xdr(w)?;
            self.slot_index.write_xdr(w)?;
            self.pledges.write_xdr(w)?;
            Ok(())
        })
    }
}

/// ScpStatementConst is a borrowing equivalent of [`ScpStatement`] over `'static`
/// data, for const XDR encoding.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScpStatementConst {
    pub node_id: NodeId,
    pub slot_index: u64,
    pub pledges: ScpStatementPledgesConst,
}

#[cfg(feature = "const")]
impl ScpStatementConst {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_scp_statement(self);
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
        w.write_type_scp_statement(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`ScpStatement`], mirroring `<ScpStatement as WriteXdr>::write_xdr`.
    pub const fn write_type_scp_statement(&mut self, v: &ScpStatementConst) {
        self.write_type_node_id(&v.node_id);
        self.write_u64(v.slot_index);
        self.write_type_scp_statement_pledges(&v.pledges);
    }
}
