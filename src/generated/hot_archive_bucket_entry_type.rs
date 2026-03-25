#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
/// HotArchiveBucketEntryType is an XDR Enum defined as:
///
/// ```text
/// enum HotArchiveBucketEntryType
/// {
///     HOT_ARCHIVE_METAENTRY = -1, // Bucket metadata, should come first.
///     HOT_ARCHIVE_ARCHIVED = 0,   // Entry is Archived
///     HOT_ARCHIVE_LIVE = 1        // Entry was previously HOT_ARCHIVE_ARCHIVED, but
///                                 // has been added back to the live BucketList.
///                                 // Does not need to be persisted.
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
pub enum HotArchiveBucketEntryType {
    #[cfg_attr(feature = "alloc", default)]
    Metaentry = -1,
    Archived = 0,
    Live = 1,
}

impl HotArchiveBucketEntryType {
    const _VARIANTS: &[HotArchiveBucketEntryType] = &[
        HotArchiveBucketEntryType::Metaentry,
        HotArchiveBucketEntryType::Archived,
        HotArchiveBucketEntryType::Live,
    ];
    pub const VARIANTS: [HotArchiveBucketEntryType; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &["Metaentry", "Archived", "Live"];
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
            Self::Archived => "Archived",
            Self::Live => "Live",
        }
    }

    #[must_use]
    pub const fn variants() -> [HotArchiveBucketEntryType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for HotArchiveBucketEntryType {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Variants<HotArchiveBucketEntryType> for HotArchiveBucketEntryType {
    fn variants() -> slice::Iter<'static, HotArchiveBucketEntryType> {
        Self::VARIANTS.iter()
    }
}

impl Enum for HotArchiveBucketEntryType {}

impl fmt::Display for HotArchiveBucketEntryType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for HotArchiveBucketEntryType {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self, Error> {
        let e = match i {
            -1 => HotArchiveBucketEntryType::Metaentry,
            0 => HotArchiveBucketEntryType::Archived,
            1 => HotArchiveBucketEntryType::Live,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<HotArchiveBucketEntryType> for i32 {
    #[must_use]
    fn from(e: HotArchiveBucketEntryType) -> Self {
        e as Self
    }
}

impl ReadXdr for HotArchiveBucketEntryType {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let e = i32::read_xdr(r)?;
            let v: Self = e.try_into()?;
            Ok(v)
        })
    }
}

impl WriteXdr for HotArchiveBucketEntryType {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            let i: i32 = (*self).into();
            i.write_xdr(w)
        })
    }
}
