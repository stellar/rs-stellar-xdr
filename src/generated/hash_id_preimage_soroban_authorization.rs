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

/// HashIdPreimageSorobanAuthorizationRef is a borrowing equivalent of [`HashIdPreimageSorobanAuthorization`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct HashIdPreimageSorobanAuthorizationRef<'a> {
    pub network_id: Hash,
    pub nonce: i64,
    pub signature_expiration_ledger: u32,
    pub invocation: SorobanAuthorizedInvocationRef<'a>,
}

#[cfg(feature = "alloc")]
impl IntoOwned for HashIdPreimageSorobanAuthorizationRef<'_> {
    type Owned = HashIdPreimageSorobanAuthorization;
    fn into_owned(self) -> HashIdPreimageSorobanAuthorization {
        HashIdPreimageSorobanAuthorization {
            network_id: self.network_id.into_owned(),
            nonce: self.nonce.into_owned(),
            signature_expiration_ledger: self.signature_expiration_ledger.into_owned(),
            invocation: self.invocation.into_owned(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&HashIdPreimageSorobanAuthorizationRef<'_>> for HashIdPreimageSorobanAuthorization {
    #[must_use]
    fn from(v: &HashIdPreimageSorobanAuthorizationRef<'_>) -> Self {
        v.into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<HashIdPreimageSorobanAuthorizationRef<'_>> for HashIdPreimageSorobanAuthorization {
    #[must_use]
    fn from(v: HashIdPreimageSorobanAuthorizationRef<'_>) -> Self {
        v.into_owned()
    }
}

impl WriteXdr for HashIdPreimageSorobanAuthorizationRef<'_> {
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

#[cfg(feature = "const")]
impl HashIdPreimageSorobanAuthorizationRef<'_> {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_hash_id_preimage_soroban_authorization(self);
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
        w.write_type_hash_id_preimage_soroban_authorization(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`HashIdPreimageSorobanAuthorization`], mirroring `<HashIdPreimageSorobanAuthorization as WriteXdr>::write_xdr`.
    pub const fn write_type_hash_id_preimage_soroban_authorization(
        &mut self,
        v: &HashIdPreimageSorobanAuthorizationRef<'_>,
    ) {
        self.write_type_hash(&v.network_id);
        self.write_i64(v.nonce);
        self.write_u32(v.signature_expiration_ledger);
        self.write_type_soroban_authorized_invocation(&v.invocation);
    }
}
