#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ScSpecEventV0 is an XDR Struct defined as:
///
/// ```text
/// struct SCSpecEventV0
/// {
///     string doc<SC_SPEC_DOC_LIMIT>;
///     string lib<80>;
///     SCSymbol name;
///     SCSymbol prefixTopics<2>;
///     SCSpecEventParamV0 params<>;
///     SCSpecEventDataFormat dataFormat;
/// };
/// ```
///
#[cfg_attr(feature = "alloc", derive(Default))]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    serde_with::serde_as,
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct ScSpecEventV0 {
    pub doc: StringM<SC_SPEC_DOC_LIMIT>,
    pub lib: StringM<80>,
    pub name: ScSymbol,
    pub prefix_topics: VecM<ScSymbol, 2>,
    pub params: VecM<ScSpecEventParamV0>,
    pub data_format: ScSpecEventDataFormat,
}

impl ReadXdr for ScSpecEventV0 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                doc: StringM::<SC_SPEC_DOC_LIMIT>::read_xdr(r)?,
                lib: StringM::<80>::read_xdr(r)?,
                name: ScSymbol::read_xdr(r)?,
                prefix_topics: VecM::<ScSymbol, 2>::read_xdr(r)?,
                params: VecM::<ScSpecEventParamV0>::read_xdr(r)?,
                data_format: ScSpecEventDataFormat::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ScSpecEventV0 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.doc.write_xdr(w)?;
            self.lib.write_xdr(w)?;
            self.name.write_xdr(w)?;
            self.prefix_topics.write_xdr(w)?;
            self.params.write_xdr(w)?;
            self.data_format.write_xdr(w)?;
            Ok(())
        })
    }
}

/// ScSpecEventV0Ref is a borrowing equivalent of [`ScSpecEventV0`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScSpecEventV0Ref<'a> {
    pub doc: StringMRef<'a, SC_SPEC_DOC_LIMIT>,
    pub lib: StringMRef<'a, 80>,
    pub name: ScSymbolRef<'a>,
    pub prefix_topics: VecMRef<'a, ScSymbolRef<'a>, 2>,
    pub params: VecMRef<'a, ScSpecEventParamV0Ref<'a>>,
    pub data_format: ScSpecEventDataFormat,
}

#[cfg(feature = "alloc")]
impl IntoOwned for ScSpecEventV0Ref<'_> {
    type Owned = ScSpecEventV0;
    fn into_owned(self) -> ScSpecEventV0 {
        ScSpecEventV0 {
            doc: self.doc.into_owned(),
            lib: self.lib.into_owned(),
            name: self.name.into_owned(),
            prefix_topics: self.prefix_topics.into_owned(),
            params: self.params.into_owned(),
            data_format: self.data_format.into_owned(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&ScSpecEventV0Ref<'_>> for ScSpecEventV0 {
    #[must_use]
    fn from(v: &ScSpecEventV0Ref<'_>) -> Self {
        v.into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<ScSpecEventV0Ref<'_>> for ScSpecEventV0 {
    #[must_use]
    fn from(v: ScSpecEventV0Ref<'_>) -> Self {
        v.into_owned()
    }
}

impl WriteXdr for ScSpecEventV0Ref<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.doc.write_xdr(w)?;
            self.lib.write_xdr(w)?;
            self.name.write_xdr(w)?;
            self.prefix_topics.write_xdr(w)?;
            self.params.write_xdr(w)?;
            self.data_format.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "const")]
impl ScSpecEventV0View<'_> {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_sc_spec_event_v0(self);
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
        w.write_type_sc_spec_event_v0(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`ScSpecEventV0`], mirroring `<ScSpecEventV0 as WriteXdr>::write_xdr`.
    pub const fn write_type_sc_spec_event_v0(&mut self, v: &ScSpecEventV0View<'_>) {
        self.write_var_opaque(v.doc.as_slice());
        self.write_var_opaque(v.lib.as_slice());
        self.write_type_sc_symbol(&v.name);
        self.write_type_vec_sc_symbol(&v.prefix_topics);
        self.write_type_vec_sc_spec_event_param_v0(&v.params);
        self.write_type_sc_spec_event_data_format(&v.data_format);
    }
}
