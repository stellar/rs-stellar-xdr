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

/// TxAdvertVectorView is a borrowing equivalent of [`TxAdvertVector`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TxAdvertVectorView<'a>(pub VecMView<'a, Hash, TX_ADVERT_VECTOR_MAX_SIZE>);

#[cfg(feature = "alloc")]
impl IntoOwned for TxAdvertVectorView<'_> {
    type Owned = TxAdvertVector;
    fn into_owned(self) -> TxAdvertVector {
        TxAdvertVector(self.0.into_owned())
    }
}

#[cfg(feature = "alloc")]
impl From<&TxAdvertVectorView<'_>> for TxAdvertVector {
    #[must_use]
    fn from(v: &TxAdvertVectorView<'_>) -> Self {
        v.into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<TxAdvertVectorView<'_>> for TxAdvertVector {
    #[must_use]
    fn from(v: TxAdvertVectorView<'_>) -> Self {
        v.into_owned()
    }
}

impl WriteXdr for TxAdvertVectorView<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| self.0.write_xdr(w))
    }
}
