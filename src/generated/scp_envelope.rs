#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
/// ScpEnvelope is an XDR Struct defined as:
///
/// ```text
/// struct SCPEnvelope
/// {
///     SCPStatement statement;
///     Signature signature;
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
pub struct ScpEnvelope {
    pub statement: ScpStatement,
    pub signature: Signature,
}

impl ReadXdr for ScpEnvelope {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                statement: ScpStatement::read_xdr(r)?,
                signature: Signature::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ScpEnvelope {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.statement.write_xdr(w)?;
            self.signature.write_xdr(w)?;
            Ok(())
        })
    }
}
