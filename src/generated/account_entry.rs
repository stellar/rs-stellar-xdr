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

/// AccountEntryRef is a borrowing equivalent of [`AccountEntry`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct AccountEntryRef<'a> {
    pub account_id: AccountId,
    pub balance: i64,
    pub seq_num: SequenceNumber,
    pub num_sub_entries: u32,
    pub inflation_dest: Option<AccountId>,
    pub flags: u32,
    pub home_domain: String32Ref<'a>,
    pub thresholds: Thresholds,
    pub signers: VecMRef<'a, SignerRef<'a>, MAX_SIGNERS>,
    pub ext: AccountEntryExtRef<'a>,
}

#[cfg(feature = "alloc")]
impl IntoOwned for AccountEntryRef<'_> {
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
impl From<&AccountEntryRef<'_>> for AccountEntry {
    #[must_use]
    fn from(v: &AccountEntryRef<'_>) -> Self {
        v.into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<AccountEntryRef<'_>> for AccountEntry {
    #[must_use]
    fn from(v: AccountEntryRef<'_>) -> Self {
        v.into_owned()
    }
}

impl WriteXdr for AccountEntryRef<'_> {
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

#[cfg(feature = "const")]
impl AccountEntryRef<'_> {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_account_entry(self);
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
        w.write_type_account_entry(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`AccountEntry`], mirroring `<AccountEntry as WriteXdr>::write_xdr`.
    pub const fn write_type_account_entry(&mut self, v: &AccountEntryRef<'_>) {
        self.write_type_account_id(&v.account_id);
        self.write_i64(v.balance);
        self.write_type_sequence_number(&v.seq_num);
        self.write_u32(v.num_sub_entries);
        self.write_type_option_account_id(&v.inflation_dest);
        self.write_u32(v.flags);
        self.write_type_string32(&v.home_domain);
        self.write_type_thresholds(&v.thresholds);
        self.write_type_vec_signer(&v.signers);
        self.write_type_account_entry_ext(&v.ext);
    }
}
