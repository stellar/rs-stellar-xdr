#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// TxAdvertVector is an XDR Typedef defined as:
///
/// ```text
/// typedef Hash TxAdvertVector<TX_ADVERT_VECTOR_MAX_SIZE>;
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
pub struct TxAdvertVector(pub VecM<Hash, TX_ADVERT_VECTOR_MAX_SIZE>);

impl From<TxAdvertVector> for VecM<Hash, TX_ADVERT_VECTOR_MAX_SIZE> {
    #[must_use]
    fn from(x: TxAdvertVector) -> Self {
        x.0
    }
}

impl From<VecM<Hash, TX_ADVERT_VECTOR_MAX_SIZE>> for TxAdvertVector {
    #[must_use]
    fn from(x: VecM<Hash, TX_ADVERT_VECTOR_MAX_SIZE>) -> Self {
        TxAdvertVector(x)
    }
}

impl AsRef<VecM<Hash, TX_ADVERT_VECTOR_MAX_SIZE>> for TxAdvertVector {
    #[must_use]
    fn as_ref(&self) -> &VecM<Hash, TX_ADVERT_VECTOR_MAX_SIZE> {
        &self.0
    }
}

impl ReadXdr for TxAdvertVector {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let i = VecM::<Hash, TX_ADVERT_VECTOR_MAX_SIZE>::read_xdr(r)?;
            let v = TxAdvertVector(i);
            Ok(v)
        })
    }
}

impl WriteXdr for TxAdvertVector {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| self.0.write_xdr(w))
    }
}

impl Deref for TxAdvertVector {
    type Target = VecM<Hash, TX_ADVERT_VECTOR_MAX_SIZE>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<TxAdvertVector> for Vec<Hash> {
    #[must_use]
    fn from(x: TxAdvertVector) -> Self {
        x.0 .0
    }
}

impl TryFrom<Vec<Hash>> for TxAdvertVector {
    type Error = Error;
    fn try_from(x: Vec<Hash>) -> Result<Self, Error> {
        Ok(TxAdvertVector(x.try_into()?))
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<&Vec<Hash>> for TxAdvertVector {
    type Error = Error;
    fn try_from(x: &Vec<Hash>) -> Result<Self, Error> {
        Ok(TxAdvertVector(x.try_into()?))
    }
}

impl AsRef<Vec<Hash>> for TxAdvertVector {
    #[must_use]
    fn as_ref(&self) -> &Vec<Hash> {
        &self.0 .0
    }
}

impl AsRef<[Hash]> for TxAdvertVector {
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

/// TxAdvertVectorRef is a borrowing equivalent of [`TxAdvertVector`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TxAdvertVectorRef<'a>(pub VecMRef<'a, Hash, TX_ADVERT_VECTOR_MAX_SIZE>);

#[cfg(feature = "alloc")]
impl IntoOwned for TxAdvertVectorRef<'_> {
    type Owned = TxAdvertVector;
    fn into_owned(self) -> TxAdvertVector {
        TxAdvertVector(self.0.into_owned())
    }
}

#[cfg(feature = "alloc")]
impl From<&TxAdvertVectorRef<'_>> for TxAdvertVector {
    #[must_use]
    fn from(v: &TxAdvertVectorRef<'_>) -> Self {
        v.into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<TxAdvertVectorRef<'_>> for TxAdvertVector {
    #[must_use]
    fn from(v: TxAdvertVectorRef<'_>) -> Self {
        v.into_owned()
    }
}

impl WriteXdr for TxAdvertVectorRef<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| self.0.write_xdr(w))
    }
}

#[cfg(feature = "const")]
impl TxAdvertVectorRef<'_> {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_tx_advert_vector(self);
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
        w.write_type_tx_advert_vector(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`TxAdvertVector`], mirroring `<TxAdvertVector as WriteXdr>::write_xdr`.
    pub const fn write_type_tx_advert_vector(&mut self, v: &TxAdvertVectorRef<'_>) {
        self.write_type_vec_hash(&v.0);
    }
}
