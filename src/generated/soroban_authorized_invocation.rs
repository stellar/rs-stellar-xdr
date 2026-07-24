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

impl SorobanAuthorizedInvocation {
    /// Serialize this value as XDR into a [`ConstWriter`] using only const
    /// operations. This is the const implementation underlying `to_xdr`.
    #[cfg(feature = "std")]
    pub const fn const_write_xdr(&self, w: &mut ConstWriter) {
        w.enter_depth();
        self.function.const_write_xdr(w);
        {
            w.enter_depth();
            let __s0 = self.sub_invocations.0.as_slice();
            let __len0 = __s0.len();
            w.write_length_prefix(__len0);
            let mut __i0 = 0usize;
            while __i0 < __len0 {
                __s0[__i0].const_write_xdr(w);
                __i0 += 1;
            }
            w.leave_depth();
        }
        w.leave_depth();
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

    #[cfg(feature = "std")]
    fn to_xdr(&self, limits: Limits) -> Result<Vec<u8>, Error> {
        to_xdr_via_const(self, &limits, Self::const_write_xdr)
    }
}
