#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// AccountEntryExtensionV1 is an XDR Struct defined as:
///
/// ```text
/// struct AccountEntryExtensionV1
/// {
///     Liabilities liabilities;
///
///     union switch (int v)
///     {
///     case 0:
///         void;
///     case 2:
///         AccountEntryExtensionV2 v2;
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
pub struct AccountEntryExtensionV1 {
    pub liabilities: Liabilities,
    pub ext: AccountEntryExtensionV1Ext,
}

impl ReadXdr for AccountEntryExtensionV1 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                liabilities: Liabilities::read_xdr(r)?,
                ext: AccountEntryExtensionV1Ext::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for AccountEntryExtensionV1 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.liabilities.write_xdr(w)?;
            self.ext.write_xdr(w)?;
            Ok(())
        })
    }
}

/// AccountEntryExtensionV1View is a borrowing equivalent of [`AccountEntryExtensionV1`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct AccountEntryExtensionV1View<'a> {
    pub liabilities: Liabilities,
    pub ext: AccountEntryExtensionV1ExtView<'a>,
}

#[cfg(feature = "alloc")]
impl IntoOwned for AccountEntryExtensionV1View<'_> {
    type Owned = AccountEntryExtensionV1;
    fn into_owned(&self) -> AccountEntryExtensionV1 {
        AccountEntryExtensionV1 {
            liabilities: self.liabilities.into_owned(),
            ext: self.ext.into_owned(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&AccountEntryExtensionV1View<'_>> for AccountEntryExtensionV1 {
    #[must_use]
    fn from(v: &AccountEntryExtensionV1View<'_>) -> Self {
        v.into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<AccountEntryExtensionV1View<'_>> for AccountEntryExtensionV1 {
    #[must_use]
    fn from(v: AccountEntryExtensionV1View<'_>) -> Self {
        v.into_owned()
    }
}

impl WriteXdr for AccountEntryExtensionV1View<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.liabilities.write_xdr(w)?;
            self.ext.write_xdr(w)?;
            Ok(())
        })
    }
}
