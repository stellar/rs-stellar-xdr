#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// Hello is an XDR Struct defined as:
///
/// ```text
/// struct Hello
/// {
///     uint32 ledgerVersion;
///     uint32 overlayVersion;
///     uint32 overlayMinVersion;
///     Hash networkID;
///     string versionStr<100>;
///     int listeningPort;
///     NodeID peerID;
///     AuthCert cert;
///     uint256 nonce;
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
pub struct Hello {
    pub ledger_version: u32,
    pub overlay_version: u32,
    pub overlay_min_version: u32,
    pub network_id: Hash,
    pub version_str: StringM<100>,
    pub listening_port: i32,
    pub peer_id: NodeId,
    pub cert: AuthCert,
    pub nonce: Uint256,
}

impl ReadXdr for Hello {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                ledger_version: u32::read_xdr(r)?,
                overlay_version: u32::read_xdr(r)?,
                overlay_min_version: u32::read_xdr(r)?,
                network_id: Hash::read_xdr(r)?,
                version_str: StringM::<100>::read_xdr(r)?,
                listening_port: i32::read_xdr(r)?,
                peer_id: NodeId::read_xdr(r)?,
                cert: AuthCert::read_xdr(r)?,
                nonce: Uint256::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for Hello {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.ledger_version.write_xdr(w)?;
            self.overlay_version.write_xdr(w)?;
            self.overlay_min_version.write_xdr(w)?;
            self.network_id.write_xdr(w)?;
            self.version_str.write_xdr(w)?;
            self.listening_port.write_xdr(w)?;
            self.peer_id.write_xdr(w)?;
            self.cert.write_xdr(w)?;
            self.nonce.write_xdr(w)?;
            Ok(())
        })
    }
}

/// HelloView is a borrowing equivalent of [`Hello`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct HelloView<'a> {
    pub ledger_version: u32,
    pub overlay_version: u32,
    pub overlay_min_version: u32,
    pub network_id: Hash,
    pub version_str: StringMView<'a, 100>,
    pub listening_port: i32,
    pub peer_id: NodeId,
    pub cert: AuthCertView<'a>,
    pub nonce: Uint256,
}

#[cfg(feature = "alloc")]
impl IntoOwned for HelloView<'_> {
    type Owned = Hello;
    fn into_owned(self) -> Hello {
        Hello {
            ledger_version: self.ledger_version.into_owned(),
            overlay_version: self.overlay_version.into_owned(),
            overlay_min_version: self.overlay_min_version.into_owned(),
            network_id: self.network_id.into_owned(),
            version_str: self.version_str.into_owned(),
            listening_port: self.listening_port.into_owned(),
            peer_id: self.peer_id.into_owned(),
            cert: self.cert.into_owned(),
            nonce: self.nonce.into_owned(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&HelloView<'_>> for Hello {
    #[must_use]
    fn from(v: &HelloView<'_>) -> Self {
        v.into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<HelloView<'_>> for Hello {
    #[must_use]
    fn from(v: HelloView<'_>) -> Self {
        v.into_owned()
    }
}

impl WriteXdr for HelloView<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.ledger_version.write_xdr(w)?;
            self.overlay_version.write_xdr(w)?;
            self.overlay_min_version.write_xdr(w)?;
            self.network_id.write_xdr(w)?;
            self.version_str.write_xdr(w)?;
            self.listening_port.write_xdr(w)?;
            self.peer_id.write_xdr(w)?;
            self.cert.write_xdr(w)?;
            self.nonce.write_xdr(w)?;
            Ok(())
        })
    }
}
