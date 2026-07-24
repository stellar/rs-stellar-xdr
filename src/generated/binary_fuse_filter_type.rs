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

impl BinaryFuseFilterType {
    /// Serialize this value as XDR into a [`ConstWriter`] using only const
    /// operations. This is the const implementation underlying `to_xdr`.
    #[cfg(feature = "std")]
    pub const fn const_write_xdr(&self, w: &mut ConstWriter) {
        w.enter_depth();
        w.write_i32(*self as i32);
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

impl WriteXdr for BinaryFuseFilterType {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            let i: i32 = (*self).into();
            i.write_xdr(w)
        })
    }

    #[cfg(feature = "std")]
    fn to_xdr(&self, limits: Limits) -> Result<Vec<u8>, Error> {
        to_xdr_via_const(self, &limits, Self::const_write_xdr)
    }
}
