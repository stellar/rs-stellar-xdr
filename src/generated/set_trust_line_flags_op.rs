#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// SetTrustLineFlagsOp is an XDR Struct defined as:
///
/// ```text
/// struct SetTrustLineFlagsOp
/// {
///     AccountID trustor;
///     Asset asset;
///
///     uint32 clearFlags; // which flags to clear
///     uint32 setFlags;   // which flags to set
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
pub struct SetTrustLineFlagsOp {
    pub trustor: AccountId,
    pub asset: Asset,
    pub clear_flags: u32,
    pub set_flags: u32,
}

impl ReadXdr for SetTrustLineFlagsOp {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                trustor: AccountId::read_xdr(r)?,
                asset: Asset::read_xdr(r)?,
                clear_flags: u32::read_xdr(r)?,
                set_flags: u32::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for SetTrustLineFlagsOp {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.trustor.write_xdr(w)?;
            self.asset.write_xdr(w)?;
            self.clear_flags.write_xdr(w)?;
            self.set_flags.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
impl IntoOwned for SetTrustLineFlagsOp {
    type Owned = SetTrustLineFlagsOp;
    fn into_owned(self) -> SetTrustLineFlagsOp {
        self
    }
}

#[cfg(feature = "const")]
impl SetTrustLineFlagsOp {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_set_trust_line_flags_op(self);
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
        w.write_type_set_trust_line_flags_op(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`SetTrustLineFlagsOp`], mirroring `<SetTrustLineFlagsOp as WriteXdr>::write_xdr`.
    pub const fn write_type_set_trust_line_flags_op(&mut self, v: &SetTrustLineFlagsOp) {
        self.write_type_account_id(&v.trustor);
        self.write_type_asset(&v.asset);
        self.write_u32(v.clear_flags);
        self.write_u32(v.set_flags);
    }
}
