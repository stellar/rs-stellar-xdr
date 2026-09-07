#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ManageDataOp is an XDR Struct defined as:
///
/// ```text
/// struct ManageDataOp
/// {
///     string64 dataName;
///     DataValue* dataValue; // set to null to clear
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
pub struct ManageDataOp {
    pub data_name: String64,
    pub data_value: Option<DataValue>,
}

impl ReadXdr for ManageDataOp {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                data_name: String64::read_xdr(r)?,
                data_value: Option::<DataValue>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ManageDataOp {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.data_name.write_xdr(w)?;
            self.data_value.write_xdr(w)?;
            Ok(())
        })
    }
}

/// ManageDataOpRef is a borrowing equivalent of [`ManageDataOp`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ManageDataOpRef<'a> {
    pub data_name: String64Ref<'a>,
    pub data_value: Option<DataValueRef<'a>>,
}

#[cfg(feature = "alloc")]
impl IntoOwned for ManageDataOpRef<'_> {
    type Owned = ManageDataOp;
    fn into_owned(self) -> ManageDataOp {
        ManageDataOp {
            data_name: self.data_name.into_owned(),
            data_value: self.data_value.into_owned(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&ManageDataOpRef<'_>> for ManageDataOp {
    #[must_use]
    fn from(v: &ManageDataOpRef<'_>) -> Self {
        v.into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<ManageDataOpRef<'_>> for ManageDataOp {
    #[must_use]
    fn from(v: ManageDataOpRef<'_>) -> Self {
        v.into_owned()
    }
}

impl WriteXdr for ManageDataOpRef<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.data_name.write_xdr(w)?;
            self.data_value.write_xdr(w)?;
            Ok(())
        })
    }
}
