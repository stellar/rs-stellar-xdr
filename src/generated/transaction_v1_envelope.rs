#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// TransactionV1Envelope is an XDR Struct defined as:
///
/// ```text
/// struct TransactionV1Envelope
/// {
///     Transaction tx;
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
pub struct TransactionV1Envelope {
    pub tx: Transaction,
    pub signatures: VecM<DecoratedSignature, 20>,
}

impl ReadXdr for TransactionV1Envelope {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                tx: Transaction::read_xdr(r)?,
                signatures: VecM::<DecoratedSignature, 20>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for TransactionV1Envelope {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.tx.write_xdr(w)?;
            self.signatures.write_xdr(w)?;
            Ok(())
        })
    }
}

/// TransactionV1EnvelopeView is a borrowing equivalent of [`TransactionV1Envelope`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TransactionV1EnvelopeView<'a> {
    pub tx: TransactionView<'a>,
    pub signatures: VecMView<'a, DecoratedSignatureView<'a>, 20>,
}

#[cfg(feature = "alloc")]
impl From<&TransactionV1EnvelopeView<'_>> for TransactionV1Envelope {
    #[must_use]
    fn from(v: &TransactionV1EnvelopeView<'_>) -> Self {
        Self {
            tx: (&v.tx).into(),
            signatures: v.signatures.to_vecm(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<TransactionV1EnvelopeView<'_>> for TransactionV1Envelope {
    #[must_use]
    fn from(v: TransactionV1EnvelopeView<'_>) -> Self {
        Self::from(&v)
    }
}

impl WriteXdr for TransactionV1EnvelopeView<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.tx.write_xdr(w)?;
            self.signatures.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "const")]
impl TransactionV1EnvelopeView<'_> {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_transaction_v1_envelope(self);
        w.len()
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
    #[must_use]
    pub const fn const_to_xdr<const N: usize>(&self) -> [u8; N] {
        let mut buf = [0u8; N];
        let mut w = ConstWriter::new(&mut buf);
        w.write_type_transaction_v1_envelope(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`TransactionV1Envelope`], mirroring `<TransactionV1Envelope as WriteXdr>::write_xdr`.
    pub const fn write_type_transaction_v1_envelope(&mut self, v: &TransactionV1EnvelopeView<'_>) {
        self.write_type_transaction(&v.tx);
        self.write_type_vec_decorated_signature(&v.signatures);
    }
}
