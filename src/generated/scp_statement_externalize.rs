#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ScpStatementExternalize is an XDR NestedStruct defined as:
///
/// ```text
/// struct
///         {
///             SCPBallot commit;         // c
///             uint32 nH;                // h.n
///             Hash commitQuorumSetHash; // D used before EXTERNALIZE
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
pub struct ScpStatementExternalize {
    pub commit: ScpBallot,
    pub n_h: u32,
    pub commit_quorum_set_hash: Hash,
}

impl ReadXdr for ScpStatementExternalize {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                commit: ScpBallot::read_xdr(r)?,
                n_h: u32::read_xdr(r)?,
                commit_quorum_set_hash: Hash::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ScpStatementExternalize {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.commit.write_xdr(w)?;
            self.n_h.write_xdr(w)?;
            self.commit_quorum_set_hash.write_xdr(w)?;
            Ok(())
        })
    }
}

/// ScpStatementExternalizeRef is a borrowing equivalent of [`ScpStatementExternalize`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScpStatementExternalizeRef<'a> {
    pub commit: ScpBallotRef<'a>,
    pub n_h: u32,
    pub commit_quorum_set_hash: Hash,
}

#[cfg(feature = "alloc")]
impl IntoOwned for ScpStatementExternalizeRef<'_> {
    type Owned = ScpStatementExternalize;
    fn into_owned(self) -> ScpStatementExternalize {
        ScpStatementExternalize {
            commit: self.commit.into_owned(),
            n_h: self.n_h.into_owned(),
            commit_quorum_set_hash: self.commit_quorum_set_hash.into_owned(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&ScpStatementExternalizeRef<'_>> for ScpStatementExternalize {
    #[must_use]
    fn from(v: &ScpStatementExternalizeRef<'_>) -> Self {
        v.into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<ScpStatementExternalizeRef<'_>> for ScpStatementExternalize {
    #[must_use]
    fn from(v: ScpStatementExternalizeRef<'_>) -> Self {
        v.into_owned()
    }
}

impl WriteXdr for ScpStatementExternalizeRef<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.commit.write_xdr(w)?;
            self.n_h.write_xdr(w)?;
            self.commit_quorum_set_hash.write_xdr(w)?;
            Ok(())
        })
    }
}
