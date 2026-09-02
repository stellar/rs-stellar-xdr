#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// TransactionEnvelope is an XDR Union defined as:
///
/// ```text
/// union TransactionEnvelope switch (EnvelopeType type)
/// {
/// case ENVELOPE_TYPE_TX_V0:
///     TransactionV0Envelope v0;
/// case ENVELOPE_TYPE_TX:
///     TransactionV1Envelope v1;
/// case ENVELOPE_TYPE_TX_FEE_BUMP:
///     FeeBumpTransactionEnvelope feeBump;
/// };
/// ```
///
// union with discriminant EnvelopeType
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval)]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    serde_with::serde_as,
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[allow(clippy::large_enum_variant)]
pub enum TransactionEnvelope {
    TxV0(TransactionV0Envelope),
    Tx(TransactionV1Envelope),
    TxFeeBump(FeeBumpTransactionEnvelope),
}

impl TransactionEnvelope {
    const _VARIANTS: &[EnvelopeType] = &[
        EnvelopeType::TxV0,
        EnvelopeType::Tx,
        EnvelopeType::TxFeeBump,
    ];
    pub const VARIANTS: [EnvelopeType; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &["TxV0", "Tx", "TxFeeBump"];
    pub const VARIANTS_STR: [&'static str; Self::_VARIANTS_STR.len()] = {
        let mut arr = [Self::_VARIANTS_STR[0]; Self::_VARIANTS_STR.len()];
        let mut i = 1;
        while i < Self::_VARIANTS_STR.len() {
            arr[i] = Self::_VARIANTS_STR[i];
            i += 1;
        }
        arr
    };

    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::TxV0(_) => "TxV0",
            Self::Tx(_) => "Tx",
            Self::TxFeeBump(_) => "TxFeeBump",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> EnvelopeType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::TxV0(_) => EnvelopeType::TxV0,
            Self::Tx(_) => EnvelopeType::Tx,
            Self::TxFeeBump(_) => EnvelopeType::TxFeeBump,
        }
    }

    #[must_use]
    pub const fn variants() -> [EnvelopeType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for TransactionEnvelope {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<EnvelopeType> for TransactionEnvelope {
    #[must_use]
    fn discriminant(&self) -> EnvelopeType {
        Self::discriminant(self)
    }
}

impl Variants<EnvelopeType> for TransactionEnvelope {
    fn variants() -> slice::Iter<'static, EnvelopeType> {
        Self::VARIANTS.iter()
    }
}

impl Union<EnvelopeType> for TransactionEnvelope {}

impl ReadXdr for TransactionEnvelope {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: EnvelopeType = <EnvelopeType as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                EnvelopeType::TxV0 => Self::TxV0(TransactionV0Envelope::read_xdr(r)?),
                EnvelopeType::Tx => Self::Tx(TransactionV1Envelope::read_xdr(r)?),
                EnvelopeType::TxFeeBump => {
                    Self::TxFeeBump(FeeBumpTransactionEnvelope::read_xdr(r)?)
                }
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for TransactionEnvelope {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::TxV0(v) => v.write_xdr(w)?,
                Self::Tx(v) => v.write_xdr(w)?,
                Self::TxFeeBump(v) => v.write_xdr(w)?,
            };
            Ok(())
        })
    }
}

/// TransactionEnvelopeRef is a borrowing equivalent of [`TransactionEnvelope`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(clippy::large_enum_variant)]
pub enum TransactionEnvelopeRef<'a> {
    TxV0(TransactionV0EnvelopeRef<'a>),
    Tx(TransactionV1EnvelopeRef<'a>),
    TxFeeBump(FeeBumpTransactionEnvelopeRef<'a>),
}

#[cfg(feature = "alloc")]
impl IntoOwned for TransactionEnvelopeRef<'_> {
    type Owned = TransactionEnvelope;
    fn into_owned(self) -> TransactionEnvelope {
        #[allow(clippy::match_same_arms)]
        match self {
            TransactionEnvelopeRef::TxV0(value) => TransactionEnvelope::TxV0(value.into_owned()),
            TransactionEnvelopeRef::Tx(value) => TransactionEnvelope::Tx(value.into_owned()),
            TransactionEnvelopeRef::TxFeeBump(value) => {
                TransactionEnvelope::TxFeeBump(value.into_owned())
            }
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&TransactionEnvelopeRef<'_>> for TransactionEnvelope {
    #[must_use]
    fn from(v: &TransactionEnvelopeRef<'_>) -> Self {
        v.into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<TransactionEnvelopeRef<'_>> for TransactionEnvelope {
    #[must_use]
    fn from(v: TransactionEnvelopeRef<'_>) -> Self {
        v.into_owned()
    }
}

impl TransactionEnvelopeRef<'_> {
    #[must_use]
    pub const fn discriminant(&self) -> EnvelopeType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::TxV0(_) => EnvelopeType::TxV0,
            Self::Tx(_) => EnvelopeType::Tx,
            Self::TxFeeBump(_) => EnvelopeType::TxFeeBump,
        }
    }
}

impl WriteXdr for TransactionEnvelopeRef<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::TxV0(v) => v.write_xdr(w)?,
                Self::Tx(v) => v.write_xdr(w)?,
                Self::TxFeeBump(v) => v.write_xdr(w)?,
            };
            Ok(())
        })
    }
}

#[cfg(feature = "const")]
impl TransactionEnvelopeView<'_> {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_transaction_envelope(self);
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
        w.write_type_transaction_envelope(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`TransactionEnvelope`], mirroring `<TransactionEnvelope as WriteXdr>::write_xdr`.
    pub const fn write_type_transaction_envelope(&mut self, v: &TransactionEnvelopeView<'_>) {
        let d = v.discriminant();
        self.write_type_envelope_type(&d);
        #[allow(clippy::match_same_arms)]
        match v {
            TransactionEnvelopeView::TxV0(value) => {
                self.write_type_transaction_v0_envelope(value);
            }
            TransactionEnvelopeView::Tx(value) => {
                self.write_type_transaction_v1_envelope(value);
            }
            TransactionEnvelopeView::TxFeeBump(value) => {
                self.write_type_fee_bump_transaction_envelope(value);
            }
        }
    }

    /// Serializes a variable-length array of [`TransactionEnvelope`], mirroring `<VecM<TransactionEnvelope, MAX> as WriteXdr>::write_xdr`.
    pub const fn write_type_vec_transaction_envelope<const MAX: u32>(
        &mut self,
        v: &VecMView<'_, TransactionEnvelopeView<'_>, MAX>,
    ) {
        let s = v.as_slice();
        let len = s.len();
        self.write_len(len);
        let mut i = 0usize;
        while i < len {
            self.write_type_transaction_envelope(&s[i]);
            i += 1;
        }
    }
}
