#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// SorobanTransactionMetaV2 is an XDR Struct defined as:
///
/// ```text
/// struct SorobanTransactionMetaV2
/// {
///     SorobanTransactionMetaExt ext;
///
///     SCVal* returnValue;
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
pub struct SorobanTransactionMetaV2 {
    pub ext: SorobanTransactionMetaExt,
    pub return_value: Option<ScVal>,
}

impl ReadXdr for SorobanTransactionMetaV2 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                ext: SorobanTransactionMetaExt::read_xdr(r)?,
                return_value: Option::<ScVal>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for SorobanTransactionMetaV2 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.ext.write_xdr(w)?;
            self.return_value.write_xdr(w)?;
            Ok(())
        })
    }
}

/// SorobanTransactionMetaV2Const is a borrowing equivalent of [`SorobanTransactionMetaV2`] over `'static`
/// data, for const XDR encoding.
#[cfg(feature = "const")]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SorobanTransactionMetaV2Const {
    pub ext: SorobanTransactionMetaExt,
    pub return_value: Option<ScValConst>,
}

#[cfg(feature = "const")]
impl SorobanTransactionMetaV2Const {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_soroban_transaction_meta_v2(self);
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
        w.write_type_soroban_transaction_meta_v2(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`SorobanTransactionMetaV2`], mirroring `<SorobanTransactionMetaV2 as WriteXdr>::write_xdr`.
    pub const fn write_type_soroban_transaction_meta_v2(
        &mut self,
        v: &SorobanTransactionMetaV2Const,
    ) {
        self.write_type_soroban_transaction_meta_ext(&v.ext);
        self.write_type_option_sc_val(&v.return_value);
    }

    /// Serializes an optional [`SorobanTransactionMetaV2`], mirroring `<Option<SorobanTransactionMetaV2> as WriteXdr>::write_xdr`.
    pub const fn write_type_option_soroban_transaction_meta_v2(
        &mut self,
        v: &Option<SorobanTransactionMetaV2Const>,
    ) {
        match v {
            Some(v) => {
                self.write_u32(1);
                self.write_type_soroban_transaction_meta_v2(v);
            }
            None => {
                self.write_u32(0);
            }
        }
    }
}
