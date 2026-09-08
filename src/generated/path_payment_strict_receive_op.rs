#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// PathPaymentStrictReceiveOp is an XDR Struct defined as:
///
/// ```text
/// struct PathPaymentStrictReceiveOp
/// {
///     Asset sendAsset; // asset we pay with
///     int64 sendMax;   // the maximum amount of sendAsset to
///                      // send (excluding fees).
///                      // The operation will fail if can't be met
///
///     MuxedAccount destination; // recipient of the payment
///     Asset destAsset;          // what they end up with
///     int64 destAmount;         // amount they end up with
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
pub struct PathPaymentStrictReceiveOp {
    pub send_asset: Asset,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub send_max: i64,
    pub destination: MuxedAccount,
    pub dest_asset: Asset,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub dest_amount: i64,
    pub path: VecM<Asset, 5>,
}

impl ReadXdr for PathPaymentStrictReceiveOp {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                send_asset: Asset::read_xdr(r)?,
                send_max: i64::read_xdr(r)?,
                destination: MuxedAccount::read_xdr(r)?,
                dest_asset: Asset::read_xdr(r)?,
                dest_amount: i64::read_xdr(r)?,
                path: VecM::<Asset, 5>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for PathPaymentStrictReceiveOp {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.send_asset.write_xdr(w)?;
            self.send_max.write_xdr(w)?;
            self.destination.write_xdr(w)?;
            self.dest_asset.write_xdr(w)?;
            self.dest_amount.write_xdr(w)?;
            self.path.write_xdr(w)?;
            Ok(())
        })
    }
}

/// PathPaymentStrictReceiveOpConst is a borrowing equivalent of [`PathPaymentStrictReceiveOp`] over `'static`
/// data, for const XDR encoding.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct PathPaymentStrictReceiveOpConst {
    pub send_asset: Asset,
    pub send_max: i64,
    pub destination: MuxedAccount,
    pub dest_asset: Asset,
    pub dest_amount: i64,
    pub path: VecMConst<Asset, 5>,
}

#[cfg(feature = "const")]
impl PathPaymentStrictReceiveOpConst {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_path_payment_strict_receive_op(self);
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
        w.write_type_path_payment_strict_receive_op(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`PathPaymentStrictReceiveOp`], mirroring `<PathPaymentStrictReceiveOp as WriteXdr>::write_xdr`.
    pub const fn write_type_path_payment_strict_receive_op(
        &mut self,
        v: &PathPaymentStrictReceiveOpConst,
    ) {
        self.write_type_asset(&v.send_asset);
        self.write_i64(v.send_max);
        self.write_type_muxed_account(&v.destination);
        self.write_type_asset(&v.dest_asset);
        self.write_i64(v.dest_amount);
        self.write_type_vec_asset(&v.path);
    }
}
