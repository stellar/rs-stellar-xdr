#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ParallelTxExecutionStage is an XDR Typedef defined as:
///
/// ```text
/// typedef DependentTxCluster ParallelTxExecutionStage<>;
/// ```
///
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval)]
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

/// ParallelTxExecutionStageConst is a borrowing equivalent of [`ParallelTxExecutionStage`] over `'static`
/// data, for const XDR encoding.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParallelTxExecutionStageConst(pub VecMConst<DependentTxClusterConst>);

#[cfg(feature = "const")]
impl ParallelTxExecutionStageConst {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_parallel_tx_execution_stage(self);
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
        w.write_type_parallel_tx_execution_stage(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`ParallelTxExecutionStage`], mirroring `<ParallelTxExecutionStage as WriteXdr>::write_xdr`.
    pub const fn write_type_parallel_tx_execution_stage(
        &mut self,
        v: &ParallelTxExecutionStageConst,
    ) {
        self.write_type_vec_dependent_tx_cluster(&v.0);
    }

    /// Serializes a variable-length array of [`ParallelTxExecutionStage`], mirroring `<VecM<ParallelTxExecutionStage, MAX> as WriteXdr>::write_xdr`.
    pub const fn write_type_vec_parallel_tx_execution_stage<const MAX: u32>(
        &mut self,
        v: &VecMConst<ParallelTxExecutionStageConst, MAX>,
    ) {
        let s = v.as_slice();
        let len = s.len();
        self.write_len(len);
        let mut i = 0usize;
        while i < len {
            self.write_type_parallel_tx_execution_stage(&s[i]);
            i += 1;
        }
    }
}
