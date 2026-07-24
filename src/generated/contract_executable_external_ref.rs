#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ContractExecutableExternalRef is an XDR Struct defined as:
///
/// ```text
/// struct ContractExecutableExternalRef {
///     SCAddress executable_owner;
///     SCString tag;
/// };
/// ```
///
#[cfg(feature = "cap_0085_executable_ref")]
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
pub struct ContractExecutableExternalRef {
    pub executable_owner: ScAddress,
    pub tag: ScString,
}

#[cfg(feature = "cap_0085_executable_ref")]
impl ReadXdr for ContractExecutableExternalRef {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                executable_owner: ScAddress::read_xdr(r)?,
                tag: ScString::read_xdr(r)?,
            })
        })
    }
}

#[cfg(feature = "cap_0085_executable_ref")]
impl ContractExecutableExternalRef {
    /// Serialize this value as XDR into a [`ConstWriter`] using only const
    /// operations. This is the const implementation underlying `to_xdr`.
    #[cfg(feature = "std")]
    pub const fn const_to_xdr(&self, w: &mut ConstWriter) {
        w.enter_depth();
        self.executable_owner.const_to_xdr(w);
        self.tag.const_to_xdr(w);
        w.leave_depth();
    }
}

#[cfg(feature = "cap_0085_executable_ref")]
impl WriteXdr for ContractExecutableExternalRef {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.executable_owner.write_xdr(w)?;
            self.tag.write_xdr(w)?;
            Ok(())
        })
    }

    #[cfg(feature = "std")]
    fn to_xdr(&self, limits: Limits) -> Result<Vec<u8>, Error> {
        to_xdr_via_const(self, &limits, Self::const_to_xdr)
    }
}
