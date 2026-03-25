#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
/// LedgerHeaderFlags is an XDR Enum defined as:
///
/// ```text
/// enum LedgerHeaderFlags
/// {
///     DISABLE_LIQUIDITY_POOL_TRADING_FLAG = 0x1,
///     DISABLE_LIQUIDITY_POOL_DEPOSIT_FLAG = 0x2,
///     DISABLE_LIQUIDITY_POOL_WITHDRAWAL_FLAG = 0x4
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
pub enum LedgerHeaderFlags {
    #[cfg_attr(feature = "alloc", default)]
    TradingFlag = 1,
    DepositFlag = 2,
    WithdrawalFlag = 4,
}

impl LedgerHeaderFlags {
    const _VARIANTS: &[LedgerHeaderFlags] = &[
        LedgerHeaderFlags::TradingFlag,
        LedgerHeaderFlags::DepositFlag,
        LedgerHeaderFlags::WithdrawalFlag,
    ];
    pub const VARIANTS: [LedgerHeaderFlags; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &["TradingFlag", "DepositFlag", "WithdrawalFlag"];
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
            Self::TradingFlag => "TradingFlag",
            Self::DepositFlag => "DepositFlag",
            Self::WithdrawalFlag => "WithdrawalFlag",
        }
    }

    #[must_use]
    pub const fn variants() -> [LedgerHeaderFlags; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for LedgerHeaderFlags {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Variants<LedgerHeaderFlags> for LedgerHeaderFlags {
    fn variants() -> slice::Iter<'static, LedgerHeaderFlags> {
        Self::VARIANTS.iter()
    }
}

impl Enum for LedgerHeaderFlags {}

impl fmt::Display for LedgerHeaderFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for LedgerHeaderFlags {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self, Error> {
        let e = match i {
            1 => LedgerHeaderFlags::TradingFlag,
            2 => LedgerHeaderFlags::DepositFlag,
            4 => LedgerHeaderFlags::WithdrawalFlag,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<LedgerHeaderFlags> for i32 {
    #[must_use]
    fn from(e: LedgerHeaderFlags) -> Self {
        e as Self
    }
}

impl ReadXdr for LedgerHeaderFlags {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let e = i32::read_xdr(r)?;
            let v: Self = e.try_into()?;
            Ok(v)
        })
    }
}

impl WriteXdr for LedgerHeaderFlags {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            let i: i32 = (*self).into();
            i.write_xdr(w)
        })
    }
}
