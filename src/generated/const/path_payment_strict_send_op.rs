#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// PathPaymentStrictSendOp is a borrowing equivalent of [`PathPaymentStrictSendOp`](super::super::PathPaymentStrictSendOp)
/// over `'static` data, for const XDR encoding.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
pub struct PathPaymentStrictSendOp {
    pub send_asset: Asset,
    pub send_amount: i64,
    pub destination: MuxedAccount,
    pub dest_asset: Asset,
    pub dest_min: i64,
    pub path: VecM<Asset, 5>,
}

impl PathPaymentStrictSendOp {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_path_payment_strict_send_op(self);
        w.len()
    }

    /// Serialize this value as XDR into a fixed-size `[u8; N]` using only const
    /// operations. This is the const counterpart to
    /// [`WriteXdr::to_xdr`](super::super::WriteXdr::to_xdr).
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
        w.write_type_path_payment_strict_send_op(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

impl ConstWriter<'_> {
    /// Serializes a [`PathPaymentStrictSendOp`], mirroring `<PathPaymentStrictSendOp as WriteXdr>::write_xdr`.
    pub const fn write_type_path_payment_strict_send_op(&mut self, v: &PathPaymentStrictSendOp) {
        self.write_type_asset(&v.send_asset);
        self.write_i64(v.send_amount);
        self.write_type_muxed_account(&v.destination);
        self.write_type_asset(&v.dest_asset);
        self.write_i64(v.dest_min);
        self.write_type_vec_asset(&v.path);
    }
}
