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

impl WriteXdr for SetOptionsOp {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.inflation_dest.write_xdr(w)?;
            self.clear_flags.write_xdr(w)?;
            self.set_flags.write_xdr(w)?;
            self.master_weight.write_xdr(w)?;
            self.low_threshold.write_xdr(w)?;
            self.med_threshold.write_xdr(w)?;
            self.high_threshold.write_xdr(w)?;
            self.home_domain.write_xdr(w)?;
            self.signer.write_xdr(w)?;
            Ok(())
        })
    }
}

/// SetOptionsOpConst is a borrowing equivalent of [`SetOptionsOp`] over `'static`
/// data, for const XDR encoding.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SetOptionsOpConst {
    pub inflation_dest: Option<AccountId>,
    pub clear_flags: Option<u32>,
    pub set_flags: Option<u32>,
    pub master_weight: Option<u32>,
    pub low_threshold: Option<u32>,
    pub med_threshold: Option<u32>,
    pub high_threshold: Option<u32>,
    pub home_domain: Option<String32Const>,
    pub signer: Option<SignerConst>,
}

#[cfg(feature = "const")]
impl SetOptionsOpConst {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_set_options_op(self);
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
        w.write_type_set_options_op(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`SetOptionsOp`], mirroring `<SetOptionsOp as WriteXdr>::write_xdr`.
    pub const fn write_type_set_options_op(&mut self, v: &SetOptionsOpConst) {
        self.write_type_option_account_id(&v.inflation_dest);
        self.write_option_u32(&v.clear_flags);
        self.write_option_u32(&v.set_flags);
        self.write_option_u32(&v.master_weight);
        self.write_option_u32(&v.low_threshold);
        self.write_option_u32(&v.med_threshold);
        self.write_option_u32(&v.high_threshold);
        self.write_type_option_string32(&v.home_domain);
        self.write_type_option_signer(&v.signer);
    }
}
