#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// TransactionSetV1 is an XDR Struct defined as:
///
/// ```text
/// struct TransactionSetV1
/// {
///     Hash previousLedgerHash;
///     TransactionPhase phases<>;
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
pub struct TransactionSetV1 {
    pub previous_ledger_hash: Hash,
    pub phases: VecM<TransactionPhase>,
}

impl ReadXdr for TransactionSetV1 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                previous_ledger_hash: Hash::read_xdr(r)?,
                phases: VecM::<TransactionPhase>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for TransactionSetV1 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.previous_ledger_hash.write_xdr(w)?;
            self.phases.write_xdr(w)?;
            Ok(())
        })
    }
}

/// TransactionSetV1Ref is a borrowing equivalent of [`TransactionSetV1`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TransactionSetV1Ref<'a> {
    pub previous_ledger_hash: Hash,
    pub phases: VecMRef<'a, TransactionPhaseRef<'a>>,
}

#[cfg(feature = "alloc")]
impl From<&TransactionSetV1Ref<'_>> for TransactionSetV1 {
    #[must_use]
    fn from(v: &TransactionSetV1Ref<'_>) -> Self {
        Self {
            previous_ledger_hash: v.previous_ledger_hash.clone(),
            phases: v.phases.to_vecm_from(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<TransactionSetV1Ref<'_>> for TransactionSetV1 {
    #[must_use]
    fn from(v: TransactionSetV1Ref<'_>) -> Self {
        Self::from(&v)
    }
}
