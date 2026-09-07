#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// HashIdPreimageSorobanAuthorizationWithAddress is an XDR NestedStruct defined as:
///
/// ```text
/// struct
///     {
///         Hash networkID;
///         int64 nonce;
///         uint32 signatureExpirationLedger;
///         SCAddress address;
///         SorobanAuthorizedInvocation invocation;
///     }
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
pub struct HashIdPreimageSorobanAuthorizationWithAddress {
    pub network_id: Hash,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub nonce: i64,
    pub signature_expiration_ledger: u32,
    pub address: ScAddress,
    pub invocation: SorobanAuthorizedInvocation,
}

impl ReadXdr for HashIdPreimageSorobanAuthorizationWithAddress {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                network_id: Hash::read_xdr(r)?,
                nonce: i64::read_xdr(r)?,
                signature_expiration_ledger: u32::read_xdr(r)?,
                address: ScAddress::read_xdr(r)?,
                invocation: SorobanAuthorizedInvocation::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for HashIdPreimageSorobanAuthorizationWithAddress {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.network_id.write_xdr(w)?;
            self.nonce.write_xdr(w)?;
            self.signature_expiration_ledger.write_xdr(w)?;
            self.address.write_xdr(w)?;
            self.invocation.write_xdr(w)?;
            Ok(())
        })
    }
}

/// HashIdPreimageSorobanAuthorizationWithAddressView is a borrowing equivalent of [`HashIdPreimageSorobanAuthorizationWithAddress`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct HashIdPreimageSorobanAuthorizationWithAddressView<'a> {
    pub network_id: Hash,
    pub nonce: i64,
    pub signature_expiration_ledger: u32,
    pub address: ScAddress,
    pub invocation: SorobanAuthorizedInvocationView<'a>,
}

#[cfg(feature = "alloc")]
impl IntoOwned for HashIdPreimageSorobanAuthorizationWithAddressView<'_> {
    type Owned = HashIdPreimageSorobanAuthorizationWithAddress;
    fn into_owned(self) -> HashIdPreimageSorobanAuthorizationWithAddress {
        HashIdPreimageSorobanAuthorizationWithAddress {
            network_id: self.network_id.into_owned(),
            nonce: self.nonce.into_owned(),
            signature_expiration_ledger: self.signature_expiration_ledger.into_owned(),
            address: self.address.into_owned(),
            invocation: self.invocation.into_owned(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&HashIdPreimageSorobanAuthorizationWithAddressView<'_>>
    for HashIdPreimageSorobanAuthorizationWithAddress
{
    #[must_use]
    fn from(v: &HashIdPreimageSorobanAuthorizationWithAddressView<'_>) -> Self {
        v.clone().into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<HashIdPreimageSorobanAuthorizationWithAddressView<'_>>
    for HashIdPreimageSorobanAuthorizationWithAddress
{
    #[must_use]
    fn from(v: HashIdPreimageSorobanAuthorizationWithAddressView<'_>) -> Self {
        v.into_owned()
    }
}

impl WriteXdr for HashIdPreimageSorobanAuthorizationWithAddressView<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.network_id.write_xdr(w)?;
            self.nonce.write_xdr(w)?;
            self.signature_expiration_ledger.write_xdr(w)?;
            self.address.write_xdr(w)?;
            self.invocation.write_xdr(w)?;
            Ok(())
        })
    }
}
