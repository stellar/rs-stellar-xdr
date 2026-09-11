#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ScError is an XDR Union defined as:
///
/// ```text
/// union SCError switch (SCErrorType type)
/// {
/// case SCE_CONTRACT:
///     uint32 contractCode;
/// case SCE_WASM_VM:
/// case SCE_CONTEXT:
/// case SCE_STORAGE:
/// case SCE_OBJECT:
/// case SCE_CRYPTO:
/// case SCE_EVENTS:
/// case SCE_BUDGET:
/// case SCE_VALUE:
/// case SCE_AUTH:
///     SCErrorCode code;
/// };
/// ```
///
// union with discriminant ScErrorType
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
pub enum ScError {
    Contract(u32),
    WasmVm(ScErrorCode),
    Context(ScErrorCode),
    Storage(ScErrorCode),
    Object(ScErrorCode),
    Crypto(ScErrorCode),
    Events(ScErrorCode),
    Budget(ScErrorCode),
    Value(ScErrorCode),
    Auth(ScErrorCode),
}

#[cfg(feature = "alloc")]
impl Default for ScError {
    fn default() -> Self {
        Self::Contract(u32::default())
    }
}

impl ScError {
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
            Self::Contract(_) => "Contract",
            Self::WasmVm(_) => "WasmVm",
            Self::Context(_) => "Context",
            Self::Storage(_) => "Storage",
            Self::Object(_) => "Object",
            Self::Crypto(_) => "Crypto",
            Self::Events(_) => "Events",
            Self::Budget(_) => "Budget",
            Self::Value(_) => "Value",
            Self::Auth(_) => "Auth",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> ScErrorType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Contract(_) => ScErrorType::Contract,
            Self::WasmVm(_) => ScErrorType::WasmVm,
            Self::Context(_) => ScErrorType::Context,
            Self::Storage(_) => ScErrorType::Storage,
            Self::Object(_) => ScErrorType::Object,
            Self::Crypto(_) => ScErrorType::Crypto,
            Self::Events(_) => ScErrorType::Events,
            Self::Budget(_) => ScErrorType::Budget,
            Self::Value(_) => ScErrorType::Value,
            Self::Auth(_) => ScErrorType::Auth,
        }
    }

    #[must_use]
    pub const fn variants() -> [ScErrorType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for ScError {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<ScErrorType> for ScError {
    #[must_use]
    fn discriminant(&self) -> ScErrorType {
        Self::discriminant(self)
    }
}

impl Variants<ScErrorType> for ScError {
    fn variants() -> slice::Iter<'static, ScErrorType> {
        Self::VARIANTS.iter()
    }
}

impl Union<ScErrorType> for ScError {}

impl ReadXdr for ScError {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: ScErrorType = <ScErrorType as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                ScErrorType::Contract => Self::Contract(u32::read_xdr(r)?),
                ScErrorType::WasmVm => Self::WasmVm(ScErrorCode::read_xdr(r)?),
                ScErrorType::Context => Self::Context(ScErrorCode::read_xdr(r)?),
                ScErrorType::Storage => Self::Storage(ScErrorCode::read_xdr(r)?),
                ScErrorType::Object => Self::Object(ScErrorCode::read_xdr(r)?),
                ScErrorType::Crypto => Self::Crypto(ScErrorCode::read_xdr(r)?),
                ScErrorType::Events => Self::Events(ScErrorCode::read_xdr(r)?),
                ScErrorType::Budget => Self::Budget(ScErrorCode::read_xdr(r)?),
                ScErrorType::Value => Self::Value(ScErrorCode::read_xdr(r)?),
                ScErrorType::Auth => Self::Auth(ScErrorCode::read_xdr(r)?),
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for ScError {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::Contract(v) => v.write_xdr(w)?,
                Self::WasmVm(v) => v.write_xdr(w)?,
                Self::Context(v) => v.write_xdr(w)?,
                Self::Storage(v) => v.write_xdr(w)?,
                Self::Object(v) => v.write_xdr(w)?,
                Self::Crypto(v) => v.write_xdr(w)?,
                Self::Events(v) => v.write_xdr(w)?,
                Self::Budget(v) => v.write_xdr(w)?,
                Self::Value(v) => v.write_xdr(w)?,
                Self::Auth(v) => v.write_xdr(w)?,
            };
            Ok(())
        })
    }
}
