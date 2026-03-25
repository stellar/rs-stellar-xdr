#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
/// PreconditionsV2 is an XDR Struct defined as:
///
/// ```text
/// struct PreconditionsV2
/// {
///     TimeBounds* timeBounds;
///
///     // Transaction only valid for ledger numbers n such that
///     // minLedger <= n < maxLedger (if maxLedger == 0, then
///     // only minLedger is checked)
///     LedgerBounds* ledgerBounds;
///
///     // If NULL, only valid when sourceAccount's sequence number
///     // is seqNum - 1.  Otherwise, valid when sourceAccount's
///     // sequence number n satisfies minSeqNum <= n < tx.seqNum.
///     // Note that after execution the account's sequence number
///     // is always raised to tx.seqNum, and a transaction is not
///     // valid if tx.seqNum is too high to ensure replay protection.
///     SequenceNumber* minSeqNum;
///
///     // For the transaction to be valid, the current ledger time must
///     // be at least minSeqAge greater than sourceAccount's seqTime.
///     Duration minSeqAge;
///
///     // For the transaction to be valid, the current ledger number
///     // must be at least minSeqLedgerGap greater than sourceAccount's
///     // seqLedger.
///     uint32 minSeqLedgerGap;
///
///     // For the transaction to be valid, there must be a signature
///     // corresponding to every Signer in this array, even if the
///     // signature is not otherwise required by the sourceAccount or
///     // operations.
///     SignerKey extraSigners<2>;
/// };
/// ```
///
#[cfg_attr(feature = "alloc", derive(Default))]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_eval::cfg_eval]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    serde_with::serde_as,
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct PreconditionsV2 {
    pub time_bounds: Option<TimeBounds>,
    pub ledger_bounds: Option<LedgerBounds>,
    pub min_seq_num: Option<SequenceNumber>,
    pub min_seq_age: Duration,
    pub min_seq_ledger_gap: u32,
    pub extra_signers: VecM<SignerKey, 2>,
}

impl ReadXdr for PreconditionsV2 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                time_bounds: Option::<TimeBounds>::read_xdr(r)?,
                ledger_bounds: Option::<LedgerBounds>::read_xdr(r)?,
                min_seq_num: Option::<SequenceNumber>::read_xdr(r)?,
                min_seq_age: Duration::read_xdr(r)?,
                min_seq_ledger_gap: u32::read_xdr(r)?,
                extra_signers: VecM::<SignerKey, 2>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for PreconditionsV2 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.time_bounds.write_xdr(w)?;
            self.ledger_bounds.write_xdr(w)?;
            self.min_seq_num.write_xdr(w)?;
            self.min_seq_age.write_xdr(w)?;
            self.min_seq_ledger_gap.write_xdr(w)?;
            self.extra_signers.write_xdr(w)?;
            Ok(())
        })
    }
}
