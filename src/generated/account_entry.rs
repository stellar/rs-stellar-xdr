#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// AccountEntry is an XDR Struct defined as:
///
/// ```text
/// struct AccountEntry
/// {
///     AccountID accountID;      // master public key for this account
///     int64 balance;            // in stroops
///     SequenceNumber seqNum;    // last sequence number used for this account
///     uint32 numSubEntries;     // number of sub-entries this account has
///                               // drives the reserve
///     AccountID* inflationDest; // Account to vote for during inflation
///     uint32 flags;             // see AccountFlags
///
///     string32 homeDomain; // can be used for reverse federation and memo lookup
///
///     // fields used for signatures
///     // thresholds stores unsigned bytes: [weight of master|low|medium|high]
///     Thresholds thresholds;
///
///     Signer signers<MAX_SIGNERS>; // possible signers for this account
///
///     // reserved for future use
///     union switch (int v)
///     {
///     case 0:
///         void;
///     case 1:
///         AccountEntryExtensionV1 v1;
///     }
///     ext;
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
pub struct AccountEntry {
    pub account_id: AccountId,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub balance: i64,
    pub seq_num: SequenceNumber,
    pub num_sub_entries: u32,
    pub inflation_dest: Option<AccountId>,
    pub flags: u32,
    pub home_domain: String32,
    pub thresholds: Thresholds,
    pub signers: VecM<Signer, MAX_SIGNERS>,
    pub ext: AccountEntryExt,
}

impl ReadXdr for AccountEntry {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                account_id: AccountId::read_xdr(r)?,
                balance: i64::read_xdr(r)?,
                seq_num: SequenceNumber::read_xdr(r)?,
                num_sub_entries: u32::read_xdr(r)?,
                inflation_dest: Option::<AccountId>::read_xdr(r)?,
                flags: u32::read_xdr(r)?,
                home_domain: String32::read_xdr(r)?,
                thresholds: Thresholds::read_xdr(r)?,
                signers: VecM::<Signer, MAX_SIGNERS>::read_xdr(r)?,
                ext: AccountEntryExt::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for AccountEntry {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.account_id.write_xdr(w)?;
            self.balance.write_xdr(w)?;
            self.seq_num.write_xdr(w)?;
            self.num_sub_entries.write_xdr(w)?;
            self.inflation_dest.write_xdr(w)?;
            self.flags.write_xdr(w)?;
            self.home_domain.write_xdr(w)?;
            self.thresholds.write_xdr(w)?;
            self.signers.write_xdr(w)?;
            self.ext.write_xdr(w)?;
            Ok(())
        })
    }
}

/// AccountEntryView is a borrowing equivalent of [`AccountEntry`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct AccountEntryView<'a> {
    pub account_id: AccountId,
    pub balance: i64,
    pub seq_num: SequenceNumber,
    pub num_sub_entries: u32,
    pub inflation_dest: Option<AccountId>,
    pub flags: u32,
    pub home_domain: String32View<'a>,
    pub thresholds: Thresholds,
    pub signers: VecMView<'a, SignerView<'a>, MAX_SIGNERS>,
    pub ext: AccountEntryExtView<'a>,
}

#[cfg(feature = "alloc")]
impl IntoOwned for AccountEntryView<'_> {
    type Owned = AccountEntry;
    fn into_owned(self) -> AccountEntry {
        AccountEntry {
            account_id: self.account_id.into_owned(),
            balance: self.balance.into_owned(),
            seq_num: self.seq_num.into_owned(),
            num_sub_entries: self.num_sub_entries.into_owned(),
            inflation_dest: self.inflation_dest.into_owned(),
            flags: self.flags.into_owned(),
            home_domain: self.home_domain.into_owned(),
            thresholds: self.thresholds.into_owned(),
            signers: self.signers.into_owned(),
            ext: self.ext.into_owned(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&AccountEntryView<'_>> for AccountEntry {
    #[must_use]
    fn from(v: &AccountEntryView<'_>) -> Self {
        v.clone().into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<AccountEntryView<'_>> for AccountEntry {
    #[must_use]
    fn from(v: AccountEntryView<'_>) -> Self {
        v.into_owned()
    }
}

impl WriteXdr for AccountEntryView<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.account_id.write_xdr(w)?;
            self.balance.write_xdr(w)?;
            self.seq_num.write_xdr(w)?;
            self.num_sub_entries.write_xdr(w)?;
            self.inflation_dest.write_xdr(w)?;
            self.flags.write_xdr(w)?;
            self.home_domain.write_xdr(w)?;
            self.thresholds.write_xdr(w)?;
            self.signers.write_xdr(w)?;
            self.ext.write_xdr(w)?;
            Ok(())
        })
    }
}
