#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// DecoratedSignature is an XDR Struct defined as:
///
/// ```text
/// struct DecoratedSignature
/// {
///     SignatureHint hint;  // last 4 bytes of the public key, used as a hint
///     Signature signature; // actual signature
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
pub struct DecoratedSignature {
    pub hint: SignatureHint,
    pub signature: Signature,
}

impl ReadXdr for DecoratedSignature {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                hint: SignatureHint::read_xdr(r)?,
                signature: Signature::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for DecoratedSignature {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.hint.write_xdr(w)?;
            self.signature.write_xdr(w)?;
            Ok(())
        })
    }
}

/// DecoratedSignatureConst is a borrowing equivalent of [`DecoratedSignature`] over `'static`
/// data, for const XDR encoding.
#[cfg(feature = "const")]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct DecoratedSignatureConst {
    pub hint: SignatureHint,
    pub signature: SignatureConst,
}

#[cfg(feature = "const")]
impl DecoratedSignatureConst {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_decorated_signature(self);
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
        w.write_type_decorated_signature(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`DecoratedSignature`], mirroring `<DecoratedSignature as WriteXdr>::write_xdr`.
    pub const fn write_type_decorated_signature(&mut self, v: &DecoratedSignatureConst) {
        self.write_type_signature_hint(&v.hint);
        self.write_type_signature(&v.signature);
    }

    /// Serializes a variable-length array of [`DecoratedSignature`], mirroring `<VecM<DecoratedSignature, MAX> as WriteXdr>::write_xdr`.
    pub const fn write_type_vec_decorated_signature<const MAX: u32>(
        &mut self,
        v: &VecMConst<DecoratedSignatureConst, MAX>,
    ) {
        let s = v.as_slice();
        let len = s.len();
        self.write_len(len);
        let mut i = 0usize;
        while i < len {
            self.write_type_decorated_signature(&s[i]);
            i += 1;
        }
    }
}
