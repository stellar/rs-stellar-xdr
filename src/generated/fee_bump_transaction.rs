#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// FeeBumpTransaction is an XDR Struct defined as:
///
/// ```text
/// struct FeeBumpTransaction
/// {
///     MuxedAccount feeSource;
///     int64 fee;
///     union switch (EnvelopeType type)
///     {
///     case ENVELOPE_TYPE_TX:
///         TransactionV1Envelope v1;
///     }
///     innerTx;
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
pub struct FeeBumpTransaction {
    pub fee_source: MuxedAccount,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub fee: i64,
    pub inner_tx: FeeBumpTransactionInnerTx,
    pub ext: FeeBumpTransactionExt,
}

impl ReadXdr for FeeBumpTransaction {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                fee_source: MuxedAccount::read_xdr(r)?,
                fee: i64::read_xdr(r)?,
                inner_tx: FeeBumpTransactionInnerTx::read_xdr(r)?,
                ext: FeeBumpTransactionExt::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for FeeBumpTransaction {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.fee_source.write_xdr(w)?;
            self.fee.write_xdr(w)?;
            self.inner_tx.write_xdr(w)?;
            self.ext.write_xdr(w)?;
            Ok(())
        })
    }
}

/// FeeBumpTransactionConst is a borrowing equivalent of [`FeeBumpTransaction`] over `'static`
/// data, for const XDR encoding.
#[cfg(feature = "const")]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct FeeBumpTransactionConst {
    pub fee_source: MuxedAccount,
    pub fee: i64,
    pub inner_tx: FeeBumpTransactionInnerTxConst,
    pub ext: FeeBumpTransactionExt,
}

#[cfg(feature = "const")]
impl FeeBumpTransactionConst {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_fee_bump_transaction(self);
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
        w.write_type_fee_bump_transaction(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`FeeBumpTransaction`], mirroring `<FeeBumpTransaction as WriteXdr>::write_xdr`.
    pub const fn write_type_fee_bump_transaction(&mut self, v: &FeeBumpTransactionConst) {
        self.write_type_muxed_account(&v.fee_source);
        self.write_i64(v.fee);
        self.write_type_fee_bump_transaction_inner_tx(&v.inner_tx);
        self.write_type_fee_bump_transaction_ext(&v.ext);
    }
}
