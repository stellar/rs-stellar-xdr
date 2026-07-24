#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// TransactionV0 is an XDR Struct defined as:
///
/// ```text
/// struct TransactionV0
/// {
///     uint256 sourceAccountEd25519;
///     uint32 fee;
///     SequenceNumber seqNum;
///     TimeBounds* timeBounds;
///     Memo memo;
///     Operation operations<MAX_OPS_PER_TX>;
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
pub struct TransactionV0 {
    pub source_account_ed25519: Uint256,
    pub fee: u32,
    pub seq_num: SequenceNumber,
    pub time_bounds: Option<TimeBounds>,
    pub memo: Memo,
    pub operations: VecM<Operation, 100>,
    pub ext: TransactionV0Ext,
}

impl ReadXdr for TransactionV0 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                source_account_ed25519: Uint256::read_xdr(r)?,
                fee: u32::read_xdr(r)?,
                seq_num: SequenceNumber::read_xdr(r)?,
                time_bounds: Option::<TimeBounds>::read_xdr(r)?,
                memo: Memo::read_xdr(r)?,
                operations: VecM::<Operation, 100>::read_xdr(r)?,
                ext: TransactionV0Ext::read_xdr(r)?,
            })
        })
    }
}

impl TransactionV0 {
    /// Serialize this value as XDR into a [`ConstWriter`] using only const
    /// operations. This is the const counterpart to [`WriteXdr::write_xdr`].
    #[cfg(feature = "const")]
    pub const fn const_write_xdr(&self, w: &mut ConstWriter) {
        w.enter_depth();
        self.source_account_ed25519.const_write_xdr(w);
        w.write_u32(self.fee);
        self.seq_num.const_write_xdr(w);
        {
            w.enter_depth();
            match &self.time_bounds {
                Some(__v0) => {
                    w.write_u32(1);
                    __v0.const_write_xdr(w);
                }
                None => {
                    w.write_u32(0);
                }
            }
            w.leave_depth();
        }
        self.memo.const_write_xdr(w);
        {
            w.enter_depth();
            let __s0 = self.operations.0.as_slice();
            let __len0 = __s0.len();
            w.write_length_prefix(__len0);
            let mut __i0 = 0usize;
            while __i0 < __len0 {
                __s0[__i0].const_write_xdr(w);
                __i0 += 1;
            }
            w.leave_depth();
        }
        self.ext.const_write_xdr(w);
        w.leave_depth();
    }
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[cfg(feature = "const")]
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
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
    /// operations. This is the const counterpart to [`WriteXdr::to_xdr`].
    ///
    /// `N` must equal [`Self::const_xdr_len`]. It is intended for callers, such
    /// as a proc-macro, that compute the length with `const_xdr_len` and pass
    /// it as `N`; `const_to_xdr` itself does not need to call `const_xdr_len`.
    ///
    /// # Panics
    ///
    /// Panics if `N` does not equal the value's [`Self::const_xdr_len`].
    #[cfg(feature = "const")]
    #[must_use]
    pub const fn const_to_xdr<const N: usize>(&self) -> [u8; N] {
        let limits = Limits {
            depth: u32::MAX,
            len: usize::MAX,
        };
        let mut buf = [0u8; N];
        let mut w = ConstWriter::new(&mut buf, &limits);
        self.const_write_xdr(&mut w);
        assert!(
            w.position() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

impl WriteXdr for TransactionV0 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.source_account_ed25519.write_xdr(w)?;
            self.fee.write_xdr(w)?;
            self.seq_num.write_xdr(w)?;
            self.time_bounds.write_xdr(w)?;
            self.memo.write_xdr(w)?;
            self.operations.write_xdr(w)?;
            self.ext.write_xdr(w)?;
            Ok(())
        })
    }
}
