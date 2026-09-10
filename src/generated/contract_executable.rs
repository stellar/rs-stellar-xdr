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
/// case CONTRACT_EXECUTABLE_EXTERNAL_REF:
///     ContractExecutableExternalRef external_ref;
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
    const _VARIANTS_STR: &[&str] = &["Wasm", "StellarAsset", "ExternalRef"];
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
            Self::ExternalRef(_) => "ExternalRef",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> ContractExecutableType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Wasm(_) => ContractExecutableType::Wasm,
            Self::StellarAsset => ContractExecutableType::StellarAsset,
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

impl WriteXdr for ContractExecutable {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::Wasm(v) => v.write_xdr(w)?,
                Self::StellarAsset => ().write_xdr(w)?,
                Self::ExternalRef(v) => v.write_xdr(w)?,
            };
            Ok(())
        })
    }
}

/// ContractExecutableConst is a borrowing equivalent of [`ContractExecutable`] over `'static`
/// data, for const XDR encoding.
#[cfg(feature = "const")]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(clippy::large_enum_variant)]
pub enum ContractExecutableConst {
    Wasm(Hash),
    StellarAsset,
    ExternalRef(ContractExecutableExternalRefConst),
}

#[cfg(feature = "const")]
impl ContractExecutableConst {
    #[must_use]
    pub const fn discriminant(&self) -> ContractExecutableType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Wasm(_) => ContractExecutableType::Wasm,
            Self::StellarAsset => ContractExecutableType::StellarAsset,
            Self::ExternalRef(_) => ContractExecutableType::ExternalRef,
        }
    }
}

#[cfg(feature = "const")]
impl ContractExecutableConst {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_contract_executable(self);
        w.len()
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
    #[must_use]
    pub const fn const_to_xdr<const N: usize>(&self) -> [u8; N] {
        let mut buf = [0u8; N];
        let mut w = ConstWriter::new(&mut buf);
        w.write_type_contract_executable(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`ContractExecutable`], mirroring `<ContractExecutable as WriteXdr>::write_xdr`.
    pub const fn write_type_contract_executable(&mut self, v: &ContractExecutableConst) {
        let d = v.discriminant();
        self.write_type_contract_executable_type(&d);
        #[allow(clippy::match_same_arms)]
        match v {
            ContractExecutableConst::Wasm(value) => {
                self.write_type_hash(value);
            }
            ContractExecutableConst::StellarAsset => {}
            ContractExecutableConst::ExternalRef(value) => {
                self.write_type_contract_executable_external_ref(value);
            }
        }
    }
}
