#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// CreateAccountResultCode is an XDR Enum defined as:
///
/// ```text
/// enum CreateAccountResultCode
/// {
///     // codes considered as "success" for the operation
///     CREATE_ACCOUNT_SUCCESS = 0, // account was created
/// 
///     // codes considered as "failure" for the operation
///     CREATE_ACCOUNT_MALFORMED = -1,   // invalid destination
///     CREATE_ACCOUNT_UNDERFUNDED = -2, // not enough funds in source account
///     CREATE_ACCOUNT_LOW_RESERVE =
///         -3, // would create an account below the min reserve
///     CREATE_ACCOUNT_ALREADY_EXIST = -4 // account already exists
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
pub enum CreateAccountResultCode {
    #[cfg_attr(feature = "alloc", default)]
    Success = 0,
    Malformed = -1,
    Underfunded = -2,
    LowReserve = -3,
    AlreadyExist = -4,
}

impl CreateAccountResultCode {
    const _VARIANTS: &[CreateAccountResultCode] = &[
        CreateAccountResultCode::Success,
        CreateAccountResultCode::Malformed,
        CreateAccountResultCode::Underfunded,
        CreateAccountResultCode::LowReserve,
        CreateAccountResultCode::AlreadyExist,
    ];
    pub const VARIANTS: [CreateAccountResultCode; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &[
        "Success",
        "Malformed",
        "Underfunded",
        "LowReserve",
        "AlreadyExist",
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
            Self::Success => "Success",
            Self::Malformed => "Malformed",
            Self::Underfunded => "Underfunded",
            Self::LowReserve => "LowReserve",
            Self::AlreadyExist => "AlreadyExist",
        }
    }

    #[must_use]
    pub const fn variants() -> [CreateAccountResultCode; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for CreateAccountResultCode {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Variants<CreateAccountResultCode> for CreateAccountResultCode {
    fn variants() -> slice::Iter<'static, CreateAccountResultCode> {
        Self::VARIANTS.iter()
    }
}

impl Enum for CreateAccountResultCode {}

impl fmt::Display for CreateAccountResultCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for CreateAccountResultCode {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self, Error> {
        let e = match i {
            0 => CreateAccountResultCode::Success,
            -1 => CreateAccountResultCode::Malformed,
            -2 => CreateAccountResultCode::Underfunded,
            -3 => CreateAccountResultCode::LowReserve,
            -4 => CreateAccountResultCode::AlreadyExist,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<CreateAccountResultCode> for i32 {
    #[must_use]
    fn from(e: CreateAccountResultCode) -> Self {
        e as Self
    }
}

impl ReadXdr for CreateAccountResultCode {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let e = i32::read_xdr(r)?;
            let v: Self = e.try_into()?;
            Ok(v)
        })
    }
}

impl WriteXdr for CreateAccountResultCode {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            let i: i32 = (*self).into();
            i.write_xdr(w)
        })
    }
}
