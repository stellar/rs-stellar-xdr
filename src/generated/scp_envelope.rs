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
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval)]
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

/// ScpEnvelopeRef is a borrowing equivalent of [`ScpEnvelope`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScpEnvelopeRef<'a> {
    pub statement: ScpStatementRef<'a>,
    pub signature: SignatureRef<'a>,
}

#[cfg(feature = "alloc")]
impl From<&ScpEnvelopeRef<'_>> for ScpEnvelope {
    #[must_use]
    fn from(v: &ScpEnvelopeRef<'_>) -> Self {
        Self {
            statement: (&v.statement).into(),
            signature: (&v.signature).into(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<ScpEnvelopeRef<'_>> for ScpEnvelope {
    #[must_use]
    fn from(v: ScpEnvelopeRef<'_>) -> Self {
        Self::from(&v)
    }
}

impl WriteXdr for ScpEnvelopeRef<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.statement.write_xdr(w)?;
            self.signature.write_xdr(w)?;
            Ok(())
        })
    }
}
