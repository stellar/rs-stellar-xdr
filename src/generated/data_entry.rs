#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// DataEntry is an XDR Struct defined as:
///
/// ```text
/// struct DataEntry
/// {
///     AccountID accountID; // account this data belongs to
///     string64 dataName;
///     DataValue dataValue;
///
///     // reserved for future use
///     union switch (int v)
///     {
///     case 0:
///         void;
///     }
///     ext;
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
pub struct DataEntry {
    pub account_id: AccountId,
    pub data_name: String64,
    pub data_value: DataValue,
    pub ext: DataEntryExt,
}

impl ReadXdr for DataEntry {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                account_id: AccountId::read_xdr(r)?,
                data_name: String64::read_xdr(r)?,
                data_value: DataValue::read_xdr(r)?,
                ext: DataEntryExt::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for DataEntry {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.account_id.write_xdr(w)?;
            self.data_name.write_xdr(w)?;
            self.data_value.write_xdr(w)?;
            self.ext.write_xdr(w)?;
            Ok(())
        })
    }
}

/// DataEntryView is a borrowing equivalent of [`DataEntry`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct DataEntryView<'a> {
    pub account_id: AccountId,
    pub data_name: String64View<'a>,
    pub data_value: DataValueView<'a>,
    pub ext: DataEntryExt,
}

#[cfg(feature = "alloc")]
impl IntoOwned for DataEntryView<'_> {
    type Owned = DataEntry;
    fn into_owned(&self) -> DataEntry {
        DataEntry {
            account_id: self.account_id.into_owned(),
            data_name: self.data_name.into_owned(),
            data_value: self.data_value.into_owned(),
            ext: self.ext.into_owned(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&DataEntryView<'_>> for DataEntry {
    #[must_use]
    fn from(v: &DataEntryView<'_>) -> Self {
        v.into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<DataEntryView<'_>> for DataEntry {
    #[must_use]
    fn from(v: DataEntryView<'_>) -> Self {
        v.into_owned()
    }
}

impl WriteXdr for DataEntryView<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.account_id.write_xdr(w)?;
            self.data_name.write_xdr(w)?;
            self.data_value.write_xdr(w)?;
            self.ext.write_xdr(w)?;
            Ok(())
        })
    }
}
