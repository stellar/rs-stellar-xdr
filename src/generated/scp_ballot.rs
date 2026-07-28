#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ScpBallot is an XDR Struct defined as:
///
/// ```text
/// struct SCPBallot
/// {
///     uint32 counter; // n
///     Value value;    // x
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
pub struct ScpBallot {
    pub counter: u32,
    pub value: Value,
}

impl ReadXdr for ScpBallot {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                counter: u32::read_xdr(r)?,
                value: Value::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ScpBallot {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.counter.write_xdr(w)?;
            self.value.write_xdr(w)?;
            Ok(())
        })
    }
}

/// ScpBallotRef is a borrowing equivalent of [`ScpBallot`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScpBallotRef<'a> {
    pub counter: u32,
    pub value: ValueRef<'a>,
}

#[cfg(feature = "alloc")]
impl From<&ScpBallotRef<'_>> for ScpBallot {
    #[must_use]
    fn from(v: &ScpBallotRef<'_>) -> Self {
        Self {
            counter: v.counter,
            value: (&v.value).into(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<ScpBallotRef<'_>> for ScpBallot {
    #[must_use]
    fn from(v: ScpBallotRef<'_>) -> Self {
        Self::from(&v)
    }
}

impl WriteXdr for ScpBallotRef<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.counter.write_xdr(w)?;
            self.value.write_xdr(w)?;
            Ok(())
        })
    }
}
