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
    pub signers: VecM<Signer, 20>,
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
                signers: VecM::<Signer, 20>::read_xdr(r)?,
                ext: AccountEntryExt::read_xdr(r)?,
            })
        })
    }
}

impl AccountEntry {
    /// Serialize this value as XDR into a [`ConstWriter`] using only const
    /// operations. This is the const implementation underlying `to_xdr`.
    #[cfg(feature = "std")]
    pub const fn const_write_xdr(&self, w: &mut ConstWriter) {
        w.enter_depth();
        self.account_id.const_write_xdr(w);
        w.write_i64(self.balance);
        self.seq_num.const_write_xdr(w);
        w.write_u32(self.num_sub_entries);
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
        w.write_u32(self.flags);
        self.home_domain.const_write_xdr(w);
        self.thresholds.const_write_xdr(w);
        {
            w.enter_depth();
            let __s0 = self.signers.0.as_slice();
            let __len0 = __s0.len();
            w.write_length_prefix(__len0);
            let mut __i0 = 0usize;
            while __i0 < __len0 {
                __s0[__i0].const_write_xdr(w);
                __i0 += 1;
            }
            w.leave_depth();
        }
        self.ext.const_write_xdr(w);
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

    #[cfg(feature = "std")]
    fn to_xdr(&self, limits: Limits) -> Result<Vec<u8>, Error> {
        to_xdr_via_const(self, &limits, Self::const_write_xdr)
    }
}
