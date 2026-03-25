#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
/// ScAddressType is an XDR Enum defined as:
///
/// ```text
/// enum SCAddressType
/// {
///     SC_ADDRESS_TYPE_ACCOUNT = 0,
///     SC_ADDRESS_TYPE_CONTRACT = 1,
///     SC_ADDRESS_TYPE_MUXED_ACCOUNT = 2,
///     SC_ADDRESS_TYPE_CLAIMABLE_BALANCE = 3,
///     SC_ADDRESS_TYPE_LIQUIDITY_POOL = 4
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
pub enum ScAddressType {
    #[cfg_attr(feature = "alloc", default)]
    Account = 0,
    Contract = 1,
    MuxedAccount = 2,
    ClaimableBalance = 3,
    LiquidityPool = 4,
}

impl ScAddressType {
    const _VARIANTS: &[ScAddressType] = &[
        ScAddressType::Account,
        ScAddressType::Contract,
        ScAddressType::MuxedAccount,
        ScAddressType::ClaimableBalance,
        ScAddressType::LiquidityPool,
    ];
    pub const VARIANTS: [ScAddressType; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &[
        "Account",
        "Contract",
        "MuxedAccount",
        "ClaimableBalance",
        "LiquidityPool",
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
            Self::Account => "Account",
            Self::Contract => "Contract",
            Self::MuxedAccount => "MuxedAccount",
            Self::ClaimableBalance => "ClaimableBalance",
            Self::LiquidityPool => "LiquidityPool",
        }
    }

    #[must_use]
    pub const fn variants() -> [ScAddressType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for ScAddressType {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Variants<ScAddressType> for ScAddressType {
    fn variants() -> slice::Iter<'static, ScAddressType> {
        Self::VARIANTS.iter()
    }
}

impl Enum for ScAddressType {}

impl fmt::Display for ScAddressType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for ScAddressType {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self, Error> {
        let e = match i {
            0 => ScAddressType::Account,
            1 => ScAddressType::Contract,
            2 => ScAddressType::MuxedAccount,
            3 => ScAddressType::ClaimableBalance,
            4 => ScAddressType::LiquidityPool,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<ScAddressType> for i32 {
    #[must_use]
    fn from(e: ScAddressType) -> Self {
        e as Self
    }
}

impl ReadXdr for ScAddressType {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let e = i32::read_xdr(r)?;
            let v: Self = e.try_into()?;
            Ok(v)
        })
    }
}

impl WriteXdr for ScAddressType {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            let i: i32 = (*self).into();
            i.write_xdr(w)
        })
    }
}
