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
#[cfg_eval::cfg_eval]
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
