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

/// TransactionResultMetaV1Const is a borrowing equivalent of [`TransactionResultMetaV1`] over `'static`
/// data, for const XDR encoding.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TransactionResultMetaV1Const {
    pub ext: ExtensionPoint,
    pub result: TransactionResultPairConst,
    pub fee_processing: LedgerEntryChangesConst,
    pub tx_apply_processing: TransactionMetaConst,
    pub post_tx_apply_fee_processing: LedgerEntryChangesConst,
}

#[cfg(feature = "const")]
impl TransactionResultMetaV1Const {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_transaction_result_meta_v1(self);
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
        w.write_type_transaction_result_meta_v1(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`TransactionResultMetaV1`], mirroring `<TransactionResultMetaV1 as WriteXdr>::write_xdr`.
    pub const fn write_type_transaction_result_meta_v1(
        &mut self,
        v: &TransactionResultMetaV1Const,
    ) {
        self.write_type_extension_point(&v.ext);
        self.write_type_transaction_result_pair(&v.result);
        self.write_type_ledger_entry_changes(&v.fee_processing);
        self.write_type_transaction_meta(&v.tx_apply_processing);
        self.write_type_ledger_entry_changes(&v.post_tx_apply_fee_processing);
    }

    /// Serializes a variable-length array of [`TransactionResultMetaV1`], mirroring `<VecM<TransactionResultMetaV1, MAX> as WriteXdr>::write_xdr`.
    pub const fn write_type_vec_transaction_result_meta_v1<const MAX: u32>(
        &mut self,
        v: &VecMConst<TransactionResultMetaV1Const, MAX>,
    ) {
        let s = v.as_slice();
        let len = s.len();
        self.write_len(len);
        let mut i = 0usize;
        while i < len {
            self.write_type_transaction_result_meta_v1(&s[i]);
            i += 1;
        }
    }
}
