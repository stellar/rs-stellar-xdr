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

impl ScAddressType {
    /// Serialize this value as XDR into a [`ConstWriter`] using only const
    /// operations. This is the const counterpart to [`WriteXdr::write_xdr`].
    #[cfg(feature = "const")]
    pub const fn const_write_xdr(&self, w: &mut ConstWriter) {
        w.enter_depth();
        w.write_i32(*self as i32);
        w.leave_depth();
    }
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[cfg(feature = "const")]
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
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
    /// operations. This is the const counterpart to [`WriteXdr::to_xdr`].
    ///
    /// `N` must equal [`Self::const_xdr_len`]. It is intended for callers, such
    /// as a proc-macro, that compute the length with `const_xdr_len` and pass
    /// it as `N`; `const_to_xdr` itself does not need to call `const_xdr_len`.
    ///
    /// # Panics
    ///
    /// Panics if `N` does not equal the value's [`Self::const_xdr_len`].
    #[cfg(feature = "const")]
    #[must_use]
    pub const fn const_to_xdr<const N: usize>(&self) -> [u8; N] {
        let limits = Limits {
            depth: u32::MAX,
            len: usize::MAX,
        };
        let mut buf = [0u8; N];
        let mut w = ConstWriter::new(&mut buf, &limits);
        self.const_write_xdr(&mut w);
        assert!(
            w.position() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
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
