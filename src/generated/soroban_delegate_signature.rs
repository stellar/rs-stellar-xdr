#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// SorobanDelegateSignature is an XDR Struct defined as:
///
/// ```text
/// struct SorobanDelegateSignature
/// {
///     SCAddress address;
///     SCVal signature;
///     SorobanDelegateSignature nestedDelegates<>;
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
pub struct SorobanDelegateSignature {
    pub address: ScAddress,
    pub signature: ScVal,
    pub nested_delegates: VecM<SorobanDelegateSignature>,
}

impl ReadXdr for SorobanDelegateSignature {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                address: ScAddress::read_xdr(r)?,
                signature: ScVal::read_xdr(r)?,
                nested_delegates: VecM::<SorobanDelegateSignature>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for SorobanDelegateSignature {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.address.write_xdr(w)?;
            self.signature.write_xdr(w)?;
            self.nested_delegates.write_xdr(w)?;
            Ok(())
        })
    }
}

/// SorobanDelegateSignatureRef is a borrowing equivalent of [`SorobanDelegateSignature`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SorobanDelegateSignatureRef<'a> {
    pub address: ScAddress,
    pub signature: ScValRef<'a>,
    pub nested_delegates: VecMRef<'a, SorobanDelegateSignatureRef<'a>>,
}

#[cfg(feature = "alloc")]
impl IntoOwned for SorobanDelegateSignatureRef<'_> {
    type Owned = SorobanDelegateSignature;
    fn into_owned(self) -> SorobanDelegateSignature {
        SorobanDelegateSignature {
            address: self.address.into_owned(),
            signature: self.signature.into_owned(),
            nested_delegates: self.nested_delegates.into_owned(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&SorobanDelegateSignatureRef<'_>> for SorobanDelegateSignature {
    #[must_use]
    fn from(v: &SorobanDelegateSignatureRef<'_>) -> Self {
        v.into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<SorobanDelegateSignatureRef<'_>> for SorobanDelegateSignature {
    #[must_use]
    fn from(v: SorobanDelegateSignatureRef<'_>) -> Self {
        v.into_owned()
    }
}

impl WriteXdr for SorobanDelegateSignatureRef<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.address.write_xdr(w)?;
            self.signature.write_xdr(w)?;
            self.nested_delegates.write_xdr(w)?;
            Ok(())
        })
    }
}
