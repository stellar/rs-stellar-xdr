#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// MuxedEd25519Account is an XDR Struct defined as:
///
/// ```text
/// struct MuxedEd25519Account
/// {
///     uint64 id;
///     uint256 ed25519;
/// };
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
pub struct MuxedEd25519Account {
    pub id: u64,
    pub ed25519: Uint256,
}

impl ReadXdr for MuxedEd25519Account {
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

impl MuxedEd25519Account {
    /// Serialize this value as XDR into a [`ConstWriter`] using only const
    /// operations. This is the const implementation underlying `to_xdr`.
    #[cfg(feature = "std")]
    pub const fn const_write_xdr(&self, w: &mut ConstWriter) {
        w.enter_depth();
        w.write_u64(self.id);
        self.ed25519.const_write_xdr(w);
        w.leave_depth();
    }
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::to_xdr_array`] at compile time.
    #[cfg(feature = "std")]
    #[must_use]
    pub const fn xdr_len(&self) -> usize {
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
    /// operations.
    ///
    /// `N` must equal [`Self::xdr_len`]. It is intended for callers, such as a
    /// proc-macro, that compute the length with `xdr_len` and pass it as `N`;
    /// `to_xdr_array` itself does not need to call `xdr_len`.
    ///
    /// # Panics
    ///
    /// Panics if `N` does not equal the value's [`Self::xdr_len`].
    #[cfg(feature = "std")]
    #[must_use]
    pub const fn to_xdr_array<const N: usize>(&self) -> [u8; N] {
        let limits = Limits {
            depth: u32::MAX,
            len: usize::MAX,
        };
        let mut buf = [0u8; N];
        let mut w = ConstWriter::new(&mut buf, &limits);
        self.const_write_xdr(&mut w);
        assert!(
            w.position() == N,
            "to_xdr_array: N does not equal the XDR-encoded length"
        );
        buf
    }
}

impl WriteXdr for MuxedEd25519Account {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        write_xdr_via_const(self, w, Self::const_write_xdr)
    }
}
#[cfg(all(feature = "serde", feature = "alloc"))]
impl<'de> serde::Deserialize<'de> for MuxedEd25519Account {
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::Deserialize;
        #[derive(Deserialize)]
        struct MuxedEd25519Account {
            id: u64,
            ed25519: Uint256,
        }
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum MuxedEd25519AccountOrString<'a> {
            Str(&'a str),
            String(String),
            MuxedEd25519Account(MuxedEd25519Account),
        }
        match MuxedEd25519AccountOrString::deserialize(deserializer)? {
            MuxedEd25519AccountOrString::Str(s) => s.parse().map_err(serde::de::Error::custom),
            MuxedEd25519AccountOrString::String(s) => s.parse().map_err(serde::de::Error::custom),
            MuxedEd25519AccountOrString::MuxedEd25519Account(MuxedEd25519Account {
                id,
                ed25519,
            }) => Ok(self::MuxedEd25519Account { id, ed25519 }),
        }
    }
}
