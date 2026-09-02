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

/// SorobanTransactionDataRef is a borrowing equivalent of [`SorobanTransactionData`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SorobanTransactionDataRef<'a> {
    pub ext: SorobanTransactionDataExtRef<'a>,
    pub resources: SorobanResourcesRef<'a>,
    pub resource_fee: i64,
}

#[cfg(feature = "alloc")]
impl IntoOwned for SorobanTransactionDataRef<'_> {
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
impl From<&SorobanTransactionDataRef<'_>> for SorobanTransactionData {
    #[must_use]
    fn from(v: &SorobanTransactionDataRef<'_>) -> Self {
        v.into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<SorobanTransactionDataRef<'_>> for SorobanTransactionData {
    #[must_use]
    fn from(v: SorobanTransactionDataRef<'_>) -> Self {
        v.into_owned()
    }
}

impl WriteXdr for SorobanTransactionDataRef<'_> {
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

#[cfg(feature = "const")]
impl SorobanTransactionDataView<'_> {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_soroban_transaction_data(self);
        w.len()
    }

    /// Serialize this value as XDR into a fixed-size `[u8; N]` using only const
    /// operations. This is the const counterpart to [`WriteXdr::to_xdr`].
    ///
    /// `N` must equal [`Self::const_xdr_len`]. It is intended for callers, such
    /// as a proc-macro, that compute the length with `const_xdr_len` and pass
    /// it as `N`; `const_to_xdr` itself does not need to call `const_xdr_len`.
    ///
    /// # Panics
    ///
    /// Panics if `N` does not equal the value's [`Self::const_xdr_len`].
    #[must_use]
    pub const fn const_to_xdr<const N: usize>(&self) -> [u8; N] {
        let mut buf = [0u8; N];
        let mut w = ConstWriter::new(&mut buf);
        w.write_type_soroban_transaction_data(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`SorobanTransactionData`], mirroring `<SorobanTransactionData as WriteXdr>::write_xdr`.
    pub const fn write_type_soroban_transaction_data(
        &mut self,
        v: &SorobanTransactionDataView<'_>,
    ) {
        self.write_type_soroban_transaction_data_ext(&v.ext);
        self.write_type_soroban_resources(&v.resources);
        self.write_i64(v.resource_fee);
    }
}
