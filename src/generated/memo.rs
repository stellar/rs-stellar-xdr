#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// Memo is an XDR Union defined as:
///
/// ```text
/// union Memo switch (MemoType type)
/// {
/// case MEMO_NONE:
///     void;
/// case MEMO_TEXT:
///     string text<28>;
/// case MEMO_ID:
///     uint64 id;
/// case MEMO_HASH:
///     Hash hash; // the hash of what to pull from the content server
/// case MEMO_RETURN:
///     Hash retHash; // the hash of the tx you are rejecting
/// };
/// ```
///
// union with discriminant MemoType
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
pub enum Memo {
    None,
    Text(StringM<28>),
    Id(
        #[cfg_attr(
            all(feature = "serde", feature = "alloc"),
            serde_as(as = "NumberOrString")
        )]
        u64,
    ),
    Hash(Hash),
    Return(Hash),
}

#[cfg(feature = "alloc")]
impl Default for Memo {
    fn default() -> Self {
        Self::None
    }
}

impl Memo {
    const _VARIANTS: &[MemoType] = &[
        MemoType::None,
        MemoType::Text,
        MemoType::Id,
        MemoType::Hash,
        MemoType::Return,
    ];
    pub const VARIANTS: [MemoType; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &["None", "Text", "Id", "Hash", "Return"];
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
            Self::None => "None",
            Self::Text(_) => "Text",
            Self::Id(_) => "Id",
            Self::Hash(_) => "Hash",
            Self::Return(_) => "Return",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> MemoType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::None => MemoType::None,
            Self::Text(_) => MemoType::Text,
            Self::Id(_) => MemoType::Id,
            Self::Hash(_) => MemoType::Hash,
            Self::Return(_) => MemoType::Return,
        }
    }

    #[must_use]
    pub const fn variants() -> [MemoType; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for Memo {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<MemoType> for Memo {
    #[must_use]
    fn discriminant(&self) -> MemoType {
        Self::discriminant(self)
    }
}

impl Variants<MemoType> for Memo {
    fn variants() -> slice::Iter<'static, MemoType> {
        Self::VARIANTS.iter()
    }
}

impl Union<MemoType> for Memo {}

impl ReadXdr for Memo {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: MemoType = <MemoType as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                MemoType::None => Self::None,
                MemoType::Text => Self::Text(StringM::<28>::read_xdr(r)?),
                MemoType::Id => Self::Id(u64::read_xdr(r)?),
                MemoType::Hash => Self::Hash(Hash::read_xdr(r)?),
                MemoType::Return => Self::Return(Hash::read_xdr(r)?),
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for Memo {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::None => ().write_xdr(w)?,
                Self::Text(v) => v.write_xdr(w)?,
                Self::Id(v) => v.write_xdr(w)?,
                Self::Hash(v) => v.write_xdr(w)?,
                Self::Return(v) => v.write_xdr(w)?,
            };
            Ok(())
        })
    }
}

/// MemoRef is a borrowing equivalent of [`Memo`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(clippy::large_enum_variant)]
pub enum MemoRef<'a> {
    None,
    Text(StringMRef<'a, 28>),
    Id(u64),
    Hash(Hash),
    Return(Hash),
}

#[cfg(feature = "alloc")]
impl IntoOwned for MemoRef<'_> {
    type Owned = Memo;
    fn into_owned(self) -> Memo {
        #[allow(clippy::match_same_arms)]
        match self {
            MemoRef::None => Memo::None,
            MemoRef::Text(value) => Memo::Text(value.into_owned()),
            MemoRef::Id(value) => Memo::Id(value.into_owned()),
            MemoRef::Hash(value) => Memo::Hash(value.into_owned()),
            MemoRef::Return(value) => Memo::Return(value.into_owned()),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&MemoRef<'_>> for Memo {
    #[must_use]
    fn from(v: &MemoRef<'_>) -> Self {
        v.into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<MemoRef<'_>> for Memo {
    #[must_use]
    fn from(v: MemoRef<'_>) -> Self {
        v.into_owned()
    }
}

impl MemoRef<'_> {
    #[must_use]
    pub const fn discriminant(&self) -> MemoType {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::None => MemoType::None,
            Self::Text(_) => MemoType::Text,
            Self::Id(_) => MemoType::Id,
            Self::Hash(_) => MemoType::Hash,
            Self::Return(_) => MemoType::Return,
        }
    }
}

impl WriteXdr for MemoRef<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::None => ().write_xdr(w)?,
                Self::Text(v) => v.write_xdr(w)?,
                Self::Id(v) => v.write_xdr(w)?,
                Self::Hash(v) => v.write_xdr(w)?,
                Self::Return(v) => v.write_xdr(w)?,
            };
            Ok(())
        })
    }
}

#[cfg(feature = "const")]
impl MemoView<'_> {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_memo(self);
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
        w.write_type_memo(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`Memo`], mirroring `<Memo as WriteXdr>::write_xdr`.
    pub const fn write_type_memo(&mut self, v: &MemoView<'_>) {
        let d = v.discriminant();
        self.write_type_memo_type(&d);
        #[allow(clippy::match_same_arms)]
        match v {
            MemoView::None => {}
            MemoView::Text(value) => {
                self.write_var_opaque(value.as_slice());
            }
            MemoView::Id(value) => {
                self.write_u64(*value);
            }
            MemoView::Hash(value) => {
                self.write_type_hash(value);
            }
            MemoView::Return(value) => {
                self.write_type_hash(value);
            }
        }
    }
}
