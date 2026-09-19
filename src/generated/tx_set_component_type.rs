#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// TxSetComponentType is an XDR Enum defined as:
///
/// ```text
/// enum TxSetComponentType
/// {
///   // txs with effective fee <= bid derived from a base fee (if any).
///   // If base fee is not specified, no discount is applied.
///   TXSET_COMP_TXS_MAYBE_DISCOUNTED_FEE = 0
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
pub enum TxSetComponentType {
    #[cfg_attr(feature = "alloc", default)]
    TxsetCompTxsMaybeDiscountedFee = 0,
}

impl TxSetComponentType {
    const _VARIANTS: &[TxSetComponentType] = &[TxSetComponentType::TxsetCompTxsMaybeDiscountedFee];
    pub const VARIANTS: [TxSetComponentType; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &["TxsetCompTxsMaybeDiscountedFee"];
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
            Self::TxsetCompTxsMaybeDiscountedFee => "TxsetCompTxsMaybeDiscountedFee",
        }
    }

    #[must_use]
    pub const fn variants() -> [TxSetComponentType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for TxSetComponentType {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Variants<TxSetComponentType> for TxSetComponentType {
    fn variants() -> slice::Iter<'static, TxSetComponentType> {
        Self::VARIANTS.iter()
    }
}

impl Enum for TxSetComponentType {}

impl fmt::Display for TxSetComponentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for TxSetComponentType {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self, Error> {
        let e = match i {
            0 => TxSetComponentType::TxsetCompTxsMaybeDiscountedFee,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<TxSetComponentType> for i32 {
    #[must_use]
    fn from(e: TxSetComponentType) -> Self {
        e as Self
    }
}

impl ReadXdr for TxSetComponentType {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let e = i32::read_xdr(r)?;
            let v: Self = e.try_into()?;
            Ok(v)
        })
    }
}

impl WriteXdr for TxSetComponentType {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            let i: i32 = (*self).into();
            i.write_xdr(w)
        })
    }
}
