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
#[cfg_eval::cfg_eval]
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
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error> where D: serde::Deserializer<'de> {
        use serde::Deserialize;
        #[derive(Deserialize)]
        struct MuxedAccountMed25519 {
            id: u64,
            ed25519: Uint256,
        }
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum MuxedAccountMed25519OrString<'a> {
            Str(&'a str),
            String(String),
            MuxedAccountMed25519(MuxedAccountMed25519),
        }
        match MuxedAccountMed25519OrString::deserialize(deserializer)? {
            MuxedAccountMed25519OrString::Str(s) => s.parse().map_err(serde::de::Error::custom),
            MuxedAccountMed25519OrString::String(s) => s.parse().map_err(serde::de::Error::custom),
            MuxedAccountMed25519OrString::MuxedAccountMed25519(MuxedAccountMed25519 {
                id, ed25519,
            }) => Ok(self::MuxedAccountMed25519 {
                id, ed25519,
            }),
        }
    }
}
