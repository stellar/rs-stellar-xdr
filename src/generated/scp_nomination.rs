#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ScpNomination is an XDR Struct defined as:
///
/// ```text
/// struct SCPNomination
/// {
///     Hash quorumSetHash; // D
///     Value votes<>;      // X
///     Value accepted<>;   // Y
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
pub struct ScpNomination {
    pub quorum_set_hash: Hash,
    pub votes: VecM<Value>,
    pub accepted: VecM<Value>,
}

impl ReadXdr for ScpNomination {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                quorum_set_hash: Hash::read_xdr(r)?,
                votes: VecM::<Value>::read_xdr(r)?,
                accepted: VecM::<Value>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ScpNomination {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.quorum_set_hash.write_xdr(w)?;
            self.votes.write_xdr(w)?;
            self.accepted.write_xdr(w)?;
            Ok(())
        })
    }
}

/// ScpNominationView is a borrowing equivalent of [`ScpNomination`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScpNominationView<'a> {
    pub quorum_set_hash: Hash,
    pub votes: VecMView<'a, ValueView<'a>>,
    pub accepted: VecMView<'a, ValueView<'a>>,
}

#[cfg(feature = "alloc")]
impl IntoOwned for ScpNominationView<'_> {
    type Owned = ScpNomination;
    fn into_owned(self) -> ScpNomination {
        ScpNomination {
            quorum_set_hash: self.quorum_set_hash.into_owned(),
            votes: self.votes.into_owned(),
            accepted: self.accepted.into_owned(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&ScpNominationView<'_>> for ScpNomination {
    #[must_use]
    fn from(v: &ScpNominationView<'_>) -> Self {
        v.into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<ScpNominationView<'_>> for ScpNomination {
    #[must_use]
    fn from(v: ScpNominationView<'_>) -> Self {
        v.into_owned()
    }
}

impl WriteXdr for ScpNominationView<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.quorum_set_hash.write_xdr(w)?;
            self.votes.write_xdr(w)?;
            self.accepted.write_xdr(w)?;
            Ok(())
        })
    }
}
