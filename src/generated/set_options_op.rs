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

/// SetOptionsOpView is a borrowing equivalent of [`SetOptionsOp`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SetOptionsOpView<'a> {
    pub inflation_dest: Option<AccountId>,
    pub clear_flags: Option<u32>,
    pub set_flags: Option<u32>,
    pub master_weight: Option<u32>,
    pub low_threshold: Option<u32>,
    pub med_threshold: Option<u32>,
    pub high_threshold: Option<u32>,
    pub home_domain: Option<String32View<'a>>,
    pub signer: Option<SignerView<'a>>,
}

#[cfg(feature = "alloc")]
impl IntoOwned for SetOptionsOpView<'_> {
    type Owned = SetOptionsOp;
    fn into_owned(&self) -> SetOptionsOp {
        SetOptionsOp {
            inflation_dest: IntoOwned::into_owned(&self.inflation_dest),
            clear_flags: IntoOwned::into_owned(&self.clear_flags),
            set_flags: IntoOwned::into_owned(&self.set_flags),
            master_weight: IntoOwned::into_owned(&self.master_weight),
            low_threshold: IntoOwned::into_owned(&self.low_threshold),
            med_threshold: IntoOwned::into_owned(&self.med_threshold),
            high_threshold: IntoOwned::into_owned(&self.high_threshold),
            home_domain: IntoOwned::into_owned(&self.home_domain),
            signer: IntoOwned::into_owned(&self.signer),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&SetOptionsOpView<'_>> for SetOptionsOp {
    #[must_use]
    fn from(v: &SetOptionsOpView<'_>) -> Self {
        IntoOwned::into_owned(v)
    }
}

#[cfg(feature = "alloc")]
impl From<SetOptionsOpView<'_>> for SetOptionsOp {
    #[must_use]
    fn from(v: SetOptionsOpView<'_>) -> Self {
        IntoOwned::into_owned(&v)
    }
}

impl WriteXdr for SetOptionsOpView<'_> {
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
