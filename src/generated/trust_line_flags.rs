#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
/// TrustLineFlags is an XDR Enum defined as:
///
/// ```text
/// enum TrustLineFlags
/// {
///     // issuer has authorized account to perform transactions with its credit
///     AUTHORIZED_FLAG = 1,
///     // issuer has authorized account to maintain and reduce liabilities for its
///     // credit
///     AUTHORIZED_TO_MAINTAIN_LIABILITIES_FLAG = 2,
///     // issuer has specified that it may clawback its credit, and that claimable
///     // balances created with its credit may also be clawed back
///     TRUSTLINE_CLAWBACK_ENABLED_FLAG = 4
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
pub enum TrustLineFlags {
    #[cfg_attr(feature = "alloc", default)]
    AuthorizedFlag = 1,
    AuthorizedToMaintainLiabilitiesFlag = 2,
    TrustlineClawbackEnabledFlag = 4,
}

impl TrustLineFlags {
    const _VARIANTS: &[TrustLineFlags] = &[
        TrustLineFlags::AuthorizedFlag,
        TrustLineFlags::AuthorizedToMaintainLiabilitiesFlag,
        TrustLineFlags::TrustlineClawbackEnabledFlag,
    ];
    pub const VARIANTS: [TrustLineFlags; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &[
        "AuthorizedFlag",
        "AuthorizedToMaintainLiabilitiesFlag",
        "TrustlineClawbackEnabledFlag",
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
            Self::AuthorizedFlag => "AuthorizedFlag",
            Self::AuthorizedToMaintainLiabilitiesFlag => "AuthorizedToMaintainLiabilitiesFlag",
            Self::TrustlineClawbackEnabledFlag => "TrustlineClawbackEnabledFlag",
        }
    }

    #[must_use]
    pub const fn variants() -> [TrustLineFlags; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for TrustLineFlags {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Variants<TrustLineFlags> for TrustLineFlags {
    fn variants() -> slice::Iter<'static, TrustLineFlags> {
        Self::VARIANTS.iter()
    }
}

impl Enum for TrustLineFlags {}

impl fmt::Display for TrustLineFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for TrustLineFlags {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self, Error> {
        let e = match i {
            1 => TrustLineFlags::AuthorizedFlag,
            2 => TrustLineFlags::AuthorizedToMaintainLiabilitiesFlag,
            4 => TrustLineFlags::TrustlineClawbackEnabledFlag,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<TrustLineFlags> for i32 {
    #[must_use]
    fn from(e: TrustLineFlags) -> Self {
        e as Self
    }
}

impl ReadXdr for TrustLineFlags {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let e = i32::read_xdr(r)?;
            let v: Self = e.try_into()?;
            Ok(v)
        })
    }
}

impl WriteXdr for TrustLineFlags {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            let i: i32 = (*self).into();
            i.write_xdr(w)
        })
    }
}
