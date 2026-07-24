#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// TrustLineEntry is an XDR Struct defined as:
///
/// ```text
/// struct TrustLineEntry
/// {
///     AccountID accountID;  // account this trustline belongs to
///     TrustLineAsset asset; // type of asset (with issuer)
///     int64 balance;        // how much of this asset the user has.
///                           // Asset defines the unit for this;
///
///     int64 limit;  // balance cannot be above this
///     uint32 flags; // see TrustLineFlags
///
///     // reserved for future use
///     union switch (int v)
///     {
///     case 0:
///         void;
///     case 1:
///         struct
///         {
///             Liabilities liabilities;
///
///             union switch (int v)
///             {
///             case 0:
///                 void;
///             case 2:
///                 TrustLineEntryExtensionV2 v2;
///             }
///             ext;
///         } v1;
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
pub struct TrustLineEntry {
    pub account_id: AccountId,
    pub asset: TrustLineAsset,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub balance: i64,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub limit: i64,
    pub flags: u32,
    pub ext: TrustLineEntryExt,
}

impl ReadXdr for TrustLineEntry {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                account_id: AccountId::read_xdr(r)?,
                asset: TrustLineAsset::read_xdr(r)?,
                balance: i64::read_xdr(r)?,
                limit: i64::read_xdr(r)?,
                flags: u32::read_xdr(r)?,
                ext: TrustLineEntryExt::read_xdr(r)?,
            })
        })
    }
}

impl TrustLineEntry {
    /// Serialize this value as XDR into a [`ConstWriter`] using only const
    /// operations. This is the const implementation underlying `to_xdr`.
    #[cfg(feature = "std")]
    pub const fn const_write_xdr(&self, w: &mut ConstWriter) {
        w.enter_depth();
        self.account_id.const_write_xdr(w);
        self.asset.const_write_xdr(w);
        w.write_i64(self.balance);
        w.write_i64(self.limit);
        w.write_u32(self.flags);
        self.ext.const_write_xdr(w);
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

impl WriteXdr for TrustLineEntry {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.account_id.write_xdr(w)?;
            self.asset.write_xdr(w)?;
            self.balance.write_xdr(w)?;
            self.limit.write_xdr(w)?;
            self.flags.write_xdr(w)?;
            self.ext.write_xdr(w)?;
            Ok(())
        })
    }

    #[cfg(feature = "std")]
    fn to_xdr(&self, limits: Limits) -> Result<Vec<u8>, Error> {
        to_xdr_via_const(self, &limits, Self::const_write_xdr)
    }
}
