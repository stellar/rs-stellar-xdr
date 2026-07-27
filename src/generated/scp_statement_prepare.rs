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

/// ScpStatementPrepareRef is a borrowing equivalent of [`ScpStatementPrepare`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScpStatementPrepareRef<'a> {
    pub quorum_set_hash: Hash,
    pub ballot: ScpBallotRef<'a>,
    pub prepared: Option<ScpBallotRef<'a>>,
    pub prepared_prime: Option<ScpBallotRef<'a>>,
    pub n_c: u32,
    pub n_h: u32,
}

#[cfg(feature = "alloc")]
impl From<&ScpStatementPrepareRef<'_>> for ScpStatementPrepare {
    #[must_use]
    fn from(v: &ScpStatementPrepareRef<'_>) -> Self {
        Self {
            quorum_set_hash: v.quorum_set_hash.clone(),
            ballot: (&v.ballot).into(),
            prepared: v.prepared.as_ref().map(Into::into),
            prepared_prime: v.prepared_prime.as_ref().map(Into::into),
            n_c: v.n_c,
            n_h: v.n_h,
        }
    }
}

#[cfg(feature = "alloc")]
impl From<ScpStatementPrepareRef<'_>> for ScpStatementPrepare {
    #[must_use]
    fn from(v: ScpStatementPrepareRef<'_>) -> Self {
        Self::from(&v)
    }
}
