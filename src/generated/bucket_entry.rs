#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// BucketEntry is an XDR Union defined as:
///
/// ```text
/// union BucketEntry switch (BucketEntryType type)
/// {
/// case LIVEENTRY:
/// case INITENTRY:
///     LedgerEntry liveEntry;
///
/// case DEADENTRY:
///     LedgerKey deadEntry;
/// case METAENTRY:
///     BucketMetadata metaEntry;
/// };
/// ```
///
// union with discriminant BucketEntryType
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
pub enum BucketEntry {
    Liveentry(LedgerEntry),
    Initentry(LedgerEntry),
    Deadentry(LedgerKey),
    Metaentry(BucketMetadata),
}

#[cfg(feature = "alloc")]
impl Default for BucketEntry {
    fn default() -> Self {
        Self::Liveentry(LedgerEntry::default())
    }
}

impl BucketEntry {
    const _VARIANTS: &[BucketEntryType] = &[
        BucketEntryType::Liveentry,
        BucketEntryType::Initentry,
        BucketEntryType::Deadentry,
        BucketEntryType::Metaentry,
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
    const _VARIANTS_STR: &[&str] = &["Liveentry", "Initentry", "Deadentry", "Metaentry"];
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
            Self::Liveentry(_) => "Liveentry",
            Self::Initentry(_) => "Initentry",
            Self::Deadentry(_) => "Deadentry",
            Self::Metaentry(_) => "Metaentry",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> BucketEntryType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Liveentry(_) => BucketEntryType::Liveentry,
            Self::Initentry(_) => BucketEntryType::Initentry,
            Self::Deadentry(_) => BucketEntryType::Deadentry,
            Self::Metaentry(_) => BucketEntryType::Metaentry,
        }
    }

    #[must_use]
    pub const fn variants() -> [BucketEntryType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for BucketEntry {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<BucketEntryType> for BucketEntry {
    #[must_use]
    fn discriminant(&self) -> BucketEntryType {
        Self::discriminant(self)
    }
}

impl Variants<BucketEntryType> for BucketEntry {
    fn variants() -> slice::Iter<'static, BucketEntryType> {
        Self::VARIANTS.iter()
    }
}

impl Union<BucketEntryType> for BucketEntry {}

impl ReadXdr for BucketEntry {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: BucketEntryType = <BucketEntryType as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                BucketEntryType::Liveentry => Self::Liveentry(LedgerEntry::read_xdr(r)?),
                BucketEntryType::Initentry => Self::Initentry(LedgerEntry::read_xdr(r)?),
                BucketEntryType::Deadentry => Self::Deadentry(LedgerKey::read_xdr(r)?),
                BucketEntryType::Metaentry => Self::Metaentry(BucketMetadata::read_xdr(r)?),
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for BucketEntry {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::Liveentry(v) => v.write_xdr(w)?,
                Self::Initentry(v) => v.write_xdr(w)?,
                Self::Deadentry(v) => v.write_xdr(w)?,
                Self::Metaentry(v) => v.write_xdr(w)?,
            };
            Ok(())
        })
    }
}
