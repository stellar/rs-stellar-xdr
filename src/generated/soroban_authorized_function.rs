#[allow(unused_imports)]
use super::*;
/// SorobanAuthorizedFunction is an XDR Union defined as:
///
/// ```text
/// union SorobanAuthorizedFunction switch (SorobanAuthorizedFunctionType type)
/// {
/// case SOROBAN_AUTHORIZED_FUNCTION_TYPE_CONTRACT_FN:
///     InvokeContractArgs contractFn;
/// // This variant of auth payload for creating new contract instances
/// // doesn't allow specifying the constructor arguments, creating contracts
/// // with constructors that take arguments is only possible by authorizing
/// // `SOROBAN_AUTHORIZED_FUNCTION_TYPE_CREATE_CONTRACT_V2_HOST_FN`
/// // (protocol 22+).
/// case SOROBAN_AUTHORIZED_FUNCTION_TYPE_CREATE_CONTRACT_HOST_FN:
///     CreateContractArgs createContractHostFn;
/// // This variant of auth payload for creating new contract instances
/// // is only accepted in and after protocol 22. It allows authorizing the
/// // contract constructor arguments.
/// case SOROBAN_AUTHORIZED_FUNCTION_TYPE_CREATE_CONTRACT_V2_HOST_FN:
///     CreateContractArgsV2 createContractV2HostFn;
/// };
/// ```
///
// union with discriminant SorobanAuthorizedFunctionType
#[cfg_eval::cfg_eval]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    serde_with::serde_as,
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[allow(clippy::large_enum_variant)]
pub enum SorobanAuthorizedFunction {
    ContractFn(InvokeContractArgs),
    CreateContractHostFn(CreateContractArgs),
    CreateContractV2HostFn(CreateContractArgsV2),
}

#[cfg(feature = "alloc")]
impl Default for SorobanAuthorizedFunction {
    fn default() -> Self {
        Self::ContractFn(InvokeContractArgs::default())
    }
}

impl SorobanAuthorizedFunction {
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
            Self::ContractFn(_) => "ContractFn",
            Self::CreateContractHostFn(_) => "CreateContractHostFn",
            Self::CreateContractV2HostFn(_) => "CreateContractV2HostFn",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> SorobanAuthorizedFunctionType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::ContractFn(_) => SorobanAuthorizedFunctionType::ContractFn,
            Self::CreateContractHostFn(_) => SorobanAuthorizedFunctionType::CreateContractHostFn,
            Self::CreateContractV2HostFn(_) => {
                SorobanAuthorizedFunctionType::CreateContractV2HostFn
            }
        }
    }

    #[must_use]
    pub const fn variants() -> [SorobanAuthorizedFunctionType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for SorobanAuthorizedFunction {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<SorobanAuthorizedFunctionType> for SorobanAuthorizedFunction {
    #[must_use]
    fn discriminant(&self) -> SorobanAuthorizedFunctionType {
        Self::discriminant(self)
    }
}

impl Variants<SorobanAuthorizedFunctionType> for SorobanAuthorizedFunction {
    fn variants() -> slice::Iter<'static, SorobanAuthorizedFunctionType> {
        Self::VARIANTS.iter()
    }
}

impl Union<SorobanAuthorizedFunctionType> for SorobanAuthorizedFunction {}

impl ReadXdr for SorobanAuthorizedFunction {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: SorobanAuthorizedFunctionType =
                <SorobanAuthorizedFunctionType as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                SorobanAuthorizedFunctionType::ContractFn => {
                    Self::ContractFn(InvokeContractArgs::read_xdr(r)?)
                }
                SorobanAuthorizedFunctionType::CreateContractHostFn => {
                    Self::CreateContractHostFn(CreateContractArgs::read_xdr(r)?)
                }
                SorobanAuthorizedFunctionType::CreateContractV2HostFn => {
                    Self::CreateContractV2HostFn(CreateContractArgsV2::read_xdr(r)?)
                }
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for SorobanAuthorizedFunction {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::ContractFn(v) => v.write_xdr(w)?,
                Self::CreateContractHostFn(v) => v.write_xdr(w)?,
                Self::CreateContractV2HostFn(v) => v.write_xdr(w)?,
            };
            Ok(())
        })
    }
}
