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

/// LedgerCloseMetaBatchView is a borrowing equivalent of [`LedgerCloseMetaBatch`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct LedgerCloseMetaBatchView<'a> {
    pub start_sequence: u32,
    pub end_sequence: u32,
    pub ledger_close_metas: VecMView<'a, LedgerCloseMetaView<'a>>,
}

#[cfg(feature = "alloc")]
impl From<&LedgerCloseMetaBatchView<'_>> for LedgerCloseMetaBatch {
    #[must_use]
    fn from(v: &LedgerCloseMetaBatchView<'_>) -> Self {
        Self {
            start_sequence: v.start_sequence,
            end_sequence: v.end_sequence,
            ledger_close_metas: v.ledger_close_metas.to_vecm(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<LedgerCloseMetaBatchView<'_>> for LedgerCloseMetaBatch {
    #[must_use]
    fn from(v: LedgerCloseMetaBatchView<'_>) -> Self {
        Self::from(&v)
    }
}

impl WriteXdr for LedgerCloseMetaBatchView<'_> {
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

#[cfg(feature = "const")]
impl LedgerCloseMetaBatchView<'_> {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_ledger_close_meta_batch(self);
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
        w.write_type_ledger_close_meta_batch(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`LedgerCloseMetaBatch`], mirroring `<LedgerCloseMetaBatch as WriteXdr>::write_xdr`.
    pub const fn write_type_ledger_close_meta_batch(&mut self, v: &LedgerCloseMetaBatchView<'_>) {
        self.write_u32(v.start_sequence);
        self.write_u32(v.end_sequence);
        self.write_type_vec_ledger_close_meta(&v.ledger_close_metas);
    }
}
