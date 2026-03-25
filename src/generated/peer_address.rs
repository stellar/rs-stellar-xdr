#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;
/// PeerAddress is an XDR Struct defined as:
///
/// ```text
/// struct PeerAddress
/// {
///     union switch (IPAddrType type)
///     {
///     case IPv4:
///         opaque ipv4[4];
///     case IPv6:
///         opaque ipv6[16];
///     }
///     ip;
///     uint32 port;
///     uint32 numFailures;
/// };
/// ```
///
#[cfg_attr(feature = "alloc", derive(Default))]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_eval::cfg_eval]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    serde_with::serde_as,
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct PeerAddress {
    pub ip: PeerAddressIp,
    pub port: u32,
    pub num_failures: u32,
}

impl ReadXdr for PeerAddress {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                ip: PeerAddressIp::read_xdr(r)?,
                port: u32::read_xdr(r)?,
                num_failures: u32::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for PeerAddress {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.ip.write_xdr(w)?;
            self.port.write_xdr(w)?;
            self.num_failures.write_xdr(w)?;
            Ok(())
        })
    }
}
