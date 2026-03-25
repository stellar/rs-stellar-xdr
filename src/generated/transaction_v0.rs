#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
/// TransactionV0 is an XDR Struct defined as:
///
/// ```text
/// struct TransactionV0
/// {
///     uint256 sourceAccountEd25519;
///     uint32 fee;
///     SequenceNumber seqNum;
///     TimeBounds* timeBounds;
///     Memo memo;
///     Operation operations<MAX_OPS_PER_TX>;
///     union switch (int v)
///     {
///     case 0:
///         void;
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
pub struct TransactionV0 {
    pub source_account_ed25519: Uint256,
    pub fee: u32,
    pub seq_num: SequenceNumber,
    pub time_bounds: Option<TimeBounds>,
    pub memo: Memo,
    pub operations: VecM<Operation, 100>,
    pub ext: TransactionV0Ext,
}

impl ReadXdr for TransactionV0 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                source_account_ed25519: Uint256::read_xdr(r)?,
                fee: u32::read_xdr(r)?,
                seq_num: SequenceNumber::read_xdr(r)?,
                time_bounds: Option::<TimeBounds>::read_xdr(r)?,
                memo: Memo::read_xdr(r)?,
                operations: VecM::<Operation, 100>::read_xdr(r)?,
                ext: TransactionV0Ext::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for TransactionV0 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.source_account_ed25519.write_xdr(w)?;
            self.fee.write_xdr(w)?;
            self.seq_num.write_xdr(w)?;
            self.time_bounds.write_xdr(w)?;
            self.memo.write_xdr(w)?;
            self.operations.write_xdr(w)?;
            self.ext.write_xdr(w)?;
            Ok(())
        })
    }
}
