#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
/// BucketEntryType is an XDR Enum defined as:
///
/// ```text
/// enum BucketEntryType
/// {
///     METAENTRY =
///         -1, // At-and-after protocol 11: bucket metadata, should come first.
///     LIVEENTRY = 0, // Before protocol 11: created-or-updated;
///                    // At-and-after protocol 11: only updated.
///     DEADENTRY = 1,
///     INITENTRY = 2 // At-and-after protocol 11: only created.
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
pub enum BucketEntryType {
    #[cfg_attr(feature = "alloc", default)]
    Metaentry = -1,
    Liveentry = 0,
    Deadentry = 1,
    Initentry = 2,
}

impl BucketEntryType {
    const _VARIANTS: &[BucketEntryType] = &[
        BucketEntryType::Metaentry,
        BucketEntryType::Liveentry,
        BucketEntryType::Deadentry,
        BucketEntryType::Initentry,
    ];
    pub const VARIANTS: [BucketEntryType; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &["Metaentry", "Liveentry", "Deadentry", "Initentry"];
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
            Self::Metaentry => "Metaentry",
            Self::Liveentry => "Liveentry",
            Self::Deadentry => "Deadentry",
            Self::Initentry => "Initentry",
        }
    }

    #[must_use]
    pub const fn variants() -> [BucketEntryType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for BucketEntryType {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Variants<BucketEntryType> for BucketEntryType {
    fn variants() -> slice::Iter<'static, BucketEntryType> {
        Self::VARIANTS.iter()
    }
}

impl Enum for BucketEntryType {}

impl fmt::Display for BucketEntryType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for BucketEntryType {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self, Error> {
        let e = match i {
            -1 => BucketEntryType::Metaentry,
            0 => BucketEntryType::Liveentry,
            1 => BucketEntryType::Deadentry,
            2 => BucketEntryType::Initentry,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<BucketEntryType> for i32 {
    #[must_use]
    fn from(e: BucketEntryType) -> Self {
        e as Self
    }
}

impl ReadXdr for BucketEntryType {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let e = i32::read_xdr(r)?;
            let v: Self = e.try_into()?;
            Ok(v)
        })
    }
}

impl WriteXdr for BucketEntryType {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            let i: i32 = (*self).into();
            i.write_xdr(w)
        })
    }
}
