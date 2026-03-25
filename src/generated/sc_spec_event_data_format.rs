#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
/// ScSpecEventDataFormat is an XDR Enum defined as:
///
/// ```text
/// enum SCSpecEventDataFormat
/// {
///     SC_SPEC_EVENT_DATA_FORMAT_SINGLE_VALUE = 0,
///     SC_SPEC_EVENT_DATA_FORMAT_VEC = 1,
///     SC_SPEC_EVENT_DATA_FORMAT_MAP = 2
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
pub enum ScSpecEventDataFormat {
    #[cfg_attr(feature = "alloc", default)]
    SingleValue = 0,
    Vec = 1,
    Map = 2,
}

impl ScSpecEventDataFormat {
    const _VARIANTS: &[ScSpecEventDataFormat] = &[
        ScSpecEventDataFormat::SingleValue,
        ScSpecEventDataFormat::Vec,
        ScSpecEventDataFormat::Map,
    ];
    pub const VARIANTS: [ScSpecEventDataFormat; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &["SingleValue", "Vec", "Map"];
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
            Self::SingleValue => "SingleValue",
            Self::Vec => "Vec",
            Self::Map => "Map",
        }
    }

    #[must_use]
    pub const fn variants() -> [ScSpecEventDataFormat; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for ScSpecEventDataFormat {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Variants<ScSpecEventDataFormat> for ScSpecEventDataFormat {
    fn variants() -> slice::Iter<'static, ScSpecEventDataFormat> {
        Self::VARIANTS.iter()
    }
}

impl Enum for ScSpecEventDataFormat {}

impl fmt::Display for ScSpecEventDataFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for ScSpecEventDataFormat {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self, Error> {
        let e = match i {
            0 => ScSpecEventDataFormat::SingleValue,
            1 => ScSpecEventDataFormat::Vec,
            2 => ScSpecEventDataFormat::Map,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<ScSpecEventDataFormat> for i32 {
    #[must_use]
    fn from(e: ScSpecEventDataFormat) -> Self {
        e as Self
    }
}

impl ReadXdr for ScSpecEventDataFormat {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let e = i32::read_xdr(r)?;
            let v: Self = e.try_into()?;
            Ok(v)
        })
    }
}

impl WriteXdr for ScSpecEventDataFormat {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            let i: i32 = (*self).into();
            i.write_xdr(w)
        })
    }
}
