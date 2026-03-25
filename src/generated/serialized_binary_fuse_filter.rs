#[allow(unused_imports)]
use super::*;
/// SerializedBinaryFuseFilter is an XDR Struct defined as:
///
/// ```text
/// struct SerializedBinaryFuseFilter
/// {
///     BinaryFuseFilterType type;
///
///     // Seed used to hash input to filter
///     ShortHashSeed inputHashSeed;
///
///     // Seed used for internal filter hash operations
///     ShortHashSeed filterSeed;
///     uint32 segmentLength;
///     uint32 segementLengthMask;
///     uint32 segmentCount;
///     uint32 segmentCountLength;
///     uint32 fingerprintLength; // Length in terms of element count, not bytes
///
///     // Array of uint8_t, uint16_t, or uint32_t depending on filter type
///     opaque fingerprints<>;
/// };
/// ```
///
#[cfg_attr(feature = "alloc", derive(Default))]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_eval::cfg_eval]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    serde_with::serde_as,
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct SerializedBinaryFuseFilter {
    pub type_: BinaryFuseFilterType,
    pub input_hash_seed: ShortHashSeed,
    pub filter_seed: ShortHashSeed,
    pub segment_length: u32,
    pub segement_length_mask: u32,
    pub segment_count: u32,
    pub segment_count_length: u32,
    pub fingerprint_length: u32,
    pub fingerprints: BytesM,
}

impl ReadXdr for SerializedBinaryFuseFilter {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                type_: BinaryFuseFilterType::read_xdr(r)?,
                input_hash_seed: ShortHashSeed::read_xdr(r)?,
                filter_seed: ShortHashSeed::read_xdr(r)?,
                segment_length: u32::read_xdr(r)?,
                segement_length_mask: u32::read_xdr(r)?,
                segment_count: u32::read_xdr(r)?,
                segment_count_length: u32::read_xdr(r)?,
                fingerprint_length: u32::read_xdr(r)?,
                fingerprints: BytesM::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for SerializedBinaryFuseFilter {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.type_.write_xdr(w)?;
            self.input_hash_seed.write_xdr(w)?;
            self.filter_seed.write_xdr(w)?;
            self.segment_length.write_xdr(w)?;
            self.segement_length_mask.write_xdr(w)?;
            self.segment_count.write_xdr(w)?;
            self.segment_count_length.write_xdr(w)?;
            self.fingerprint_length.write_xdr(w)?;
            self.fingerprints.write_xdr(w)?;
            Ok(())
        })
    }
}
