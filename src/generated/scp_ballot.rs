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
impl IntoOwned for ScpBallotRef<'_> {
    type Owned = ScpBallot;
    fn into_owned(self) -> ScpBallot {
        ScpBallot {
            counter: self.counter.into_owned(),
            value: self.value.into_owned(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&ScpBallotRef<'_>> for ScpBallot {
    #[must_use]
    fn from(v: &ScpBallotRef<'_>) -> Self {
        v.into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<ScpBallotRef<'_>> for ScpBallot {
    #[must_use]
    fn from(v: ScpBallotRef<'_>) -> Self {
        v.into_owned()
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

#[cfg(feature = "const")]
impl ScpBallotRef<'_> {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_scp_ballot(self);
        w.len()
    }

    /// Serialize this value as XDR into a fixed-size `[u8; N]` using only const
    /// operations. This is the const counterpart to [`WriteXdr::to_xdr`].
    ///
    /// `N` must equal [`Self::const_xdr_len`]. It is intended for callers, such
    /// as a proc-macro, that compute the length with `const_xdr_len` and pass
    /// it as `N`; `const_to_xdr` itself does not need to call `const_xdr_len`.
    ///
    /// # Panics
    ///
    /// Panics if `N` does not equal the value's [`Self::const_xdr_len`].
    #[must_use]
    pub const fn const_to_xdr<const N: usize>(&self) -> [u8; N] {
        let mut buf = [0u8; N];
        let mut w = ConstWriter::new(&mut buf);
        w.write_type_scp_ballot(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`ScpBallot`], mirroring `<ScpBallot as WriteXdr>::write_xdr`.
    pub const fn write_type_scp_ballot(&mut self, v: &ScpBallotRef<'_>) {
        self.write_u32(v.counter);
        self.write_type_value(&v.value);
    }

    /// Serializes an optional [`ScpBallot`], mirroring `<Option<ScpBallot> as WriteXdr>::write_xdr`.
    pub const fn write_type_option_scp_ballot(&mut self, v: &Option<ScpBallotRef<'_>>) {
        match v {
            Some(v) => {
                self.write_u32(1);
                self.write_type_scp_ballot(v);
            }
            None => {
                self.write_u32(0);
            }
        }
    }
}
