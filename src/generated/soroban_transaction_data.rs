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

impl SorobanTransactionData {
    /// Serialize this value as XDR into a [`ConstWriter`] using only const
    /// operations. This is the const implementation underlying `to_xdr`.
    #[cfg(feature = "std")]
    pub const fn const_write_xdr(&self, w: &mut ConstWriter) {
        w.enter_depth();
        self.ext.const_write_xdr(w);
        self.resources.const_write_xdr(w);
        w.write_i64(self.resource_fee);
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

impl WriteXdr for SorobanTransactionData {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        write_xdr_via_const(self, w, Self::const_write_xdr)
    }
}
