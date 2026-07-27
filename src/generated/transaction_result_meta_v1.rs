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

/// TransactionResultMetaV1Ref is a borrowing equivalent of [`TransactionResultMetaV1`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TransactionResultMetaV1Ref<'a> {
    pub ext: ExtensionPoint,
    pub result: TransactionResultPairRef<'a>,
    pub fee_processing: LedgerEntryChangesRef<'a>,
    pub tx_apply_processing: TransactionMetaRef<'a>,
    pub post_tx_apply_fee_processing: LedgerEntryChangesRef<'a>,
}

#[cfg(feature = "alloc")]
impl From<&TransactionResultMetaV1Ref<'_>> for TransactionResultMetaV1 {
    #[must_use]
    fn from(v: &TransactionResultMetaV1Ref<'_>) -> Self {
        Self {
            ext: v.ext.clone(),
            result: (&v.result).into(),
            fee_processing: (&v.fee_processing).into(),
            tx_apply_processing: (&v.tx_apply_processing).into(),
            post_tx_apply_fee_processing: (&v.post_tx_apply_fee_processing).into(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<TransactionResultMetaV1Ref<'_>> for TransactionResultMetaV1 {
    #[must_use]
    fn from(v: TransactionResultMetaV1Ref<'_>) -> Self {
        Self::from(&v)
    }
}
