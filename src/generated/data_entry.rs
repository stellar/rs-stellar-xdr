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

impl DataEntry {
    /// Serialize this value as XDR into a [`ConstWriter`] using only const
    /// operations. This is the const implementation underlying `to_xdr`.
    #[cfg(feature = "std")]
    pub const fn const_to_xdr(&self, w: &mut ConstWriter) {
        w.enter_depth();
        self.account_id.const_to_xdr(w);
        self.data_name.const_to_xdr(w);
        self.data_value.const_to_xdr(w);
        self.ext.const_to_xdr(w);
        w.leave_depth();
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

    #[cfg(feature = "std")]
    fn to_xdr(&self, limits: Limits) -> Result<Vec<u8>, Error> {
        to_xdr_via_const(self, &limits, Self::const_to_xdr)
    }
}
