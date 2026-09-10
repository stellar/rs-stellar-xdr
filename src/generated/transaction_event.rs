#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// TransactionEvent is an XDR Struct defined as:
///
/// ```text
/// struct TransactionEvent {    
///     TransactionEventStage stage;  // Stage at which an event has occurred.
///     ContractEvent event;  // The contract event that has occurred.
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
pub struct TransactionEvent {
    pub stage: TransactionEventStage,
    pub event: ContractEvent,
}

impl ReadXdr for TransactionEvent {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                stage: TransactionEventStage::read_xdr(r)?,
                event: ContractEvent::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for TransactionEvent {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.stage.write_xdr(w)?;
            self.event.write_xdr(w)?;
            Ok(())
        })
    }
}

/// TransactionEventConst is a borrowing equivalent of [`TransactionEvent`] over `'static`
/// data, for const XDR encoding.
#[cfg(feature = "const")]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TransactionEventConst {
    pub stage: TransactionEventStage,
    pub event: ContractEventConst,
}

#[cfg(feature = "const")]
impl TransactionEventConst {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_transaction_event(self);
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
        w.write_type_transaction_event(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`TransactionEvent`], mirroring `<TransactionEvent as WriteXdr>::write_xdr`.
    pub const fn write_type_transaction_event(&mut self, v: &TransactionEventConst) {
        self.write_type_transaction_event_stage(&v.stage);
        self.write_type_contract_event(&v.event);
    }

    /// Serializes a variable-length array of [`TransactionEvent`], mirroring `<VecM<TransactionEvent, MAX> as WriteXdr>::write_xdr`.
    pub const fn write_type_vec_transaction_event<const MAX: u32>(
        &mut self,
        v: &VecMConst<TransactionEventConst, MAX>,
    ) {
        let s = v.as_slice();
        let len = s.len();
        self.write_len(len);
        let mut i = 0usize;
        while i < len {
            self.write_type_transaction_event(&s[i]);
            i += 1;
        }
    }
}
