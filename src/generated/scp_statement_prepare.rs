#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ScpStatementPrepare is an XDR NestedStruct defined as:
///
/// ```text
/// struct
///         {
///             Hash quorumSetHash;       // D
///             SCPBallot ballot;         // b
///             SCPBallot* prepared;      // p
///             SCPBallot* preparedPrime; // p'
///             uint32 nC;                // c.n
///             uint32 nH;                // h.n
///         }
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
pub struct ScpStatementPrepare {
    pub quorum_set_hash: Hash,
    pub ballot: ScpBallot,
    pub prepared: Option<ScpBallot>,
    pub prepared_prime: Option<ScpBallot>,
    pub n_c: u32,
    pub n_h: u32,
}

impl ReadXdr for ScpStatementPrepare {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                quorum_set_hash: Hash::read_xdr(r)?,
                ballot: ScpBallot::read_xdr(r)?,
                prepared: Option::<ScpBallot>::read_xdr(r)?,
                prepared_prime: Option::<ScpBallot>::read_xdr(r)?,
                n_c: u32::read_xdr(r)?,
                n_h: u32::read_xdr(r)?,
            })
        })
    }
}

impl ScpStatementPrepare {
    /// Serialize this value as XDR into a [`ConstWriter`] using only const
    /// operations. This is the const implementation underlying `to_xdr`.
    #[cfg(feature = "std")]
    pub const fn const_to_xdr(&self, w: &mut ConstWriter) {
        w.enter_depth();
        self.quorum_set_hash.const_to_xdr(w);
        self.ballot.const_to_xdr(w);
        {
            w.enter_depth();
            match &self.prepared {
                Some(__v0) => {
                    w.write_u32(1);
                    __v0.const_to_xdr(w);
                }
                None => {
                    w.write_u32(0);
                }
            }
            w.leave_depth();
        }
        {
            w.enter_depth();
            match &self.prepared_prime {
                Some(__v0) => {
                    w.write_u32(1);
                    __v0.const_to_xdr(w);
                }
                None => {
                    w.write_u32(0);
                }
            }
            w.leave_depth();
        }
        w.write_u32(self.n_c);
        w.write_u32(self.n_h);
        w.leave_depth();
    }
}

impl WriteXdr for ScpStatementPrepare {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.quorum_set_hash.write_xdr(w)?;
            self.ballot.write_xdr(w)?;
            self.prepared.write_xdr(w)?;
            self.prepared_prime.write_xdr(w)?;
            self.n_c.write_xdr(w)?;
            self.n_h.write_xdr(w)?;
            Ok(())
        })
    }

    #[cfg(feature = "std")]
    fn to_xdr(&self, limits: Limits) -> Result<Vec<u8>, Error> {
        to_xdr_via_const(self, &limits, Self::const_to_xdr)
    }
}
