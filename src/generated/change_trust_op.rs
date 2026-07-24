#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ChangeTrustOp is an XDR Struct defined as:
///
/// ```text
/// struct ChangeTrustOp
/// {
///     ChangeTrustAsset line;
///
///     // if limit is set to 0, deletes the trust line
///     int64 limit;
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
pub struct ChangeTrustOp {
    pub line: ChangeTrustAsset,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub limit: i64,
}

impl ReadXdr for ChangeTrustOp {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                line: ChangeTrustAsset::read_xdr(r)?,
                limit: i64::read_xdr(r)?,
            })
        })
    }
}

impl ChangeTrustOp {
    /// Serialize this value as XDR into a [`ConstWriter`] using only const
    /// operations. This is the const implementation underlying `to_xdr`.
    #[cfg(feature = "std")]
    pub const fn const_to_xdr(&self, w: &mut ConstWriter) {
        w.enter_depth();
        self.line.const_to_xdr(w);
        w.write_i64(self.limit);
        w.leave_depth();
    }
}

impl WriteXdr for ChangeTrustOp {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.line.write_xdr(w)?;
            self.limit.write_xdr(w)?;
            Ok(())
        })
    }

    #[cfg(feature = "std")]
    fn to_xdr(&self, limits: Limits) -> Result<Vec<u8>, Error> {
        to_xdr_via_const(self, &limits, Self::const_to_xdr)
    }
}
