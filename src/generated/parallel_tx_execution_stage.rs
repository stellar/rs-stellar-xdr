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

impl ParallelTxExecutionStage {
    /// Serialize this value as XDR into a [`ConstWriter`] using only const
    /// operations. This is the const implementation underlying `to_xdr`.
    #[cfg(feature = "std")]
    pub const fn const_write_xdr(&self, w: &mut ConstWriter) {
        w.enter_depth();
        {
            w.enter_depth();
            let __s0 = self.0 .0.as_slice();
            let __len0 = __s0.len();
            w.write_length_prefix(__len0);
            let mut __i0 = 0usize;
            while __i0 < __len0 {
                __s0[__i0].const_write_xdr(w);
                __i0 += 1;
            }
            w.leave_depth();
        }
        w.leave_depth();
    }
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::to_xdr_array`] at compile time.
    #[cfg(feature = "std")]
    #[must_use]
    pub const fn xdr_len(&self) -> usize {
        let limits = Limits {
            depth: u32::MAX,
            len: usize::MAX,
        };
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty, &limits);
        self.const_write_xdr(&mut w);
        w.position()
    }

    /// Serialize this value as XDR into a fixed-size `[u8; N]` using only const
    /// operations.
    ///
    /// `N` must equal [`Self::xdr_len`]. It is intended for callers, such as a
    /// proc-macro, that compute the length with `xdr_len` and pass it as `N`;
    /// `to_xdr_array` itself does not need to call `xdr_len`.
    ///
    /// # Panics
    ///
    /// Panics if `N` does not equal the value's [`Self::xdr_len`].
    #[cfg(feature = "std")]
    #[must_use]
    pub const fn to_xdr_array<const N: usize>(&self) -> [u8; N] {
        let limits = Limits {
            depth: u32::MAX,
            len: usize::MAX,
        };
        let mut buf = [0u8; N];
        let mut w = ConstWriter::new(&mut buf, &limits);
        self.const_write_xdr(&mut w);
        assert!(
            w.position() == N,
            "to_xdr_array: N does not equal the XDR-encoded length"
        );
        buf
    }
}

impl WriteXdr for ParallelTxExecutionStage {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        write_xdr_via_const(self, w, Self::const_write_xdr)
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
