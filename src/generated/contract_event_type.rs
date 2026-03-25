#[allow(unused_imports)]
use super::*;
/// ContractEventType is an XDR Enum defined as:
///
/// ```text
/// enum ContractEventType
/// {
///     SYSTEM = 0,
///     CONTRACT = 1,
///     DIAGNOSTIC = 2
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
pub enum ContractEventType {
    #[cfg_attr(feature = "alloc", default)]
    System = 0,
    Contract = 1,
    Diagnostic = 2,
}

impl ContractEventType {
    const _VARIANTS: &[ContractEventType] = &[
        ContractEventType::System,
        ContractEventType::Contract,
        ContractEventType::Diagnostic,
    ];
    pub const VARIANTS: [ContractEventType; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &["System", "Contract", "Diagnostic"];
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
            Self::System => "System",
            Self::Contract => "Contract",
            Self::Diagnostic => "Diagnostic",
        }
    }

    #[must_use]
    pub const fn variants() -> [ContractEventType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for ContractEventType {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Variants<ContractEventType> for ContractEventType {
    fn variants() -> slice::Iter<'static, ContractEventType> {
        Self::VARIANTS.iter()
    }
}

impl Enum for ContractEventType {}

impl fmt::Display for ContractEventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for ContractEventType {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self, Error> {
        let e = match i {
            0 => ContractEventType::System,
            1 => ContractEventType::Contract,
            2 => ContractEventType::Diagnostic,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<ContractEventType> for i32 {
    #[must_use]
    fn from(e: ContractEventType) -> Self {
        e as Self
    }
}

impl ReadXdr for ContractEventType {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let e = i32::read_xdr(r)?;
            let v: Self = e.try_into()?;
            Ok(v)
        })
    }
}

impl WriteXdr for ContractEventType {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            let i: i32 = (*self).into();
            i.write_xdr(w)
        })
    }
}
