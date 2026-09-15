#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// BucketEntry is a borrowing equivalent of [`BucketEntry`](super::super::BucketEntry)
/// over `'static` data, for const XDR encoding.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[allow(clippy::large_enum_variant)]
pub enum BucketEntry {
    Liveentry(LedgerEntry),
    Initentry(LedgerEntry),
    Deadentry(LedgerKey),
    Metaentry(BucketMetadata),
}

impl BucketEntry {
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
}

impl BucketEntry {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_bucket_entry(self);
        w.len()
    }

    /// Serialize this value as XDR into a fixed-size `[u8; N]` using only const
    /// operations. This is the const counterpart to
    /// [`WriteXdr::to_xdr`](super::super::WriteXdr::to_xdr).
    ///
    /// `N` must equal [`Self::const_xdr_len`]. It is intended for callers, such
    /// as a proc-macro, that compute the length with `const_xdr_len` and pass
    /// it as `N`; `const_to_xdr` itself does not need to call `const_xdr_len`.
    ///
    /// # Panics
    ///
    /// Panics if `N` does not equal the value's [`Self::const_xdr_len`].
    #[must_use]
    pub const fn const_to_xdr<const N: usize>(&self) -> [u8; N] {
        let mut buf = [0u8; N];
        let mut w = ConstWriter::new(&mut buf);
        w.write_type_bucket_entry(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

impl ConstWriter<'_> {
    /// Serializes a [`BucketEntry`], mirroring `<BucketEntry as WriteXdr>::write_xdr`.
    pub const fn write_type_bucket_entry(&mut self, v: &BucketEntry) {
        let d = v.discriminant();
        self.write_type_bucket_entry_type(&d);
        #[allow(clippy::match_same_arms)]
        match v {
            BucketEntry::Liveentry(value) => {
                self.write_type_ledger_entry(value);
            }
            BucketEntry::Initentry(value) => {
                self.write_type_ledger_entry(value);
            }
            BucketEntry::Deadentry(value) => {
                self.write_type_ledger_key(value);
            }
            BucketEntry::Metaentry(value) => {
                self.write_type_bucket_metadata(value);
            }
        }
    }
}
