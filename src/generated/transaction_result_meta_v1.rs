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
impl From<&TransactionResultMetaV1View<'_>> for TransactionResultMetaV1 {
    #[must_use]
    fn from(v: &TransactionResultMetaV1View<'_>) -> Self {
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
impl From<TransactionResultMetaV1View<'_>> for TransactionResultMetaV1 {
    #[must_use]
    fn from(v: TransactionResultMetaV1View<'_>) -> Self {
        Self::from(&v)
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
#[cfg(feature = "const")]
impl TransactionResultMetaV1View<'_> {
    /// Serialize this value as XDR into a [`ConstWriter`] using only const
    /// operations. This is the const counterpart to the owned type's
    /// [`WriteXdr::write_xdr`].
    pub const fn const_write_xdr(&self, w: &mut ConstWriter) {
        w.enter_depth();
        self.ext.const_write_xdr(w);
        self.result.const_write_xdr(w);
        self.fee_processing.const_write_xdr(w);
        self.tx_apply_processing.const_write_xdr(w);
        self.post_tx_apply_fee_processing.const_write_xdr(w);
        w.leave_depth();
    }
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[cfg(feature = "const")]
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let limits = Limits {
            depth: u32::MAX,
            len: usize::MAX,
        };
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty, &limits);
        self.const_write_xdr(&mut w);
        w.position()
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
    #[cfg(feature = "const")]
    #[must_use]
    pub const fn const_to_xdr<const N: usize>(&self) -> [u8; N] {
        let limits = Limits {
            depth: u32::MAX,
            len: usize::MAX,
        };
        let mut buf = [0u8; N];
        let mut w = ConstWriter::new(&mut buf, &limits);
        self.const_write_xdr(&mut w);
        assert!(
            w.position() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}
