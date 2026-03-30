#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// SorobanTransactionMetaExtV1 is an XDR Struct defined as:
///
/// ```text
/// struct SorobanTransactionMetaExtV1
/// {
///     ExtensionPoint ext;
/// 
///     // The following are the components of the overall Soroban resource fee
///     // charged for the transaction.
///     // The following relation holds:
///     // `resourceFeeCharged = totalNonRefundableResourceFeeCharged + totalRefundableResourceFeeCharged`
///     // where `resourceFeeCharged` is the overall fee charged for the 
///     // transaction. Also, `resourceFeeCharged` <= `sorobanData.resourceFee` 
///     // i.e.we never charge more than the declared resource fee.
///     // The inclusion fee for charged the Soroban transaction can be found using 
///     // the following equation:
///     // `result.feeCharged = resourceFeeCharged + inclusionFeeCharged`.
/// 
///     // Total amount (in stroops) that has been charged for non-refundable
///     // Soroban resources.
///     // Non-refundable resources are charged based on the usage declared in
///     // the transaction envelope (such as `instructions`, `readBytes` etc.) and 
///     // is charged regardless of the success of the transaction.
///     int64 totalNonRefundableResourceFeeCharged;
///     // Total amount (in stroops) that has been charged for refundable
///     // Soroban resource fees.
///     // Currently this comprises the rent fee (`rentFeeCharged`) and the
///     // fee for the events and return value.
///     // Refundable resources are charged based on the actual resources usage.
///     // Since currently refundable resources are only used for the successful
///     // transactions, this will be `0` for failed transactions.
///     int64 totalRefundableResourceFeeCharged;
///     // Amount (in stroops) that has been charged for rent.
///     // This is a part of `totalNonRefundableResourceFeeCharged`.
///     int64 rentFeeCharged;
/// };
/// ```
///
#[cfg_attr(feature = "alloc", derive(Default))]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_eval::cfg_eval]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    serde_with::serde_as,
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct SorobanTransactionMetaExtV1 {
    pub ext: ExtensionPoint,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub total_non_refundable_resource_fee_charged: i64,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub total_refundable_resource_fee_charged: i64,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub rent_fee_charged: i64,
}

impl ReadXdr for SorobanTransactionMetaExtV1 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                ext: ExtensionPoint::read_xdr(r)?,
                total_non_refundable_resource_fee_charged: i64::read_xdr(r)?,
                total_refundable_resource_fee_charged: i64::read_xdr(r)?,
                rent_fee_charged: i64::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for SorobanTransactionMetaExtV1 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.ext.write_xdr(w)?;
            self.total_non_refundable_resource_fee_charged.write_xdr(w)?;
            self.total_refundable_resource_fee_charged.write_xdr(w)?;
            self.rent_fee_charged.write_xdr(w)?;
            Ok(())
        })
    }
}
