#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ParallelTxsComponent is an XDR Struct defined as:
///
/// ```text
/// struct ParallelTxsComponent
/// {
///   int64* baseFee;
///   // A sequence of stages that *may* have arbitrary data dependencies between
///   // each other, i.e. in a general case the stage execution order may not be
///   // arbitrarily shuffled without affecting the end result.
///   ParallelTxExecutionStage executionStages<>;
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
pub struct ParallelTxsComponent {
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "Option<NumberOrString>")
    )]
    pub base_fee: Option<i64>,
    pub execution_stages: VecM<ParallelTxExecutionStage>,
}

impl ReadXdr for ParallelTxsComponent {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                base_fee: Option::<i64>::read_xdr(r)?,
                execution_stages: VecM::<ParallelTxExecutionStage>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ParallelTxsComponent {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.base_fee.write_xdr(w)?;
            self.execution_stages.write_xdr(w)?;
            Ok(())
        })
    }
}

/// ParallelTxsComponentRef is a borrowing equivalent of [`ParallelTxsComponent`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParallelTxsComponentRef<'a> {
    pub base_fee: Option<i64>,
    pub execution_stages: VecMRef<'a, ParallelTxExecutionStageRef<'a>>,
}

#[cfg(feature = "alloc")]
impl From<&ParallelTxsComponentRef<'_>> for ParallelTxsComponent {
    #[must_use]
    fn from(v: &ParallelTxsComponentRef<'_>) -> Self {
        Self {
            base_fee: v.base_fee,
            execution_stages: v.execution_stages.to_vecm_from(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<ParallelTxsComponentRef<'_>> for ParallelTxsComponent {
    #[must_use]
    fn from(v: ParallelTxsComponentRef<'_>) -> Self {
        Self::from(&v)
    }
}
