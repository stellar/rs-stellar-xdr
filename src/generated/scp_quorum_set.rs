#[allow(unused_imports)]
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
#[cfg_eval::cfg_eval]
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
