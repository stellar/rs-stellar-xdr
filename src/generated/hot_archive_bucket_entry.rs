#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// HotArchiveBucketEntry is an XDR Union defined as:
///
/// ```text
/// union HotArchiveBucketEntry switch (HotArchiveBucketEntryType type)
/// {
/// case HOT_ARCHIVE_ARCHIVED:
///     LedgerEntry archivedEntry;
///
/// case HOT_ARCHIVE_LIVE:
///     LedgerKey key;
/// case HOT_ARCHIVE_METAENTRY:
///     BucketMetadata metaEntry;
/// };
/// ```
///
// union with discriminant HotArchiveBucketEntryType
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval)]
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
pub enum HotArchiveBucketEntry {
    Archived(LedgerEntry),
    Live(LedgerKey),
    Metaentry(BucketMetadata),
}

#[cfg(feature = "alloc")]
impl Default for HotArchiveBucketEntry {
    fn default() -> Self {
        Self::Archived(LedgerEntry::default())
    }
}

impl HotArchiveBucketEntry {
    const _VARIANTS: &[HotArchiveBucketEntryType] = &[
        HotArchiveBucketEntryType::Archived,
        HotArchiveBucketEntryType::Live,
        HotArchiveBucketEntryType::Metaentry,
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
    const _VARIANTS_STR: &[&str] = &["Archived", "Live", "Metaentry"];
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
            Self::Archived(_) => "Archived",
            Self::Live(_) => "Live",
            Self::Metaentry(_) => "Metaentry",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> HotArchiveBucketEntryType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Archived(_) => HotArchiveBucketEntryType::Archived,
            Self::Live(_) => HotArchiveBucketEntryType::Live,
            Self::Metaentry(_) => HotArchiveBucketEntryType::Metaentry,
        }
    }

    #[must_use]
    pub const fn variants() -> [HotArchiveBucketEntryType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for HotArchiveBucketEntry {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<HotArchiveBucketEntryType> for HotArchiveBucketEntry {
    #[must_use]
    fn discriminant(&self) -> HotArchiveBucketEntryType {
        Self::discriminant(self)
    }
}

impl Variants<HotArchiveBucketEntryType> for HotArchiveBucketEntry {
    fn variants() -> slice::Iter<'static, HotArchiveBucketEntryType> {
        Self::VARIANTS.iter()
    }
}

impl Union<HotArchiveBucketEntryType> for HotArchiveBucketEntry {}

impl ReadXdr for HotArchiveBucketEntry {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: HotArchiveBucketEntryType =
                <HotArchiveBucketEntryType as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                HotArchiveBucketEntryType::Archived => Self::Archived(LedgerEntry::read_xdr(r)?),
                HotArchiveBucketEntryType::Live => Self::Live(LedgerKey::read_xdr(r)?),
                HotArchiveBucketEntryType::Metaentry => {
                    Self::Metaentry(BucketMetadata::read_xdr(r)?)
                }
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for HotArchiveBucketEntry {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::Archived(v) => v.write_xdr(w)?,
                Self::Live(v) => v.write_xdr(w)?,
                Self::Metaentry(v) => v.write_xdr(w)?,
            };
            Ok(())
        })
    }
}

/// HotArchiveBucketEntryRef is a borrowing equivalent of [`HotArchiveBucketEntry`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(clippy::large_enum_variant)]
pub enum HotArchiveBucketEntryRef<'a> {
    Archived(LedgerEntryRef<'a>),
    Live(LedgerKeyRef<'a>),
    Metaentry(BucketMetadata),
}

#[cfg(feature = "alloc")]
impl From<&HotArchiveBucketEntryRef<'_>> for HotArchiveBucketEntry {
    #[must_use]
    fn from(v: &HotArchiveBucketEntryRef<'_>) -> Self {
        #[allow(clippy::match_same_arms)]
        match v {
            HotArchiveBucketEntryRef::Archived(value) => Self::Archived(value.into()),
            HotArchiveBucketEntryRef::Live(value) => Self::Live(value.into()),
            HotArchiveBucketEntryRef::Metaentry(value) => Self::Metaentry(value.clone()),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<HotArchiveBucketEntryRef<'_>> for HotArchiveBucketEntry {
    #[must_use]
    fn from(v: HotArchiveBucketEntryRef<'_>) -> Self {
        Self::from(&v)
    }
}

impl HotArchiveBucketEntryRef<'_> {
    #[must_use]
    pub const fn discriminant(&self) -> HotArchiveBucketEntryType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Archived(_) => HotArchiveBucketEntryType::Archived,
            Self::Live(_) => HotArchiveBucketEntryType::Live,
            Self::Metaentry(_) => HotArchiveBucketEntryType::Metaentry,
        }
    }
}

impl WriteXdr for HotArchiveBucketEntryRef<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::Archived(v) => v.write_xdr(w)?,
                Self::Live(v) => v.write_xdr(w)?,
                Self::Metaentry(v) => v.write_xdr(w)?,
            };
            Ok(())
        })
    }
}
