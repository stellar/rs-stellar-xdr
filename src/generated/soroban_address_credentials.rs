#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// SorobanAddressCredentials is an XDR Struct defined as:
///
/// ```text
/// struct SorobanAddressCredentials
/// {
///     SCAddress address;
///     int64 nonce;
///     uint32 signatureExpirationLedger;    
///     SCVal signature;
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
pub struct SorobanAddressCredentials {
    pub address: ScAddress,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub nonce: i64,
    pub signature_expiration_ledger: u32,
    pub signature: ScVal,
}

impl ReadXdr for SorobanAddressCredentials {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                address: ScAddress::read_xdr(r)?,
                nonce: i64::read_xdr(r)?,
                signature_expiration_ledger: u32::read_xdr(r)?,
                signature: ScVal::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for SorobanAddressCredentials {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.address.write_xdr(w)?;
            self.nonce.write_xdr(w)?;
            self.signature_expiration_ledger.write_xdr(w)?;
            self.signature.write_xdr(w)?;
            Ok(())
        })
    }
}

/// SorobanAddressCredentialsRef is a borrowing equivalent of [`SorobanAddressCredentials`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SorobanAddressCredentialsRef<'a> {
    pub address: ScAddress,
    pub nonce: i64,
    pub signature_expiration_ledger: u32,
    pub signature: ScValRef<'a>,
}

#[cfg(feature = "alloc")]
impl IntoOwned for SorobanAddressCredentialsRef<'_> {
    type Owned = SorobanAddressCredentials;
    fn into_owned(self) -> SorobanAddressCredentials {
        SorobanAddressCredentials {
            address: self.address.into_owned(),
            nonce: self.nonce.into_owned(),
            signature_expiration_ledger: self.signature_expiration_ledger.into_owned(),
            signature: self.signature.into_owned(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&SorobanAddressCredentialsRef<'_>> for SorobanAddressCredentials {
    #[must_use]
    fn from(v: &SorobanAddressCredentialsRef<'_>) -> Self {
        v.into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<SorobanAddressCredentialsRef<'_>> for SorobanAddressCredentials {
    #[must_use]
    fn from(v: SorobanAddressCredentialsRef<'_>) -> Self {
        v.into_owned()
    }
}

impl WriteXdr for SorobanAddressCredentialsRef<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.address.write_xdr(w)?;
            self.nonce.write_xdr(w)?;
            self.signature_expiration_ledger.write_xdr(w)?;
            self.signature.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "const")]
impl SorobanAddressCredentialsRef<'_> {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_soroban_address_credentials(self);
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
        w.write_type_soroban_address_credentials(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`SorobanAddressCredentials`], mirroring `<SorobanAddressCredentials as WriteXdr>::write_xdr`.
    pub const fn write_type_soroban_address_credentials(
        &mut self,
        v: &SorobanAddressCredentialsRef<'_>,
    ) {
        self.write_type_sc_address(&v.address);
        self.write_i64(v.nonce);
        self.write_u32(v.signature_expiration_ledger);
        self.write_type_sc_val(&v.signature);
    }
}
