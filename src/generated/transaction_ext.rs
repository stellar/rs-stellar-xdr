#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// TransactionExt is an XDR NestedUnion defined as:
///
/// ```text
/// union switch (int v)
///     {
///     case 0:
///         void;
///     case 1:
///         SorobanTransactionData sorobanData;
///     }
/// ```
///
// union with discriminant i32
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
pub enum TransactionExt {
    V0,
    V1(SorobanTransactionData),
}

#[cfg(feature = "alloc")]
impl Default for TransactionExt {
    fn default() -> Self {
        Self::V0
    }
}

impl TransactionExt {
    const _VARIANTS: &[i32] = &[0, 1];
    pub const VARIANTS: [i32; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &["V0", "V1"];
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
            Self::V0 => "V0",
            Self::V1(_) => "V1",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> i32 {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0 => 0,
            Self::V1(_) => 1,
        }
    }

    #[must_use]
    pub const fn variants() -> [i32; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for TransactionExt {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<i32> for TransactionExt {
    #[must_use]
    fn discriminant(&self) -> i32 {
        Self::discriminant(self)
    }
}

impl Variants<i32> for TransactionExt {
    fn variants() -> slice::Iter<'static, i32> {
        Self::VARIANTS.iter()
    }
}

impl Union<i32> for TransactionExt {}

impl ReadXdr for TransactionExt {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: i32 = <i32 as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                0 => Self::V0,
                1 => Self::V1(SorobanTransactionData::read_xdr(r)?),
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for TransactionExt {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::V0 => ().write_xdr(w)?,
                Self::V1(v) => v.write_xdr(w)?,
            };
            Ok(())
        })
    }
}

/// TransactionExtRef is a borrowing equivalent of [`TransactionExt`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(clippy::large_enum_variant)]
pub enum TransactionExtRef<'a> {
    V0,
    V1(SorobanTransactionDataRef<'a>),
}

#[cfg(feature = "alloc")]
impl IntoOwned for TransactionExtRef<'_> {
    type Owned = TransactionExt;
    fn into_owned(self) -> TransactionExt {
        #[allow(clippy::match_same_arms)]
        match self {
            TransactionExtRef::V0 => TransactionExt::V0,
            TransactionExtRef::V1(value) => TransactionExt::V1(value.into_owned()),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&TransactionExtRef<'_>> for TransactionExt {
    #[must_use]
    fn from(v: &TransactionExtRef<'_>) -> Self {
        v.into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<TransactionExtRef<'_>> for TransactionExt {
    #[must_use]
    fn from(v: TransactionExtRef<'_>) -> Self {
        v.into_owned()
    }
}

impl TransactionExtRef<'_> {
    #[must_use]
    pub const fn discriminant(&self) -> i32 {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::V0 => 0,
            Self::V1(_) => 1,
        }
    }
}

impl WriteXdr for TransactionExtRef<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::V0 => ().write_xdr(w)?,
                Self::V1(v) => v.write_xdr(w)?,
            };
            Ok(())
        })
    }
}

#[cfg(feature = "const")]
impl TransactionExtRef<'_> {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_transaction_ext(self);
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
        w.write_type_transaction_ext(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`TransactionExt`], mirroring `<TransactionExt as WriteXdr>::write_xdr`.
    pub const fn write_type_transaction_ext(&mut self, v: &TransactionExtRef<'_>) {
        let d = v.discriminant();
        self.write_i32(d);
        #[allow(clippy::match_same_arms)]
        match v {
            TransactionExtRef::V0 => {}
            TransactionExtRef::V1(value) => {
                self.write_type_soroban_transaction_data(value);
            }
        }
    }
}
