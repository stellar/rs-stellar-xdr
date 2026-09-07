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

/// TransactionSetV1View is a borrowing equivalent of [`TransactionSetV1`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TransactionSetV1View<'a> {
    pub previous_ledger_hash: Hash,
    pub phases: VecMView<'a, TransactionPhaseView<'a>>,
}

#[cfg(feature = "alloc")]
impl IntoOwned for TransactionSetV1View<'_> {
    type Owned = TransactionSetV1;
    fn into_owned(&self) -> TransactionSetV1 {
        TransactionSetV1 {
            previous_ledger_hash: self.previous_ledger_hash.into_owned(),
            phases: self.phases.into_owned(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&TransactionSetV1View<'_>> for TransactionSetV1 {
    #[must_use]
    fn from(v: &TransactionSetV1View<'_>) -> Self {
        v.into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<TransactionSetV1View<'_>> for TransactionSetV1 {
    #[must_use]
    fn from(v: TransactionSetV1View<'_>) -> Self {
        v.into_owned()
    }
}

impl WriteXdr for TransactionSetV1View<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.previous_ledger_hash.write_xdr(w)?;
            self.phases.write_xdr(w)?;
            Ok(())
        })
    }
}
