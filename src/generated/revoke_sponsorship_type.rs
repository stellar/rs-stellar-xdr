#[allow(unused_imports)]
use super::*;
/// RevokeSponsorshipType is an XDR Enum defined as:
///
/// ```text
/// enum RevokeSponsorshipType
/// {
///     REVOKE_SPONSORSHIP_LEDGER_ENTRY = 0,
///     REVOKE_SPONSORSHIP_SIGNER = 1
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
pub enum RevokeSponsorshipType {
    #[cfg_attr(feature = "alloc", default)]
    LedgerEntry = 0,
    Signer = 1,
}

impl RevokeSponsorshipType {
    const _VARIANTS: &[RevokeSponsorshipType] = &[
        RevokeSponsorshipType::LedgerEntry,
        RevokeSponsorshipType::Signer,
    ];
    pub const VARIANTS: [RevokeSponsorshipType; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &["LedgerEntry", "Signer"];
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
            Self::LedgerEntry => "LedgerEntry",
            Self::Signer => "Signer",
        }
    }

    #[must_use]
    pub const fn variants() -> [RevokeSponsorshipType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for RevokeSponsorshipType {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Variants<RevokeSponsorshipType> for RevokeSponsorshipType {
    fn variants() -> slice::Iter<'static, RevokeSponsorshipType> {
        Self::VARIANTS.iter()
    }
}

impl Enum for RevokeSponsorshipType {}

impl fmt::Display for RevokeSponsorshipType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for RevokeSponsorshipType {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self, Error> {
        let e = match i {
            0 => RevokeSponsorshipType::LedgerEntry,
            1 => RevokeSponsorshipType::Signer,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<RevokeSponsorshipType> for i32 {
    #[must_use]
    fn from(e: RevokeSponsorshipType) -> Self {
        e as Self
    }
}

impl ReadXdr for RevokeSponsorshipType {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let e = i32::read_xdr(r)?;
            let v: Self = e.try_into()?;
            Ok(v)
        })
    }
}

impl WriteXdr for RevokeSponsorshipType {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            let i: i32 = (*self).into();
            i.write_xdr(w)
        })
    }
}
