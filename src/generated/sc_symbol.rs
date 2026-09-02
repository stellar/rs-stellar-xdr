#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ScSymbol is an XDR Typedef defined as:
///
/// ```text
/// typedef string SCSymbol<SCSYMBOL_LIMIT>;
/// ```
///
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval)]
#[derive(Default, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    serde_with::serde_as,
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Debug)]
pub struct ScSymbol(pub StringM<SCSYMBOL_LIMIT>);

impl From<ScSymbol> for StringM<SCSYMBOL_LIMIT> {
    #[must_use]
    fn from(x: ScSymbol) -> Self {
        x.0
    }
}

impl From<StringM<SCSYMBOL_LIMIT>> for ScSymbol {
    #[must_use]
    fn from(x: StringM<SCSYMBOL_LIMIT>) -> Self {
        ScSymbol(x)
    }
}

impl AsRef<StringM<SCSYMBOL_LIMIT>> for ScSymbol {
    #[must_use]
    fn as_ref(&self) -> &StringM<SCSYMBOL_LIMIT> {
        &self.0
    }
}

impl ReadXdr for ScSymbol {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let i = StringM::<SCSYMBOL_LIMIT>::read_xdr(r)?;
            let v = ScSymbol(i);
            Ok(v)
        })
    }
}

impl WriteXdr for ScSymbol {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| self.0.write_xdr(w))
    }
}

impl Deref for ScSymbol {
    type Target = StringM<SCSYMBOL_LIMIT>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<ScSymbol> for Vec<u8> {
    #[must_use]
    fn from(x: ScSymbol) -> Self {
        x.0 .0
    }
}

impl TryFrom<Vec<u8>> for ScSymbol {
    type Error = Error;
    fn try_from(x: Vec<u8>) -> Result<Self, Error> {
        Ok(ScSymbol(x.try_into()?))
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<&Vec<u8>> for ScSymbol {
    type Error = Error;
    fn try_from(x: &Vec<u8>) -> Result<Self, Error> {
        Ok(ScSymbol(x.try_into()?))
    }
}

impl AsRef<Vec<u8>> for ScSymbol {
    #[must_use]
    fn as_ref(&self) -> &Vec<u8> {
        &self.0 .0
    }
}

impl AsRef<[u8]> for ScSymbol {
    #[cfg(feature = "alloc")]
    #[must_use]
    fn as_ref(&self) -> &[u8] {
        &self.0 .0
    }
    #[cfg(not(feature = "alloc"))]
    #[must_use]
    fn as_ref(&self) -> &[u8] {
        self.0 .0
    }
}

/// ScSymbolRef is a borrowing equivalent of [`ScSymbol`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScSymbolRef<'a>(pub StringMRef<'a, SCSYMBOL_LIMIT>);

#[cfg(feature = "alloc")]
impl IntoOwned for ScSymbolRef<'_> {
    type Owned = ScSymbol;
    fn into_owned(self) -> ScSymbol {
        ScSymbol(self.0.into_owned())
    }
}

#[cfg(feature = "alloc")]
impl From<&ScSymbolRef<'_>> for ScSymbol {
    #[must_use]
    fn from(v: &ScSymbolRef<'_>) -> Self {
        v.into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<ScSymbolRef<'_>> for ScSymbol {
    #[must_use]
    fn from(v: ScSymbolRef<'_>) -> Self {
        v.into_owned()
    }
}

impl WriteXdr for ScSymbolRef<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| self.0.write_xdr(w))
    }
}

#[cfg(feature = "const")]
impl ScSymbolView<'_> {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_sc_symbol(self);
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
        w.write_type_sc_symbol(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`ScSymbol`], mirroring `<ScSymbol as WriteXdr>::write_xdr`.
    pub const fn write_type_sc_symbol(&mut self, v: &ScSymbolView<'_>) {
        self.write_var_opaque(v.0.as_slice());
    }

    /// Serializes a variable-length array of [`ScSymbol`], mirroring `<VecM<ScSymbol, MAX> as WriteXdr>::write_xdr`.
    pub const fn write_type_vec_sc_symbol<const MAX: u32>(
        &mut self,
        v: &VecMView<'_, ScSymbolView<'_>, MAX>,
    ) {
        let s = v.as_slice();
        let len = s.len();
        self.write_len(len);
        let mut i = 0usize;
        while i < len {
            self.write_type_sc_symbol(&s[i]);
            i += 1;
        }
    }
}
