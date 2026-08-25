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

/// SorobanDelegateSignatureView is a borrowing equivalent of [`SorobanDelegateSignature`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SorobanDelegateSignatureView<'a> {
    pub address: ScAddress,
    pub signature: ScValView<'a>,
    pub nested_delegates: VecMView<'a, SorobanDelegateSignatureView<'a>>,
}

#[cfg(feature = "alloc")]
impl From<&SorobanDelegateSignatureView<'_>> for SorobanDelegateSignature {
    #[must_use]
    fn from(v: &SorobanDelegateSignatureView<'_>) -> Self {
        Self {
            address: v.address.clone(),
            signature: (&v.signature).into(),
            nested_delegates: v.nested_delegates.to_vecm(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<SorobanDelegateSignatureView<'_>> for SorobanDelegateSignature {
    #[must_use]
    fn from(v: SorobanDelegateSignatureView<'_>) -> Self {
        Self::from(&v)
    }
}

impl WriteXdr for SorobanDelegateSignatureView<'_> {
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
