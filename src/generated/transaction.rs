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

impl Transaction {
    /// Serialize this value as XDR into a [`ConstWriter`] using only const
    /// operations. This is the const implementation underlying `to_xdr`.
    #[cfg(feature = "std")]
    pub const fn const_write_xdr(&self, w: &mut ConstWriter) {
        w.enter_depth();
        self.source_account.const_write_xdr(w);
        w.write_u32(self.fee);
        self.seq_num.const_write_xdr(w);
        self.cond.const_write_xdr(w);
        self.memo.const_write_xdr(w);
        {
            w.enter_depth();
            let __s0 = self.operations.0.as_slice();
            let __len0 = __s0.len();
            w.write_length_prefix(__len0);
            let mut __i0 = 0usize;
            while __i0 < __len0 {
                __s0[__i0].const_write_xdr(w);
                __i0 += 1;
            }
            w.leave_depth();
        }
        self.ext.const_write_xdr(w);
        w.leave_depth();
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

    #[cfg(feature = "std")]
    fn to_xdr(&self, limits: Limits) -> Result<Vec<u8>, Error> {
        to_xdr_via_const(self, &limits, Self::const_write_xdr)
    }
}
