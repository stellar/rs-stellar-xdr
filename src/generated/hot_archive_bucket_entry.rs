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

impl HotArchiveBucketEntry {
    /// Serialize this value as XDR into a [`ConstWriter`] using only const
    /// operations. This is the const implementation underlying `to_xdr`.
    #[cfg(feature = "std")]
    pub const fn const_write_xdr(&self, w: &mut ConstWriter) {
        w.enter_depth();
        let d = self.discriminant();
        d.const_write_xdr(w);
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Archived(v) => {
                v.const_write_xdr(w);
            }
            Self::Live(v) => {
                v.const_write_xdr(w);
            }
            Self::Metaentry(v) => {
                v.const_write_xdr(w);
            }
        }
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

impl WriteXdr for HotArchiveBucketEntry {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        write_xdr_via_const(self, w, Self::const_write_xdr)
    }
}
