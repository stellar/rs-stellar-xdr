#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// TransactionResultMetaV1 is an XDR Struct defined as:
///
/// ```text
/// struct TransactionResultMetaV1
/// {
///     ExtensionPoint ext;
///
///     TransactionResultPair result;
///     LedgerEntryChanges feeProcessing;
///     TransactionMeta txApplyProcessing;
///
///     LedgerEntryChanges postTxApplyFeeProcessing;
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
pub struct TransactionResultMetaV1 {
    pub ext: ExtensionPoint,
    pub result: TransactionResultPair,
    pub fee_processing: LedgerEntryChanges,
    pub tx_apply_processing: TransactionMeta,
    pub post_tx_apply_fee_processing: LedgerEntryChanges,
}

impl ReadXdr for TransactionResultMetaV1 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                ext: ExtensionPoint::read_xdr(r)?,
                result: TransactionResultPair::read_xdr(r)?,
                fee_processing: LedgerEntryChanges::read_xdr(r)?,
                tx_apply_processing: TransactionMeta::read_xdr(r)?,
                post_tx_apply_fee_processing: LedgerEntryChanges::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for TransactionResultMetaV1 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.ext.write_xdr(w)?;
            self.result.write_xdr(w)?;
            self.fee_processing.write_xdr(w)?;
            self.tx_apply_processing.write_xdr(w)?;
            self.post_tx_apply_fee_processing.write_xdr(w)?;
            Ok(())
        })
    }
}

/// TransactionResultMetaV1View is a borrowing equivalent of [`TransactionResultMetaV1`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TransactionResultMetaV1View<'a> {
    pub ext: ExtensionPoint,
    pub result: TransactionResultPairView<'a>,
    pub fee_processing: LedgerEntryChangesView<'a>,
    pub tx_apply_processing: TransactionMetaView<'a>,
    pub post_tx_apply_fee_processing: LedgerEntryChangesView<'a>,
}

#[cfg(feature = "alloc")]
impl IntoOwned for TransactionResultMetaV1View<'_> {
    type Owned = TransactionResultMetaV1;
    fn into_owned(&self) -> TransactionResultMetaV1 {
        TransactionResultMetaV1 {
            ext: IntoOwned::into_owned(&self.ext),
            result: IntoOwned::into_owned(&self.result),
            fee_processing: IntoOwned::into_owned(&self.fee_processing),
            tx_apply_processing: IntoOwned::into_owned(&self.tx_apply_processing),
            post_tx_apply_fee_processing: IntoOwned::into_owned(&self.post_tx_apply_fee_processing),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&TransactionResultMetaV1View<'_>> for TransactionResultMetaV1 {
    #[must_use]
    fn from(v: &TransactionResultMetaV1View<'_>) -> Self {
        IntoOwned::into_owned(v)
    }
}

#[cfg(feature = "alloc")]
impl From<TransactionResultMetaV1View<'_>> for TransactionResultMetaV1 {
    #[must_use]
    fn from(v: TransactionResultMetaV1View<'_>) -> Self {
        IntoOwned::into_owned(&v)
    }
}

impl WriteXdr for TransactionResultMetaV1View<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.ext.write_xdr(w)?;
            self.result.write_xdr(w)?;
            self.fee_processing.write_xdr(w)?;
            self.tx_apply_processing.write_xdr(w)?;
            self.post_tx_apply_fee_processing.write_xdr(w)?;
            Ok(())
        })
    }
}
