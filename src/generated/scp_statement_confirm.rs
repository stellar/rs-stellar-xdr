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

/// ScpStatementConfirmView is a borrowing equivalent of [`ScpStatementConfirm`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScpStatementConfirmView<'a> {
    pub ballot: ScpBallotView<'a>,
    pub n_prepared: u32,
    pub n_commit: u32,
    pub n_h: u32,
    pub quorum_set_hash: Hash,
}

#[cfg(feature = "alloc")]
impl IntoOwned for ScpStatementConfirmView<'_> {
    type Owned = ScpStatementConfirm;
    fn into_owned(self) -> ScpStatementConfirm {
        ScpStatementConfirm {
            ballot: self.ballot.into_owned(),
            n_prepared: self.n_prepared.into_owned(),
            n_commit: self.n_commit.into_owned(),
            n_h: self.n_h.into_owned(),
            quorum_set_hash: self.quorum_set_hash.into_owned(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&ScpStatementConfirmView<'_>> for ScpStatementConfirm {
    #[must_use]
    fn from(v: &ScpStatementConfirmView<'_>) -> Self {
        v.into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<ScpStatementConfirmView<'_>> for ScpStatementConfirm {
    #[must_use]
    fn from(v: ScpStatementConfirmView<'_>) -> Self {
        v.into_owned()
    }
}

impl WriteXdr for ScpStatementConfirmView<'_> {
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
