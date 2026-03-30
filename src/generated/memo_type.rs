#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// MemoType is an XDR Enum defined as:
///
/// ```text
/// enum MemoType
/// {
///     MEMO_NONE = 0,
///     MEMO_TEXT = 1,
///     MEMO_ID = 2,
///     MEMO_HASH = 3,
///     MEMO_RETURN = 4
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
pub enum MemoType {
    #[cfg_attr(feature = "alloc", default)]
    None = 0,
    Text = 1,
    Id = 2,
    Hash = 3,
    Return = 4,
}

impl MemoType {
    const _VARIANTS: &[MemoType] = &[
        MemoType::None,
        MemoType::Text,
        MemoType::Id,
        MemoType::Hash,
        MemoType::Return,
    ];
    pub const VARIANTS: [MemoType; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &[
        "None",
        "Text",
        "Id",
        "Hash",
        "Return",
    ];
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
            Self::None => "None",
            Self::Text => "Text",
            Self::Id => "Id",
            Self::Hash => "Hash",
            Self::Return => "Return",
        }
    }

    #[must_use]
    pub const fn variants() -> [MemoType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for MemoType {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Variants<MemoType> for MemoType {
    fn variants() -> slice::Iter<'static, MemoType> {
        Self::VARIANTS.iter()
    }
}

impl Enum for MemoType {}

impl fmt::Display for MemoType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for MemoType {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self, Error> {
        let e = match i {
            0 => MemoType::None,
            1 => MemoType::Text,
            2 => MemoType::Id,
            3 => MemoType::Hash,
            4 => MemoType::Return,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<MemoType> for i32 {
    #[must_use]
    fn from(e: MemoType) -> Self {
        e as Self
    }
}

impl ReadXdr for MemoType {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let e = i32::read_xdr(r)?;
            let v: Self = e.try_into()?;
            Ok(v)
        })
    }
}

impl WriteXdr for MemoType {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            let i: i32 = (*self).into();
            i.write_xdr(w)
        })
    }
}
