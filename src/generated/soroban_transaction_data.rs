#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// SorobanTransactionData is an XDR Struct defined as:
///
/// ```text
/// struct SorobanTransactionData
/// {
///     union switch (int v)
///     {
///     case 0:
///         void;
///     case 1:
///         SorobanResourcesExtV0 resourceExt;
///     } ext;
///     SorobanResources resources;
///     // Amount of the transaction `fee` allocated to the Soroban resource fees.
///     // The fraction of `resourceFee` corresponding to `resources` specified
///     // above is *not* refundable (i.e. fees for instructions, ledger I/O), as
///     // well as fees for the transaction size.
///     // The remaining part of the fee is refundable and the charged value is
///     // based on the actual consumption of refundable resources (events, ledger
///     // rent bumps).
///     // The `inclusionFee` used for prioritization of the transaction is defined
///     // as `tx.fee - resourceFee`.
///     int64 resourceFee;
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
pub struct SorobanTransactionData {
    pub ext: SorobanTransactionDataExt,
    pub resources: SorobanResources,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub resource_fee: i64,
}

impl ReadXdr for SorobanTransactionData {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                ext: SorobanTransactionDataExt::read_xdr(r)?,
                resources: SorobanResources::read_xdr(r)?,
                resource_fee: i64::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for SorobanTransactionData {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.ext.write_xdr(w)?;
            self.resources.write_xdr(w)?;
            self.resource_fee.write_xdr(w)?;
            Ok(())
        })
    }
}

/// SorobanTransactionDataView is a borrowing equivalent of [`SorobanTransactionData`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SorobanTransactionDataView<'a> {
    pub ext: SorobanTransactionDataExtView<'a>,
    pub resources: SorobanResourcesView<'a>,
    pub resource_fee: i64,
}

#[cfg(feature = "alloc")]
impl IntoOwned for SorobanTransactionDataView<'_> {
    type Owned = SorobanTransactionData;
    fn into_owned(self) -> SorobanTransactionData {
        SorobanTransactionData {
            ext: self.ext.into_owned(),
            resources: self.resources.into_owned(),
            resource_fee: self.resource_fee.into_owned(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&SorobanTransactionDataView<'_>> for SorobanTransactionData {
    #[must_use]
    fn from(v: &SorobanTransactionDataView<'_>) -> Self {
        v.clone().into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<SorobanTransactionDataView<'_>> for SorobanTransactionData {
    #[must_use]
    fn from(v: SorobanTransactionDataView<'_>) -> Self {
        v.into_owned()
    }
}

impl WriteXdr for SorobanTransactionDataView<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.ext.write_xdr(w)?;
            self.resources.write_xdr(w)?;
            self.resource_fee.write_xdr(w)?;
            Ok(())
        })
    }
}
