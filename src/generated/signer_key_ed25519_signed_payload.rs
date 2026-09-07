#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// SignerKeyEd25519SignedPayload is an XDR NestedStruct defined as:
///
/// ```text
/// struct
///     {
///         /* Public key that must sign the payload. */
///         uint256 ed25519;
///         /* Payload to be raw signed by ed25519. */
///         opaque payload<64>;
///     }
/// ```
///
#[cfg_attr(feature = "alloc", derive(Default))]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde_with::SerializeDisplay)
)]
pub struct SignerKeyEd25519SignedPayload {
    pub ed25519: Uint256,
    pub payload: BytesM<64>,
}

impl ReadXdr for SignerKeyEd25519SignedPayload {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                ed25519: Uint256::read_xdr(r)?,
                payload: BytesM::<64>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for SignerKeyEd25519SignedPayload {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.ed25519.write_xdr(w)?;
            self.payload.write_xdr(w)?;
            Ok(())
        })
    }
}
#[cfg(all(feature = "serde", feature = "alloc"))]
impl<'de> serde::Deserialize<'de> for SignerKeyEd25519SignedPayload {
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::Deserialize;
        #[derive(Deserialize)]
        struct SignerKeyEd25519SignedPayload {
            ed25519: Uint256,
            payload: BytesM<64>,
        }
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum SignerKeyEd25519SignedPayloadOrString<'a> {
            Str(&'a str),
            String(String),
            SignerKeyEd25519SignedPayload(SignerKeyEd25519SignedPayload),
        }
        match SignerKeyEd25519SignedPayloadOrString::deserialize(deserializer)? {
            SignerKeyEd25519SignedPayloadOrString::Str(s) => {
                s.parse().map_err(serde::de::Error::custom)
            }
            SignerKeyEd25519SignedPayloadOrString::String(s) => {
                s.parse().map_err(serde::de::Error::custom)
            }
            SignerKeyEd25519SignedPayloadOrString::SignerKeyEd25519SignedPayload(
                SignerKeyEd25519SignedPayload { ed25519, payload },
            ) => Ok(self::SignerKeyEd25519SignedPayload { ed25519, payload }),
        }
    }
}

/// SignerKeyEd25519SignedPayloadView is a borrowing equivalent of [`SignerKeyEd25519SignedPayload`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SignerKeyEd25519SignedPayloadView<'a> {
    pub ed25519: Uint256,
    pub payload: BytesMView<'a, 64>,
}

#[cfg(feature = "alloc")]
impl IntoOwned for SignerKeyEd25519SignedPayloadView<'_> {
    type Owned = SignerKeyEd25519SignedPayload;
    fn into_owned(&self) -> SignerKeyEd25519SignedPayload {
        SignerKeyEd25519SignedPayload {
            ed25519: IntoOwned::into_owned(&self.ed25519),
            payload: IntoOwned::into_owned(&self.payload),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&SignerKeyEd25519SignedPayloadView<'_>> for SignerKeyEd25519SignedPayload {
    #[must_use]
    fn from(v: &SignerKeyEd25519SignedPayloadView<'_>) -> Self {
        IntoOwned::into_owned(v)
    }
}

#[cfg(feature = "alloc")]
impl From<SignerKeyEd25519SignedPayloadView<'_>> for SignerKeyEd25519SignedPayload {
    #[must_use]
    fn from(v: SignerKeyEd25519SignedPayloadView<'_>) -> Self {
        IntoOwned::into_owned(&v)
    }
}

impl WriteXdr for SignerKeyEd25519SignedPayloadView<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.ed25519.write_xdr(w)?;
            self.payload.write_xdr(w)?;
            Ok(())
        })
    }
}
