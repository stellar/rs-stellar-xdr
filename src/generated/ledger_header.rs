#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// LedgerHeader is an XDR Struct defined as:
///
/// ```text
/// struct LedgerHeader
/// {
///     uint32 ledgerVersion;    // the protocol version of the ledger
///     Hash previousLedgerHash; // hash of the previous ledger header
///     StellarValue scpValue;   // what consensus agreed to
///     Hash txSetResultHash;    // the TransactionResultSet that led to this ledger
///     Hash bucketListHash;     // hash of the ledger state
///
///     uint32 ledgerSeq; // sequence number of this ledger
///
///     int64 totalCoins; // total number of stroops in existence.
///                       // 10,000,000 stroops in 1 XLM
///
///     int64 feePool;       // fees burned since last inflation run
///     uint32 inflationSeq; // inflation sequence number
///
///     uint64 idPool; // last used global ID, used for generating objects
///
///     uint32 baseFee;     // base fee per operation in stroops
///     uint32 baseReserve; // account base reserve in stroops
///
///     uint32 maxTxSetSize; // maximum size a transaction set can be
///
///     Hash skipList[4]; // hashes of ledgers in the past. allows you to jump back
///                       // in time without walking the chain back ledger by ledger
///                       // each slot contains the oldest ledger that is mod of
///                       // either 50  5000  50000 or 500000 depending on index
///                       // skipList[0] mod(50), skipList[1] mod(5000), etc
///
///     // reserved for future use
///     union switch (int v)
///     {
///     case 0:
///         void;
///     case 1:
///         LedgerHeaderExtensionV1 v1;
///     }
///     ext;
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
pub struct LedgerHeader {
    pub ledger_version: u32,
    pub previous_ledger_hash: Hash,
    pub scp_value: StellarValue,
    pub tx_set_result_hash: Hash,
    pub bucket_list_hash: Hash,
    pub ledger_seq: u32,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub total_coins: i64,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub fee_pool: i64,
    pub inflation_seq: u32,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub id_pool: u64,
    pub base_fee: u32,
    pub base_reserve: u32,
    pub max_tx_set_size: u32,
    pub skip_list: [Hash; 4],
    pub ext: LedgerHeaderExt,
}

impl ReadXdr for LedgerHeader {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                ledger_version: u32::read_xdr(r)?,
                previous_ledger_hash: Hash::read_xdr(r)?,
                scp_value: StellarValue::read_xdr(r)?,
                tx_set_result_hash: Hash::read_xdr(r)?,
                bucket_list_hash: Hash::read_xdr(r)?,
                ledger_seq: u32::read_xdr(r)?,
                total_coins: i64::read_xdr(r)?,
                fee_pool: i64::read_xdr(r)?,
                inflation_seq: u32::read_xdr(r)?,
                id_pool: u64::read_xdr(r)?,
                base_fee: u32::read_xdr(r)?,
                base_reserve: u32::read_xdr(r)?,
                max_tx_set_size: u32::read_xdr(r)?,
                skip_list: <[Hash; 4]>::read_xdr(r)?,
                ext: LedgerHeaderExt::read_xdr(r)?,
            })
        })
    }
}

impl LedgerHeader {
    /// Serialize this value as XDR into a [`ConstWriter`] using only const
    /// operations. This is the const counterpart to [`WriteXdr::write_xdr`].
    #[cfg(feature = "const")]
    pub const fn const_write_xdr(&self, w: &mut ConstWriter) {
        w.enter_depth();
        w.write_u32(self.ledger_version);
        self.previous_ledger_hash.const_write_xdr(w);
        self.scp_value.const_write_xdr(w);
        self.tx_set_result_hash.const_write_xdr(w);
        self.bucket_list_hash.const_write_xdr(w);
        w.write_u32(self.ledger_seq);
        w.write_i64(self.total_coins);
        w.write_i64(self.fee_pool);
        w.write_u32(self.inflation_seq);
        w.write_u64(self.id_pool);
        w.write_u32(self.base_fee);
        w.write_u32(self.base_reserve);
        w.write_u32(self.max_tx_set_size);
        {
            w.enter_depth();
            let mut __i0 = 0usize;
            while __i0 < 4 {
                self.skip_list[__i0].const_write_xdr(w);
                __i0 += 1;
            }
            w.leave_depth();
        }
        self.ext.const_write_xdr(w);
        w.leave_depth();
    }
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[cfg(feature = "const")]
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let limits = Limits {
            depth: u32::MAX,
            len: usize::MAX,
        };
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty, &limits);
        self.const_write_xdr(&mut w);
        w.position()
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
    #[cfg(feature = "const")]
    #[must_use]
    pub const fn const_to_xdr<const N: usize>(&self) -> [u8; N] {
        let limits = Limits {
            depth: u32::MAX,
            len: usize::MAX,
        };
        let mut buf = [0u8; N];
        let mut w = ConstWriter::new(&mut buf, &limits);
        self.const_write_xdr(&mut w);
        assert!(
            w.position() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

impl WriteXdr for LedgerHeader {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.ledger_version.write_xdr(w)?;
            self.previous_ledger_hash.write_xdr(w)?;
            self.scp_value.write_xdr(w)?;
            self.tx_set_result_hash.write_xdr(w)?;
            self.bucket_list_hash.write_xdr(w)?;
            self.ledger_seq.write_xdr(w)?;
            self.total_coins.write_xdr(w)?;
            self.fee_pool.write_xdr(w)?;
            self.inflation_seq.write_xdr(w)?;
            self.id_pool.write_xdr(w)?;
            self.base_fee.write_xdr(w)?;
            self.base_reserve.write_xdr(w)?;
            self.max_tx_set_size.write_xdr(w)?;
            self.skip_list.write_xdr(w)?;
            self.ext.write_xdr(w)?;
            Ok(())
        })
    }
}
