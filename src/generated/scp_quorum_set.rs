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

/// ScpQuorumSetView is a borrowing equivalent of [`ScpQuorumSet`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScpQuorumSetView<'a> {
    pub threshold: u32,
    pub validators: VecMView<'a, NodeId>,
    pub inner_sets: VecMView<'a, ScpQuorumSetView<'a>>,
}

#[cfg(feature = "alloc")]
impl IntoOwned for ScpQuorumSetView<'_> {
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
impl From<&ScpQuorumSetView<'_>> for ScpQuorumSet {
    #[must_use]
    fn from(v: &ScpQuorumSetView<'_>) -> Self {
        v.into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<ScpQuorumSetView<'_>> for ScpQuorumSet {
    #[must_use]
    fn from(v: ScpQuorumSetView<'_>) -> Self {
        v.into_owned()
    }
}

impl WriteXdr for ScpQuorumSetView<'_> {
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
