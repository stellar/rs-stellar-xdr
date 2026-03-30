#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ContractIdPreimage is an XDR Union defined as:
///
/// ```text
/// union ContractIDPreimage switch (ContractIDPreimageType type)
/// {
/// case CONTRACT_ID_PREIMAGE_FROM_ADDRESS:
///     struct
///     {
///         SCAddress address;
///         uint256 salt;
///     } fromAddress;
/// case CONTRACT_ID_PREIMAGE_FROM_ASSET:
///     Asset fromAsset;
/// };
/// ```
///
// union with discriminant ContractIdPreimageType
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
pub enum ContractIdPreimage {
    Address(
        ContractIdPreimageFromAddress,
    ),
    Asset(
        Asset,
    ),
}


#[cfg(feature = "alloc")]
impl Default for ContractIdPreimage {
    fn default() -> Self {
        Self::Address(ContractIdPreimageFromAddress::default())
    }
}

impl ContractIdPreimage {
    const _VARIANTS: &[ContractIdPreimageType] = &[
        ContractIdPreimageType::Address,
        ContractIdPreimageType::Asset,
    ];
    pub const VARIANTS: [ContractIdPreimageType; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &[
        "Address",
        "Asset",
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
            Self::Address(_) => "Address",
            Self::Asset(_) => "Asset",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> ContractIdPreimageType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Address(_) => ContractIdPreimageType::Address,
            Self::Asset(_) => ContractIdPreimageType::Asset,
        }
    }

    #[must_use]
    pub const fn variants() -> [ContractIdPreimageType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for ContractIdPreimage {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<ContractIdPreimageType> for ContractIdPreimage {
    #[must_use]
    fn discriminant(&self) -> ContractIdPreimageType {
        Self::discriminant(self)
    }
}

impl Variants<ContractIdPreimageType> for ContractIdPreimage {
    fn variants() -> slice::Iter<'static, ContractIdPreimageType> {
        Self::VARIANTS.iter()
    }
}

impl Union<ContractIdPreimageType> for ContractIdPreimage {}

impl ReadXdr for ContractIdPreimage {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: ContractIdPreimageType = <ContractIdPreimageType as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                ContractIdPreimageType::Address => Self::Address(ContractIdPreimageFromAddress::read_xdr(r)?),
                ContractIdPreimageType::Asset => Self::Asset(Asset::read_xdr(r)?),
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for ContractIdPreimage {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::Address(v) => v.write_xdr(w)?,
                Self::Asset(v) => v.write_xdr(w)?,
            };
            Ok(())
        })
    }
}
