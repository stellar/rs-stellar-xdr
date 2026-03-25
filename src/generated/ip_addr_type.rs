#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
/// IpAddrType is an XDR Enum defined as:
///
/// ```text
/// enum IPAddrType
/// {
///     IPv4 = 0,
///     IPv6 = 1
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
pub enum IpAddrType {
    #[cfg_attr(feature = "alloc", default)]
    IPv4 = 0,
    IPv6 = 1,
}

impl IpAddrType {
    const _VARIANTS: &[IpAddrType] = &[IpAddrType::IPv4, IpAddrType::IPv6];
    pub const VARIANTS: [IpAddrType; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &["IPv4", "IPv6"];
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
            Self::IPv4 => "IPv4",
            Self::IPv6 => "IPv6",
        }
    }

    #[must_use]
    pub const fn variants() -> [IpAddrType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for IpAddrType {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Variants<IpAddrType> for IpAddrType {
    fn variants() -> slice::Iter<'static, IpAddrType> {
        Self::VARIANTS.iter()
    }
}

impl Enum for IpAddrType {}

impl fmt::Display for IpAddrType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for IpAddrType {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self, Error> {
        let e = match i {
            0 => IpAddrType::IPv4,
            1 => IpAddrType::IPv6,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<IpAddrType> for i32 {
    #[must_use]
    fn from(e: IpAddrType) -> Self {
        e as Self
    }
}

impl ReadXdr for IpAddrType {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let e = i32::read_xdr(r)?;
            let v: Self = e.try_into()?;
            Ok(v)
        })
    }
}

impl WriteXdr for IpAddrType {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            let i: i32 = (*self).into();
            i.write_xdr(w)
        })
    }
}
