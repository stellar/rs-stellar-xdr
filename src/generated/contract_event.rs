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

/// ContractEventRef is a borrowing equivalent of [`ContractEvent`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ContractEventRef<'a> {
    pub ext: ExtensionPoint,
    pub contract_id: Option<ContractId>,
    pub type_: ContractEventType,
    pub body: ContractEventBodyRef<'a>,
}

#[cfg(feature = "alloc")]
impl From<&ContractEventRef<'_>> for ContractEvent {
    #[must_use]
    fn from(v: &ContractEventRef<'_>) -> Self {
        Self {
            ext: v.ext.clone(),
            contract_id: v.contract_id.clone(),
            type_: v.type_,
            body: (&v.body).into(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<ContractEventRef<'_>> for ContractEvent {
    #[must_use]
    fn from(v: ContractEventRef<'_>) -> Self {
        Self::from(&v)
    }
}

impl WriteXdr for ContractEventRef<'_> {
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
