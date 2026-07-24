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

impl ContractEvent {
    /// Serialize this value as XDR into a [`ConstWriter`] using only const
    /// operations. This is the const implementation underlying `to_xdr`.
    #[cfg(feature = "std")]
    pub const fn const_to_xdr(&self, w: &mut ConstWriter) {
        w.enter_depth();
        self.ext.const_to_xdr(w);
        {
            w.enter_depth();
            match &self.contract_id {
                Some(__v0) => {
                    w.write_u32(1);
                    __v0.const_to_xdr(w);
                }
                None => {
                    w.write_u32(0);
                }
            }
            w.leave_depth();
        }
        self.type_.const_to_xdr(w);
        self.body.const_to_xdr(w);
        w.leave_depth();
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

    #[cfg(feature = "std")]
    fn to_xdr(&self, limits: Limits) -> Result<Vec<u8>, Error> {
        to_xdr_via_const(self, &limits, Self::const_to_xdr)
    }
}
