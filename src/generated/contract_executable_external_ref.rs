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
impl WriteXdr for ContractExecutableExternalRef {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.executable_owner.write_xdr(w)?;
            self.tag.write_xdr(w)?;
            Ok(())
        })
    }
}
