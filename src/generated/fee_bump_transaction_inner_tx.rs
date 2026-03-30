#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// FeeBumpTransactionInnerTx is an XDR NestedUnion defined as:
///
/// ```text
/// union switch (EnvelopeType type)
///     {
///     case ENVELOPE_TYPE_TX:
///         TransactionV1Envelope v1;
///     }
/// ```
///
// union with discriminant EnvelopeType
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
pub enum FeeBumpTransactionInnerTx {
    Tx(TransactionV1Envelope),
}

#[cfg(feature = "alloc")]
impl Default for FeeBumpTransactionInnerTx {
    fn default() -> Self {
        Self::Tx(TransactionV1Envelope::default())
    }
}

impl FeeBumpTransactionInnerTx {
    const _VARIANTS: &[EnvelopeType] = &[EnvelopeType::Tx];
    pub const VARIANTS: [EnvelopeType; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &["Tx"];
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
            Self::Tx(_) => "Tx",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> EnvelopeType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Tx(_) => EnvelopeType::Tx,
        }
    }

    #[must_use]
    pub const fn variants() -> [EnvelopeType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for FeeBumpTransactionInnerTx {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<EnvelopeType> for FeeBumpTransactionInnerTx {
    #[must_use]
    fn discriminant(&self) -> EnvelopeType {
        Self::discriminant(self)
    }
}

impl Variants<EnvelopeType> for FeeBumpTransactionInnerTx {
    fn variants() -> slice::Iter<'static, EnvelopeType> {
        Self::VARIANTS.iter()
    }
}

impl Union<EnvelopeType> for FeeBumpTransactionInnerTx {}

impl ReadXdr for FeeBumpTransactionInnerTx {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: EnvelopeType = <EnvelopeType as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                EnvelopeType::Tx => Self::Tx(TransactionV1Envelope::read_xdr(r)?),
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for FeeBumpTransactionInnerTx {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::Tx(v) => v.write_xdr(w)?,
            };
            Ok(())
        })
    }
}
