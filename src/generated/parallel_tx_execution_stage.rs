#[allow(unused_imports)]
use super::*;
/// ParallelTxExecutionStage is an XDR Typedef defined as:
///
/// ```text
/// typedef DependentTxCluster ParallelTxExecutionStage<>;
/// ```
///
#[cfg_eval::cfg_eval]
#[derive(Default, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    serde_with::serde_as,
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Debug)]
pub struct ParallelTxExecutionStage(pub VecM<DependentTxCluster>);

impl From<ParallelTxExecutionStage> for VecM<DependentTxCluster> {
    #[must_use]
    fn from(x: ParallelTxExecutionStage) -> Self {
        x.0
    }
}

impl From<VecM<DependentTxCluster>> for ParallelTxExecutionStage {
    #[must_use]
    fn from(x: VecM<DependentTxCluster>) -> Self {
        ParallelTxExecutionStage(x)
    }
}

impl AsRef<VecM<DependentTxCluster>> for ParallelTxExecutionStage {
    #[must_use]
    fn as_ref(&self) -> &VecM<DependentTxCluster> {
        &self.0
    }
}

impl ReadXdr for ParallelTxExecutionStage {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let i = VecM::<DependentTxCluster>::read_xdr(r)?;
            let v = ParallelTxExecutionStage(i);
            Ok(v)
        })
    }
}

impl WriteXdr for ParallelTxExecutionStage {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| self.0.write_xdr(w))
    }
}

impl Deref for ParallelTxExecutionStage {
    type Target = VecM<DependentTxCluster>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<ParallelTxExecutionStage> for Vec<DependentTxCluster> {
    #[must_use]
    fn from(x: ParallelTxExecutionStage) -> Self {
        x.0 .0
    }
}

impl TryFrom<Vec<DependentTxCluster>> for ParallelTxExecutionStage {
    type Error = Error;
    fn try_from(x: Vec<DependentTxCluster>) -> Result<Self, Error> {
        Ok(ParallelTxExecutionStage(x.try_into()?))
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<&Vec<DependentTxCluster>> for ParallelTxExecutionStage {
    type Error = Error;
    fn try_from(x: &Vec<DependentTxCluster>) -> Result<Self, Error> {
        Ok(ParallelTxExecutionStage(x.try_into()?))
    }
}

impl AsRef<Vec<DependentTxCluster>> for ParallelTxExecutionStage {
    #[must_use]
    fn as_ref(&self) -> &Vec<DependentTxCluster> {
        &self.0 .0
    }
}

impl AsRef<[DependentTxCluster]> for ParallelTxExecutionStage {
    #[cfg(feature = "alloc")]
    #[must_use]
    fn as_ref(&self) -> &[DependentTxCluster] {
        &self.0 .0
    }
    #[cfg(not(feature = "alloc"))]
    #[must_use]
    fn as_ref(&self) -> &[DependentTxCluster] {
        self.0 .0
    }
}
