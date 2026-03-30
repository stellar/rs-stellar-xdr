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
#[cfg_eval::cfg_eval]
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
