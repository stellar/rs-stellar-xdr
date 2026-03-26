#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// AssetCode is an XDR Union defined as:
///
/// ```text
/// union AssetCode switch (AssetType type)
/// {
/// case ASSET_TYPE_CREDIT_ALPHANUM4:
///     AssetCode4 assetCode4;
///
/// case ASSET_TYPE_CREDIT_ALPHANUM12:
///     AssetCode12 assetCode12;
///
///     // add other asset types here in the future
/// };
/// ```
///
// union with discriminant AssetType
#[cfg_eval::cfg_eval]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde_with::SerializeDisplay, serde_with::DeserializeFromStr)
)]
#[allow(clippy::large_enum_variant)]
pub enum AssetCode {
    CreditAlphanum4(AssetCode4),
    CreditAlphanum12(AssetCode12),
}

#[cfg(feature = "alloc")]
impl Default for AssetCode {
    fn default() -> Self {
        Self::CreditAlphanum4(AssetCode4::default())
    }
}

impl AssetCode {
    const _VARIANTS: &[AssetType] = &[AssetType::CreditAlphanum4, AssetType::CreditAlphanum12];
    pub const VARIANTS: [AssetType; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &["CreditAlphanum4", "CreditAlphanum12"];
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
            Self::CreditAlphanum4(_) => "CreditAlphanum4",
            Self::CreditAlphanum12(_) => "CreditAlphanum12",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> AssetType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::CreditAlphanum4(_) => AssetType::CreditAlphanum4,
            Self::CreditAlphanum12(_) => AssetType::CreditAlphanum12,
        }
    }

    #[must_use]
    pub const fn variants() -> [AssetType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for AssetCode {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<AssetType> for AssetCode {
    #[must_use]
    fn discriminant(&self) -> AssetType {
        Self::discriminant(self)
    }
}

impl Variants<AssetType> for AssetCode {
    fn variants() -> slice::Iter<'static, AssetType> {
        Self::VARIANTS.iter()
    }
}

impl Union<AssetType> for AssetCode {}

impl ReadXdr for AssetCode {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: AssetType = <AssetType as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                AssetType::CreditAlphanum4 => Self::CreditAlphanum4(AssetCode4::read_xdr(r)?),
                AssetType::CreditAlphanum12 => Self::CreditAlphanum12(AssetCode12::read_xdr(r)?),
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for AssetCode {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::CreditAlphanum4(v) => v.write_xdr(w)?,
                Self::CreditAlphanum12(v) => v.write_xdr(w)?,
            };
            Ok(())
        })
    }
}
