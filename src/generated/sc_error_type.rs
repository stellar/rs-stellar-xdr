#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ScErrorType is an XDR Enum defined as:
///
/// ```text
/// enum SCErrorType
/// {
///     SCE_CONTRACT = 0,          // Contract-specific, user-defined codes.
///     SCE_WASM_VM = 1,           // Errors while interpreting WASM bytecode.
///     SCE_CONTEXT = 2,           // Errors in the contract's host context.
///     SCE_STORAGE = 3,           // Errors accessing host storage.
///     SCE_OBJECT = 4,            // Errors working with host objects.
///     SCE_CRYPTO = 5,            // Errors in cryptographic operations.
///     SCE_EVENTS = 6,            // Errors while emitting events.
///     SCE_BUDGET = 7,            // Errors relating to budget limits.
///     SCE_VALUE = 8,             // Errors working with host values or SCVals.
///     SCE_AUTH = 9               // Errors from the authentication subsystem.
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
pub enum ScErrorType {
    #[cfg_attr(feature = "alloc", default)]
    Contract = 0,
    WasmVm = 1,
    Context = 2,
    Storage = 3,
    Object = 4,
    Crypto = 5,
    Events = 6,
    Budget = 7,
    Value = 8,
    Auth = 9,
}

impl ScErrorType {
    const _VARIANTS: &[ScErrorType] = &[
        ScErrorType::Contract,
        ScErrorType::WasmVm,
        ScErrorType::Context,
        ScErrorType::Storage,
        ScErrorType::Object,
        ScErrorType::Crypto,
        ScErrorType::Events,
        ScErrorType::Budget,
        ScErrorType::Value,
        ScErrorType::Auth,
    ];
    pub const VARIANTS: [ScErrorType; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &[
        "Contract", "WasmVm", "Context", "Storage", "Object", "Crypto", "Events", "Budget",
        "Value", "Auth",
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
            Self::Contract => "Contract",
            Self::WasmVm => "WasmVm",
            Self::Context => "Context",
            Self::Storage => "Storage",
            Self::Object => "Object",
            Self::Crypto => "Crypto",
            Self::Events => "Events",
            Self::Budget => "Budget",
            Self::Value => "Value",
            Self::Auth => "Auth",
        }
    }

    #[must_use]
    pub const fn variants() -> [ScErrorType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for ScErrorType {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Variants<ScErrorType> for ScErrorType {
    fn variants() -> slice::Iter<'static, ScErrorType> {
        Self::VARIANTS.iter()
    }
}

impl Enum for ScErrorType {}

impl fmt::Display for ScErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for ScErrorType {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self, Error> {
        let e = match i {
            0 => ScErrorType::Contract,
            1 => ScErrorType::WasmVm,
            2 => ScErrorType::Context,
            3 => ScErrorType::Storage,
            4 => ScErrorType::Object,
            5 => ScErrorType::Crypto,
            6 => ScErrorType::Events,
            7 => ScErrorType::Budget,
            8 => ScErrorType::Value,
            9 => ScErrorType::Auth,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<ScErrorType> for i32 {
    #[must_use]
    fn from(e: ScErrorType) -> Self {
        e as Self
    }
}

impl ReadXdr for ScErrorType {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let e = i32::read_xdr(r)?;
            let v: Self = e.try_into()?;
            Ok(v)
        })
    }
}

impl WriteXdr for ScErrorType {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            let i: i32 = (*self).into();
            i.write_xdr(w)
        })
    }
}
