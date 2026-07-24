#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ContractExecutable is an XDR Union defined as:
///
/// ```text
/// union ContractExecutable switch (ContractExecutableType type)
/// {
/// case CONTRACT_EXECUTABLE_WASM:
///     Hash wasm_hash;
/// case CONTRACT_EXECUTABLE_STELLAR_ASSET:
///     void;
/// #ifdef CAP_0085_EXECUTABLE_REF
/// case CONTRACT_EXECUTABLE_EXTERNAL_REF:
///     ContractExecutableExternalRef external_ref;
/// #endif
/// };
/// ```
///
// union with discriminant ContractExecutableType
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval)]
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
pub enum ContractExecutable {
    Wasm(Hash),
    StellarAsset,
    #[cfg(feature = "cap_0085_executable_ref")]
    ExternalRef(ContractExecutableExternalRef),
}

#[cfg(feature = "alloc")]
impl Default for ContractExecutable {
    fn default() -> Self {
        Self::Wasm(Hash::default())
    }
}

impl ContractExecutable {
    const _VARIANTS: &[ContractExecutableType] = &[
        ContractExecutableType::Wasm,
        ContractExecutableType::StellarAsset,
        #[cfg(feature = "cap_0085_executable_ref")]
        ContractExecutableType::ExternalRef,
    ];
    pub const VARIANTS: [ContractExecutableType; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &[
        "Wasm",
        "StellarAsset",
        #[cfg(feature = "cap_0085_executable_ref")]
        "ExternalRef",
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
            Self::Wasm(_) => "Wasm",
            Self::StellarAsset => "StellarAsset",
            #[cfg(feature = "cap_0085_executable_ref")]
            Self::ExternalRef(_) => "ExternalRef",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> ContractExecutableType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Wasm(_) => ContractExecutableType::Wasm,
            Self::StellarAsset => ContractExecutableType::StellarAsset,
            #[cfg(feature = "cap_0085_executable_ref")]
            Self::ExternalRef(_) => ContractExecutableType::ExternalRef,
        }
    }

    #[must_use]
    pub const fn variants() -> [ContractExecutableType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for ContractExecutable {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<ContractExecutableType> for ContractExecutable {
    #[must_use]
    fn discriminant(&self) -> ContractExecutableType {
        Self::discriminant(self)
    }
}

impl Variants<ContractExecutableType> for ContractExecutable {
    fn variants() -> slice::Iter<'static, ContractExecutableType> {
        Self::VARIANTS.iter()
    }
}

impl Union<ContractExecutableType> for ContractExecutable {}

impl ReadXdr for ContractExecutable {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: ContractExecutableType = <ContractExecutableType as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                ContractExecutableType::Wasm => Self::Wasm(Hash::read_xdr(r)?),
                ContractExecutableType::StellarAsset => Self::StellarAsset,
                #[cfg(feature = "cap_0085_executable_ref")]
                ContractExecutableType::ExternalRef => {
                    Self::ExternalRef(ContractExecutableExternalRef::read_xdr(r)?)
                }
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl ContractExecutable {
    /// Serialize this value as XDR into a [`ConstWriter`] using only const
    /// operations. This is the const implementation underlying `to_xdr`.
    #[cfg(feature = "std")]
    pub const fn const_write_xdr(&self, w: &mut ConstWriter) {
        w.enter_depth();
        let d = self.discriminant();
        d.const_write_xdr(w);
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Wasm(v) => {
                v.const_write_xdr(w);
            }
            Self::StellarAsset => {}
            #[cfg(feature = "cap_0085_executable_ref")]
            Self::ExternalRef(v) => {
                v.const_write_xdr(w);
            }
        }
        w.leave_depth();
    }
}

impl WriteXdr for ContractExecutable {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::Wasm(v) => v.write_xdr(w)?,
                Self::StellarAsset => ().write_xdr(w)?,
                #[cfg(feature = "cap_0085_executable_ref")]
                Self::ExternalRef(v) => v.write_xdr(w)?,
            };
            Ok(())
        })
    }

    #[cfg(feature = "std")]
    fn to_xdr(&self, limits: Limits) -> Result<Vec<u8>, Error> {
        to_xdr_via_const(self, &limits, Self::const_write_xdr)
    }
}
