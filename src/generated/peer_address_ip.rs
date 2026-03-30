#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// PeerAddressIp is an XDR NestedUnion defined as:
///
/// ```text
/// union switch (IPAddrType type)
///     {
///     case IPv4:
///         opaque ipv4[4];
///     case IPv6:
///         opaque ipv6[16];
///     }
/// ```
///
// union with discriminant IpAddrType
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
pub enum PeerAddressIp {
    IPv4(
        [u8; 4],
    ),
    IPv6(
        [u8; 16],
    ),
}


#[cfg(feature = "alloc")]
impl Default for PeerAddressIp {
    fn default() -> Self {
        Self::IPv4(<[u8; 4]>::default())
    }
}

impl PeerAddressIp {
    const _VARIANTS: &[IpAddrType] = &[
        IpAddrType::IPv4,
        IpAddrType::IPv6,
    ];
    pub const VARIANTS: [IpAddrType; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &[
        "IPv4",
        "IPv6",
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
            Self::IPv4(_) => "IPv4",
            Self::IPv6(_) => "IPv6",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> IpAddrType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::IPv4(_) => IpAddrType::IPv4,
            Self::IPv6(_) => IpAddrType::IPv6,
        }
    }

    #[must_use]
    pub const fn variants() -> [IpAddrType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for PeerAddressIp {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<IpAddrType> for PeerAddressIp {
    #[must_use]
    fn discriminant(&self) -> IpAddrType {
        Self::discriminant(self)
    }
}

impl Variants<IpAddrType> for PeerAddressIp {
    fn variants() -> slice::Iter<'static, IpAddrType> {
        Self::VARIANTS.iter()
    }
}

impl Union<IpAddrType> for PeerAddressIp {}

impl ReadXdr for PeerAddressIp {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: IpAddrType = <IpAddrType as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                IpAddrType::IPv4 => Self::IPv4(<[u8; 4]>::read_xdr(r)?),
                IpAddrType::IPv6 => Self::IPv6(<[u8; 16]>::read_xdr(r)?),
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for PeerAddressIp {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::IPv4(v) => v.write_xdr(w)?,
                Self::IPv6(v) => v.write_xdr(w)?,
            };
            Ok(())
        })
    }
}
