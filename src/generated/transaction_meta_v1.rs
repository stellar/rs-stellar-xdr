#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// TransactionMetaV1 is an XDR Struct defined as:
///
/// ```text
/// struct TransactionMetaV1
/// {
///     LedgerEntryChanges txChanges; // tx level changes if any
///     OperationMeta operations<>;   // meta for each operation
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
pub struct TransactionMetaV1 {
    pub tx_changes: LedgerEntryChanges,
    pub operations: VecM<OperationMeta>,
}

impl ReadXdr for TransactionMetaV1 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                tx_changes: LedgerEntryChanges::read_xdr(r)?,
                operations: VecM::<OperationMeta>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for TransactionMetaV1 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.tx_changes.write_xdr(w)?;
            self.operations.write_xdr(w)?;
            Ok(())
        })
    }
}

/// TransactionMetaV1Ref is a borrowing equivalent of [`TransactionMetaV1`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TransactionMetaV1Ref<'a> {
    pub tx_changes: LedgerEntryChangesRef<'a>,
    pub operations: VecMRef<'a, OperationMetaRef<'a>>,
}

#[cfg(feature = "alloc")]
impl IntoOwned for TransactionMetaV1Ref<'_> {
    type Owned = TransactionMetaV1;
    fn into_owned(self) -> TransactionMetaV1 {
        TransactionMetaV1 {
            tx_changes: self.tx_changes.into_owned(),
            operations: self.operations.into_owned(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&TransactionMetaV1Ref<'_>> for TransactionMetaV1 {
    #[must_use]
    fn from(v: &TransactionMetaV1Ref<'_>) -> Self {
        v.into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<TransactionMetaV1Ref<'_>> for TransactionMetaV1 {
    #[must_use]
    fn from(v: TransactionMetaV1Ref<'_>) -> Self {
        v.into_owned()
    }
}

impl WriteXdr for TransactionMetaV1Ref<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.tx_changes.write_xdr(w)?;
            self.operations.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "const")]
impl TransactionMetaV1View<'_> {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_transaction_meta_v1(self);
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
        w.write_type_transaction_meta_v1(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`TransactionMetaV1`], mirroring `<TransactionMetaV1 as WriteXdr>::write_xdr`.
    pub const fn write_type_transaction_meta_v1(&mut self, v: &TransactionMetaV1View<'_>) {
        self.write_type_ledger_entry_changes(&v.tx_changes);
        self.write_type_vec_operation_meta(&v.operations);
    }
}
