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
#[cfg_eval::cfg_eval]
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

impl WriteXdr for InvokeHostFunctionOp {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.host_function.write_xdr(w)?;
            self.auth.write_xdr(w)?;
            Ok(())
        })
    }
}
