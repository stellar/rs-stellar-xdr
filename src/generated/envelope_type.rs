#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// EnvelopeType is an XDR Enum defined as:
///
/// ```text
/// enum EnvelopeType
/// {
///     ENVELOPE_TYPE_TX_V0 = 0,
///     ENVELOPE_TYPE_SCP = 1,
///     ENVELOPE_TYPE_TX = 2,
///     ENVELOPE_TYPE_AUTH = 3,
///     ENVELOPE_TYPE_SCPVALUE = 4,
///     ENVELOPE_TYPE_TX_FEE_BUMP = 5,
///     ENVELOPE_TYPE_OP_ID = 6,
///     ENVELOPE_TYPE_POOL_REVOKE_OP_ID = 7,
///     ENVELOPE_TYPE_CONTRACT_ID = 8,
///     ENVELOPE_TYPE_SOROBAN_AUTHORIZATION = 9,
///     ENVELOPE_TYPE_SOROBAN_AUTHORIZATION_WITH_ADDRESS = 10
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
pub enum EnvelopeType {
    #[cfg_attr(feature = "alloc", default)]
    TxV0 = 0,
    Scp = 1,
    Tx = 2,
    Auth = 3,
    Scpvalue = 4,
    TxFeeBump = 5,
    OpId = 6,
    PoolRevokeOpId = 7,
    ContractId = 8,
    SorobanAuthorization = 9,
    SorobanAuthorizationWithAddress = 10,
}

impl EnvelopeType {
    const _VARIANTS: &[EnvelopeType] = &[
        EnvelopeType::TxV0,
        EnvelopeType::Scp,
        EnvelopeType::Tx,
        EnvelopeType::Auth,
        EnvelopeType::Scpvalue,
        EnvelopeType::TxFeeBump,
        EnvelopeType::OpId,
        EnvelopeType::PoolRevokeOpId,
        EnvelopeType::ContractId,
        EnvelopeType::SorobanAuthorization,
        EnvelopeType::SorobanAuthorizationWithAddress,
    ];
    pub const VARIANTS: [EnvelopeType; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &[
        "TxV0",
        "Scp",
        "Tx",
        "Auth",
        "Scpvalue",
        "TxFeeBump",
        "OpId",
        "PoolRevokeOpId",
        "ContractId",
        "SorobanAuthorization",
        "SorobanAuthorizationWithAddress",
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
            Self::TxV0 => "TxV0",
            Self::Scp => "Scp",
            Self::Tx => "Tx",
            Self::Auth => "Auth",
            Self::Scpvalue => "Scpvalue",
            Self::TxFeeBump => "TxFeeBump",
            Self::OpId => "OpId",
            Self::PoolRevokeOpId => "PoolRevokeOpId",
            Self::ContractId => "ContractId",
            Self::SorobanAuthorization => "SorobanAuthorization",
            Self::SorobanAuthorizationWithAddress => "SorobanAuthorizationWithAddress",
        }
    }

    #[must_use]
    pub const fn variants() -> [EnvelopeType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for EnvelopeType {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Variants<EnvelopeType> for EnvelopeType {
    fn variants() -> slice::Iter<'static, EnvelopeType> {
        Self::VARIANTS.iter()
    }
}

impl Enum for EnvelopeType {}

impl fmt::Display for EnvelopeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for EnvelopeType {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self, Error> {
        let e = match i {
            0 => EnvelopeType::TxV0,
            1 => EnvelopeType::Scp,
            2 => EnvelopeType::Tx,
            3 => EnvelopeType::Auth,
            4 => EnvelopeType::Scpvalue,
            5 => EnvelopeType::TxFeeBump,
            6 => EnvelopeType::OpId,
            7 => EnvelopeType::PoolRevokeOpId,
            8 => EnvelopeType::ContractId,
            9 => EnvelopeType::SorobanAuthorization,
            10 => EnvelopeType::SorobanAuthorizationWithAddress,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<EnvelopeType> for i32 {
    #[must_use]
    fn from(e: EnvelopeType) -> Self {
        e as Self
    }
}

impl ReadXdr for EnvelopeType {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let e = i32::read_xdr(r)?;
            let v: Self = e.try_into()?;
            Ok(v)
        })
    }
}

impl EnvelopeType {
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

impl WriteXdr for EnvelopeType {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        write_xdr_via_const(self, w, Self::const_write_xdr)
    }
}
