#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
/// SorobanAuthorizedFunctionType is an XDR Enum defined as:
///
/// ```text
/// enum SorobanAuthorizedFunctionType
/// {
///     SOROBAN_AUTHORIZED_FUNCTION_TYPE_CONTRACT_FN = 0,
///     SOROBAN_AUTHORIZED_FUNCTION_TYPE_CREATE_CONTRACT_HOST_FN = 1,
///     SOROBAN_AUTHORIZED_FUNCTION_TYPE_CREATE_CONTRACT_V2_HOST_FN = 2
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
pub enum SorobanAuthorizedFunctionType {
    #[cfg_attr(feature = "alloc", default)]
    ContractFn = 0,
    CreateContractHostFn = 1,
    CreateContractV2HostFn = 2,
}

impl SorobanAuthorizedFunctionType {
    const _VARIANTS: &[SorobanAuthorizedFunctionType] = &[
        SorobanAuthorizedFunctionType::ContractFn,
        SorobanAuthorizedFunctionType::CreateContractHostFn,
        SorobanAuthorizedFunctionType::CreateContractV2HostFn,
    ];
    pub const VARIANTS: [SorobanAuthorizedFunctionType; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &[
        "ContractFn",
        "CreateContractHostFn",
        "CreateContractV2HostFn",
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
            Self::ContractFn => "ContractFn",
            Self::CreateContractHostFn => "CreateContractHostFn",
            Self::CreateContractV2HostFn => "CreateContractV2HostFn",
        }
    }

    #[must_use]
    pub const fn variants() -> [SorobanAuthorizedFunctionType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for SorobanAuthorizedFunctionType {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Variants<SorobanAuthorizedFunctionType> for SorobanAuthorizedFunctionType {
    fn variants() -> slice::Iter<'static, SorobanAuthorizedFunctionType> {
        Self::VARIANTS.iter()
    }
}

impl Enum for SorobanAuthorizedFunctionType {}

impl fmt::Display for SorobanAuthorizedFunctionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for SorobanAuthorizedFunctionType {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self, Error> {
        let e = match i {
            0 => SorobanAuthorizedFunctionType::ContractFn,
            1 => SorobanAuthorizedFunctionType::CreateContractHostFn,
            2 => SorobanAuthorizedFunctionType::CreateContractV2HostFn,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<SorobanAuthorizedFunctionType> for i32 {
    #[must_use]
    fn from(e: SorobanAuthorizedFunctionType) -> Self {
        e as Self
    }
}

impl ReadXdr for SorobanAuthorizedFunctionType {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let e = i32::read_xdr(r)?;
            let v: Self = e.try_into()?;
            Ok(v)
        })
    }
}

impl WriteXdr for SorobanAuthorizedFunctionType {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            let i: i32 = (*self).into();
            i.write_xdr(w)
        })
    }
}
