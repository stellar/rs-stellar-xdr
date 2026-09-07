#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ContractEventV0 is an XDR NestedStruct defined as:
///
/// ```text
/// struct
///         {
///             SCVal topics<>;
///             SCVal data;
///         }
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
pub struct ContractEventV0 {
    pub topics: VecM<ScVal>,
    pub data: ScVal,
}

impl ReadXdr for ContractEventV0 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                topics: VecM::<ScVal>::read_xdr(r)?,
                data: ScVal::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ContractEventV0 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.topics.write_xdr(w)?;
            self.data.write_xdr(w)?;
            Ok(())
        })
    }
}

/// ContractEventV0Ref is a borrowing equivalent of [`ContractEventV0`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ContractEventV0Ref<'a> {
    pub topics: VecMRef<'a, ScValRef<'a>>,
    pub data: ScValRef<'a>,
}

#[cfg(feature = "alloc")]
impl IntoOwned for ContractEventV0Ref<'_> {
    type Owned = ContractEventV0;
    fn into_owned(self) -> ContractEventV0 {
        ContractEventV0 {
            topics: self.topics.into_owned(),
            data: self.data.into_owned(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&ContractEventV0Ref<'_>> for ContractEventV0 {
    #[must_use]
    fn from(v: &ContractEventV0Ref<'_>) -> Self {
        v.into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<ContractEventV0Ref<'_>> for ContractEventV0 {
    #[must_use]
    fn from(v: ContractEventV0Ref<'_>) -> Self {
        v.into_owned()
    }
}

impl WriteXdr for ContractEventV0Ref<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.topics.write_xdr(w)?;
            self.data.write_xdr(w)?;
            Ok(())
        })
    }
}
