#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// SorobanAuthorizedInvocation is an XDR Struct defined as:
///
/// ```text
/// struct SorobanAuthorizedInvocation
/// {
///     SorobanAuthorizedFunction function;
///     SorobanAuthorizedInvocation subInvocations<>;
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
pub struct SorobanAuthorizedInvocation {
    pub function: SorobanAuthorizedFunction,
    pub sub_invocations: VecM<SorobanAuthorizedInvocation>,
}

impl ReadXdr for SorobanAuthorizedInvocation {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                function: SorobanAuthorizedFunction::read_xdr(r)?,
                sub_invocations: VecM::<SorobanAuthorizedInvocation>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for SorobanAuthorizedInvocation {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.function.write_xdr(w)?;
            self.sub_invocations.write_xdr(w)?;
            Ok(())
        })
    }
}

/// SorobanAuthorizedInvocationRef is a borrowing equivalent of [`SorobanAuthorizedInvocation`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SorobanAuthorizedInvocationRef<'a> {
    pub function: SorobanAuthorizedFunctionRef<'a>,
    pub sub_invocations: VecMRef<'a, SorobanAuthorizedInvocationRef<'a>>,
}

#[cfg(feature = "alloc")]
impl From<&SorobanAuthorizedInvocationRef<'_>> for SorobanAuthorizedInvocation {
    #[must_use]
    fn from(v: &SorobanAuthorizedInvocationRef<'_>) -> Self {
        Self {
            function: (&v.function).into(),
            sub_invocations: v.sub_invocations.to_vecm_from(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<SorobanAuthorizedInvocationRef<'_>> for SorobanAuthorizedInvocation {
    #[must_use]
    fn from(v: SorobanAuthorizedInvocationRef<'_>) -> Self {
        Self::from(&v)
    }
}

impl WriteXdr for SorobanAuthorizedInvocationRef<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.function.write_xdr(w)?;
            self.sub_invocations.write_xdr(w)?;
            Ok(())
        })
    }
}
