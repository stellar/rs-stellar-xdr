#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// SorobanTransactionMeta is an XDR Struct defined as:
///
/// ```text
/// struct SorobanTransactionMeta
/// {
///     SorobanTransactionMetaExt ext;
///
///     ContractEvent events<>;             // custom events populated by the
///                                         // contracts themselves.
///     SCVal returnValue;                  // return value of the host fn invocation
///
///     // Diagnostics events that are not hashed.
///     // This will contain all contract and diagnostic events. Even ones
///     // that were emitted in a failed contract call.
///     DiagnosticEvent diagnosticEvents<>;
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
pub struct SorobanTransactionMeta {
    pub ext: SorobanTransactionMetaExt,
    pub events: VecM<ContractEvent>,
    pub return_value: ScVal,
    pub diagnostic_events: VecM<DiagnosticEvent>,
}

impl ReadXdr for SorobanTransactionMeta {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                ext: SorobanTransactionMetaExt::read_xdr(r)?,
                events: VecM::<ContractEvent>::read_xdr(r)?,
                return_value: ScVal::read_xdr(r)?,
                diagnostic_events: VecM::<DiagnosticEvent>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for SorobanTransactionMeta {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.ext.write_xdr(w)?;
            self.events.write_xdr(w)?;
            self.return_value.write_xdr(w)?;
            self.diagnostic_events.write_xdr(w)?;
            Ok(())
        })
    }
}

/// SorobanTransactionMetaView is a borrowing equivalent of [`SorobanTransactionMeta`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SorobanTransactionMetaView<'a> {
    pub ext: SorobanTransactionMetaExt,
    pub events: VecMView<'a, ContractEventView<'a>>,
    pub return_value: ScValView<'a>,
    pub diagnostic_events: VecMView<'a, DiagnosticEventView<'a>>,
}

#[cfg(feature = "alloc")]
impl From<&SorobanTransactionMetaView<'_>> for SorobanTransactionMeta {
    #[must_use]
    fn from(v: &SorobanTransactionMetaView<'_>) -> Self {
        Self {
            ext: v.ext.clone(),
            events: v.events.to_vecm(),
            return_value: (&v.return_value).into(),
            diagnostic_events: v.diagnostic_events.to_vecm(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<SorobanTransactionMetaView<'_>> for SorobanTransactionMeta {
    #[must_use]
    fn from(v: SorobanTransactionMetaView<'_>) -> Self {
        Self::from(&v)
    }
}

impl WriteXdr for SorobanTransactionMetaView<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.ext.write_xdr(w)?;
            self.events.write_xdr(w)?;
            self.return_value.write_xdr(w)?;
            self.diagnostic_events.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "const")]
impl SorobanTransactionMetaView<'_> {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_soroban_transaction_meta(self);
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
        w.write_type_soroban_transaction_meta(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`SorobanTransactionMeta`], mirroring `<SorobanTransactionMeta as WriteXdr>::write_xdr`.
    pub const fn write_type_soroban_transaction_meta(
        &mut self,
        v: &SorobanTransactionMetaView<'_>,
    ) {
        self.write_type_soroban_transaction_meta_ext(&v.ext);
        self.write_type_vec_contract_event(&v.events);
        self.write_type_sc_val(&v.return_value);
        self.write_type_vec_diagnostic_event(&v.diagnostic_events);
    }

    /// Serializes an optional [`SorobanTransactionMeta`], mirroring `<Option<SorobanTransactionMeta> as WriteXdr>::write_xdr`.
    pub const fn write_type_option_soroban_transaction_meta(
        &mut self,
        v: &Option<SorobanTransactionMetaView<'_>>,
    ) {
        match v {
            Some(v) => {
                self.write_u32(1);
                self.write_type_soroban_transaction_meta(v);
            }
            None => {
                self.write_u32(0);
            }
        }
    }
}
