#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// LedgerFootprint is an XDR Struct defined as:
///
/// ```text
/// struct LedgerFootprint
/// {
///     LedgerKey readOnly<>;
///     LedgerKey readWrite<>;
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
pub struct LedgerFootprint {
    pub read_only: VecM<LedgerKey>,
    pub read_write: VecM<LedgerKey>,
}

impl ReadXdr for LedgerFootprint {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                read_only: VecM::<LedgerKey>::read_xdr(r)?,
                read_write: VecM::<LedgerKey>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for LedgerFootprint {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.read_only.write_xdr(w)?;
            self.read_write.write_xdr(w)?;
            Ok(())
        })
    }
}

/// LedgerFootprintConst is a borrowing equivalent of [`LedgerFootprint`] over `'static`
/// data, for const XDR encoding.
#[cfg(feature = "const")]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct LedgerFootprintConst {
    pub read_only: VecMConst<LedgerKeyConst>,
    pub read_write: VecMConst<LedgerKeyConst>,
}

#[cfg(feature = "const")]
impl LedgerFootprintConst {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_ledger_footprint(self);
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
        w.write_type_ledger_footprint(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`LedgerFootprint`], mirroring `<LedgerFootprint as WriteXdr>::write_xdr`.
    pub const fn write_type_ledger_footprint(&mut self, v: &LedgerFootprintConst) {
        self.write_type_vec_ledger_key(&v.read_only);
        self.write_type_vec_ledger_key(&v.read_write);
    }
}
