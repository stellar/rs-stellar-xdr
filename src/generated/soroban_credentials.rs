#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// SorobanCredentials is an XDR Union defined as:
///
/// ```text
/// union SorobanCredentials switch (SorobanCredentialsType type)
/// {
/// case SOROBAN_CREDENTIALS_SOURCE_ACCOUNT:
///     void;
/// case SOROBAN_CREDENTIALS_ADDRESS:
///     SorobanAddressCredentials address;
/// };
/// ```
///
// union with discriminant SorobanCredentialsType
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
pub enum SorobanCredentials {
    SourceAccount,
    Address(
        SorobanAddressCredentials,
    ),
}


#[cfg(feature = "alloc")]
impl Default for SorobanCredentials {
    fn default() -> Self {
        Self::SourceAccount
    }
}

impl SorobanCredentials {
    const _VARIANTS: &[SorobanCredentialsType] = &[
        SorobanCredentialsType::SourceAccount,
        SorobanCredentialsType::Address,
    ];
    pub const VARIANTS: [SorobanCredentialsType; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &[
        "SourceAccount",
        "Address",
    ];
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
            Self::SourceAccount => "SourceAccount",
            Self::Address(_) => "Address",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> SorobanCredentialsType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::SourceAccount => SorobanCredentialsType::SourceAccount,
            Self::Address(_) => SorobanCredentialsType::Address,
        }
    }

    #[must_use]
    pub const fn variants() -> [SorobanCredentialsType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for SorobanCredentials {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<SorobanCredentialsType> for SorobanCredentials {
    #[must_use]
    fn discriminant(&self) -> SorobanCredentialsType {
        Self::discriminant(self)
    }
}

impl Variants<SorobanCredentialsType> for SorobanCredentials {
    fn variants() -> slice::Iter<'static, SorobanCredentialsType> {
        Self::VARIANTS.iter()
    }
}

impl Union<SorobanCredentialsType> for SorobanCredentials {}

impl ReadXdr for SorobanCredentials {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: SorobanCredentialsType = <SorobanCredentialsType as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                SorobanCredentialsType::SourceAccount => Self::SourceAccount,
                SorobanCredentialsType::Address => Self::Address(SorobanAddressCredentials::read_xdr(r)?),
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for SorobanCredentials {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::SourceAccount => ().write_xdr(w)?,
                Self::Address(v) => v.write_xdr(w)?,
            };
            Ok(())
        })
    }
}
