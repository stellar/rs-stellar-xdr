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
#[cfg_eval::cfg_eval]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde_with::SerializeDisplay)
)]
pub struct SignerKeyEd25519SignedPayload {
    pub ed25519: Uint256,
    pub payload: BytesM::<64>,
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
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error> where D: serde::Deserializer<'de> {
        use serde::Deserialize;
        #[derive(Deserialize)]
        struct SignerKeyEd25519SignedPayload {
            ed25519: Uint256,
            payload: BytesM::<64>,
        }
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum SignerKeyEd25519SignedPayloadOrString<'a> {
            Str(&'a str),
            String(String),
            SignerKeyEd25519SignedPayload(SignerKeyEd25519SignedPayload),
        }
        match SignerKeyEd25519SignedPayloadOrString::deserialize(deserializer)? {
            SignerKeyEd25519SignedPayloadOrString::Str(s) => s.parse().map_err(serde::de::Error::custom),
            SignerKeyEd25519SignedPayloadOrString::String(s) => s.parse().map_err(serde::de::Error::custom),
            SignerKeyEd25519SignedPayloadOrString::SignerKeyEd25519SignedPayload(SignerKeyEd25519SignedPayload {
                ed25519, payload,
            }) => Ok(self::SignerKeyEd25519SignedPayload {
                ed25519, payload,
            }),
        }
    }
}
