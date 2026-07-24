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

impl HostFunction {
    /// Serialize this value as XDR into a [`ConstWriter`] using only const
    /// operations. This is the const implementation underlying `to_xdr`.
    #[cfg(feature = "std")]
    pub const fn const_write_xdr(&self, w: &mut ConstWriter) {
        w.enter_depth();
        let d = self.discriminant();
        d.const_write_xdr(w);
        #[allow(clippy::match_same_arms)]
        match self {
            Self::InvokeContract(v) => {
                v.const_write_xdr(w);
            }
            Self::CreateContract(v) => {
                v.const_write_xdr(w);
            }
            Self::UploadContractWasm(v) => {
                w.write_len_prefixed(v.0.as_slice());
            }
            Self::CreateContractV2(v) => {
                v.const_write_xdr(w);
            }
        }
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

    #[cfg(feature = "std")]
    fn to_xdr(&self, limits: Limits) -> Result<Vec<u8>, Error> {
        to_xdr_via_const(self, &limits, Self::const_write_xdr)
    }
}
