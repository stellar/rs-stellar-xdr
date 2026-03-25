#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
/// AccountMergeResult is an XDR Union defined as:
///
/// ```text
/// union AccountMergeResult switch (AccountMergeResultCode code)
/// {
/// case ACCOUNT_MERGE_SUCCESS:
///     int64 sourceAccountBalance; // how much got transferred from source account
/// case ACCOUNT_MERGE_MALFORMED:
/// case ACCOUNT_MERGE_NO_ACCOUNT:
/// case ACCOUNT_MERGE_IMMUTABLE_SET:
/// case ACCOUNT_MERGE_HAS_SUB_ENTRIES:
/// case ACCOUNT_MERGE_SEQNUM_TOO_FAR:
/// case ACCOUNT_MERGE_DEST_FULL:
/// case ACCOUNT_MERGE_IS_SPONSOR:
///     void;
/// };
/// ```
///
// union with discriminant AccountMergeResultCode
#[cfg_eval::cfg_eval]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    serde_with::serde_as,
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[allow(clippy::large_enum_variant)]
pub enum AccountMergeResult {
    Success(
        #[cfg_attr(
            all(feature = "serde", feature = "alloc"),
            serde_as(as = "NumberOrString")
        )]
        i64,
    ),
    Malformed,
    NoAccount,
    ImmutableSet,
    HasSubEntries,
    SeqnumTooFar,
    DestFull,
    IsSponsor,
}

#[cfg(feature = "alloc")]
impl Default for AccountMergeResult {
    fn default() -> Self {
        Self::Success(i64::default())
    }
}

impl AccountMergeResult {
    const _VARIANTS: &[AccountMergeResultCode] = &[
        AccountMergeResultCode::Success,
        AccountMergeResultCode::Malformed,
        AccountMergeResultCode::NoAccount,
        AccountMergeResultCode::ImmutableSet,
        AccountMergeResultCode::HasSubEntries,
        AccountMergeResultCode::SeqnumTooFar,
        AccountMergeResultCode::DestFull,
        AccountMergeResultCode::IsSponsor,
    ];
    pub const VARIANTS: [AccountMergeResultCode; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &[
        "Success",
        "Malformed",
        "NoAccount",
        "ImmutableSet",
        "HasSubEntries",
        "SeqnumTooFar",
        "DestFull",
        "IsSponsor",
    ];
    pub const VARIANTS_STR: [&'static str; Self::_VARIANTS_STR.len()] = {
        let mut arr = [Self::_VARIANTS_STR[0]; Self::_VARIANTS_STR.len()];
        let mut i = 1;
        while i < Self::_VARIANTS_STR.len() {
            arr[i] = Self::_VARIANTS_STR[i];
            i += 1;
        }
        arr
    };

    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Success(_) => "Success",
            Self::Malformed => "Malformed",
            Self::NoAccount => "NoAccount",
            Self::ImmutableSet => "ImmutableSet",
            Self::HasSubEntries => "HasSubEntries",
            Self::SeqnumTooFar => "SeqnumTooFar",
            Self::DestFull => "DestFull",
            Self::IsSponsor => "IsSponsor",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> AccountMergeResultCode {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Success(_) => AccountMergeResultCode::Success,
            Self::Malformed => AccountMergeResultCode::Malformed,
            Self::NoAccount => AccountMergeResultCode::NoAccount,
            Self::ImmutableSet => AccountMergeResultCode::ImmutableSet,
            Self::HasSubEntries => AccountMergeResultCode::HasSubEntries,
            Self::SeqnumTooFar => AccountMergeResultCode::SeqnumTooFar,
            Self::DestFull => AccountMergeResultCode::DestFull,
            Self::IsSponsor => AccountMergeResultCode::IsSponsor,
        }
    }

    #[must_use]
    pub const fn variants() -> [AccountMergeResultCode; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for AccountMergeResult {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<AccountMergeResultCode> for AccountMergeResult {
    #[must_use]
    fn discriminant(&self) -> AccountMergeResultCode {
        Self::discriminant(self)
    }
}

impl Variants<AccountMergeResultCode> for AccountMergeResult {
    fn variants() -> slice::Iter<'static, AccountMergeResultCode> {
        Self::VARIANTS.iter()
    }
}

impl Union<AccountMergeResultCode> for AccountMergeResult {}

impl ReadXdr for AccountMergeResult {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: AccountMergeResultCode = <AccountMergeResultCode as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                AccountMergeResultCode::Success => Self::Success(i64::read_xdr(r)?),
                AccountMergeResultCode::Malformed => Self::Malformed,
                AccountMergeResultCode::NoAccount => Self::NoAccount,
                AccountMergeResultCode::ImmutableSet => Self::ImmutableSet,
                AccountMergeResultCode::HasSubEntries => Self::HasSubEntries,
                AccountMergeResultCode::SeqnumTooFar => Self::SeqnumTooFar,
                AccountMergeResultCode::DestFull => Self::DestFull,
                AccountMergeResultCode::IsSponsor => Self::IsSponsor,
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for AccountMergeResult {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::Success(v) => v.write_xdr(w)?,
                Self::Malformed => ().write_xdr(w)?,
                Self::NoAccount => ().write_xdr(w)?,
                Self::ImmutableSet => ().write_xdr(w)?,
                Self::HasSubEntries => ().write_xdr(w)?,
                Self::SeqnumTooFar => ().write_xdr(w)?,
                Self::DestFull => ().write_xdr(w)?,
                Self::IsSponsor => ().write_xdr(w)?,
            };
            Ok(())
        })
    }
}
