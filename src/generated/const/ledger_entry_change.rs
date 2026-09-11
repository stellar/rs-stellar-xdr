#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// LedgerEntryChange is a borrowing equivalent of [`LedgerEntryChange`](super::super::LedgerEntryChange)
/// over `'static` data, for const XDR encoding.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[allow(clippy::large_enum_variant)]
pub enum LedgerEntryChange {
    Created(LedgerEntry),
    Updated(LedgerEntry),
    Removed(LedgerKey),
    State(LedgerEntry),
    Restored(LedgerEntry),
}

impl LedgerEntryChange {
    #[must_use]
    pub const fn discriminant(&self) -> LedgerEntryChangeType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Created(_) => LedgerEntryChangeType::Created,
            Self::Updated(_) => LedgerEntryChangeType::Updated,
            Self::Removed(_) => LedgerEntryChangeType::Removed,
            Self::State(_) => LedgerEntryChangeType::State,
            Self::Restored(_) => LedgerEntryChangeType::Restored,
        }
    }
}

impl LedgerEntryChange {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_ledger_entry_change(self);
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
        w.write_type_ledger_entry_change(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

impl ConstWriter<'_> {
    /// Serializes a [`LedgerEntryChange`], mirroring `<LedgerEntryChange as WriteXdr>::write_xdr`.
    pub const fn write_type_ledger_entry_change(&mut self, v: &LedgerEntryChange) {
        let d = v.discriminant();
        self.write_type_ledger_entry_change_type(&d);
        #[allow(clippy::match_same_arms)]
        match v {
            LedgerEntryChange::Created(value) => {
                self.write_type_ledger_entry(value);
            }
            LedgerEntryChange::Updated(value) => {
                self.write_type_ledger_entry(value);
            }
            LedgerEntryChange::Removed(value) => {
                self.write_type_ledger_key(value);
            }
            LedgerEntryChange::State(value) => {
                self.write_type_ledger_entry(value);
            }
            LedgerEntryChange::Restored(value) => {
                self.write_type_ledger_entry(value);
            }
        }
    }

    /// Serializes a variable-length array of [`LedgerEntryChange`], mirroring `<VecM<LedgerEntryChange, MAX> as WriteXdr>::write_xdr`.
    pub const fn write_type_vec_ledger_entry_change<const MAX: u32>(
        &mut self,
        v: &VecM<LedgerEntryChange, MAX>,
    ) {
        let s = v.as_slice();
        let len = s.len();
        self.write_len(len);
        let mut i = 0usize;
        while i < len {
            self.write_type_ledger_entry_change(&s[i]);
            i += 1;
        }
    }
}
