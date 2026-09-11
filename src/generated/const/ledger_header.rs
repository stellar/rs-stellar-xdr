#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// LedgerHeader is a borrowing equivalent of [`LedgerHeader`](super::super::LedgerHeader)
/// over `'static` data, for const XDR encoding.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct LedgerHeader {
    pub ledger_version: u32,
    pub previous_ledger_hash: Hash,
    pub scp_value: StellarValue,
    pub tx_set_result_hash: Hash,
    pub bucket_list_hash: Hash,
    pub ledger_seq: u32,
    pub total_coins: i64,
    pub fee_pool: i64,
    pub inflation_seq: u32,
    pub id_pool: u64,
    pub base_fee: u32,
    pub base_reserve: u32,
    pub max_tx_set_size: u32,
    pub skip_list: [Hash; 4],
    pub ext: LedgerHeaderExt,
}

impl LedgerHeader {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_ledger_header(self);
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
        w.write_type_ledger_header(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

impl ConstWriter<'_> {
    /// Serializes a [`LedgerHeader`], mirroring `<LedgerHeader as WriteXdr>::write_xdr`.
    pub const fn write_type_ledger_header(&mut self, v: &LedgerHeader) {
        self.write_u32(v.ledger_version);
        self.write_type_hash(&v.previous_ledger_hash);
        self.write_type_stellar_value(&v.scp_value);
        self.write_type_hash(&v.tx_set_result_hash);
        self.write_type_hash(&v.bucket_list_hash);
        self.write_u32(v.ledger_seq);
        self.write_i64(v.total_coins);
        self.write_i64(v.fee_pool);
        self.write_u32(v.inflation_seq);
        self.write_u64(v.id_pool);
        self.write_u32(v.base_fee);
        self.write_u32(v.base_reserve);
        self.write_u32(v.max_tx_set_size);
        {
            let mut i = 0usize;
            while i < 4 {
                self.write_type_hash(&v.skip_list[i]);
                i += 1;
            }
        }
        self.write_type_ledger_header_ext(&v.ext);
    }
}
