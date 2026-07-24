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

impl Hello {
    /// Serialize this value as XDR into a [`ConstWriter`] using only const
    /// operations. This is the const counterpart to [`WriteXdr::write_xdr`].
    #[cfg(feature = "const")]
    pub const fn const_write_xdr(&self, w: &mut ConstWriter) {
        w.enter_depth();
        w.write_u32(self.ledger_version);
        w.write_u32(self.overlay_version);
        w.write_u32(self.overlay_min_version);
        self.network_id.const_write_xdr(w);
        w.write_len_prefixed(self.version_str.0.as_slice());
        w.write_i32(self.listening_port);
        self.peer_id.const_write_xdr(w);
        self.cert.const_write_xdr(w);
        self.nonce.const_write_xdr(w);
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
