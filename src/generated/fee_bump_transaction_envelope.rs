#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
/// FeeBumpTransactionEnvelope is an XDR Struct defined as:
///
/// ```text
/// struct FeeBumpTransactionEnvelope
/// {
///     FeeBumpTransaction tx;
///     /* Each decorated signature is a signature over the SHA256 hash of
///      * a TransactionSignaturePayload */
///     DecoratedSignature signatures<20>;
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
pub struct FeeBumpTransactionEnvelope {
    pub tx: FeeBumpTransaction,
    pub signatures: VecM<DecoratedSignature, 20>,
}

impl ReadXdr for FeeBumpTransactionEnvelope {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                tx: FeeBumpTransaction::read_xdr(r)?,
                signatures: VecM::<DecoratedSignature, 20>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for FeeBumpTransactionEnvelope {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.tx.write_xdr(w)?;
            self.signatures.write_xdr(w)?;
            Ok(())
        })
    }
}
