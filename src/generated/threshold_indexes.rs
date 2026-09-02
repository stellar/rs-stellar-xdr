#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ThresholdIndexes is an XDR Enum defined as:
///
/// ```text
/// enum ThresholdIndexes
/// {
///     THRESHOLD_MASTER_WEIGHT = 0,
///     THRESHOLD_LOW = 1,
///     THRESHOLD_MED = 2,
///     THRESHOLD_HIGH = 3
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
pub enum ThresholdIndexes {
    #[cfg_attr(feature = "alloc", default)]
    MasterWeight = 0,
    Low = 1,
    Med = 2,
    High = 3,
}

impl ThresholdIndexes {
    const _VARIANTS: &[ThresholdIndexes] = &[
        ThresholdIndexes::MasterWeight,
        ThresholdIndexes::Low,
        ThresholdIndexes::Med,
        ThresholdIndexes::High,
    ];
    pub const VARIANTS: [ThresholdIndexes; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &["MasterWeight", "Low", "Med", "High"];
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
            Self::MasterWeight => "MasterWeight",
            Self::Low => "Low",
            Self::Med => "Med",
            Self::High => "High",
        }
    }

    #[must_use]
    pub const fn variants() -> [ThresholdIndexes; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for ThresholdIndexes {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Variants<ThresholdIndexes> for ThresholdIndexes {
    fn variants() -> slice::Iter<'static, ThresholdIndexes> {
        Self::VARIANTS.iter()
    }
}

impl Enum for ThresholdIndexes {}

impl fmt::Display for ThresholdIndexes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for ThresholdIndexes {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self, Error> {
        let e = match i {
            0 => ThresholdIndexes::MasterWeight,
            1 => ThresholdIndexes::Low,
            2 => ThresholdIndexes::Med,
            3 => ThresholdIndexes::High,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<ThresholdIndexes> for i32 {
    #[must_use]
    fn from(e: ThresholdIndexes) -> Self {
        e as Self
    }
}

impl ReadXdr for ThresholdIndexes {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let e = i32::read_xdr(r)?;
            let v: Self = e.try_into()?;
            Ok(v)
        })
    }
}

impl WriteXdr for ThresholdIndexes {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            let i: i32 = (*self).into();
            i.write_xdr(w)
        })
    }
}

#[cfg(feature = "const")]
impl ThresholdIndexes {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_threshold_indexes(self);
        w.len()
    }

    /// Serialize this value as XDR into a fixed-size `[u8; N]` using only const
    /// operations. This is the const counterpart to [`WriteXdr::to_xdr`].
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
        w.write_type_threshold_indexes(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`ThresholdIndexes`], mirroring `<ThresholdIndexes as WriteXdr>::write_xdr`.
    pub const fn write_type_threshold_indexes(&mut self, v: &ThresholdIndexes) {
        self.write_i32(*v as i32);
    }
}
