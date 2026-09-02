#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ScSpecUdtStructV0 is an XDR Struct defined as:
///
/// ```text
/// struct SCSpecUDTStructV0
/// {
///     string doc<SC_SPEC_DOC_LIMIT>;
///     string lib<80>;
///     string name<60>;
///     SCSpecUDTStructFieldV0 fields<>;
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
pub struct ScSpecUdtStructV0 {
    pub doc: StringM<SC_SPEC_DOC_LIMIT>,
    pub lib: StringM<80>,
    pub name: StringM<60>,
    pub fields: VecM<ScSpecUdtStructFieldV0>,
}

impl ReadXdr for ScSpecUdtStructV0 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                doc: StringM::<SC_SPEC_DOC_LIMIT>::read_xdr(r)?,
                lib: StringM::<80>::read_xdr(r)?,
                name: StringM::<60>::read_xdr(r)?,
                fields: VecM::<ScSpecUdtStructFieldV0>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ScSpecUdtStructV0 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.doc.write_xdr(w)?;
            self.lib.write_xdr(w)?;
            self.name.write_xdr(w)?;
            self.fields.write_xdr(w)?;
            Ok(())
        })
    }
}

/// ScSpecUdtStructV0Ref is a borrowing equivalent of [`ScSpecUdtStructV0`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScSpecUdtStructV0Ref<'a> {
    pub doc: StringMRef<'a, SC_SPEC_DOC_LIMIT>,
    pub lib: StringMRef<'a, 80>,
    pub name: StringMRef<'a, 60>,
    pub fields: VecMRef<'a, ScSpecUdtStructFieldV0Ref<'a>>,
}

#[cfg(feature = "alloc")]
impl IntoOwned for ScSpecUdtStructV0Ref<'_> {
    type Owned = ScSpecUdtStructV0;
    fn into_owned(self) -> ScSpecUdtStructV0 {
        ScSpecUdtStructV0 {
            doc: self.doc.into_owned(),
            lib: self.lib.into_owned(),
            name: self.name.into_owned(),
            fields: self.fields.into_owned(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&ScSpecUdtStructV0Ref<'_>> for ScSpecUdtStructV0 {
    #[must_use]
    fn from(v: &ScSpecUdtStructV0Ref<'_>) -> Self {
        v.into_owned()
    }
}

#[cfg(feature = "alloc")]
impl From<ScSpecUdtStructV0Ref<'_>> for ScSpecUdtStructV0 {
    #[must_use]
    fn from(v: ScSpecUdtStructV0Ref<'_>) -> Self {
        v.into_owned()
    }
}

impl WriteXdr for ScSpecUdtStructV0Ref<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.doc.write_xdr(w)?;
            self.lib.write_xdr(w)?;
            self.name.write_xdr(w)?;
            self.fields.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "const")]
impl ScSpecUdtStructV0View<'_> {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_sc_spec_udt_struct_v0(self);
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
        w.write_type_sc_spec_udt_struct_v0(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`ScSpecUdtStructV0`], mirroring `<ScSpecUdtStructV0 as WriteXdr>::write_xdr`.
    pub const fn write_type_sc_spec_udt_struct_v0(&mut self, v: &ScSpecUdtStructV0View<'_>) {
        self.write_var_opaque(v.doc.as_slice());
        self.write_var_opaque(v.lib.as_slice());
        self.write_var_opaque(v.name.as_slice());
        self.write_type_vec_sc_spec_udt_struct_field_v0(&v.fields);
    }
}
