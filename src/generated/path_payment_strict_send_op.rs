#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// PathPaymentStrictSendOp is an XDR Struct defined as:
///
/// ```text
/// struct PathPaymentStrictSendOp
/// {
///     Asset sendAsset;  // asset we pay with
///     int64 sendAmount; // amount of sendAsset to send (excluding fees)
///
///     MuxedAccount destination; // recipient of the payment
///     Asset destAsset;          // what they end up with
///     int64 destMin;            // the minimum amount of dest asset to
///                               // be received
///                               // The operation will fail if it can't be met
///
///     Asset path<5>; // additional hops it must go through to get there
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
pub struct PathPaymentStrictSendOp {
    pub send_asset: Asset,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub send_amount: i64,
    pub destination: MuxedAccount,
    pub dest_asset: Asset,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub dest_min: i64,
    pub path: VecM<Asset, 5>,
}

impl ReadXdr for PathPaymentStrictSendOp {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                send_asset: Asset::read_xdr(r)?,
                send_amount: i64::read_xdr(r)?,
                destination: MuxedAccount::read_xdr(r)?,
                dest_asset: Asset::read_xdr(r)?,
                dest_min: i64::read_xdr(r)?,
                path: VecM::<Asset, 5>::read_xdr(r)?,
            })
        })
    }
}

impl PathPaymentStrictSendOp {
    /// Serialize this value as XDR into a [`ConstWriter`] using only const
    /// operations. This is the const implementation underlying `to_xdr`.
    #[cfg(feature = "std")]
    pub const fn const_write_xdr(&self, w: &mut ConstWriter) {
        w.enter_depth();
        self.send_asset.const_write_xdr(w);
        w.write_i64(self.send_amount);
        self.destination.const_write_xdr(w);
        self.dest_asset.const_write_xdr(w);
        w.write_i64(self.dest_min);
        {
            w.enter_depth();
            let __s0 = self.path.0.as_slice();
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

impl WriteXdr for PathPaymentStrictSendOp {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.send_asset.write_xdr(w)?;
            self.send_amount.write_xdr(w)?;
            self.destination.write_xdr(w)?;
            self.dest_asset.write_xdr(w)?;
            self.dest_min.write_xdr(w)?;
            self.path.write_xdr(w)?;
            Ok(())
        })
    }

    #[cfg(feature = "std")]
    fn to_xdr(&self, limits: Limits) -> Result<Vec<u8>, Error> {
        to_xdr_via_const(self, &limits, Self::const_write_xdr)
    }
}
