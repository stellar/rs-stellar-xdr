#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

impl ClaimOfferAtomV0 {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_claim_offer_atom_v0(self);
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
        w.write_type_claim_offer_atom_v0(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

impl ConstWriter<'_> {
    /// Serializes a [`ClaimOfferAtomV0`], mirroring `<ClaimOfferAtomV0 as WriteXdr>::write_xdr`.
    pub const fn write_type_claim_offer_atom_v0(&mut self, v: &ClaimOfferAtomV0) {
        self.write_type_uint256(&v.seller_ed25519);
        self.write_i64(v.offer_id);
        self.write_type_asset(&v.asset_sold);
        self.write_i64(v.amount_sold);
        self.write_type_asset(&v.asset_bought);
        self.write_i64(v.amount_bought);
    }
}
