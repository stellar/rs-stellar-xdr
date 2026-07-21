#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// Int256Parts is an XDR Struct defined as:
///
/// ```text
/// struct Int256Parts {
///     int64 hi_hi;
///     uint64 hi_lo;
///     uint64 lo_hi;
///     uint64 lo_lo;
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
pub struct Int256Parts {
    pub hi_hi: i64,
    pub hi_lo: u64,
    pub lo_hi: u64,
    pub lo_lo: u64,
}

impl ReadXdr for Int256Parts {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                hi_hi: i64::read_xdr(r)?,
                hi_lo: u64::read_xdr(r)?,
                lo_hi: u64::read_xdr(r)?,
                lo_lo: u64::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for Int256Parts {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.hi_hi.write_xdr(w)?;
            self.hi_lo.write_xdr(w)?;
            self.lo_hi.write_xdr(w)?;
            self.lo_lo.write_xdr(w)?;
            Ok(())
        })
    }
}
#[cfg(all(feature = "serde", feature = "alloc"))]
impl<'de> serde::Deserialize<'de> for Int256Parts {
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::Deserialize;
        #[derive(Deserialize)]
        struct Int256Parts {
            hi_hi: i64,
            hi_lo: u64,
            lo_hi: u64,
            lo_lo: u64,
        }
        if cfg!(feature = "serde_ignored") {
            // With the serde_ignored feature enabled, deserialize transparently
            // through the given deserializer so unknown fields remain observable
            // by serde_ignored at runtime. An untagged enum can't be used here
            // because it buffers the input, hiding ignored fields from
            // serde_ignored.
            struct V;
            impl<'de> serde::de::Visitor<'de> for V {
                type Value = self::Int256Parts;
                fn expecting(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                    f.write_str("Int256Parts as a string or a map")
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
                    let Int256Parts {
                        hi_hi,
                        hi_lo,
                        lo_hi,
                        lo_lo,
                    } = <Int256Parts as serde::Deserialize>::deserialize(
                        serde::de::value::MapAccessDeserializer::new(map),
                    )?;
                    Ok(self::Int256Parts {
                        hi_hi,
                        hi_lo,
                        lo_hi,
                        lo_lo,
                    })
                }
            }
            deserializer.deserialize_any(V)
        } else {
            #[derive(Deserialize)]
            #[serde(untagged)]
            enum Int256PartsOrString<'a> {
                Str(&'a str),
                String(String),
                Int256Parts(Int256Parts),
            }
            match Int256PartsOrString::deserialize(deserializer)? {
                Int256PartsOrString::Str(s) => s.parse().map_err(serde::de::Error::custom),
                Int256PartsOrString::String(s) => s.parse().map_err(serde::de::Error::custom),
                Int256PartsOrString::Int256Parts(Int256Parts {
                    hi_hi,
                    hi_lo,
                    lo_hi,
                    lo_lo,
                }) => Ok(self::Int256Parts {
                    hi_hi,
                    hi_lo,
                    lo_hi,
                    lo_lo,
                }),
            }
        }
    }
}
