#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// TransactionResultMeta is an XDR Struct defined as:
///
/// ```text
/// struct TransactionResultMeta
/// {
///     TransactionResultPair result;
///     LedgerEntryChanges feeProcessing;
///     TransactionMeta txApplyProcessing;
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
pub struct TransactionResultMeta {
    pub result: TransactionResultPair,
    pub fee_processing: LedgerEntryChanges,
    pub tx_apply_processing: TransactionMeta,
}

impl ReadXdr for TransactionResultMeta {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                result: TransactionResultPair::read_xdr(r)?,
                fee_processing: LedgerEntryChanges::read_xdr(r)?,
                tx_apply_processing: TransactionMeta::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for TransactionResultMeta {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.result.write_xdr(w)?;
            self.fee_processing.write_xdr(w)?;
            self.tx_apply_processing.write_xdr(w)?;
            Ok(())
        })
    }
}

/// TransactionResultMetaView is a borrowing equivalent of [`TransactionResultMeta`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TransactionResultMetaView<'a> {
    pub result: TransactionResultPairView<'a>,
    pub fee_processing: LedgerEntryChangesView<'a>,
    pub tx_apply_processing: TransactionMetaView<'a>,
}

#[cfg(feature = "alloc")]
impl From<&TransactionResultMetaView<'_>> for TransactionResultMeta {
    #[must_use]
    fn from(v: &TransactionResultMetaView<'_>) -> Self {
        Self {
            result: (&v.result).into(),
            fee_processing: (&v.fee_processing).into(),
            tx_apply_processing: (&v.tx_apply_processing).into(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<TransactionResultMetaView<'_>> for TransactionResultMeta {
    #[must_use]
    fn from(v: TransactionResultMetaView<'_>) -> Self {
        Self::from(&v)
    }
}

impl WriteXdr for TransactionResultMetaView<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.result.write_xdr(w)?;
            self.fee_processing.write_xdr(w)?;
            self.tx_apply_processing.write_xdr(w)?;
            Ok(())
        })
    }
}
