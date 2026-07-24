#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// TxSetComponentTxsMaybeDiscountedFee is an XDR NestedStruct defined as:
///
/// ```text
/// struct
///   {
///     int64* baseFee;
///     TransactionEnvelope txs<>;
///   }
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
pub struct TxSetComponentTxsMaybeDiscountedFee {
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "Option<NumberOrString>")
    )]
    pub base_fee: Option<i64>,
    pub txs: VecM<TransactionEnvelope>,
}

impl ReadXdr for TxSetComponentTxsMaybeDiscountedFee {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                base_fee: Option::<i64>::read_xdr(r)?,
                txs: VecM::<TransactionEnvelope>::read_xdr(r)?,
            })
        })
    }
}

impl TxSetComponentTxsMaybeDiscountedFee {
    /// Serialize this value as XDR into a [`ConstWriter`] using only const
    /// operations. This is the const implementation underlying `to_xdr`.
    #[cfg(feature = "std")]
    pub const fn const_write_xdr(&self, w: &mut ConstWriter) {
        w.enter_depth();
        {
            w.enter_depth();
            match &self.base_fee {
                Some(__v0) => {
                    w.write_u32(1);
                    w.write_i64(*__v0);
                }
                None => {
                    w.write_u32(0);
                }
            }
            w.leave_depth();
        }
        {
            w.enter_depth();
            let __s0 = self.txs.0.as_slice();
            let __len0 = __s0.len();
            w.write_length_prefix(__len0);
            let mut __i0 = 0usize;
            while __i0 < __len0 {
                __s0[__i0].const_write_xdr(w);
                __i0 += 1;
            }
            w.leave_depth();
        }
        w.leave_depth();
    }
}

impl WriteXdr for TxSetComponentTxsMaybeDiscountedFee {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.base_fee.write_xdr(w)?;
            self.txs.write_xdr(w)?;
            Ok(())
        })
    }

    #[cfg(feature = "std")]
    fn to_xdr(&self, limits: Limits) -> Result<Vec<u8>, Error> {
        to_xdr_via_const(self, &limits, Self::const_write_xdr)
    }
}
