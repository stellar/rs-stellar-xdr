#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// LedgerCloseMetaBatch is an XDR Struct defined as:
///
/// ```text
/// struct LedgerCloseMetaBatch
/// {
///     // starting ledger sequence number in the batch
///     uint32 startSequence;
///
///     // ending ledger sequence number in the batch
///     uint32 endSequence;
///
///     // Ledger close meta for each ledger within the batch
///     LedgerCloseMeta ledgerCloseMetas<>;
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
pub struct LedgerCloseMetaBatch {
    pub start_sequence: u32,
    pub end_sequence: u32,
    pub ledger_close_metas: VecM<LedgerCloseMeta>,
}

impl ReadXdr for LedgerCloseMetaBatch {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                start_sequence: u32::read_xdr(r)?,
                end_sequence: u32::read_xdr(r)?,
                ledger_close_metas: VecM::<LedgerCloseMeta>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for LedgerCloseMetaBatch {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.start_sequence.write_xdr(w)?;
            self.end_sequence.write_xdr(w)?;
            self.ledger_close_metas.write_xdr(w)?;
            Ok(())
        })
    }
}

/// LedgerCloseMetaBatchRef is a borrowing equivalent of [`LedgerCloseMetaBatch`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct LedgerCloseMetaBatchRef<'a> {
    pub start_sequence: u32,
    pub end_sequence: u32,
    pub ledger_close_metas: VecMRef<'a, LedgerCloseMetaRef<'a>>,
}

#[cfg(feature = "alloc")]
impl From<&LedgerCloseMetaBatchRef<'_>> for LedgerCloseMetaBatch {
    #[must_use]
    fn from(v: &LedgerCloseMetaBatchRef<'_>) -> Self {
        Self {
            start_sequence: v.start_sequence,
            end_sequence: v.end_sequence,
            ledger_close_metas: v.ledger_close_metas.to_vecm_from(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<LedgerCloseMetaBatchRef<'_>> for LedgerCloseMetaBatch {
    #[must_use]
    fn from(v: LedgerCloseMetaBatchRef<'_>) -> Self {
        Self::from(&v)
    }
}

impl WriteXdr for LedgerCloseMetaBatchRef<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.start_sequence.write_xdr(w)?;
            self.end_sequence.write_xdr(w)?;
            self.ledger_close_metas.write_xdr(w)?;
            Ok(())
        })
    }
}
