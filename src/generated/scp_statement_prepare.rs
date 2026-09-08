#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ScpStatementPrepare is an XDR NestedStruct defined as:
///
/// ```text
/// struct
///         {
///             Hash quorumSetHash;       // D
///             SCPBallot ballot;         // b
///             SCPBallot* prepared;      // p
///             SCPBallot* preparedPrime; // p'
///             uint32 nC;                // c.n
///             uint32 nH;                // h.n
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
pub struct ScpStatementPrepare {
    pub quorum_set_hash: Hash,
    pub ballot: ScpBallot,
    pub prepared: Option<ScpBallot>,
    pub prepared_prime: Option<ScpBallot>,
    pub n_c: u32,
    pub n_h: u32,
}

impl ReadXdr for ScpStatementPrepare {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                quorum_set_hash: Hash::read_xdr(r)?,
                ballot: ScpBallot::read_xdr(r)?,
                prepared: Option::<ScpBallot>::read_xdr(r)?,
                prepared_prime: Option::<ScpBallot>::read_xdr(r)?,
                n_c: u32::read_xdr(r)?,
                n_h: u32::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ScpStatementPrepare {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.quorum_set_hash.write_xdr(w)?;
            self.ballot.write_xdr(w)?;
            self.prepared.write_xdr(w)?;
            self.prepared_prime.write_xdr(w)?;
            self.n_c.write_xdr(w)?;
            self.n_h.write_xdr(w)?;
            Ok(())
        })
    }
}

/// ScpStatementPrepareConst is a borrowing equivalent of [`ScpStatementPrepare`] over `'static`
/// data, for const XDR encoding.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScpStatementPrepareConst {
    pub quorum_set_hash: Hash,
    pub ballot: ScpBallotConst,
    pub prepared: Option<ScpBallotConst>,
    pub prepared_prime: Option<ScpBallotConst>,
    pub n_c: u32,
    pub n_h: u32,
}

#[cfg(feature = "const")]
impl ScpStatementPrepareConst {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_scp_statement_prepare(self);
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
        w.write_type_scp_statement_prepare(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`ScpStatementPrepare`], mirroring `<ScpStatementPrepare as WriteXdr>::write_xdr`.
    pub const fn write_type_scp_statement_prepare(&mut self, v: &ScpStatementPrepareConst) {
        self.write_type_hash(&v.quorum_set_hash);
        self.write_type_scp_ballot(&v.ballot);
        self.write_type_option_scp_ballot(&v.prepared);
        self.write_type_option_scp_ballot(&v.prepared_prime);
        self.write_u32(v.n_c);
        self.write_u32(v.n_h);
    }
}
