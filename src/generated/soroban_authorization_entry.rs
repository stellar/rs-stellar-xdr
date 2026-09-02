#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// SorobanAuthorizationEntry is an XDR Struct defined as:
///
/// ```text
/// struct SorobanAuthorizationEntry
/// {
///     SorobanCredentials credentials;
///     SorobanAuthorizedInvocation rootInvocation;
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
pub struct SorobanAuthorizationEntry {
    pub credentials: SorobanCredentials,
    pub root_invocation: SorobanAuthorizedInvocation,
}

impl ReadXdr for SorobanAuthorizationEntry {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                credentials: SorobanCredentials::read_xdr(r)?,
                root_invocation: SorobanAuthorizedInvocation::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for SorobanAuthorizationEntry {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.credentials.write_xdr(w)?;
            self.root_invocation.write_xdr(w)?;
            Ok(())
        })
    }
}

/// SorobanAuthorizationEntryRef is a borrowing equivalent of [`SorobanAuthorizationEntry`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SorobanAuthorizationEntryRef<'a> {
    pub credentials: SorobanCredentialsRef<'a>,
    pub root_invocation: SorobanAuthorizedInvocationRef<'a>,
}

#[cfg(feature = "alloc")]
impl IntoOwned for SorobanAuthorizationEntryRef<'_> {
    type Owned = SorobanAuthorizationEntry;
    fn into_owned(self) -> SorobanAuthorizationEntry {
        SorobanAuthorizationEntry {
            credentials: self.credentials.into_owned(),
            root_invocation: self.root_invocation.into_owned(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&SorobanAuthorizationEntryRef<'_>> for SorobanAuthorizationEntry {
    #[must_use]
    fn from(v: &SorobanAuthorizationEntryRef<'_>) -> Self {
        v.into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<SorobanAuthorizationEntryRef<'_>> for SorobanAuthorizationEntry {
    #[must_use]
    fn from(v: SorobanAuthorizationEntryRef<'_>) -> Self {
        v.into_owned()
    }
}

impl WriteXdr for SorobanAuthorizationEntryRef<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.credentials.write_xdr(w)?;
            self.root_invocation.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "const")]
impl SorobanAuthorizationEntryView<'_> {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_soroban_authorization_entry(self);
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
        w.write_type_soroban_authorization_entry(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`SorobanAuthorizationEntry`], mirroring `<SorobanAuthorizationEntry as WriteXdr>::write_xdr`.
    pub const fn write_type_soroban_authorization_entry(
        &mut self,
        v: &SorobanAuthorizationEntryView<'_>,
    ) {
        self.write_type_soroban_credentials(&v.credentials);
        self.write_type_soroban_authorized_invocation(&v.root_invocation);
    }

    /// Serializes a variable-length array of [`SorobanAuthorizationEntry`], mirroring `<VecM<SorobanAuthorizationEntry, MAX> as WriteXdr>::write_xdr`.
    pub const fn write_type_vec_soroban_authorization_entry<const MAX: u32>(
        &mut self,
        v: &VecMView<'_, SorobanAuthorizationEntryView<'_>, MAX>,
    ) {
        let s = v.as_slice();
        let len = s.len();
        self.write_len(len);
        let mut i = 0usize;
        while i < len {
            self.write_type_soroban_authorization_entry(&s[i]);
            i += 1;
        }
    }
}
