#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// TransactionV0Envelope is an XDR Struct defined as:
///
/// ```text
/// struct TransactionV0Envelope
/// {
///     TransactionV0 tx;
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
pub struct TransactionV0Envelope {
    pub tx: TransactionV0,
    pub signatures: VecM<DecoratedSignature, 20>,
}

impl ReadXdr for TransactionV0Envelope {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                tx: TransactionV0::read_xdr(r)?,
                signatures: VecM::<DecoratedSignature, 20>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for TransactionV0Envelope {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.tx.write_xdr(w)?;
            self.signatures.write_xdr(w)?;
            Ok(())
        })
    }
}

/// TransactionV0EnvelopeView is a borrowing equivalent of [`TransactionV0Envelope`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TransactionV0EnvelopeView<'a> {
    pub tx: TransactionV0View<'a>,
    pub signatures: VecMView<'a, DecoratedSignatureView<'a>, 20>,
}

#[cfg(feature = "alloc")]
impl From<&TransactionV0EnvelopeView<'_>> for TransactionV0Envelope {
    #[must_use]
    fn from(v: &TransactionV0EnvelopeView<'_>) -> Self {
        Self {
            tx: (&v.tx).into(),
            signatures: v.signatures.to_vecm(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<TransactionV0EnvelopeView<'_>> for TransactionV0Envelope {
    #[must_use]
    fn from(v: TransactionV0EnvelopeView<'_>) -> Self {
        Self::from(&v)
    }
}

impl WriteXdr for TransactionV0EnvelopeView<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.tx.write_xdr(w)?;
            self.signatures.write_xdr(w)?;
            Ok(())
        })
    }
}
