#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// AuthCert is an XDR Struct defined as:
///
/// ```text
/// struct AuthCert
/// {
///     Curve25519Public pubkey;
///     uint64 expiration;
///     Signature sig;
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
pub struct AuthCert {
    pub pubkey: Curve25519Public,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub expiration: u64,
    pub sig: Signature,
}

impl ReadXdr for AuthCert {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                pubkey: Curve25519Public::read_xdr(r)?,
                expiration: u64::read_xdr(r)?,
                sig: Signature::read_xdr(r)?,
            })
        })
    }
}

impl AuthCert {
    /// Serialize this value as XDR into a [`ConstWriter`] using only const
    /// operations. This is the const implementation underlying `to_xdr`.
    #[cfg(feature = "std")]
    pub const fn const_write_xdr(&self, w: &mut ConstWriter) {
        w.enter_depth();
        self.pubkey.const_write_xdr(w);
        w.write_u64(self.expiration);
        self.sig.const_write_xdr(w);
        w.leave_depth();
    }
}

impl WriteXdr for AuthCert {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.pubkey.write_xdr(w)?;
            self.expiration.write_xdr(w)?;
            self.sig.write_xdr(w)?;
            Ok(())
        })
    }

    #[cfg(feature = "std")]
    fn to_xdr(&self, limits: Limits) -> Result<Vec<u8>, Error> {
        to_xdr_via_const(self, &limits, Self::const_write_xdr)
    }
}
