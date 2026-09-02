#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// Transaction is an XDR Struct defined as:
///
/// ```text
/// struct Transaction
/// {
///     // account used to run the transaction
///     MuxedAccount sourceAccount;
///
///     // the fee the sourceAccount will pay
///     uint32 fee;
///
///     // sequence number to consume in the account
///     SequenceNumber seqNum;
///
///     // validity conditions
///     Preconditions cond;
///
///     Memo memo;
///
///     Operation operations<MAX_OPS_PER_TX>;
///
///     union switch (int v)
///     {
///     case 0:
///         void;
///     case 1:
///         SorobanTransactionData sorobanData;
///     }
///     ext;
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
pub struct Transaction {
    pub source_account: MuxedAccount,
    pub fee: u32,
    pub seq_num: SequenceNumber,
    pub cond: Preconditions,
    pub memo: Memo,
    pub operations: VecM<Operation, 100>,
    pub ext: TransactionExt,
}

impl ReadXdr for Transaction {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                source_account: MuxedAccount::read_xdr(r)?,
                fee: u32::read_xdr(r)?,
                seq_num: SequenceNumber::read_xdr(r)?,
                cond: Preconditions::read_xdr(r)?,
                memo: Memo::read_xdr(r)?,
                operations: VecM::<Operation, 100>::read_xdr(r)?,
                ext: TransactionExt::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for Transaction {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.source_account.write_xdr(w)?;
            self.fee.write_xdr(w)?;
            self.seq_num.write_xdr(w)?;
            self.cond.write_xdr(w)?;
            self.memo.write_xdr(w)?;
            self.operations.write_xdr(w)?;
            self.ext.write_xdr(w)?;
            Ok(())
        })
    }
}

/// TransactionView is a borrowing equivalent of [`Transaction`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TransactionView<'a> {
    pub source_account: MuxedAccount,
    pub fee: u32,
    pub seq_num: SequenceNumber,
    pub cond: PreconditionsView<'a>,
    pub memo: MemoView<'a>,
    pub operations: VecMView<'a, OperationView<'a>, 100>,
    pub ext: TransactionExtView<'a>,
}

#[cfg(feature = "alloc")]
impl From<&TransactionView<'_>> for Transaction {
    #[must_use]
    fn from(v: &TransactionView<'_>) -> Self {
        Self {
            source_account: v.source_account.clone(),
            fee: v.fee,
            seq_num: v.seq_num.clone(),
            cond: (&v.cond).into(),
            memo: (&v.memo).into(),
            operations: v.operations.to_vecm(),
            ext: (&v.ext).into(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<TransactionView<'_>> for Transaction {
    #[must_use]
    fn from(v: TransactionView<'_>) -> Self {
        Self::from(&v)
    }
}

impl WriteXdr for TransactionView<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.source_account.write_xdr(w)?;
            self.fee.write_xdr(w)?;
            self.seq_num.write_xdr(w)?;
            self.cond.write_xdr(w)?;
            self.memo.write_xdr(w)?;
            self.operations.write_xdr(w)?;
            self.ext.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "const")]
impl TransactionView<'_> {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_transaction(self);
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
        w.write_type_transaction(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`Transaction`], mirroring `<Transaction as WriteXdr>::write_xdr`.
    pub const fn write_type_transaction(&mut self, v: &TransactionView<'_>) {
        self.write_type_muxed_account(&v.source_account);
        self.write_u32(v.fee);
        self.write_type_sequence_number(&v.seq_num);
        self.write_type_preconditions(&v.cond);
        self.write_type_memo(&v.memo);
        self.write_type_vec_operation(&v.operations);
        self.write_type_transaction_ext(&v.ext);
    }
}
