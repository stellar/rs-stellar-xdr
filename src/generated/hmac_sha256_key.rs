#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// HmacSha256Key is an XDR Struct defined as:
///
/// ```text
/// struct HmacSha256Key
/// {
///     opaque key[32];
/// };
/// ```
///
#[cfg_attr(feature = "alloc", derive(Default))]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_eval::cfg_eval]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    serde_with::serde_as,
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct HmacSha256Key {
    pub key: [u8; 32],
}

impl ReadXdr for HmacSha256Key {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                key: <[u8; 32]>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for HmacSha256Key {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.key.write_xdr(w)?;
            Ok(())
        })
    }
}
