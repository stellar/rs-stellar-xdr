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
#[cfg_eval::cfg_eval]
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
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum MuxedEd25519AccountOrString<'a> {
            Str(&'a str),
            String(String),
            MuxedEd25519Account(MuxedEd25519Account),
        }
        match MuxedEd25519AccountOrString::deserialize(deserializer)? {
            MuxedEd25519AccountOrString::Str(s) => s.parse().map_err(serde::de::Error::custom),
            MuxedEd25519AccountOrString::String(s) => s.parse().map_err(serde::de::Error::custom),
            MuxedEd25519AccountOrString::MuxedEd25519Account(MuxedEd25519Account {
                id,
                ed25519,
            }) => Ok(self::MuxedEd25519Account { id, ed25519 }),
        }
    }
}
