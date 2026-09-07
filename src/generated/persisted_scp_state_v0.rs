#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// PersistedScpStateV0 is an XDR Struct defined as:
///
/// ```text
/// struct PersistedSCPStateV0
/// {
/// 	SCPEnvelope scpEnvelopes<>;
/// 	SCPQuorumSet quorumSets<>;
/// 	StoredTransactionSet txSets<>;
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
pub struct PersistedScpStateV0 {
    pub scp_envelopes: VecM<ScpEnvelope>,
    pub quorum_sets: VecM<ScpQuorumSet>,
    pub tx_sets: VecM<StoredTransactionSet>,
}

impl ReadXdr for PersistedScpStateV0 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                scp_envelopes: VecM::<ScpEnvelope>::read_xdr(r)?,
                quorum_sets: VecM::<ScpQuorumSet>::read_xdr(r)?,
                tx_sets: VecM::<StoredTransactionSet>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for PersistedScpStateV0 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.scp_envelopes.write_xdr(w)?;
            self.quorum_sets.write_xdr(w)?;
            self.tx_sets.write_xdr(w)?;
            Ok(())
        })
    }
}

/// PersistedScpStateV0View is a borrowing equivalent of [`PersistedScpStateV0`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct PersistedScpStateV0View<'a> {
    pub scp_envelopes: VecMView<'a, ScpEnvelopeView<'a>>,
    pub quorum_sets: VecMView<'a, ScpQuorumSetView<'a>>,
    pub tx_sets: VecMView<'a, StoredTransactionSetView<'a>>,
}

#[cfg(feature = "alloc")]
impl IntoOwned for PersistedScpStateV0View<'_> {
    type Owned = PersistedScpStateV0;
    fn into_owned(self) -> PersistedScpStateV0 {
        PersistedScpStateV0 {
            scp_envelopes: self.scp_envelopes.into_owned(),
            quorum_sets: self.quorum_sets.into_owned(),
            tx_sets: self.tx_sets.into_owned(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&PersistedScpStateV0View<'_>> for PersistedScpStateV0 {
    #[must_use]
    fn from(v: &PersistedScpStateV0View<'_>) -> Self {
        v.clone().into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<PersistedScpStateV0View<'_>> for PersistedScpStateV0 {
    #[must_use]
    fn from(v: PersistedScpStateV0View<'_>) -> Self {
        v.into_owned()
    }
}

impl WriteXdr for PersistedScpStateV0View<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.scp_envelopes.write_xdr(w)?;
            self.quorum_sets.write_xdr(w)?;
            self.tx_sets.write_xdr(w)?;
            Ok(())
        })
    }
}
