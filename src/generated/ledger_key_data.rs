#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// LedgerKeyData is an XDR NestedStruct defined as:
///
/// ```text
/// struct
///     {
///         AccountID accountID;
///         string64 dataName;
///     }
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
pub struct LedgerKeyData {
    pub account_id: AccountId,
    pub data_name: String64,
}

impl ReadXdr for LedgerKeyData {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                account_id: AccountId::read_xdr(r)?,
                data_name: String64::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for LedgerKeyData {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.account_id.write_xdr(w)?;
            self.data_name.write_xdr(w)?;
            Ok(())
        })
    }
}

/// LedgerKeyDataRef is a borrowing equivalent of [`LedgerKeyData`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct LedgerKeyDataRef<'a> {
    pub account_id: AccountId,
    pub data_name: String64Ref<'a>,
}

#[cfg(feature = "alloc")]
impl IntoOwned for LedgerKeyDataRef<'_> {
    type Owned = LedgerKeyData;
    fn into_owned(self) -> LedgerKeyData {
        LedgerKeyData {
            account_id: self.account_id.into_owned(),
            data_name: self.data_name.into_owned(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&LedgerKeyDataRef<'_>> for LedgerKeyData {
    #[must_use]
    fn from(v: &LedgerKeyDataRef<'_>) -> Self {
        v.into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<LedgerKeyDataRef<'_>> for LedgerKeyData {
    #[must_use]
    fn from(v: LedgerKeyDataRef<'_>) -> Self {
        v.into_owned()
    }
}

impl WriteXdr for LedgerKeyDataRef<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.account_id.write_xdr(w)?;
            self.data_name.write_xdr(w)?;
            Ok(())
        })
    }
}
