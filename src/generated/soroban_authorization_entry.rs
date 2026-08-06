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

/// SorobanAuthorizationEntryView is a borrowing equivalent of [`SorobanAuthorizationEntry`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SorobanAuthorizationEntryView<'a> {
    pub credentials: SorobanCredentialsView<'a>,
    pub root_invocation: SorobanAuthorizedInvocationView<'a>,
}

#[cfg(feature = "alloc")]
impl From<&SorobanAuthorizationEntryView<'_>> for SorobanAuthorizationEntry {
    #[must_use]
    fn from(v: &SorobanAuthorizationEntryView<'_>) -> Self {
        Self {
            credentials: (&v.credentials).into(),
            root_invocation: (&v.root_invocation).into(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<SorobanAuthorizationEntryView<'_>> for SorobanAuthorizationEntry {
    #[must_use]
    fn from(v: SorobanAuthorizationEntryView<'_>) -> Self {
        Self::from(&v)
    }
}

impl WriteXdr for SorobanAuthorizationEntryView<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.credentials.write_xdr(w)?;
            self.root_invocation.write_xdr(w)?;
            Ok(())
        })
    }
}
