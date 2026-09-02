#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// TransactionSet is an XDR Struct defined as:
///
/// ```text
/// struct TransactionSet
/// {
///     Hash previousLedgerHash;
///     TransactionEnvelope txs<>;
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
pub struct TransactionSet {
    pub previous_ledger_hash: Hash,
    pub txs: VecM<TransactionEnvelope>,
}

impl ReadXdr for TransactionSet {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                previous_ledger_hash: Hash::read_xdr(r)?,
                txs: VecM::<TransactionEnvelope>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for TransactionSet {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.previous_ledger_hash.write_xdr(w)?;
            self.txs.write_xdr(w)?;
            Ok(())
        })
    }
}

/// TransactionSetView is a borrowing equivalent of [`TransactionSet`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TransactionSetView<'a> {
    pub previous_ledger_hash: Hash,
    pub txs: VecMView<'a, TransactionEnvelopeView<'a>>,
}

#[cfg(feature = "alloc")]
impl From<&TransactionSetView<'_>> for TransactionSet {
    #[must_use]
    fn from(v: &TransactionSetView<'_>) -> Self {
        Self {
            previous_ledger_hash: v.previous_ledger_hash.clone(),
            txs: v.txs.to_vecm(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<TransactionSetView<'_>> for TransactionSet {
    #[must_use]
    fn from(v: TransactionSetView<'_>) -> Self {
        Self::from(&v)
    }
}

impl WriteXdr for TransactionSetView<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.previous_ledger_hash.write_xdr(w)?;
            self.txs.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "const")]
impl TransactionSetView<'_> {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_transaction_set(self);
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
        w.write_type_transaction_set(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`TransactionSet`], mirroring `<TransactionSet as WriteXdr>::write_xdr`.
    pub const fn write_type_transaction_set(&mut self, v: &TransactionSetView<'_>) {
        self.write_type_hash(&v.previous_ledger_hash);
        self.write_type_vec_transaction_envelope(&v.txs);
    }
}
