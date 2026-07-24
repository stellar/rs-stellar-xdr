#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// SignerKeyEd25519SignedPayload is an XDR NestedStruct defined as:
///
/// ```text
/// struct
///     {
///         /* Public key that must sign the payload. */
///         uint256 ed25519;
///         /* Payload to be raw signed by ed25519. */
///         opaque payload<64>;
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
pub struct SignerKeyEd25519SignedPayload {
    pub ed25519: Uint256,
    pub payload: BytesM<64>,
}

impl ReadXdr for SignerKeyEd25519SignedPayload {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                ed25519: Uint256::read_xdr(r)?,
                payload: BytesM::<64>::read_xdr(r)?,
            })
        })
    }
}

impl SignerKeyEd25519SignedPayload {
    /// Serialize this value as XDR into a [`ConstWriter`] using only const
    /// operations. This is the const implementation underlying `to_xdr`.
    #[cfg(feature = "std")]
    pub const fn const_write_xdr(&self, w: &mut ConstWriter) {
        w.enter_depth();
        self.ed25519.const_write_xdr(w);
        w.write_len_prefixed(self.payload.0.as_slice());
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

impl WriteXdr for SignerKeyEd25519SignedPayload {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        write_xdr_via_const(self, w, Self::const_write_xdr)
    }
}
#[cfg(all(feature = "serde", feature = "alloc"))]
impl<'de> serde::Deserialize<'de> for SignerKeyEd25519SignedPayload {
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::Deserialize;
        #[derive(Deserialize)]
        struct SignerKeyEd25519SignedPayload {
            ed25519: Uint256,
            payload: BytesM<64>,
        }
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum SignerKeyEd25519SignedPayloadOrString<'a> {
            Str(&'a str),
            String(String),
            SignerKeyEd25519SignedPayload(SignerKeyEd25519SignedPayload),
        }
        match SignerKeyEd25519SignedPayloadOrString::deserialize(deserializer)? {
            SignerKeyEd25519SignedPayloadOrString::Str(s) => {
                s.parse().map_err(serde::de::Error::custom)
            }
            SignerKeyEd25519SignedPayloadOrString::String(s) => {
                s.parse().map_err(serde::de::Error::custom)
            }
            SignerKeyEd25519SignedPayloadOrString::SignerKeyEd25519SignedPayload(
                SignerKeyEd25519SignedPayload { ed25519, payload },
            ) => Ok(self::SignerKeyEd25519SignedPayload { ed25519, payload }),
        }
    }
}
