#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// DiagnosticEvent is a borrowing equivalent of [`DiagnosticEvent`](super::super::DiagnosticEvent)
/// over `'static` data, for const XDR encoding.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
pub struct DiagnosticEvent {
    pub in_successful_contract_call: bool,
    pub event: ContractEvent,
}

impl DiagnosticEvent {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_diagnostic_event(self);
        w.len()
    }

    /// Serialize this value as XDR into a fixed-size `[u8; N]` using only const
    /// operations. This is the const counterpart to
    /// [`WriteXdr::to_xdr`](super::super::WriteXdr::to_xdr).
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
        w.write_type_diagnostic_event(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

impl ConstWriter<'_> {
    /// Serializes a [`DiagnosticEvent`], mirroring `<DiagnosticEvent as WriteXdr>::write_xdr`.
    pub const fn write_type_diagnostic_event(&mut self, v: &DiagnosticEvent) {
        self.write_bool(v.in_successful_contract_call);
        self.write_type_contract_event(&v.event);
    }

    /// Serializes a variable-length array of [`DiagnosticEvent`], mirroring `<VecM<DiagnosticEvent, MAX> as WriteXdr>::write_xdr`.
    pub const fn write_type_vec_diagnostic_event<const MAX: u32>(
        &mut self,
        v: &VecM<DiagnosticEvent, MAX>,
    ) {
        let s = v.as_slice();
        let len = s.len();
        self.write_len(len);
        let mut i = 0usize;
        while i < len {
            self.write_type_diagnostic_event(&s[i]);
            i += 1;
        }
    }
}
