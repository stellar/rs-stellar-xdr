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
impl From<&TxSetComponentTxsMaybeDiscountedFeeView<'_>> for TxSetComponentTxsMaybeDiscountedFee {
    #[must_use]
    fn from(v: &TxSetComponentTxsMaybeDiscountedFeeView<'_>) -> Self {
        Self {
            base_fee: v.base_fee,
            txs: v.txs.to_vecm(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<TxSetComponentTxsMaybeDiscountedFeeView<'_>> for TxSetComponentTxsMaybeDiscountedFee {
    #[must_use]
    fn from(v: TxSetComponentTxsMaybeDiscountedFeeView<'_>) -> Self {
        Self::from(&v)
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

#[cfg(feature = "const")]
impl TxSetComponentTxsMaybeDiscountedFeeView<'_> {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_tx_set_component_txs_maybe_discounted_fee(self);
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
        w.write_type_tx_set_component_txs_maybe_discounted_fee(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`TxSetComponentTxsMaybeDiscountedFee`], mirroring `<TxSetComponentTxsMaybeDiscountedFee as WriteXdr>::write_xdr`.
    pub const fn write_type_tx_set_component_txs_maybe_discounted_fee(
        &mut self,
        v: &TxSetComponentTxsMaybeDiscountedFeeView<'_>,
    ) {
        self.write_option_i64(&v.base_fee);
        self.write_type_vec_transaction_envelope(&v.txs);
    }
}
