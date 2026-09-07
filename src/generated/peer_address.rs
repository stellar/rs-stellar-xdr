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
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval)]
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

#[cfg(feature = "alloc")]
impl IntoOwned for PeerAddress {
    type Owned = PeerAddress;
    fn into_owned(self) -> PeerAddress {
        self
    }
}

#[cfg(feature = "const")]
impl PeerAddress {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_peer_address(self);
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
        w.write_type_peer_address(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`PeerAddress`], mirroring `<PeerAddress as WriteXdr>::write_xdr`.
    pub const fn write_type_peer_address(&mut self, v: &PeerAddress) {
        self.write_type_peer_address_ip(&v.ip);
        self.write_u32(v.port);
        self.write_u32(v.num_failures);
    }

    /// Serializes a variable-length array of [`PeerAddress`], mirroring `<VecM<PeerAddress, MAX> as WriteXdr>::write_xdr`.
    pub const fn write_type_vec_peer_address<const MAX: u32>(
        &mut self,
        v: &VecMRef<'_, PeerAddress, MAX>,
    ) {
        let s = v.as_slice();
        let len = s.len();
        self.write_len(len);
        let mut i = 0usize;
        while i < len {
            self.write_type_peer_address(&s[i]);
            i += 1;
        }
    }
}
