#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
/// ClaimAtomType is an XDR Enum defined as:
///
/// ```text
/// enum ClaimAtomType
/// {
///     CLAIM_ATOM_TYPE_V0 = 0,
///     CLAIM_ATOM_TYPE_ORDER_BOOK = 1,
///     CLAIM_ATOM_TYPE_LIQUIDITY_POOL = 2
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
pub enum ClaimAtomType {
    #[cfg_attr(feature = "alloc", default)]
    V0 = 0,
    OrderBook = 1,
    LiquidityPool = 2,
}

impl ClaimAtomType {
    const _VARIANTS: &[ClaimAtomType] = &[
        ClaimAtomType::V0,
        ClaimAtomType::OrderBook,
        ClaimAtomType::LiquidityPool,
    ];
    pub const VARIANTS: [ClaimAtomType; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &["V0", "OrderBook", "LiquidityPool"];
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
            Self::V0 => "V0",
            Self::OrderBook => "OrderBook",
            Self::LiquidityPool => "LiquidityPool",
        }
    }

    #[must_use]
    pub const fn variants() -> [ClaimAtomType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for ClaimAtomType {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Variants<ClaimAtomType> for ClaimAtomType {
    fn variants() -> slice::Iter<'static, ClaimAtomType> {
        Self::VARIANTS.iter()
    }
}

impl Enum for ClaimAtomType {}

impl fmt::Display for ClaimAtomType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for ClaimAtomType {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self, Error> {
        let e = match i {
            0 => ClaimAtomType::V0,
            1 => ClaimAtomType::OrderBook,
            2 => ClaimAtomType::LiquidityPool,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<ClaimAtomType> for i32 {
    #[must_use]
    fn from(e: ClaimAtomType) -> Self {
        e as Self
    }
}

impl ReadXdr for ClaimAtomType {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let e = i32::read_xdr(r)?;
            let v: Self = e.try_into()?;
            Ok(v)
        })
    }
}

impl WriteXdr for ClaimAtomType {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            let i: i32 = (*self).into();
            i.write_xdr(w)
        })
    }
}
