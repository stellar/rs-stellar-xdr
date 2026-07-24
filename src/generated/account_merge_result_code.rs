#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// AccountMergeResultCode is an XDR Enum defined as:
///
/// ```text
/// enum AccountMergeResultCode
/// {
///     // codes considered as "success" for the operation
///     ACCOUNT_MERGE_SUCCESS = 0,
///     // codes considered as "failure" for the operation
///     ACCOUNT_MERGE_MALFORMED = -1,       // can't merge onto itself
///     ACCOUNT_MERGE_NO_ACCOUNT = -2,      // destination does not exist
///     ACCOUNT_MERGE_IMMUTABLE_SET = -3,   // source account has AUTH_IMMUTABLE set
///     ACCOUNT_MERGE_HAS_SUB_ENTRIES = -4, // account has trust lines/offers
///     ACCOUNT_MERGE_SEQNUM_TOO_FAR = -5,  // sequence number is over max allowed
///     ACCOUNT_MERGE_DEST_FULL = -6,       // can't add source balance to
///                                         // destination balance
///     ACCOUNT_MERGE_IS_SPONSOR = -7       // can't merge account that is a sponsor
/// };
/// ```
///
// enum
#[cfg_attr(feature = "alloc", derive(Default))]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[repr(i32)]
pub enum AccountMergeResultCode {
    #[cfg_attr(feature = "alloc", default)]
    Success = 0,
    Malformed = -1,
    NoAccount = -2,
    ImmutableSet = -3,
    HasSubEntries = -4,
    SeqnumTooFar = -5,
    DestFull = -6,
    IsSponsor = -7,
}

impl AccountMergeResultCode {
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
            Self::Success => "Success",
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
    pub const fn variants() -> [AccountMergeResultCode; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for AccountMergeResultCode {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Variants<AccountMergeResultCode> for AccountMergeResultCode {
    fn variants() -> slice::Iter<'static, AccountMergeResultCode> {
        Self::VARIANTS.iter()
    }
}

impl Enum for AccountMergeResultCode {}

impl fmt::Display for AccountMergeResultCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for AccountMergeResultCode {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self, Error> {
        let e = match i {
            0 => AccountMergeResultCode::Success,
            -1 => AccountMergeResultCode::Malformed,
            -2 => AccountMergeResultCode::NoAccount,
            -3 => AccountMergeResultCode::ImmutableSet,
            -4 => AccountMergeResultCode::HasSubEntries,
            -5 => AccountMergeResultCode::SeqnumTooFar,
            -6 => AccountMergeResultCode::DestFull,
            -7 => AccountMergeResultCode::IsSponsor,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<AccountMergeResultCode> for i32 {
    #[must_use]
    fn from(e: AccountMergeResultCode) -> Self {
        e as Self
    }
}

impl ReadXdr for AccountMergeResultCode {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let e = i32::read_xdr(r)?;
            let v: Self = e.try_into()?;
            Ok(v)
        })
    }
}

impl AccountMergeResultCode {
    /// Serialize this value as XDR into a [`ConstWriter`] using only const
    /// operations. This is the const counterpart to [`WriteXdr::write_xdr`].
    #[cfg(feature = "const")]
    pub const fn const_write_xdr(&self, w: &mut ConstWriter) {
        w.enter_depth();
        w.write_i32(*self as i32);
        w.leave_depth();
    }
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[cfg(feature = "const")]
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
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
    /// operations. This is the const counterpart to [`WriteXdr::to_xdr`].
    ///
    /// `N` must equal [`Self::const_xdr_len`]. It is intended for callers, such
    /// as a proc-macro, that compute the length with `const_xdr_len` and pass
    /// it as `N`; `const_to_xdr` itself does not need to call `const_xdr_len`.
    ///
    /// # Panics
    ///
    /// Panics if `N` does not equal the value's [`Self::const_xdr_len`].
    #[cfg(feature = "const")]
    #[must_use]
    pub const fn const_to_xdr<const N: usize>(&self) -> [u8; N] {
        let limits = Limits {
            depth: u32::MAX,
            len: usize::MAX,
        };
        let mut buf = [0u8; N];
        let mut w = ConstWriter::new(&mut buf, &limits);
        self.const_write_xdr(&mut w);
        assert!(
            w.position() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

impl WriteXdr for AccountMergeResultCode {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            let i: i32 = (*self).into();
            i.write_xdr(w)
        })
    }
}
