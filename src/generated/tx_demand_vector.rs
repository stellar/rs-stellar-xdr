#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// TxDemandVector is an XDR Typedef defined as:
///
/// ```text
/// typedef Hash TxDemandVector<TX_DEMAND_VECTOR_MAX_SIZE>;
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
pub struct TxDemandVector(pub VecM<Hash, TX_DEMAND_VECTOR_MAX_SIZE>);

impl From<TxDemandVector> for VecM<Hash, TX_DEMAND_VECTOR_MAX_SIZE> {
    #[must_use]
    fn from(x: TxDemandVector) -> Self {
        x.0
    }
}

impl From<VecM<Hash, TX_DEMAND_VECTOR_MAX_SIZE>> for TxDemandVector {
    #[must_use]
    fn from(x: VecM<Hash, TX_DEMAND_VECTOR_MAX_SIZE>) -> Self {
        TxDemandVector(x)
    }
}

impl AsRef<VecM<Hash, TX_DEMAND_VECTOR_MAX_SIZE>> for TxDemandVector {
    #[must_use]
    fn as_ref(&self) -> &VecM<Hash, TX_DEMAND_VECTOR_MAX_SIZE> {
        &self.0
    }
}

impl ReadXdr for TxDemandVector {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let i = VecM::<Hash, TX_DEMAND_VECTOR_MAX_SIZE>::read_xdr(r)?;
            let v = TxDemandVector(i);
            Ok(v)
        })
    }
}

impl WriteXdr for TxDemandVector {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| self.0.write_xdr(w))
    }
}

impl Deref for TxDemandVector {
    type Target = VecM<Hash, TX_DEMAND_VECTOR_MAX_SIZE>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<TxDemandVector> for Vec<Hash> {
    #[must_use]
    fn from(x: TxDemandVector) -> Self {
        x.0 .0
    }
}

impl TryFrom<Vec<Hash>> for TxDemandVector {
    type Error = Error;
    fn try_from(x: Vec<Hash>) -> Result<Self, Error> {
        Ok(TxDemandVector(x.try_into()?))
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<&Vec<Hash>> for TxDemandVector {
    type Error = Error;
    fn try_from(x: &Vec<Hash>) -> Result<Self, Error> {
        Ok(TxDemandVector(x.try_into()?))
    }
}

impl AsRef<Vec<Hash>> for TxDemandVector {
    #[must_use]
    fn as_ref(&self) -> &Vec<Hash> {
        &self.0 .0
    }
}

impl AsRef<[Hash]> for TxDemandVector {
    #[cfg(feature = "alloc")]
    #[must_use]
    fn as_ref(&self) -> &[Hash] {
        &self.0 .0
    }
    #[cfg(not(feature = "alloc"))]
    #[must_use]
    fn as_ref(&self) -> &[Hash] {
        self.0 .0
    }
}

/// TxDemandVectorRef is a borrowing equivalent of [`TxDemandVector`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TxDemandVectorRef<'a>(pub VecMRef<'a, Hash, TX_DEMAND_VECTOR_MAX_SIZE>);

#[cfg(feature = "alloc")]
impl IntoOwned for TxDemandVectorRef<'_> {
    type Owned = TxDemandVector;
    fn into_owned(self) -> TxDemandVector {
        TxDemandVector(self.0.into_owned())
    }
}

#[cfg(feature = "alloc")]
impl From<&TxDemandVectorRef<'_>> for TxDemandVector {
    #[must_use]
    fn from(v: &TxDemandVectorRef<'_>) -> Self {
        v.into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<TxDemandVectorRef<'_>> for TxDemandVector {
    #[must_use]
    fn from(v: TxDemandVectorRef<'_>) -> Self {
        v.into_owned()
    }
}

impl WriteXdr for TxDemandVectorRef<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| self.0.write_xdr(w))
    }
}

#[cfg(feature = "const")]
impl TxDemandVectorView<'_> {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_tx_demand_vector(self);
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
        w.write_type_tx_demand_vector(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`TxDemandVector`], mirroring `<TxDemandVector as WriteXdr>::write_xdr`.
    pub const fn write_type_tx_demand_vector(&mut self, v: &TxDemandVectorView<'_>) {
        self.write_type_vec_hash(&v.0);
    }
}
