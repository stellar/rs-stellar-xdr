#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// LedgerEntry is an XDR Struct defined as:
///
/// ```text
/// struct LedgerEntry
/// {
///     uint32 lastModifiedLedgerSeq; // ledger the LedgerEntry was last changed
/// 
///     union switch (LedgerEntryType type)
///     {
///     case ACCOUNT:
///         AccountEntry account;
///     case TRUSTLINE:
///         TrustLineEntry trustLine;
///     case OFFER:
///         OfferEntry offer;
///     case DATA:
///         DataEntry data;
///     case CLAIMABLE_BALANCE:
///         ClaimableBalanceEntry claimableBalance;
///     case LIQUIDITY_POOL:
///         LiquidityPoolEntry liquidityPool;
///     case CONTRACT_DATA:
///         ContractDataEntry contractData;
///     case CONTRACT_CODE:
///         ContractCodeEntry contractCode;
///     case CONFIG_SETTING:
///         ConfigSettingEntry configSetting;
///     case TTL:
///         TTLEntry ttl;
///     }
///     data;
/// 
///     // reserved for future use
///     union switch (int v)
///     {
///     case 0:
///         void;
///     case 1:
///         LedgerEntryExtensionV1 v1;
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
pub struct LedgerEntry {
    pub last_modified_ledger_seq: u32,
    pub data: LedgerEntryData,
    pub ext: LedgerEntryExt,
}

impl ReadXdr for LedgerEntry {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                last_modified_ledger_seq: u32::read_xdr(r)?,
                data: LedgerEntryData::read_xdr(r)?,
                ext: LedgerEntryExt::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for LedgerEntry {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.last_modified_ledger_seq.write_xdr(w)?;
            self.data.write_xdr(w)?;
            self.ext.write_xdr(w)?;
            Ok(())
        })
    }
}
