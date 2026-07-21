#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// MuxedAccountMed25519 is an XDR NestedStruct defined as:
///
/// ```text
/// struct
///     {
///         uint64 id;
///         uint256 ed25519;
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
pub struct MuxedAccountMed25519 {
    pub id: u64,
    pub ed25519: Uint256,
}

impl ReadXdr for MuxedAccountMed25519 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                id: u64::read_xdr(r)?,
                ed25519: Uint256::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for MuxedAccountMed25519 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.id.write_xdr(w)?;
            self.ed25519.write_xdr(w)?;
            Ok(())
        })
    }
}
#[cfg(all(feature = "serde", feature = "alloc"))]
impl<'de> serde::Deserialize<'de> for MuxedAccountMed25519 {
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::Deserialize;
        #[derive(Deserialize)]
        struct MuxedAccountMed25519 {
            id: u64,
            ed25519: Uint256,
        }
        if cfg!(feature = "serde_ignored") {
            // With the serde_ignored feature enabled, deserialize transparently
            // through the given deserializer so unknown fields remain observable
            // by serde_ignored at runtime. An untagged enum can't be used here
            // because it buffers the input, hiding ignored fields from
            // serde_ignored.
            struct V;
            impl<'de> serde::de::Visitor<'de> for V {
                type Value = self::MuxedAccountMed25519;
                fn expecting(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                    f.write_str("MuxedAccountMed25519 as a string or a map")
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
                    let MuxedAccountMed25519 { id, ed25519 } =
                        <MuxedAccountMed25519 as serde::Deserialize>::deserialize(
                            serde::de::value::MapAccessDeserializer::new(map),
                        )?;
                    Ok(self::MuxedAccountMed25519 { id, ed25519 })
                }
            }
            deserializer.deserialize_any(V)
        } else {
            #[derive(Deserialize)]
            #[serde(untagged)]
            enum MuxedAccountMed25519OrString<'a> {
                Str(&'a str),
                String(String),
                MuxedAccountMed25519(MuxedAccountMed25519),
            }
            match MuxedAccountMed25519OrString::deserialize(deserializer)? {
                MuxedAccountMed25519OrString::Str(s) => s.parse().map_err(serde::de::Error::custom),
                MuxedAccountMed25519OrString::String(s) => {
                    s.parse().map_err(serde::de::Error::custom)
                }
                MuxedAccountMed25519OrString::MuxedAccountMed25519(MuxedAccountMed25519 {
                    id,
                    ed25519,
                }) => Ok(self::MuxedAccountMed25519 { id, ed25519 }),
            }
        }
    }
}
