#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// TxSetComponentTxsMaybeDiscountedFee is an XDR NestedStruct defined as:
///
/// ```text
/// struct
///   {
///     int64* baseFee;
///     TransactionEnvelope txs<>;
///   }
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
pub struct TxSetComponentTxsMaybeDiscountedFee {
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "Option<NumberOrString>")
    )]
    pub base_fee: Option<i64>,
    pub txs: VecM<TransactionEnvelope>,
}

impl ReadXdr for TxSetComponentTxsMaybeDiscountedFee {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                base_fee: Option::<i64>::read_xdr(r)?,
                txs: VecM::<TransactionEnvelope>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for TxSetComponentTxsMaybeDiscountedFee {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.base_fee.write_xdr(w)?;
            self.txs.write_xdr(w)?;
            Ok(())
        })
    }
}

/// TxSetComponentTxsMaybeDiscountedFeeView is a borrowing equivalent of [`TxSetComponentTxsMaybeDiscountedFee`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TxSetComponentTxsMaybeDiscountedFeeView<'a> {
    pub base_fee: Option<i64>,
    pub txs: VecMView<'a, TransactionEnvelopeView<'a>>,
}

#[cfg(feature = "alloc")]
impl IntoOwned for TxSetComponentTxsMaybeDiscountedFeeView<'_> {
    type Owned = TxSetComponentTxsMaybeDiscountedFee;
    fn into_owned(&self) -> TxSetComponentTxsMaybeDiscountedFee {
        TxSetComponentTxsMaybeDiscountedFee {
            base_fee: self.base_fee.into_owned(),
            txs: self.txs.into_owned(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&TxSetComponentTxsMaybeDiscountedFeeView<'_>> for TxSetComponentTxsMaybeDiscountedFee {
    #[must_use]
    fn from(v: &TxSetComponentTxsMaybeDiscountedFeeView<'_>) -> Self {
        v.into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<TxSetComponentTxsMaybeDiscountedFeeView<'_>> for TxSetComponentTxsMaybeDiscountedFee {
    #[must_use]
    fn from(v: TxSetComponentTxsMaybeDiscountedFeeView<'_>) -> Self {
        v.into_owned()
    }
}

impl WriteXdr for TxSetComponentTxsMaybeDiscountedFeeView<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.base_fee.write_xdr(w)?;
            self.txs.write_xdr(w)?;
            Ok(())
        })
    }
}
