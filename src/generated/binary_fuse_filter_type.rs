#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// BinaryFuseFilterType is an XDR Enum defined as:
///
/// ```text
/// enum BinaryFuseFilterType
/// {
///     BINARY_FUSE_FILTER_8_BIT = 0,
///     BINARY_FUSE_FILTER_16_BIT = 1,
///     BINARY_FUSE_FILTER_32_BIT = 2
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
pub enum BinaryFuseFilterType {
    #[cfg_attr(feature = "alloc", default)]
    B8Bit = 0,
    B16Bit = 1,
    B32Bit = 2,
}

impl BinaryFuseFilterType {
    const _VARIANTS: &[BinaryFuseFilterType] = &[
        BinaryFuseFilterType::B8Bit,
        BinaryFuseFilterType::B16Bit,
        BinaryFuseFilterType::B32Bit,
    ];
    pub const VARIANTS: [BinaryFuseFilterType; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &["B8Bit", "B16Bit", "B32Bit"];
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
            Self::B8Bit => "B8Bit",
            Self::B16Bit => "B16Bit",
            Self::B32Bit => "B32Bit",
        }
    }

    #[must_use]
    pub const fn variants() -> [BinaryFuseFilterType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for BinaryFuseFilterType {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Variants<BinaryFuseFilterType> for BinaryFuseFilterType {
    fn variants() -> slice::Iter<'static, BinaryFuseFilterType> {
        Self::VARIANTS.iter()
    }
}

impl Enum for BinaryFuseFilterType {}

impl fmt::Display for BinaryFuseFilterType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for BinaryFuseFilterType {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self, Error> {
        let e = match i {
            0 => BinaryFuseFilterType::B8Bit,
            1 => BinaryFuseFilterType::B16Bit,
            2 => BinaryFuseFilterType::B32Bit,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<BinaryFuseFilterType> for i32 {
    #[must_use]
    fn from(e: BinaryFuseFilterType) -> Self {
        e as Self
    }
}

impl ReadXdr for BinaryFuseFilterType {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let e = i32::read_xdr(r)?;
            let v: Self = e.try_into()?;
            Ok(v)
        })
    }
}

impl WriteXdr for BinaryFuseFilterType {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            let i: i32 = (*self).into();
            i.write_xdr(w)
        })
    }
}

#[cfg(feature = "alloc")]
impl IntoOwned for BinaryFuseFilterType {
    type Owned = BinaryFuseFilterType;
    fn into_owned(self) -> BinaryFuseFilterType {
        self
    }
}

#[cfg(feature = "const")]
impl BinaryFuseFilterType {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_binary_fuse_filter_type(self);
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
        w.write_type_binary_fuse_filter_type(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`BinaryFuseFilterType`], mirroring `<BinaryFuseFilterType as WriteXdr>::write_xdr`.
    pub const fn write_type_binary_fuse_filter_type(&mut self, v: &BinaryFuseFilterType) {
        self.write_i32(*v as i32);
    }
}
