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
pub struct TxDemandVector(pub VecM<Hash, 1000>);

impl From<TxDemandVector> for VecM<Hash, 1000> {
    #[must_use]
    fn from(x: TxDemandVector) -> Self {
        x.0
    }
}

impl From<VecM<Hash, 1000>> for TxDemandVector {
    #[must_use]
    fn from(x: VecM<Hash, 1000>) -> Self {
        TxDemandVector(x)
    }
}

impl AsRef<VecM<Hash, 1000>> for TxDemandVector {
    #[must_use]
    fn as_ref(&self) -> &VecM<Hash, 1000> {
        &self.0
    }
}

impl ReadXdr for TxDemandVector {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let i = VecM::<Hash, 1000>::read_xdr(r)?;
            let v = TxDemandVector(i);
            Ok(v)
        })
    }
}

impl TxDemandVector {
    /// Serialize this value as XDR into a [`ConstWriter`] using only const
    /// operations. This is the const implementation underlying `to_xdr`.
    #[cfg(feature = "std")]
    pub const fn const_to_xdr(&self, w: &mut ConstWriter) {
        w.enter_depth();
        {
            w.enter_depth();
            let __s0 = self.0 .0.as_slice();
            let __len0 = __s0.len();
            w.write_length_prefix(__len0);
            let mut __i0 = 0usize;
            while __i0 < __len0 {
                __s0[__i0].const_to_xdr(w);
                __i0 += 1;
            }
            w.leave_depth();
        }
        w.leave_depth();
    }
}

impl WriteXdr for TxDemandVector {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| self.0.write_xdr(w))
    }

    #[cfg(feature = "std")]
    fn to_xdr(&self, limits: Limits) -> Result<Vec<u8>, Error> {
        to_xdr_via_const(self, &limits, Self::const_to_xdr)
    }
}

impl Deref for TxDemandVector {
    type Target = VecM<Hash, 1000>;
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
