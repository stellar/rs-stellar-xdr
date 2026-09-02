#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// LedgerBounds is an XDR Struct defined as:
///
/// ```text
/// struct LedgerBounds
/// {
///     uint32 minLedger;
///     uint32 maxLedger; // 0 here means no maxLedger
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
pub struct LedgerBounds {
    pub min_ledger: u32,
    pub max_ledger: u32,
}

impl ReadXdr for LedgerBounds {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                min_ledger: u32::read_xdr(r)?,
                max_ledger: u32::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for LedgerBounds {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.min_ledger.write_xdr(w)?;
            self.max_ledger.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "alloc")]
impl IntoOwned for LedgerBounds {
    type Owned = LedgerBounds;
    fn into_owned(self) -> LedgerBounds {
        self
    }
}

#[cfg(feature = "const")]
impl LedgerBounds {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_ledger_bounds(self);
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
        w.write_type_ledger_bounds(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`LedgerBounds`], mirroring `<LedgerBounds as WriteXdr>::write_xdr`.
    pub const fn write_type_ledger_bounds(&mut self, v: &LedgerBounds) {
        self.write_u32(v.min_ledger);
        self.write_u32(v.max_ledger);
    }

    /// Serializes an optional [`LedgerBounds`], mirroring `<Option<LedgerBounds> as WriteXdr>::write_xdr`.
    pub const fn write_type_option_ledger_bounds(&mut self, v: &Option<LedgerBounds>) {
        match v {
            Some(v) => {
                self.write_u32(1);
                self.write_type_ledger_bounds(v);
            }
            None => {
                self.write_u32(0);
            }
        }
    }
}
