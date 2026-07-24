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
///     // This is a part of `totalRefundableResourceFeeCharged`.
///     int64 rentFeeCharged;
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

impl SorobanTransactionMetaExtV1 {
    /// Serialize this value as XDR into a [`ConstWriter`] using only const
    /// operations. This is the const implementation underlying `to_xdr`.
    #[cfg(feature = "std")]
    pub const fn const_write_xdr(&self, w: &mut ConstWriter) {
        w.enter_depth();
        self.ext.const_write_xdr(w);
        w.write_i64(self.total_non_refundable_resource_fee_charged);
        w.write_i64(self.total_refundable_resource_fee_charged);
        w.write_i64(self.rent_fee_charged);
        w.leave_depth();
    }
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::to_xdr_array`] at compile time.
    #[cfg(feature = "std")]
    #[must_use]
    pub const fn xdr_len(&self) -> usize {
        let limits = Limits {
            depth: u32::MAX,
            len: usize::MAX,
        };
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty, &limits);
        self.const_write_xdr(&mut w);
        w.position()
    }

    /// Serialize this value as XDR into a fixed-size `[u8; N]` using only const
    /// operations.
    ///
    /// `N` must equal [`Self::xdr_len`]. It is intended for callers, such as a
    /// proc-macro, that compute the length with `xdr_len` and pass it as `N`;
    /// `to_xdr_array` itself does not need to call `xdr_len`.
    ///
    /// # Panics
    ///
    /// Panics if `N` does not equal the value's [`Self::xdr_len`].
    #[cfg(feature = "std")]
    #[must_use]
    pub const fn to_xdr_array<const N: usize>(&self) -> [u8; N] {
        let limits = Limits {
            depth: u32::MAX,
            len: usize::MAX,
        };
        let mut buf = [0u8; N];
        let mut w = ConstWriter::new(&mut buf, &limits);
        self.const_write_xdr(&mut w);
        assert!(
            w.position() == N,
            "to_xdr_array: N does not equal the XDR-encoded length"
        );
        buf
    }
}

impl WriteXdr for SorobanTransactionMetaExtV1 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        write_xdr_via_const(self, w, Self::const_write_xdr)
    }
}
