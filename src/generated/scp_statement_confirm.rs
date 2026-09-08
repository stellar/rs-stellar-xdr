#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ScpStatementConfirm is an XDR NestedStruct defined as:
///
/// ```text
/// struct
///         {
///             SCPBallot ballot;   // b
///             uint32 nPrepared;   // p.n
///             uint32 nCommit;     // c.n
///             uint32 nH;          // h.n
///             Hash quorumSetHash; // D
///         }
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
pub struct ScpStatementConfirm {
    pub ballot: ScpBallot,
    pub n_prepared: u32,
    pub n_commit: u32,
    pub n_h: u32,
    pub quorum_set_hash: Hash,
}

impl ReadXdr for ScpStatementConfirm {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                ballot: ScpBallot::read_xdr(r)?,
                n_prepared: u32::read_xdr(r)?,
                n_commit: u32::read_xdr(r)?,
                n_h: u32::read_xdr(r)?,
                quorum_set_hash: Hash::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ScpStatementConfirm {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.ballot.write_xdr(w)?;
            self.n_prepared.write_xdr(w)?;
            self.n_commit.write_xdr(w)?;
            self.n_h.write_xdr(w)?;
            self.quorum_set_hash.write_xdr(w)?;
            Ok(())
        })
    }
}

/// ScpStatementConfirmConst is a borrowing equivalent of [`ScpStatementConfirm`] over `'static`
/// data, for const XDR encoding.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScpStatementConfirmConst {
    pub ballot: ScpBallotConst,
    pub n_prepared: u32,
    pub n_commit: u32,
    pub n_h: u32,
    pub quorum_set_hash: Hash,
}

#[cfg(feature = "const")]
impl ScpStatementConfirmConst {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_scp_statement_confirm(self);
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
        w.write_type_scp_statement_confirm(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`ScpStatementConfirm`], mirroring `<ScpStatementConfirm as WriteXdr>::write_xdr`.
    pub const fn write_type_scp_statement_confirm(&mut self, v: &ScpStatementConfirmConst) {
        self.write_type_scp_ballot(&v.ballot);
        self.write_u32(v.n_prepared);
        self.write_u32(v.n_commit);
        self.write_u32(v.n_h);
        self.write_type_hash(&v.quorum_set_hash);
    }
}
