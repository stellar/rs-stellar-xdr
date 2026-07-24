#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// SetOptionsOp is an XDR Struct defined as:
///
/// ```text
/// struct SetOptionsOp
/// {
///     AccountID* inflationDest; // sets the inflation destination
///
///     uint32* clearFlags; // which flags to clear
///     uint32* setFlags;   // which flags to set
///
///     // account threshold manipulation
///     uint32* masterWeight; // weight of the master account
///     uint32* lowThreshold;
///     uint32* medThreshold;
///     uint32* highThreshold;
///
///     string32* homeDomain; // sets the home domain
///
///     // Add, update or remove a signer for the account
///     // signer is deleted if the weight is 0
///     Signer* signer;
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
pub struct SetOptionsOp {
    pub inflation_dest: Option<AccountId>,
    pub clear_flags: Option<u32>,
    pub set_flags: Option<u32>,
    pub master_weight: Option<u32>,
    pub low_threshold: Option<u32>,
    pub med_threshold: Option<u32>,
    pub high_threshold: Option<u32>,
    pub home_domain: Option<String32>,
    pub signer: Option<Signer>,
}

impl ReadXdr for SetOptionsOp {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                inflation_dest: Option::<AccountId>::read_xdr(r)?,
                clear_flags: Option::<u32>::read_xdr(r)?,
                set_flags: Option::<u32>::read_xdr(r)?,
                master_weight: Option::<u32>::read_xdr(r)?,
                low_threshold: Option::<u32>::read_xdr(r)?,
                med_threshold: Option::<u32>::read_xdr(r)?,
                high_threshold: Option::<u32>::read_xdr(r)?,
                home_domain: Option::<String32>::read_xdr(r)?,
                signer: Option::<Signer>::read_xdr(r)?,
            })
        })
    }
}

impl SetOptionsOp {
    /// Serialize this value as XDR into a [`ConstWriter`] using only const
    /// operations. This is the const implementation underlying `to_xdr`.
    #[cfg(feature = "std")]
    pub const fn const_write_xdr(&self, w: &mut ConstWriter) {
        w.enter_depth();
        {
            w.enter_depth();
            match &self.inflation_dest {
                Some(__v0) => {
                    w.write_u32(1);
                    __v0.const_write_xdr(w);
                }
                None => {
                    w.write_u32(0);
                }
            }
            w.leave_depth();
        }
        {
            w.enter_depth();
            match &self.clear_flags {
                Some(__v0) => {
                    w.write_u32(1);
                    w.write_u32(*__v0);
                }
                None => {
                    w.write_u32(0);
                }
            }
            w.leave_depth();
        }
        {
            w.enter_depth();
            match &self.set_flags {
                Some(__v0) => {
                    w.write_u32(1);
                    w.write_u32(*__v0);
                }
                None => {
                    w.write_u32(0);
                }
            }
            w.leave_depth();
        }
        {
            w.enter_depth();
            match &self.master_weight {
                Some(__v0) => {
                    w.write_u32(1);
                    w.write_u32(*__v0);
                }
                None => {
                    w.write_u32(0);
                }
            }
            w.leave_depth();
        }
        {
            w.enter_depth();
            match &self.low_threshold {
                Some(__v0) => {
                    w.write_u32(1);
                    w.write_u32(*__v0);
                }
                None => {
                    w.write_u32(0);
                }
            }
            w.leave_depth();
        }
        {
            w.enter_depth();
            match &self.med_threshold {
                Some(__v0) => {
                    w.write_u32(1);
                    w.write_u32(*__v0);
                }
                None => {
                    w.write_u32(0);
                }
            }
            w.leave_depth();
        }
        {
            w.enter_depth();
            match &self.high_threshold {
                Some(__v0) => {
                    w.write_u32(1);
                    w.write_u32(*__v0);
                }
                None => {
                    w.write_u32(0);
                }
            }
            w.leave_depth();
        }
        {
            w.enter_depth();
            match &self.home_domain {
                Some(__v0) => {
                    w.write_u32(1);
                    __v0.const_write_xdr(w);
                }
                None => {
                    w.write_u32(0);
                }
            }
            w.leave_depth();
        }
        {
            w.enter_depth();
            match &self.signer {
                Some(__v0) => {
                    w.write_u32(1);
                    __v0.const_write_xdr(w);
                }
                None => {
                    w.write_u32(0);
                }
            }
            w.leave_depth();
        }
        w.leave_depth();
    }
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::to_xdr_array`] at compile time.
    #[cfg(feature = "std")]
    #[must_use]
    pub const fn xdr_len(&self) -> usize {
        let limits = Limits {
            depth: u32::MAX,
            len: usize::MAX,
        };
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty, &limits);
        self.const_write_xdr(&mut w);
        w.position()
    }

    /// Serialize this value as XDR into a fixed-size `[u8; N]` using only const
    /// operations.
    ///
    /// `N` must equal [`Self::xdr_len`]. It is intended for callers, such as a
    /// proc-macro, that compute the length with `xdr_len` and pass it as `N`;
    /// `to_xdr_array` itself does not need to call `xdr_len`.
    ///
    /// # Panics
    ///
    /// Panics if `N` does not equal the value's [`Self::xdr_len`].
    #[cfg(feature = "std")]
    #[must_use]
    pub const fn to_xdr_array<const N: usize>(&self) -> [u8; N] {
        let limits = Limits {
            depth: u32::MAX,
            len: usize::MAX,
        };
        let mut buf = [0u8; N];
        let mut w = ConstWriter::new(&mut buf, &limits);
        self.const_write_xdr(&mut w);
        assert!(
            w.position() == N,
            "to_xdr_array: N does not equal the XDR-encoded length"
        );
        buf
    }
}

impl WriteXdr for SetOptionsOp {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        write_xdr_via_const(self, w, Self::const_write_xdr)
    }
}
