#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// SorobanAddressCredentialsWithDelegates is an XDR Struct defined as:
///
/// ```text
/// struct SorobanAddressCredentialsWithDelegates
/// {
///     SorobanAddressCredentials addressCredentials;
///     SorobanDelegateSignature delegates<>;
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
pub struct SorobanAddressCredentialsWithDelegates {
    pub address_credentials: SorobanAddressCredentials,
    pub delegates: VecM<SorobanDelegateSignature>,
}

impl ReadXdr for SorobanAddressCredentialsWithDelegates {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                address_credentials: SorobanAddressCredentials::read_xdr(r)?,
                delegates: VecM::<SorobanDelegateSignature>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for SorobanAddressCredentialsWithDelegates {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.address_credentials.write_xdr(w)?;
            self.delegates.write_xdr(w)?;
            Ok(())
        })
    }
}

/// SorobanAddressCredentialsWithDelegatesView is a borrowing equivalent of [`SorobanAddressCredentialsWithDelegates`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SorobanAddressCredentialsWithDelegatesView<'a> {
    pub address_credentials: SorobanAddressCredentialsView<'a>,
    pub delegates: VecMView<'a, SorobanDelegateSignatureView<'a>>,
}

#[cfg(feature = "alloc")]
impl IntoOwned for SorobanAddressCredentialsWithDelegatesView<'_> {
    type Owned = SorobanAddressCredentialsWithDelegates;
    fn into_owned(&self) -> SorobanAddressCredentialsWithDelegates {
        SorobanAddressCredentialsWithDelegates {
            address_credentials: IntoOwned::into_owned(&self.address_credentials),
            delegates: IntoOwned::into_owned(&self.delegates),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&SorobanAddressCredentialsWithDelegatesView<'_>>
    for SorobanAddressCredentialsWithDelegates
{
    #[must_use]
    fn from(v: &SorobanAddressCredentialsWithDelegatesView<'_>) -> Self {
        IntoOwned::into_owned(v)
    }
}

#[cfg(feature = "alloc")]
impl From<SorobanAddressCredentialsWithDelegatesView<'_>>
    for SorobanAddressCredentialsWithDelegates
{
    #[must_use]
    fn from(v: SorobanAddressCredentialsWithDelegatesView<'_>) -> Self {
        IntoOwned::into_owned(&v)
    }
}

impl WriteXdr for SorobanAddressCredentialsWithDelegatesView<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.address_credentials.write_xdr(w)?;
            self.delegates.write_xdr(w)?;
            Ok(())
        })
    }
}
