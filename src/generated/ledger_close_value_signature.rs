#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// LedgerCloseValueSignature is an XDR Struct defined as:
///
/// ```text
/// struct LedgerCloseValueSignature
/// {
///     NodeID nodeID;       // which node introduced the value
///     Signature signature; // nodeID's signature
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
pub struct LedgerCloseValueSignature {
    pub node_id: NodeId,
    pub signature: Signature,
}

impl ReadXdr for LedgerCloseValueSignature {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                node_id: NodeId::read_xdr(r)?,
                signature: Signature::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for LedgerCloseValueSignature {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.node_id.write_xdr(w)?;
            self.signature.write_xdr(w)?;
            Ok(())
        })
    }
}

/// LedgerCloseValueSignatureRef is a borrowing equivalent of [`LedgerCloseValueSignature`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct LedgerCloseValueSignatureRef<'a> {
    pub node_id: NodeId,
    pub signature: SignatureRef<'a>,
}

#[cfg(feature = "alloc")]
impl IntoOwned for LedgerCloseValueSignatureRef<'_> {
    type Owned = LedgerCloseValueSignature;
    fn into_owned(self) -> LedgerCloseValueSignature {
        LedgerCloseValueSignature {
            node_id: self.node_id.into_owned(),
            signature: self.signature.into_owned(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&LedgerCloseValueSignatureRef<'_>> for LedgerCloseValueSignature {
    #[must_use]
    fn from(v: &LedgerCloseValueSignatureRef<'_>) -> Self {
        v.into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<LedgerCloseValueSignatureRef<'_>> for LedgerCloseValueSignature {
    #[must_use]
    fn from(v: LedgerCloseValueSignatureRef<'_>) -> Self {
        v.into_owned()
    }
}

impl WriteXdr for LedgerCloseValueSignatureRef<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.node_id.write_xdr(w)?;
            self.signature.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "const")]
impl LedgerCloseValueSignatureRef<'_> {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_ledger_close_value_signature(self);
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
        w.write_type_ledger_close_value_signature(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`LedgerCloseValueSignature`], mirroring `<LedgerCloseValueSignature as WriteXdr>::write_xdr`.
    pub const fn write_type_ledger_close_value_signature(
        &mut self,
        v: &LedgerCloseValueSignatureRef<'_>,
    ) {
        self.write_type_node_id(&v.node_id);
        self.write_type_signature(&v.signature);
    }
}
