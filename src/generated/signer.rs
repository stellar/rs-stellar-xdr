#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// Signer is an XDR Struct defined as:
///
/// ```text
/// struct Signer
/// {
///     SignerKey key;
///     uint32 weight; // really only need 1 byte
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
pub struct Signer {
    pub key: SignerKey,
    pub weight: u32,
}

impl ReadXdr for Signer {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                key: SignerKey::read_xdr(r)?,
                weight: u32::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for Signer {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.key.write_xdr(w)?;
            self.weight.write_xdr(w)?;
            Ok(())
        })
    }
}

/// SignerView is a borrowing equivalent of [`Signer`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SignerView<'a> {
    pub key: SignerKeyView<'a>,
    pub weight: u32,
}

#[cfg(feature = "alloc")]
impl IntoOwned for SignerView<'_> {
    type Owned = Signer;
    fn into_owned(&self) -> Signer {
        Signer {
            key: IntoOwned::into_owned(&self.key),
            weight: IntoOwned::into_owned(&self.weight),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&SignerView<'_>> for Signer {
    #[must_use]
    fn from(v: &SignerView<'_>) -> Self {
        IntoOwned::into_owned(v)
    }
}

#[cfg(feature = "alloc")]
impl From<SignerView<'_>> for Signer {
    #[must_use]
    fn from(v: SignerView<'_>) -> Self {
        IntoOwned::into_owned(&v)
    }
}

impl WriteXdr for SignerView<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.key.write_xdr(w)?;
            self.weight.write_xdr(w)?;
            Ok(())
        })
    }
}
