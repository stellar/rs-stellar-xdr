#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// HashIdPreimageSorobanAuthorization is an XDR NestedStruct defined as:
///
/// ```text
/// struct
///     {
///         Hash networkID;
///         int64 nonce;
///         uint32 signatureExpirationLedger;
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
pub struct HashIdPreimageSorobanAuthorization {
    pub network_id: Hash,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub nonce: i64,
    pub signature_expiration_ledger: u32,
    pub invocation: SorobanAuthorizedInvocation,
}

impl ReadXdr for HashIdPreimageSorobanAuthorization {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                network_id: Hash::read_xdr(r)?,
                nonce: i64::read_xdr(r)?,
                signature_expiration_ledger: u32::read_xdr(r)?,
                invocation: SorobanAuthorizedInvocation::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for HashIdPreimageSorobanAuthorization {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.network_id.write_xdr(w)?;
            self.nonce.write_xdr(w)?;
            self.signature_expiration_ledger.write_xdr(w)?;
            self.invocation.write_xdr(w)?;
            Ok(())
        })
    }
}

/// HashIdPreimageSorobanAuthorizationView is a borrowing equivalent of [`HashIdPreimageSorobanAuthorization`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct HashIdPreimageSorobanAuthorizationView<'a> {
    pub network_id: Hash,
    pub nonce: i64,
    pub signature_expiration_ledger: u32,
    pub invocation: SorobanAuthorizedInvocationView<'a>,
}

#[cfg(feature = "alloc")]
impl IntoOwned for HashIdPreimageSorobanAuthorizationView<'_> {
    type Owned = HashIdPreimageSorobanAuthorization;
    fn into_owned(&self) -> HashIdPreimageSorobanAuthorization {
        HashIdPreimageSorobanAuthorization {
            network_id: IntoOwned::into_owned(&self.network_id),
            nonce: IntoOwned::into_owned(&self.nonce),
            signature_expiration_ledger: IntoOwned::into_owned(&self.signature_expiration_ledger),
            invocation: IntoOwned::into_owned(&self.invocation),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&HashIdPreimageSorobanAuthorizationView<'_>> for HashIdPreimageSorobanAuthorization {
    #[must_use]
    fn from(v: &HashIdPreimageSorobanAuthorizationView<'_>) -> Self {
        IntoOwned::into_owned(v)
    }
}

#[cfg(feature = "alloc")]
impl From<HashIdPreimageSorobanAuthorizationView<'_>> for HashIdPreimageSorobanAuthorization {
    #[must_use]
    fn from(v: HashIdPreimageSorobanAuthorizationView<'_>) -> Self {
        IntoOwned::into_owned(&v)
    }
}

impl WriteXdr for HashIdPreimageSorobanAuthorizationView<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.network_id.write_xdr(w)?;
            self.nonce.write_xdr(w)?;
            self.signature_expiration_ledger.write_xdr(w)?;
            self.invocation.write_xdr(w)?;
            Ok(())
        })
    }
}
