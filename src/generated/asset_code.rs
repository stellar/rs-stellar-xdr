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
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval)]
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

impl AssetCode {
    /// Serialize this value as XDR into a [`ConstWriter`] using only const
    /// operations. This is the const counterpart to [`WriteXdr::write_xdr`].
    #[cfg(feature = "const")]
    pub const fn const_write_xdr(&self, w: &mut ConstWriter) {
        w.enter_depth();
        let d = self.discriminant();
        d.const_write_xdr(w);
        #[allow(clippy::match_same_arms)]
        match self {
            Self::CreditAlphanum4(v) => {
                v.const_write_xdr(w);
            }
            Self::CreditAlphanum12(v) => {
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
