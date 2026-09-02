#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// TxSetComponent is an XDR Union defined as:
///
/// ```text
/// union TxSetComponent switch (TxSetComponentType type)
/// {
/// case TXSET_COMP_TXS_MAYBE_DISCOUNTED_FEE:
///   struct
///   {
///     int64* baseFee;
///     TransactionEnvelope txs<>;
///   } txsMaybeDiscountedFee;
/// };
/// ```
///
// union with discriminant TxSetComponentType
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
pub enum TxSetComponent {
    TxsetCompTxsMaybeDiscountedFee(TxSetComponentTxsMaybeDiscountedFee),
}

#[cfg(feature = "alloc")]
impl Default for TxSetComponent {
    fn default() -> Self {
        Self::TxsetCompTxsMaybeDiscountedFee(TxSetComponentTxsMaybeDiscountedFee::default())
    }
}

impl TxSetComponent {
    const _VARIANTS: &[TxSetComponentType] = &[TxSetComponentType::TxsetCompTxsMaybeDiscountedFee];
    pub const VARIANTS: [TxSetComponentType; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &["TxsetCompTxsMaybeDiscountedFee"];
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
            Self::TxsetCompTxsMaybeDiscountedFee(_) => "TxsetCompTxsMaybeDiscountedFee",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> TxSetComponentType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::TxsetCompTxsMaybeDiscountedFee(_) => {
                TxSetComponentType::TxsetCompTxsMaybeDiscountedFee
            }
        }
    }

    #[must_use]
    pub const fn variants() -> [TxSetComponentType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for TxSetComponent {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<TxSetComponentType> for TxSetComponent {
    #[must_use]
    fn discriminant(&self) -> TxSetComponentType {
        Self::discriminant(self)
    }
}

impl Variants<TxSetComponentType> for TxSetComponent {
    fn variants() -> slice::Iter<'static, TxSetComponentType> {
        Self::VARIANTS.iter()
    }
}

impl Union<TxSetComponentType> for TxSetComponent {}

impl ReadXdr for TxSetComponent {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: TxSetComponentType = <TxSetComponentType as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                TxSetComponentType::TxsetCompTxsMaybeDiscountedFee => {
                    Self::TxsetCompTxsMaybeDiscountedFee(
                        TxSetComponentTxsMaybeDiscountedFee::read_xdr(r)?,
                    )
                }
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for TxSetComponent {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::TxsetCompTxsMaybeDiscountedFee(v) => v.write_xdr(w)?,
            };
            Ok(())
        })
    }
}

/// TxSetComponentRef is a borrowing equivalent of [`TxSetComponent`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(clippy::large_enum_variant)]
pub enum TxSetComponentRef<'a> {
    TxsetCompTxsMaybeDiscountedFee(TxSetComponentTxsMaybeDiscountedFeeRef<'a>),
}

#[cfg(feature = "alloc")]
impl IntoOwned for TxSetComponentRef<'_> {
    type Owned = TxSetComponent;
    fn into_owned(self) -> TxSetComponent {
        #[allow(clippy::match_same_arms)]
        match self {
            TxSetComponentRef::TxsetCompTxsMaybeDiscountedFee(value) => {
                TxSetComponent::TxsetCompTxsMaybeDiscountedFee(value.into_owned())
            }
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&TxSetComponentRef<'_>> for TxSetComponent {
    #[must_use]
    fn from(v: &TxSetComponentRef<'_>) -> Self {
        v.into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<TxSetComponentRef<'_>> for TxSetComponent {
    #[must_use]
    fn from(v: TxSetComponentRef<'_>) -> Self {
        v.into_owned()
    }
}

impl TxSetComponentRef<'_> {
    #[must_use]
    pub const fn discriminant(&self) -> TxSetComponentType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::TxsetCompTxsMaybeDiscountedFee(_) => {
                TxSetComponentType::TxsetCompTxsMaybeDiscountedFee
            }
        }
    }
}

impl WriteXdr for TxSetComponentRef<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::TxsetCompTxsMaybeDiscountedFee(v) => v.write_xdr(w)?,
            };
            Ok(())
        })
    }
}

#[cfg(feature = "const")]
impl TxSetComponentView<'_> {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_tx_set_component(self);
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
        w.write_type_tx_set_component(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`TxSetComponent`], mirroring `<TxSetComponent as WriteXdr>::write_xdr`.
    pub const fn write_type_tx_set_component(&mut self, v: &TxSetComponentView<'_>) {
        let d = v.discriminant();
        self.write_type_tx_set_component_type(&d);
        #[allow(clippy::match_same_arms)]
        match v {
            TxSetComponentView::TxsetCompTxsMaybeDiscountedFee(value) => {
                self.write_type_tx_set_component_txs_maybe_discounted_fee(value);
            }
        }
    }

    /// Serializes a variable-length array of [`TxSetComponent`], mirroring `<VecM<TxSetComponent, MAX> as WriteXdr>::write_xdr`.
    pub const fn write_type_vec_tx_set_component<const MAX: u32>(
        &mut self,
        v: &VecMView<'_, TxSetComponentView<'_>, MAX>,
    ) {
        let s = v.as_slice();
        let len = s.len();
        self.write_len(len);
        let mut i = 0usize;
        while i < len {
            self.write_type_tx_set_component(&s[i]);
            i += 1;
        }
    }
}
