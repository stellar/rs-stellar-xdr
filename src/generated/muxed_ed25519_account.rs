#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// MuxedEd25519Account is an XDR Struct defined as:
///
/// ```text
/// struct MuxedEd25519Account
/// {
///     uint64 id;
///     uint256 ed25519;
/// };
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
pub struct MuxedEd25519Account {
    pub id: u64,
    pub ed25519: Uint256,
}

impl ReadXdr for MuxedEd25519Account {
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

impl WriteXdr for MuxedEd25519Account {
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
impl<'de> serde::Deserialize<'de> for MuxedEd25519Account {
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::Deserialize;
        #[derive(Deserialize)]
        struct MuxedEd25519Account {
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
                type Value = self::MuxedEd25519Account;
                fn expecting(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                    f.write_str("MuxedEd25519Account as a string or a map")
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
                    let MuxedEd25519Account { id, ed25519 } =
                        <MuxedEd25519Account as serde::Deserialize>::deserialize(
                            serde::de::value::MapAccessDeserializer::new(map),
                        )?;
                    Ok(self::MuxedEd25519Account { id, ed25519 })
                }
            }
            deserializer.deserialize_any(V)
        } else {
            #[derive(Deserialize)]
            #[serde(untagged)]
            enum MuxedEd25519AccountOrString<'a> {
                Str(&'a str),
                String(String),
                MuxedEd25519Account(MuxedEd25519Account),
            }
            match MuxedEd25519AccountOrString::deserialize(deserializer)? {
                MuxedEd25519AccountOrString::Str(s) => s.parse().map_err(serde::de::Error::custom),
                MuxedEd25519AccountOrString::String(s) => {
                    s.parse().map_err(serde::de::Error::custom)
                }
                MuxedEd25519AccountOrString::MuxedEd25519Account(MuxedEd25519Account {
                    id,
                    ed25519,
                }) => Ok(self::MuxedEd25519Account { id, ed25519 }),
            }
        }
    }
}
