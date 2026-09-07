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
impl IntoOwned for TransactionSetView<'_> {
    type Owned = TransactionSet;
    fn into_owned(&self) -> TransactionSet {
        TransactionSet {
            previous_ledger_hash: IntoOwned::into_owned(&self.previous_ledger_hash),
            txs: IntoOwned::into_owned(&self.txs),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&TransactionSetView<'_>> for TransactionSet {
    #[must_use]
    fn from(v: &TransactionSetView<'_>) -> Self {
        IntoOwned::into_owned(v)
    }
}

#[cfg(feature = "alloc")]
impl From<TransactionSetView<'_>> for TransactionSet {
    #[must_use]
    fn from(v: TransactionSetView<'_>) -> Self {
        IntoOwned::into_owned(&v)
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
