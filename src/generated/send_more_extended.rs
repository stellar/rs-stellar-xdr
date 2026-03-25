#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
/// SendMoreExtended is an XDR Struct defined as:
///
/// ```text
/// struct SendMoreExtended
/// {
///     uint32 numMessages;
///     uint32 numBytes;
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
pub struct SendMoreExtended {
    pub num_messages: u32,
    pub num_bytes: u32,
}

impl ReadXdr for SendMoreExtended {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                num_messages: u32::read_xdr(r)?,
                num_bytes: u32::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for SendMoreExtended {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.num_messages.write_xdr(w)?;
            self.num_bytes.write_xdr(w)?;
            Ok(())
        })
    }
}
