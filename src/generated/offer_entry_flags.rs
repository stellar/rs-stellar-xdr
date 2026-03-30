#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// OfferEntryFlags is an XDR Enum defined as:
///
/// ```text
/// enum OfferEntryFlags
/// {
///     // an offer with this flag will not act on and take a reverse offer of equal
///     // price
///     PASSIVE_FLAG = 1
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
pub enum OfferEntryFlags {
    #[cfg_attr(feature = "alloc", default)]
    PassiveFlag = 1,
}

impl OfferEntryFlags {
    const _VARIANTS: &[OfferEntryFlags] = &[OfferEntryFlags::PassiveFlag];
    pub const VARIANTS: [OfferEntryFlags; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &["PassiveFlag"];
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
            Self::PassiveFlag => "PassiveFlag",
        }
    }

    #[must_use]
    pub const fn variants() -> [OfferEntryFlags; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for OfferEntryFlags {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Variants<OfferEntryFlags> for OfferEntryFlags {
    fn variants() -> slice::Iter<'static, OfferEntryFlags> {
        Self::VARIANTS.iter()
    }
}

impl Enum for OfferEntryFlags {}

impl fmt::Display for OfferEntryFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for OfferEntryFlags {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self, Error> {
        let e = match i {
            1 => OfferEntryFlags::PassiveFlag,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<OfferEntryFlags> for i32 {
    #[must_use]
    fn from(e: OfferEntryFlags) -> Self {
        e as Self
    }
}

impl ReadXdr for OfferEntryFlags {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let e = i32::read_xdr(r)?;
            let v: Self = e.try_into()?;
            Ok(v)
        })
    }
}

impl WriteXdr for OfferEntryFlags {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            let i: i32 = (*self).into();
            i.write_xdr(w)
        })
    }
}
