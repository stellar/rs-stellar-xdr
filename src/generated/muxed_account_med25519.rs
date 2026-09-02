#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// MuxedAccountMed25519 is an XDR NestedStruct defined as:
///
/// ```text
/// struct
///     {
///         uint64 id;
///         uint256 ed25519;
///     }
/// ```
///
#[cfg_attr(feature = "alloc", derive(Default))]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde_with::SerializeDisplay)
)]
pub struct MuxedAccountMed25519 {
    pub id: u64,
    pub ed25519: Uint256,
}

impl ReadXdr for MuxedAccountMed25519 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                id: u64::read_xdr(r)?,
                ed25519: Uint256::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for MuxedAccountMed25519 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.id.write_xdr(w)?;
            self.ed25519.write_xdr(w)?;
            Ok(())
        })
    }
}
#[cfg(all(feature = "serde", feature = "alloc"))]
impl<'de> serde::Deserialize<'de> for MuxedAccountMed25519 {
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::Deserialize;
        #[derive(Deserialize)]
        struct MuxedAccountMed25519 {
            id: u64,
            ed25519: Uint256,
        }
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum MuxedAccountMed25519OrString<'a> {
            Str(&'a str),
            String(String),
            MuxedAccountMed25519(MuxedAccountMed25519),
        }
        match MuxedAccountMed25519OrString::deserialize(deserializer)? {
            MuxedAccountMed25519OrString::Str(s) => s.parse().map_err(serde::de::Error::custom),
            MuxedAccountMed25519OrString::String(s) => s.parse().map_err(serde::de::Error::custom),
            MuxedAccountMed25519OrString::MuxedAccountMed25519(MuxedAccountMed25519 {
                id,
                ed25519,
            }) => Ok(self::MuxedAccountMed25519 { id, ed25519 }),
        }
    }
}

#[cfg(feature = "alloc")]
impl IntoOwned for MuxedAccountMed25519 {
    type Owned = MuxedAccountMed25519;
    fn into_owned(self) -> MuxedAccountMed25519 {
        self
    }
}

#[cfg(feature = "const")]
impl MuxedAccountMed25519 {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_muxed_account_med25519(self);
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
        w.write_type_muxed_account_med25519(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`MuxedAccountMed25519`], mirroring `<MuxedAccountMed25519 as WriteXdr>::write_xdr`.
    pub const fn write_type_muxed_account_med25519(&mut self, v: &MuxedAccountMed25519) {
        self.write_u64(v.id);
        self.write_type_uint256(&v.ed25519);
    }
}
