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
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval)]
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

/// PreconditionsV2View is a borrowing equivalent of [`PreconditionsV2`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct PreconditionsV2View<'a> {
    pub time_bounds: Option<TimeBounds>,
    pub ledger_bounds: Option<LedgerBounds>,
    pub min_seq_num: Option<SequenceNumber>,
    pub min_seq_age: Duration,
    pub min_seq_ledger_gap: u32,
    pub extra_signers: VecMView<'a, SignerKeyView<'a>, 2>,
}

#[cfg(feature = "alloc")]
impl From<&PreconditionsV2View<'_>> for PreconditionsV2 {
    #[must_use]
    fn from(v: &PreconditionsV2View<'_>) -> Self {
        Self {
            time_bounds: v.time_bounds.clone(),
            ledger_bounds: v.ledger_bounds.clone(),
            min_seq_num: v.min_seq_num.clone(),
            min_seq_age: v.min_seq_age.clone(),
            min_seq_ledger_gap: v.min_seq_ledger_gap,
            extra_signers: v.extra_signers.to_vecm(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<PreconditionsV2View<'_>> for PreconditionsV2 {
    #[must_use]
    fn from(v: PreconditionsV2View<'_>) -> Self {
        Self::from(&v)
    }
}

impl WriteXdr for PreconditionsV2View<'_> {
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

#[cfg(feature = "const")]
impl PreconditionsV2View<'_> {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_preconditions_v2(self);
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
        w.write_type_preconditions_v2(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`PreconditionsV2`], mirroring `<PreconditionsV2 as WriteXdr>::write_xdr`.
    pub const fn write_type_preconditions_v2(&mut self, v: &PreconditionsV2View<'_>) {
        self.write_type_option_time_bounds(&v.time_bounds);
        self.write_type_option_ledger_bounds(&v.ledger_bounds);
        self.write_type_option_sequence_number(&v.min_seq_num);
        self.write_type_duration(&v.min_seq_age);
        self.write_u32(v.min_seq_ledger_gap);
        self.write_type_vec_signer_key(&v.extra_signers);
    }
}
