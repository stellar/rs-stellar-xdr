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
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval)]
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

/// FeeBumpTransactionEnvelopeView is a borrowing equivalent of [`FeeBumpTransactionEnvelope`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct FeeBumpTransactionEnvelopeView<'a> {
    pub tx: FeeBumpTransactionView<'a>,
    pub signatures: VecMView<'a, DecoratedSignatureView<'a>, 20>,
}

#[cfg(feature = "alloc")]
impl IntoOwned for FeeBumpTransactionEnvelopeView<'_> {
    type Owned = FeeBumpTransactionEnvelope;
    fn into_owned(&self) -> FeeBumpTransactionEnvelope {
        FeeBumpTransactionEnvelope {
            tx: self.tx.into_owned(),
            signatures: self.signatures.into_owned(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&FeeBumpTransactionEnvelopeView<'_>> for FeeBumpTransactionEnvelope {
    #[must_use]
    fn from(v: &FeeBumpTransactionEnvelopeView<'_>) -> Self {
        v.into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<FeeBumpTransactionEnvelopeView<'_>> for FeeBumpTransactionEnvelope {
    #[must_use]
    fn from(v: FeeBumpTransactionEnvelopeView<'_>) -> Self {
        v.into_owned()
    }
}

impl WriteXdr for FeeBumpTransactionEnvelopeView<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.tx.write_xdr(w)?;
            self.signatures.write_xdr(w)?;
            Ok(())
        })
    }
}
