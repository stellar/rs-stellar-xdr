#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// FeeBumpTransaction is an XDR Struct defined as:
///
/// ```text
/// struct FeeBumpTransaction
/// {
///     MuxedAccount feeSource;
///     int64 fee;
///     union switch (EnvelopeType type)
///     {
///     case ENVELOPE_TYPE_TX:
///         TransactionV1Envelope v1;
///     }
///     innerTx;
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
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    serde_with::serde_as,
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct FeeBumpTransaction {
    pub fee_source: MuxedAccount,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub fee: i64,
    pub inner_tx: FeeBumpTransactionInnerTx,
    pub ext: FeeBumpTransactionExt,
}

impl ReadXdr for FeeBumpTransaction {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                fee_source: MuxedAccount::read_xdr(r)?,
                fee: i64::read_xdr(r)?,
                inner_tx: FeeBumpTransactionInnerTx::read_xdr(r)?,
                ext: FeeBumpTransactionExt::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for FeeBumpTransaction {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.fee_source.write_xdr(w)?;
            self.fee.write_xdr(w)?;
            self.inner_tx.write_xdr(w)?;
            self.ext.write_xdr(w)?;
            Ok(())
        })
    }
}
