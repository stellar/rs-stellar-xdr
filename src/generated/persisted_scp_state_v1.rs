#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// PersistedScpStateV1 is an XDR Struct defined as:
///
/// ```text
/// struct PersistedSCPStateV1
/// {
/// 	// Tx sets are saved separately
/// 	SCPEnvelope scpEnvelopes<>;
/// 	SCPQuorumSet quorumSets<>;
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
pub struct PersistedScpStateV1 {
    pub scp_envelopes: VecM<ScpEnvelope>,
    pub quorum_sets: VecM<ScpQuorumSet>,
}

impl ReadXdr for PersistedScpStateV1 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                scp_envelopes: VecM::<ScpEnvelope>::read_xdr(r)?,
                quorum_sets: VecM::<ScpQuorumSet>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for PersistedScpStateV1 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.scp_envelopes.write_xdr(w)?;
            self.quorum_sets.write_xdr(w)?;
            Ok(())
        })
    }
}

/// PersistedScpStateV1Ref is a borrowing equivalent of [`PersistedScpStateV1`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct PersistedScpStateV1Ref<'a> {
    pub scp_envelopes: VecMRef<'a, ScpEnvelopeRef<'a>>,
    pub quorum_sets: VecMRef<'a, ScpQuorumSetRef<'a>>,
}

#[cfg(feature = "alloc")]
impl IntoOwned for PersistedScpStateV1Ref<'_> {
    type Owned = PersistedScpStateV1;
    fn into_owned(self) -> PersistedScpStateV1 {
        PersistedScpStateV1 {
            scp_envelopes: self.scp_envelopes.into_owned(),
            quorum_sets: self.quorum_sets.into_owned(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&PersistedScpStateV1Ref<'_>> for PersistedScpStateV1 {
    #[must_use]
    fn from(v: &PersistedScpStateV1Ref<'_>) -> Self {
        v.into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<PersistedScpStateV1Ref<'_>> for PersistedScpStateV1 {
    #[must_use]
    fn from(v: PersistedScpStateV1Ref<'_>) -> Self {
        v.into_owned()
    }
}

impl WriteXdr for PersistedScpStateV1Ref<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.scp_envelopes.write_xdr(w)?;
            self.quorum_sets.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "const")]
impl PersistedScpStateV1Ref<'_> {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_persisted_scp_state_v1(self);
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
        w.write_type_persisted_scp_state_v1(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`PersistedScpStateV1`], mirroring `<PersistedScpStateV1 as WriteXdr>::write_xdr`.
    pub const fn write_type_persisted_scp_state_v1(&mut self, v: &PersistedScpStateV1Ref<'_>) {
        self.write_type_vec_scp_envelope(&v.scp_envelopes);
        self.write_type_vec_scp_quorum_set(&v.quorum_sets);
    }
}
