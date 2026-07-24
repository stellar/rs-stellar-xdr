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
/// case SOROBAN_CREDENTIALS_ADDRESS_V2:
///     SorobanAddressCredentials addressV2;
/// case SOROBAN_CREDENTIALS_ADDRESS_WITH_DELEGATES:
///     SorobanAddressCredentialsWithDelegates addressWithDelegates;
/// };
/// ```
///
// union with discriminant SorobanCredentialsType
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
pub enum SorobanCredentials {
    SourceAccount,
    Address(SorobanAddressCredentials),
    AddressV2(SorobanAddressCredentials),
    AddressWithDelegates(SorobanAddressCredentialsWithDelegates),
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
        SorobanCredentialsType::AddressV2,
        SorobanCredentialsType::AddressWithDelegates,
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
        "AddressV2",
        "AddressWithDelegates",
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
            Self::AddressV2(_) => "AddressV2",
            Self::AddressWithDelegates(_) => "AddressWithDelegates",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> SorobanCredentialsType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::SourceAccount => SorobanCredentialsType::SourceAccount,
            Self::Address(_) => SorobanCredentialsType::Address,
            Self::AddressV2(_) => SorobanCredentialsType::AddressV2,
            Self::AddressWithDelegates(_) => SorobanCredentialsType::AddressWithDelegates,
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
                SorobanCredentialsType::Address => {
                    Self::Address(SorobanAddressCredentials::read_xdr(r)?)
                }
                SorobanCredentialsType::AddressV2 => {
                    Self::AddressV2(SorobanAddressCredentials::read_xdr(r)?)
                }
                SorobanCredentialsType::AddressWithDelegates => {
                    Self::AddressWithDelegates(SorobanAddressCredentialsWithDelegates::read_xdr(r)?)
                }
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl SorobanCredentials {
    /// Serialize this value as XDR into a [`ConstWriter`] using only const
    /// operations. This is the const implementation underlying `to_xdr`.
    #[cfg(feature = "std")]
    pub const fn const_write_xdr(&self, w: &mut ConstWriter) {
        w.enter_depth();
        let d = self.discriminant();
        d.const_write_xdr(w);
        #[allow(clippy::match_same_arms)]
        match self {
            Self::SourceAccount => {}
            Self::Address(v) => {
                v.const_write_xdr(w);
            }
            Self::AddressV2(v) => {
                v.const_write_xdr(w);
            }
            Self::AddressWithDelegates(v) => {
                v.const_write_xdr(w);
            }
        }
        w.leave_depth();
    }
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::to_xdr_array`] at compile time.
    #[cfg(feature = "std")]
    #[must_use]
    pub const fn xdr_len(&self) -> usize {
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
    /// operations.
    ///
    /// `N` must equal [`Self::xdr_len`]. It is intended for callers, such as a
    /// proc-macro, that compute the length with `xdr_len` and pass it as `N`;
    /// `to_xdr_array` itself does not need to call `xdr_len`.
    ///
    /// # Panics
    ///
    /// Panics if `N` does not equal the value's [`Self::xdr_len`].
    #[cfg(feature = "std")]
    #[must_use]
    pub const fn to_xdr_array<const N: usize>(&self) -> [u8; N] {
        let limits = Limits {
            depth: u32::MAX,
            len: usize::MAX,
        };
        let mut buf = [0u8; N];
        let mut w = ConstWriter::new(&mut buf, &limits);
        self.const_write_xdr(&mut w);
        assert!(
            w.position() == N,
            "to_xdr_array: N does not equal the XDR-encoded length"
        );
        buf
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
                Self::AddressV2(v) => v.write_xdr(w)?,
                Self::AddressWithDelegates(v) => v.write_xdr(w)?,
            };
            Ok(())
        })
    }

    #[cfg(feature = "std")]
    fn to_xdr(&self, limits: Limits) -> Result<Vec<u8>, Error> {
        to_xdr_via_const(self, &limits, Self::const_write_xdr)
    }
}
