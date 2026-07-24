#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// HashIdPreimageRevokeId is an XDR NestedStruct defined as:
///
/// ```text
/// struct
///     {
///         AccountID sourceAccount;
///         SequenceNumber seqNum;
///         uint32 opNum;
///         PoolID liquidityPoolID;
///         Asset asset;
///     }
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
pub struct HashIdPreimageRevokeId {
    pub source_account: AccountId,
    pub seq_num: SequenceNumber,
    pub op_num: u32,
    pub liquidity_pool_id: PoolId,
    pub asset: Asset,
}

impl ReadXdr for HashIdPreimageRevokeId {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                source_account: AccountId::read_xdr(r)?,
                seq_num: SequenceNumber::read_xdr(r)?,
                op_num: u32::read_xdr(r)?,
                liquidity_pool_id: PoolId::read_xdr(r)?,
                asset: Asset::read_xdr(r)?,
            })
        })
    }
}

impl HashIdPreimageRevokeId {
    /// Serialize this value as XDR into a [`ConstWriter`] using only const
    /// operations. This is the const counterpart to [`WriteXdr::write_xdr`].
    #[cfg(feature = "const")]
    pub const fn const_write_xdr(&self, w: &mut ConstWriter) {
        w.enter_depth();
        self.source_account.const_write_xdr(w);
        self.seq_num.const_write_xdr(w);
        w.write_u32(self.op_num);
        self.liquidity_pool_id.const_write_xdr(w);
        self.asset.const_write_xdr(w);
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

impl WriteXdr for HashIdPreimageRevokeId {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.source_account.write_xdr(w)?;
            self.seq_num.write_xdr(w)?;
            self.op_num.write_xdr(w)?;
            self.liquidity_pool_id.write_xdr(w)?;
            self.asset.write_xdr(w)?;
            Ok(())
        })
    }
}
