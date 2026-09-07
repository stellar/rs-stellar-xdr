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
    pub operations: VecM<Operation, MAX_OPS_PER_TX>,
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
                operations: VecM::<Operation, MAX_OPS_PER_TX>::read_xdr(r)?,
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

/// TransactionRef is a borrowing equivalent of [`Transaction`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TransactionRef<'a> {
    pub source_account: MuxedAccount,
    pub fee: u32,
    pub seq_num: SequenceNumber,
    pub cond: PreconditionsRef<'a>,
    pub memo: MemoRef<'a>,
    pub operations: VecMRef<'a, OperationRef<'a>, MAX_OPS_PER_TX>,
    pub ext: TransactionExtRef<'a>,
}

#[cfg(feature = "alloc")]
impl IntoOwned for TransactionRef<'_> {
    type Owned = Transaction;
    fn into_owned(self) -> Transaction {
        Transaction {
            source_account: self.source_account.into_owned(),
            fee: self.fee.into_owned(),
            seq_num: self.seq_num.into_owned(),
            cond: self.cond.into_owned(),
            memo: self.memo.into_owned(),
            operations: self.operations.into_owned(),
            ext: self.ext.into_owned(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&TransactionRef<'_>> for Transaction {
    #[must_use]
    fn from(v: &TransactionRef<'_>) -> Self {
        v.into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<TransactionRef<'_>> for Transaction {
    #[must_use]
    fn from(v: TransactionRef<'_>) -> Self {
        v.into_owned()
    }
}

impl WriteXdr for TransactionRef<'_> {
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
