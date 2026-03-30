#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// HashIdPreimageRevokeId is an XDR NestedStruct defined as:
///
/// ```text
/// struct
///     {
///         AccountID sourceAccount;
///         SequenceNumber seqNum; 
///         uint32 opNum;
///         PoolID liquidityPoolID;
///         Asset asset;
///     }
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
pub struct HashIdPreimageRevokeId {
    pub source_account: AccountId,
    pub seq_num: SequenceNumber,
    pub op_num: u32,
    pub liquidity_pool_id: PoolId,
    pub asset: Asset,
}

impl ReadXdr for HashIdPreimageRevokeId {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                source_account: AccountId::read_xdr(r)?,
                seq_num: SequenceNumber::read_xdr(r)?,
                op_num: u32::read_xdr(r)?,
                liquidity_pool_id: PoolId::read_xdr(r)?,
                asset: Asset::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for HashIdPreimageRevokeId {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.source_account.write_xdr(w)?;
            self.seq_num.write_xdr(w)?;
            self.op_num.write_xdr(w)?;
            self.liquidity_pool_id.write_xdr(w)?;
            self.asset.write_xdr(w)?;
            Ok(())
        })
    }
}
