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
impl IntoOwned for TransactionSetV1Ref<'_> {
    type Owned = TransactionSetV1;
    fn into_owned(self) -> TransactionSetV1 {
        TransactionSetV1 {
            previous_ledger_hash: self.previous_ledger_hash.into_owned(),
            phases: self.phases.into_owned(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&TransactionSetV1Ref<'_>> for TransactionSetV1 {
    #[must_use]
    fn from(v: &TransactionSetV1Ref<'_>) -> Self {
        v.into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<TransactionSetV1Ref<'_>> for TransactionSetV1 {
    #[must_use]
    fn from(v: TransactionSetV1Ref<'_>) -> Self {
        v.into_owned()
    }
}

impl WriteXdr for TransactionSetV1Ref<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.previous_ledger_hash.write_xdr(w)?;
            self.phases.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "const")]
impl TransactionSetV1View<'_> {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_transaction_set_v1(self);
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
        w.write_type_transaction_set_v1(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`TransactionSetV1`], mirroring `<TransactionSetV1 as WriteXdr>::write_xdr`.
    pub const fn write_type_transaction_set_v1(&mut self, v: &TransactionSetV1View<'_>) {
        self.write_type_hash(&v.previous_ledger_hash);
        self.write_type_vec_transaction_phase(&v.phases);
    }
}
