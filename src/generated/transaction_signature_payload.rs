#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
/// TransactionSignaturePayload is an XDR Struct defined as:
///
/// ```text
/// struct TransactionSignaturePayload
/// {
///     Hash networkId;
///     union switch (EnvelopeType type)
///     {
///     // Backwards Compatibility: Use ENVELOPE_TYPE_TX to sign ENVELOPE_TYPE_TX_V0
///     case ENVELOPE_TYPE_TX:
///         Transaction tx;
///     case ENVELOPE_TYPE_TX_FEE_BUMP:
///         FeeBumpTransaction feeBump;
///     }
///     taggedTransaction;
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
pub struct TransactionSignaturePayload {
    pub network_id: Hash,
    pub tagged_transaction: TransactionSignaturePayloadTaggedTransaction,
}

impl ReadXdr for TransactionSignaturePayload {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                network_id: Hash::read_xdr(r)?,
                tagged_transaction: TransactionSignaturePayloadTaggedTransaction::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for TransactionSignaturePayload {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.network_id.write_xdr(w)?;
            self.tagged_transaction.write_xdr(w)?;
            Ok(())
        })
    }
}
