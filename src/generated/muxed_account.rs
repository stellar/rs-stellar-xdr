#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// MuxedAccount is an XDR Union defined as:
///
/// ```text
/// union MuxedAccount switch (CryptoKeyType type)
/// {
/// case KEY_TYPE_ED25519:
///     uint256 ed25519;
/// case KEY_TYPE_MUXED_ED25519:
///     struct
///     {
///         uint64 id;
///         uint256 ed25519;
///     } med25519;
/// };
/// ```
///
// union with discriminant CryptoKeyType
#[cfg_eval::cfg_eval]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde_with::SerializeDisplay, serde_with::DeserializeFromStr)
)]
#[allow(clippy::large_enum_variant)]
pub enum MuxedAccount {
    Ed25519(
        Uint256,
    ),
    MuxedEd25519(
        MuxedAccountMed25519,
    ),
}


#[cfg(feature = "alloc")]
impl Default for MuxedAccount {
    fn default() -> Self {
        Self::Ed25519(Uint256::default())
    }
}

impl MuxedAccount {
    const _VARIANTS: &[CryptoKeyType] = &[
        CryptoKeyType::Ed25519,
        CryptoKeyType::MuxedEd25519,
    ];
    pub const VARIANTS: [CryptoKeyType; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &[
        "Ed25519",
        "MuxedEd25519",
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
            Self::Ed25519(_) => "Ed25519",
            Self::MuxedEd25519(_) => "MuxedEd25519",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> CryptoKeyType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Ed25519(_) => CryptoKeyType::Ed25519,
            Self::MuxedEd25519(_) => CryptoKeyType::MuxedEd25519,
        }
    }

    #[must_use]
    pub const fn variants() -> [CryptoKeyType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for MuxedAccount {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<CryptoKeyType> for MuxedAccount {
    #[must_use]
    fn discriminant(&self) -> CryptoKeyType {
        Self::discriminant(self)
    }
}

impl Variants<CryptoKeyType> for MuxedAccount {
    fn variants() -> slice::Iter<'static, CryptoKeyType> {
        Self::VARIANTS.iter()
    }
}

impl Union<CryptoKeyType> for MuxedAccount {}

impl ReadXdr for MuxedAccount {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: CryptoKeyType = <CryptoKeyType as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                CryptoKeyType::Ed25519 => Self::Ed25519(Uint256::read_xdr(r)?),
                CryptoKeyType::MuxedEd25519 => Self::MuxedEd25519(MuxedAccountMed25519::read_xdr(r)?),
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for MuxedAccount {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::Ed25519(v) => v.write_xdr(w)?,
                Self::MuxedEd25519(v) => v.write_xdr(w)?,
            };
            Ok(())
        })
    }
}
