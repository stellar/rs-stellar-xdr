#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// LedgerScpMessages is an XDR Struct defined as:
///
/// ```text
/// struct LedgerSCPMessages
/// {
///     uint32 ledgerSeq;
///     SCPEnvelope messages<>;
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
pub struct LedgerScpMessages {
    pub ledger_seq: u32,
    pub messages: VecM<ScpEnvelope>,
}

impl ReadXdr for LedgerScpMessages {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                ledger_seq: u32::read_xdr(r)?,
                messages: VecM::<ScpEnvelope>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for LedgerScpMessages {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.ledger_seq.write_xdr(w)?;
            self.messages.write_xdr(w)?;
            Ok(())
        })
    }
}

/// LedgerScpMessagesConst is a borrowing equivalent of [`LedgerScpMessages`] over `'static`
/// data, for const XDR encoding.
#[cfg(feature = "const")]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct LedgerScpMessagesConst {
    pub ledger_seq: u32,
    pub messages: VecMConst<ScpEnvelopeConst>,
}

#[cfg(feature = "const")]
impl LedgerScpMessagesConst {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_ledger_scp_messages(self);
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
        w.write_type_ledger_scp_messages(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`LedgerScpMessages`], mirroring `<LedgerScpMessages as WriteXdr>::write_xdr`.
    pub const fn write_type_ledger_scp_messages(&mut self, v: &LedgerScpMessagesConst) {
        self.write_u32(v.ledger_seq);
        self.write_type_vec_scp_envelope(&v.messages);
    }
}
