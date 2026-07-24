#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// HostFunctionType is an XDR Enum defined as:
///
/// ```text
/// enum HostFunctionType
/// {
///     HOST_FUNCTION_TYPE_INVOKE_CONTRACT = 0,
///     HOST_FUNCTION_TYPE_CREATE_CONTRACT = 1,
///     HOST_FUNCTION_TYPE_UPLOAD_CONTRACT_WASM = 2,
///     HOST_FUNCTION_TYPE_CREATE_CONTRACT_V2 = 3
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
pub enum HostFunctionType {
    #[cfg_attr(feature = "alloc", default)]
    InvokeContract = 0,
    CreateContract = 1,
    UploadContractWasm = 2,
    CreateContractV2 = 3,
}

impl HostFunctionType {
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
            Self::InvokeContract => "InvokeContract",
            Self::CreateContract => "CreateContract",
            Self::UploadContractWasm => "UploadContractWasm",
            Self::CreateContractV2 => "CreateContractV2",
        }
    }

    #[must_use]
    pub const fn variants() -> [HostFunctionType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for HostFunctionType {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Variants<HostFunctionType> for HostFunctionType {
    fn variants() -> slice::Iter<'static, HostFunctionType> {
        Self::VARIANTS.iter()
    }
}

impl Enum for HostFunctionType {}

impl fmt::Display for HostFunctionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for HostFunctionType {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self, Error> {
        let e = match i {
            0 => HostFunctionType::InvokeContract,
            1 => HostFunctionType::CreateContract,
            2 => HostFunctionType::UploadContractWasm,
            3 => HostFunctionType::CreateContractV2,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<HostFunctionType> for i32 {
    #[must_use]
    fn from(e: HostFunctionType) -> Self {
        e as Self
    }
}

impl ReadXdr for HostFunctionType {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let e = i32::read_xdr(r)?;
            let v: Self = e.try_into()?;
            Ok(v)
        })
    }
}

impl HostFunctionType {
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

impl WriteXdr for HostFunctionType {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            let i: i32 = (*self).into();
            i.write_xdr(w)
        })
    }
}
