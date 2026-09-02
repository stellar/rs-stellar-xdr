#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// DependentTxCluster is an XDR Typedef defined as:
///
/// ```text
/// typedef TransactionEnvelope DependentTxCluster<>;
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
pub struct DependentTxCluster(pub VecM<TransactionEnvelope>);

impl From<DependentTxCluster> for VecM<TransactionEnvelope> {
    #[must_use]
    fn from(x: DependentTxCluster) -> Self {
        x.0
    }
}

impl From<VecM<TransactionEnvelope>> for DependentTxCluster {
    #[must_use]
    fn from(x: VecM<TransactionEnvelope>) -> Self {
        DependentTxCluster(x)
    }
}

impl AsRef<VecM<TransactionEnvelope>> for DependentTxCluster {
    #[must_use]
    fn as_ref(&self) -> &VecM<TransactionEnvelope> {
        &self.0
    }
}

impl ReadXdr for DependentTxCluster {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let i = VecM::<TransactionEnvelope>::read_xdr(r)?;
            let v = DependentTxCluster(i);
            Ok(v)
        })
    }
}

impl WriteXdr for DependentTxCluster {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| self.0.write_xdr(w))
    }
}

impl Deref for DependentTxCluster {
    type Target = VecM<TransactionEnvelope>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<DependentTxCluster> for Vec<TransactionEnvelope> {
    #[must_use]
    fn from(x: DependentTxCluster) -> Self {
        x.0 .0
    }
}

impl TryFrom<Vec<TransactionEnvelope>> for DependentTxCluster {
    type Error = Error;
    fn try_from(x: Vec<TransactionEnvelope>) -> Result<Self, Error> {
        Ok(DependentTxCluster(x.try_into()?))
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<&Vec<TransactionEnvelope>> for DependentTxCluster {
    type Error = Error;
    fn try_from(x: &Vec<TransactionEnvelope>) -> Result<Self, Error> {
        Ok(DependentTxCluster(x.try_into()?))
    }
}

impl AsRef<Vec<TransactionEnvelope>> for DependentTxCluster {
    #[must_use]
    fn as_ref(&self) -> &Vec<TransactionEnvelope> {
        &self.0 .0
    }
}

impl AsRef<[TransactionEnvelope]> for DependentTxCluster {
    #[cfg(feature = "alloc")]
    #[must_use]
    fn as_ref(&self) -> &[TransactionEnvelope] {
        &self.0 .0
    }
    #[cfg(not(feature = "alloc"))]
    #[must_use]
    fn as_ref(&self) -> &[TransactionEnvelope] {
        self.0 .0
    }
}

/// DependentTxClusterRef is a borrowing equivalent of [`DependentTxCluster`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct DependentTxClusterRef<'a>(pub VecMRef<'a, TransactionEnvelopeRef<'a>>);

#[cfg(feature = "alloc")]
impl IntoOwned for DependentTxClusterRef<'_> {
    type Owned = DependentTxCluster;
    fn into_owned(self) -> DependentTxCluster {
        DependentTxCluster(self.0.into_owned())
    }
}

#[cfg(feature = "alloc")]
impl From<&DependentTxClusterRef<'_>> for DependentTxCluster {
    #[must_use]
    fn from(v: &DependentTxClusterRef<'_>) -> Self {
        v.into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<DependentTxClusterRef<'_>> for DependentTxCluster {
    #[must_use]
    fn from(v: DependentTxClusterRef<'_>) -> Self {
        v.into_owned()
    }
}

impl WriteXdr for DependentTxClusterRef<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| self.0.write_xdr(w))
    }
}

#[cfg(feature = "const")]
impl DependentTxClusterView<'_> {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_dependent_tx_cluster(self);
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
        w.write_type_dependent_tx_cluster(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`DependentTxCluster`], mirroring `<DependentTxCluster as WriteXdr>::write_xdr`.
    pub const fn write_type_dependent_tx_cluster(&mut self, v: &DependentTxClusterView<'_>) {
        self.write_type_vec_transaction_envelope(&v.0);
    }

    /// Serializes a variable-length array of [`DependentTxCluster`], mirroring `<VecM<DependentTxCluster, MAX> as WriteXdr>::write_xdr`.
    pub const fn write_type_vec_dependent_tx_cluster<const MAX: u32>(
        &mut self,
        v: &VecMView<'_, DependentTxClusterView<'_>, MAX>,
    ) {
        let s = v.as_slice();
        let len = s.len();
        self.write_len(len);
        let mut i = 0usize;
        while i < len {
            self.write_type_dependent_tx_cluster(&s[i]);
            i += 1;
        }
    }
}
