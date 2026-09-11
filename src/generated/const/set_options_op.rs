#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// SetOptionsOp is a borrowing equivalent of [`SetOptionsOp`](super::super::SetOptionsOp)
/// over `'static` data, for const XDR encoding.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SetOptionsOp {
    pub inflation_dest: Option<AccountId>,
    pub clear_flags: Option<u32>,
    pub set_flags: Option<u32>,
    pub master_weight: Option<u32>,
    pub low_threshold: Option<u32>,
    pub med_threshold: Option<u32>,
    pub high_threshold: Option<u32>,
    pub home_domain: Option<String32>,
    pub signer: Option<Signer>,
}

impl SetOptionsOp {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_set_options_op(self);
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
        w.write_type_set_options_op(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

impl ConstWriter<'_> {
    /// Serializes a [`SetOptionsOp`], mirroring `<SetOptionsOp as WriteXdr>::write_xdr`.
    pub const fn write_type_set_options_op(&mut self, v: &SetOptionsOp) {
        self.write_type_option_account_id(&v.inflation_dest);
        self.write_option_u32(&v.clear_flags);
        self.write_option_u32(&v.set_flags);
        self.write_option_u32(&v.master_weight);
        self.write_option_u32(&v.low_threshold);
        self.write_option_u32(&v.med_threshold);
        self.write_option_u32(&v.high_threshold);
        self.write_type_option_string32(&v.home_domain);
        self.write_type_option_signer(&v.signer);
    }
}
