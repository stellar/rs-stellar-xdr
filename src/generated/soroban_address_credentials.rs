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

/// SorobanAddressCredentialsView is a borrowing equivalent of [`SorobanAddressCredentials`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SorobanAddressCredentialsView<'a> {
    pub address: ScAddress,
    pub nonce: i64,
    pub signature_expiration_ledger: u32,
    pub signature: ScValView<'a>,
}

#[cfg(feature = "alloc")]
impl IntoOwned for SorobanAddressCredentialsView<'_> {
    type Owned = SorobanAddressCredentials;
    fn into_owned(&self) -> SorobanAddressCredentials {
        SorobanAddressCredentials {
            address: IntoOwned::into_owned(&self.address),
            nonce: IntoOwned::into_owned(&self.nonce),
            signature_expiration_ledger: IntoOwned::into_owned(&self.signature_expiration_ledger),
            signature: IntoOwned::into_owned(&self.signature),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&SorobanAddressCredentialsView<'_>> for SorobanAddressCredentials {
    #[must_use]
    fn from(v: &SorobanAddressCredentialsView<'_>) -> Self {
        IntoOwned::into_owned(v)
    }
}

#[cfg(feature = "alloc")]
impl From<SorobanAddressCredentialsView<'_>> for SorobanAddressCredentials {
    #[must_use]
    fn from(v: SorobanAddressCredentialsView<'_>) -> Self {
        IntoOwned::into_owned(&v)
    }
}

impl WriteXdr for SorobanAddressCredentialsView<'_> {
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
