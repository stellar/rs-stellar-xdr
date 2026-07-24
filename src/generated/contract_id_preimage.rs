#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ContractIdPreimage is an XDR Union defined as:
///
/// ```text
/// union ContractIDPreimage switch (ContractIDPreimageType type)
/// {
/// case CONTRACT_ID_PREIMAGE_FROM_ADDRESS:
///     struct
///     {
///         SCAddress address;
///         uint256 salt;
///     } fromAddress;
/// case CONTRACT_ID_PREIMAGE_FROM_ASSET:
///     Asset fromAsset;
/// };
/// ```
///
// union with discriminant ContractIdPreimageType
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
pub enum ContractIdPreimage {
    Address(ContractIdPreimageFromAddress),
    Asset(Asset),
}

#[cfg(feature = "alloc")]
impl Default for ContractIdPreimage {
    fn default() -> Self {
        Self::Address(ContractIdPreimageFromAddress::default())
    }
}

impl ContractIdPreimage {
    const _VARIANTS: &[ContractIdPreimageType] = &[
        ContractIdPreimageType::Address,
        ContractIdPreimageType::Asset,
    ];
    pub const VARIANTS: [ContractIdPreimageType; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &["Address", "Asset"];
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
            Self::Address(_) => "Address",
            Self::Asset(_) => "Asset",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> ContractIdPreimageType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Address(_) => ContractIdPreimageType::Address,
            Self::Asset(_) => ContractIdPreimageType::Asset,
        }
    }

    #[must_use]
    pub const fn variants() -> [ContractIdPreimageType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for ContractIdPreimage {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<ContractIdPreimageType> for ContractIdPreimage {
    #[must_use]
    fn discriminant(&self) -> ContractIdPreimageType {
        Self::discriminant(self)
    }
}

impl Variants<ContractIdPreimageType> for ContractIdPreimage {
    fn variants() -> slice::Iter<'static, ContractIdPreimageType> {
        Self::VARIANTS.iter()
    }
}

impl Union<ContractIdPreimageType> for ContractIdPreimage {}

impl ReadXdr for ContractIdPreimage {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: ContractIdPreimageType = <ContractIdPreimageType as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                ContractIdPreimageType::Address => {
                    Self::Address(ContractIdPreimageFromAddress::read_xdr(r)?)
                }
                ContractIdPreimageType::Asset => Self::Asset(Asset::read_xdr(r)?),
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl ContractIdPreimage {
    /// Serialize this value as XDR into a [`ConstWriter`] using only const
    /// operations. This is the const counterpart to [`WriteXdr::write_xdr`].
    #[cfg(feature = "const")]
    pub const fn const_write_xdr(&self, w: &mut ConstWriter) {
        w.enter_depth();
        let d = self.discriminant();
        d.const_write_xdr(w);
        #[allow(clippy::match_same_arms)]
        match self {
            Self::Address(v) => {
                v.const_write_xdr(w);
            }
            Self::Asset(v) => {
                v.const_write_xdr(w);
            }
        }
        w.leave_depth();
    }
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[cfg(feature = "const")]
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let limits = Limits {
            depth: u32::MAX,
            len: usize::MAX,
        };
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty, &limits);
        self.const_write_xdr(&mut w);
        w.position()
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
    #[cfg(feature = "const")]
    #[must_use]
    pub const fn const_to_xdr<const N: usize>(&self) -> [u8; N] {
        let limits = Limits {
            depth: u32::MAX,
            len: usize::MAX,
        };
        let mut buf = [0u8; N];
        let mut w = ConstWriter::new(&mut buf, &limits);
        self.const_write_xdr(&mut w);
        assert!(
            w.position() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

impl WriteXdr for ContractIdPreimage {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::Address(v) => v.write_xdr(w)?,
                Self::Asset(v) => v.write_xdr(w)?,
            };
            Ok(())
        })
    }
}
