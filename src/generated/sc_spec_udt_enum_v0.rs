#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ScSpecUdtEnumV0 is an XDR Struct defined as:
///
/// ```text
/// struct SCSpecUDTEnumV0
/// {
///     string doc<SC_SPEC_DOC_LIMIT>;
///     string lib<80>;
///     string name<60>;
///     SCSpecUDTEnumCaseV0 cases<>;
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
pub struct ScSpecUdtEnumV0 {
    pub doc: StringM<1024>,
    pub lib: StringM<80>,
    pub name: StringM<60>,
    pub cases: VecM<ScSpecUdtEnumCaseV0>,
}

impl ReadXdr for ScSpecUdtEnumV0 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                doc: StringM::<1024>::read_xdr(r)?,
                lib: StringM::<80>::read_xdr(r)?,
                name: StringM::<60>::read_xdr(r)?,
                cases: VecM::<ScSpecUdtEnumCaseV0>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ScSpecUdtEnumV0 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.doc.write_xdr(w)?;
            self.lib.write_xdr(w)?;
            self.name.write_xdr(w)?;
            self.cases.write_xdr(w)?;
            Ok(())
        })
    }
}

/// ScSpecUdtEnumV0View is a borrowing equivalent of [`ScSpecUdtEnumV0`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScSpecUdtEnumV0View<'a> {
    pub doc: StringMView<'a, 1024>,
    pub lib: StringMView<'a, 80>,
    pub name: StringMView<'a, 60>,
    pub cases: VecMView<'a, ScSpecUdtEnumCaseV0View<'a>>,
}

#[cfg(feature = "alloc")]
impl From<&ScSpecUdtEnumV0View<'_>> for ScSpecUdtEnumV0 {
    #[must_use]
    fn from(v: &ScSpecUdtEnumV0View<'_>) -> Self {
        Self {
            doc: v.doc.to_stringm(),
            lib: v.lib.to_stringm(),
            name: v.name.to_stringm(),
            cases: v.cases.to_vecm(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<ScSpecUdtEnumV0View<'_>> for ScSpecUdtEnumV0 {
    #[must_use]
    fn from(v: ScSpecUdtEnumV0View<'_>) -> Self {
        Self::from(&v)
    }
}

impl WriteXdr for ScSpecUdtEnumV0View<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.doc.write_xdr(w)?;
            self.lib.write_xdr(w)?;
            self.name.write_xdr(w)?;
            self.cases.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "const")]
impl ScSpecUdtEnumV0View<'_> {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_sc_spec_udt_enum_v0(self);
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
        w.write_type_sc_spec_udt_enum_v0(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`ScSpecUdtEnumV0`], mirroring `<ScSpecUdtEnumV0 as WriteXdr>::write_xdr`.
    pub const fn write_type_sc_spec_udt_enum_v0(&mut self, v: &ScSpecUdtEnumV0View<'_>) {
        self.write_var_opaque(v.doc.as_slice());
        self.write_var_opaque(v.lib.as_slice());
        self.write_var_opaque(v.name.as_slice());
        self.write_type_vec_sc_spec_udt_enum_case_v0(&v.cases);
    }
}
