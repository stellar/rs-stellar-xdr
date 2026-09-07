#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// HostFunction is an XDR Union defined as:
///
/// ```text
/// union HostFunction switch (HostFunctionType type)
/// {
/// case HOST_FUNCTION_TYPE_INVOKE_CONTRACT:
///     InvokeContractArgs invokeContract;
/// case HOST_FUNCTION_TYPE_CREATE_CONTRACT:
///     CreateContractArgs createContract;
/// case HOST_FUNCTION_TYPE_UPLOAD_CONTRACT_WASM:
///     opaque wasm<>;
/// case HOST_FUNCTION_TYPE_CREATE_CONTRACT_V2:
///     CreateContractArgsV2 createContractV2;
/// };
/// ```
///
// union with discriminant HostFunctionType
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
pub enum HostFunction {
    InvokeContract(InvokeContractArgs),
    CreateContract(CreateContractArgs),
    UploadContractWasm(BytesM),
    CreateContractV2(CreateContractArgsV2),
}

#[cfg(feature = "alloc")]
impl Default for HostFunction {
    fn default() -> Self {
        Self::InvokeContract(InvokeContractArgs::default())
    }
}

impl HostFunction {
    const _VARIANTS: &[HostFunctionType] = &[
        HostFunctionType::InvokeContract,
        HostFunctionType::CreateContract,
        HostFunctionType::UploadContractWasm,
        HostFunctionType::CreateContractV2,
    ];
    pub const VARIANTS: [HostFunctionType; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &[
        "InvokeContract",
        "CreateContract",
        "UploadContractWasm",
        "CreateContractV2",
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
            Self::InvokeContract(_) => "InvokeContract",
            Self::CreateContract(_) => "CreateContract",
            Self::UploadContractWasm(_) => "UploadContractWasm",
            Self::CreateContractV2(_) => "CreateContractV2",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> HostFunctionType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::InvokeContract(_) => HostFunctionType::InvokeContract,
            Self::CreateContract(_) => HostFunctionType::CreateContract,
            Self::UploadContractWasm(_) => HostFunctionType::UploadContractWasm,
            Self::CreateContractV2(_) => HostFunctionType::CreateContractV2,
        }
    }

    #[must_use]
    pub const fn variants() -> [HostFunctionType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for HostFunction {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<HostFunctionType> for HostFunction {
    #[must_use]
    fn discriminant(&self) -> HostFunctionType {
        Self::discriminant(self)
    }
}

impl Variants<HostFunctionType> for HostFunction {
    fn variants() -> slice::Iter<'static, HostFunctionType> {
        Self::VARIANTS.iter()
    }
}

impl Union<HostFunctionType> for HostFunction {}

impl ReadXdr for HostFunction {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: HostFunctionType = <HostFunctionType as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                HostFunctionType::InvokeContract => {
                    Self::InvokeContract(InvokeContractArgs::read_xdr(r)?)
                }
                HostFunctionType::CreateContract => {
                    Self::CreateContract(CreateContractArgs::read_xdr(r)?)
                }
                HostFunctionType::UploadContractWasm => {
                    Self::UploadContractWasm(BytesM::read_xdr(r)?)
                }
                HostFunctionType::CreateContractV2 => {
                    Self::CreateContractV2(CreateContractArgsV2::read_xdr(r)?)
                }
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for HostFunction {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::InvokeContract(v) => v.write_xdr(w)?,
                Self::CreateContract(v) => v.write_xdr(w)?,
                Self::UploadContractWasm(v) => v.write_xdr(w)?,
                Self::CreateContractV2(v) => v.write_xdr(w)?,
            };
            Ok(())
        })
    }
}

/// HostFunctionRef is a borrowing equivalent of [`HostFunction`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(clippy::large_enum_variant)]
pub enum HostFunctionRef<'a> {
    InvokeContract(InvokeContractArgsRef<'a>),
    CreateContract(CreateContractArgsRef<'a>),
    UploadContractWasm(BytesMRef<'a>),
    CreateContractV2(CreateContractArgsV2Ref<'a>),
}

#[cfg(feature = "alloc")]
impl IntoOwned for HostFunctionRef<'_> {
    type Owned = HostFunction;
    fn into_owned(self) -> HostFunction {
        #[allow(clippy::match_same_arms)]
        match self {
            HostFunctionRef::InvokeContract(value) => {
                HostFunction::InvokeContract(value.into_owned())
            }
            HostFunctionRef::CreateContract(value) => {
                HostFunction::CreateContract(value.into_owned())
            }
            HostFunctionRef::UploadContractWasm(value) => {
                HostFunction::UploadContractWasm(value.into_owned())
            }
            HostFunctionRef::CreateContractV2(value) => {
                HostFunction::CreateContractV2(value.into_owned())
            }
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&HostFunctionRef<'_>> for HostFunction {
    #[must_use]
    fn from(v: &HostFunctionRef<'_>) -> Self {
        v.into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<HostFunctionRef<'_>> for HostFunction {
    #[must_use]
    fn from(v: HostFunctionRef<'_>) -> Self {
        v.into_owned()
    }
}

impl HostFunctionRef<'_> {
    #[must_use]
    pub const fn discriminant(&self) -> HostFunctionType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::InvokeContract(_) => HostFunctionType::InvokeContract,
            Self::CreateContract(_) => HostFunctionType::CreateContract,
            Self::UploadContractWasm(_) => HostFunctionType::UploadContractWasm,
            Self::CreateContractV2(_) => HostFunctionType::CreateContractV2,
        }
    }
}

impl WriteXdr for HostFunctionRef<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::InvokeContract(v) => v.write_xdr(w)?,
                Self::CreateContract(v) => v.write_xdr(w)?,
                Self::UploadContractWasm(v) => v.write_xdr(w)?,
                Self::CreateContractV2(v) => v.write_xdr(w)?,
            };
            Ok(())
        })
    }
}

#[cfg(feature = "const")]
impl HostFunctionRef<'_> {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_host_function(self);
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
        w.write_type_host_function(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`HostFunction`], mirroring `<HostFunction as WriteXdr>::write_xdr`.
    pub const fn write_type_host_function(&mut self, v: &HostFunctionRef<'_>) {
        let d = v.discriminant();
        self.write_type_host_function_type(&d);
        #[allow(clippy::match_same_arms)]
        match v {
            HostFunctionRef::InvokeContract(value) => {
                self.write_type_invoke_contract_args(value);
            }
            HostFunctionRef::CreateContract(value) => {
                self.write_type_create_contract_args(value);
            }
            HostFunctionRef::UploadContractWasm(value) => {
                self.write_var_opaque(value.as_slice());
            }
            HostFunctionRef::CreateContractV2(value) => {
                self.write_type_create_contract_args_v2(value);
            }
        }
    }
}
