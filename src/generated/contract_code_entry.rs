#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ContractCodeEntry is an XDR Struct defined as:
///
/// ```text
/// struct ContractCodeEntry {
///     union switch (int v)
///     {
///         case 0:
///             void;
///         case 1:
///             struct
///             {
///                 ExtensionPoint ext;
///                 ContractCodeCostInputs costInputs;
///             } v1;
///     } ext;
///
///     Hash hash;
///     opaque code<>;
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
pub struct ContractCodeEntry {
    pub ext: ContractCodeEntryExt,
    pub hash: Hash,
    pub code: BytesM,
}

impl ReadXdr for ContractCodeEntry {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                ext: ContractCodeEntryExt::read_xdr(r)?,
                hash: Hash::read_xdr(r)?,
                code: BytesM::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ContractCodeEntry {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.ext.write_xdr(w)?;
            self.hash.write_xdr(w)?;
            self.code.write_xdr(w)?;
            Ok(())
        })
    }
}
