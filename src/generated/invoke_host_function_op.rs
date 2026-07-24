#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// InvokeHostFunctionOp is an XDR Struct defined as:
///
/// ```text
/// struct InvokeHostFunctionOp
/// {
///     // Host function to invoke.
///     HostFunction hostFunction;
///     // Per-address authorizations for this host function.
///     SorobanAuthorizationEntry auth<>;
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
pub struct InvokeHostFunctionOp {
    pub host_function: HostFunction,
    pub auth: VecM<SorobanAuthorizationEntry>,
}

impl ReadXdr for InvokeHostFunctionOp {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                host_function: HostFunction::read_xdr(r)?,
                auth: VecM::<SorobanAuthorizationEntry>::read_xdr(r)?,
            })
        })
    }
}

impl InvokeHostFunctionOp {
    /// Serialize this value as XDR into a [`ConstWriter`] using only const
    /// operations. This is the const implementation underlying `to_xdr`.
    #[cfg(feature = "std")]
    pub const fn const_to_xdr(&self, w: &mut ConstWriter) {
        w.enter_depth();
        self.host_function.const_to_xdr(w);
        {
            w.enter_depth();
            let __s0 = self.auth.0.as_slice();
            let __len0 = __s0.len();
            w.write_length_prefix(__len0);
            let mut __i0 = 0usize;
            while __i0 < __len0 {
                __s0[__i0].const_to_xdr(w);
                __i0 += 1;
            }
            w.leave_depth();
        }
        w.leave_depth();
    }
}

impl WriteXdr for InvokeHostFunctionOp {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.host_function.write_xdr(w)?;
            self.auth.write_xdr(w)?;
            Ok(())
        })
    }

    #[cfg(feature = "std")]
    fn to_xdr(&self, limits: Limits) -> Result<Vec<u8>, Error> {
        to_xdr_via_const(self, &limits, Self::const_to_xdr)
    }
}
