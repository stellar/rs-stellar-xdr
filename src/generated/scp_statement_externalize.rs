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
#[cfg_eval::cfg_eval]
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
