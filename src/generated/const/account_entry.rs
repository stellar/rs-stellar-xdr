#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// AccountEntry is a borrowing equivalent of [`AccountEntry`](super::super::AccountEntry)
/// over `'static` data, for const XDR encoding.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct AccountEntry {
    pub account_id: AccountId,
    pub balance: i64,
    pub seq_num: SequenceNumber,
    pub num_sub_entries: u32,
    pub inflation_dest: Option<AccountId>,
    pub flags: u32,
    pub home_domain: String32,
    pub thresholds: Thresholds,
    pub signers: VecM<Signer, MAX_SIGNERS>,
    pub ext: AccountEntryExt,
}

impl AccountEntry {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_account_entry(self);
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
        w.write_type_account_entry(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

impl ConstWriter<'_> {
    /// Serializes a [`AccountEntry`], mirroring `<AccountEntry as WriteXdr>::write_xdr`.
    pub const fn write_type_account_entry(&mut self, v: &AccountEntry) {
        self.write_type_account_id(&v.account_id);
        self.write_i64(v.balance);
        self.write_type_sequence_number(&v.seq_num);
        self.write_u32(v.num_sub_entries);
        self.write_type_option_account_id(&v.inflation_dest);
        self.write_u32(v.flags);
        self.write_type_string32(&v.home_domain);
        self.write_type_thresholds(&v.thresholds);
        self.write_type_vec_signer(&v.signers);
        self.write_type_account_entry_ext(&v.ext);
    }
}
