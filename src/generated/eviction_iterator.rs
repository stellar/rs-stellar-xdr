#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// EvictionIterator is an XDR Struct defined as:
///
/// ```text
/// struct EvictionIterator {
///     uint32 bucketListLevel;
///     bool isCurrBucket;
///     uint64 bucketFileOffset;
/// };
/// ```
///
#[cfg_attr(feature = "alloc", derive(Default))]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    serde_with::serde_as,
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct EvictionIterator {
    pub bucket_list_level: u32,
    pub is_curr_bucket: bool,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub bucket_file_offset: u64,
}

impl ReadXdr for EvictionIterator {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                bucket_list_level: u32::read_xdr(r)?,
                is_curr_bucket: bool::read_xdr(r)?,
                bucket_file_offset: u64::read_xdr(r)?,
            })
        })
    }
}

impl EvictionIterator {
    /// Serialize this value as XDR into a [`ConstWriter`] using only const
    /// operations. This is the const implementation underlying `to_xdr`.
    #[cfg(feature = "std")]
    pub const fn const_to_xdr(&self, w: &mut ConstWriter) {
        w.enter_depth();
        w.write_u32(self.bucket_list_level);
        w.write_bool(self.is_curr_bucket);
        w.write_u64(self.bucket_file_offset);
        w.leave_depth();
    }
}

impl WriteXdr for EvictionIterator {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.bucket_list_level.write_xdr(w)?;
            self.is_curr_bucket.write_xdr(w)?;
            self.bucket_file_offset.write_xdr(w)?;
            Ok(())
        })
    }

    #[cfg(feature = "std")]
    fn to_xdr(&self, limits: Limits) -> Result<Vec<u8>, Error> {
        to_xdr_via_const(self, &limits, Self::const_to_xdr)
    }
}
