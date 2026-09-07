#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ContractEvent is an XDR Struct defined as:
///
/// ```text
/// struct ContractEvent
/// {
///     // We can use this to add more fields, or because it
///     // is first, to change ContractEvent into a union.
///     ExtensionPoint ext;
///
///     ContractID* contractID;
///     ContractEventType type;
///
///     union switch (int v)
///     {
///     case 0:
///         struct
///         {
///             SCVal topics<>;
///             SCVal data;
///         } v0;
///     }
///     body;
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
pub struct ContractEvent {
    pub ext: ExtensionPoint,
    pub contract_id: Option<ContractId>,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde(rename = "type", alias = "type_")
    )]
    #[cfg_attr(feature = "schemars", schemars(rename = "type"))]
    pub type_: ContractEventType,
    pub body: ContractEventBody,
}

impl ReadXdr for ContractEvent {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                ext: ExtensionPoint::read_xdr(r)?,
                contract_id: Option::<ContractId>::read_xdr(r)?,
                type_: ContractEventType::read_xdr(r)?,
                body: ContractEventBody::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ContractEvent {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.ext.write_xdr(w)?;
            self.contract_id.write_xdr(w)?;
            self.type_.write_xdr(w)?;
            self.body.write_xdr(w)?;
            Ok(())
        })
    }
}

/// ContractEventView is a borrowing equivalent of [`ContractEvent`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ContractEventView<'a> {
    pub ext: ExtensionPoint,
    pub contract_id: Option<ContractId>,
    pub type_: ContractEventType,
    pub body: ContractEventBodyView<'a>,
}

#[cfg(feature = "alloc")]
impl IntoOwned for ContractEventView<'_> {
    type Owned = ContractEvent;
    fn into_owned(&self) -> ContractEvent {
        ContractEvent {
            ext: IntoOwned::into_owned(&self.ext),
            contract_id: IntoOwned::into_owned(&self.contract_id),
            type_: IntoOwned::into_owned(&self.type_),
            body: IntoOwned::into_owned(&self.body),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&ContractEventView<'_>> for ContractEvent {
    #[must_use]
    fn from(v: &ContractEventView<'_>) -> Self {
        IntoOwned::into_owned(v)
    }
}

#[cfg(feature = "alloc")]
impl From<ContractEventView<'_>> for ContractEvent {
    #[must_use]
    fn from(v: ContractEventView<'_>) -> Self {
        IntoOwned::into_owned(&v)
    }
}

impl WriteXdr for ContractEventView<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.ext.write_xdr(w)?;
            self.contract_id.write_xdr(w)?;
            self.type_.write_xdr(w)?;
            self.body.write_xdr(w)?;
            Ok(())
        })
    }
}
