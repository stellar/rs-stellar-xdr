#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ScpQuorumSet is an XDR Struct defined as:
///
/// ```text
/// struct SCPQuorumSet
/// {
///     uint32 threshold;
///     NodeID validators<>;
///     SCPQuorumSet innerSets<>;
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
pub struct ScpQuorumSet {
    pub threshold: u32,
    pub validators: VecM<NodeId>,
    pub inner_sets: VecM<ScpQuorumSet>,
}

impl ReadXdr for ScpQuorumSet {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                threshold: u32::read_xdr(r)?,
                validators: VecM::<NodeId>::read_xdr(r)?,
                inner_sets: VecM::<ScpQuorumSet>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ScpQuorumSet {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.threshold.write_xdr(w)?;
            self.validators.write_xdr(w)?;
            self.inner_sets.write_xdr(w)?;
            Ok(())
        })
    }
}

/// ScpQuorumSetRef is a borrowing equivalent of [`ScpQuorumSet`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScpQuorumSetRef<'a> {
    pub threshold: u32,
    pub validators: VecMRef<'a, NodeId>,
    pub inner_sets: VecMRef<'a, ScpQuorumSetRef<'a>>,
}

#[cfg(feature = "alloc")]
impl IntoOwned for ScpQuorumSetRef<'_> {
    type Owned = ScpQuorumSet;
    fn into_owned(self) -> ScpQuorumSet {
        ScpQuorumSet {
            threshold: self.threshold.into_owned(),
            validators: self.validators.into_owned(),
            inner_sets: self.inner_sets.into_owned(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&ScpQuorumSetRef<'_>> for ScpQuorumSet {
    #[must_use]
    fn from(v: &ScpQuorumSetRef<'_>) -> Self {
        v.into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<ScpQuorumSetRef<'_>> for ScpQuorumSet {
    #[must_use]
    fn from(v: ScpQuorumSetRef<'_>) -> Self {
        v.into_owned()
    }
}

impl WriteXdr for ScpQuorumSetRef<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.threshold.write_xdr(w)?;
            self.validators.write_xdr(w)?;
            self.inner_sets.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "const")]
impl ScpQuorumSetRef<'_> {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_scp_quorum_set(self);
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
        w.write_type_scp_quorum_set(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`ScpQuorumSet`], mirroring `<ScpQuorumSet as WriteXdr>::write_xdr`.
    pub const fn write_type_scp_quorum_set(&mut self, v: &ScpQuorumSetRef<'_>) {
        self.write_u32(v.threshold);
        self.write_type_vec_node_id(&v.validators);
        self.write_type_vec_scp_quorum_set(&v.inner_sets);
    }

    /// Serializes a variable-length array of [`ScpQuorumSet`], mirroring `<VecM<ScpQuorumSet, MAX> as WriteXdr>::write_xdr`.
    pub const fn write_type_vec_scp_quorum_set<const MAX: u32>(
        &mut self,
        v: &VecMRef<'_, ScpQuorumSetRef<'_>, MAX>,
    ) {
        let s = v.as_slice();
        let len = s.len();
        self.write_len(len);
        let mut i = 0usize;
        while i < len {
            self.write_type_scp_quorum_set(&s[i]);
            i += 1;
        }
    }
}
