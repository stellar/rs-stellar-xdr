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

/// DecoratedSignatureView is a borrowing equivalent of [`DecoratedSignature`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct DecoratedSignatureView<'a> {
    pub hint: SignatureHint,
    pub signature: SignatureView<'a>,
}

#[cfg(feature = "alloc")]
impl IntoOwned for DecoratedSignatureView<'_> {
    type Owned = DecoratedSignature;
    fn into_owned(self) -> DecoratedSignature {
        DecoratedSignature {
            hint: self.hint.into_owned(),
            signature: self.signature.into_owned(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&DecoratedSignatureView<'_>> for DecoratedSignature {
    #[must_use]
    fn from(v: &DecoratedSignatureView<'_>) -> Self {
        v.into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<DecoratedSignatureView<'_>> for DecoratedSignature {
    #[must_use]
    fn from(v: DecoratedSignatureView<'_>) -> Self {
        v.into_owned()
    }
}

impl WriteXdr for DecoratedSignatureView<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.hint.write_xdr(w)?;
            self.signature.write_xdr(w)?;
            Ok(())
        })
    }
}
