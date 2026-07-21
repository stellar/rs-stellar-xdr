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
        if cfg!(feature = "serde_ignored") {
            // With the serde_ignored feature enabled, deserialize transparently
            // through the given deserializer so unknown fields remain observable
            // by serde_ignored at runtime. An untagged enum can't be used here
            // because it buffers the input, hiding ignored fields from
            // serde_ignored.
            struct V;
            impl<'de> serde::de::Visitor<'de> for V {
                type Value = self::SignerKeyEd25519SignedPayload;
                fn expecting(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                    f.write_str("SignerKeyEd25519SignedPayload as a string or a map")
                }
                fn visit_str<E>(self, s: &str) -> core::result::Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    s.parse().map_err(serde::de::Error::custom)
                }
                fn visit_map<A>(self, map: A) -> core::result::Result<Self::Value, A::Error>
                where
                    A: serde::de::MapAccess<'de>,
                {
                    let SignerKeyEd25519SignedPayload { ed25519, payload } =
                        <SignerKeyEd25519SignedPayload as serde::Deserialize>::deserialize(
                            serde::de::value::MapAccessDeserializer::new(map),
                        )?;
                    Ok(self::SignerKeyEd25519SignedPayload { ed25519, payload })
                }
            }
            deserializer.deserialize_any(V)
        } else {
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
}
