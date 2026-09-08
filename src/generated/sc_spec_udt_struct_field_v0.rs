#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ScSpecUdtStructFieldV0 is an XDR Struct defined as:
///
/// ```text
/// struct SCSpecUDTStructFieldV0
/// {
///     string doc<SC_SPEC_DOC_LIMIT>;
///     string name<30>;
///     SCSpecTypeDef type;
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
pub struct ScSpecUdtStructFieldV0 {
    pub doc: StringM<SC_SPEC_DOC_LIMIT>,
    pub name: StringM<30>,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde(rename = "type", alias = "type_")
    )]
    #[cfg_attr(feature = "schemars", schemars(rename = "type"))]
    pub type_: ScSpecTypeDef,
}

impl ReadXdr for ScSpecUdtStructFieldV0 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                doc: StringM::<SC_SPEC_DOC_LIMIT>::read_xdr(r)?,
                name: StringM::<30>::read_xdr(r)?,
                type_: ScSpecTypeDef::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ScSpecUdtStructFieldV0 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.doc.write_xdr(w)?;
            self.name.write_xdr(w)?;
            self.type_.write_xdr(w)?;
            Ok(())
        })
    }
}

/// ScSpecUdtStructFieldV0Const is a borrowing equivalent of [`ScSpecUdtStructFieldV0`] over `'static`
/// data, for const XDR encoding.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScSpecUdtStructFieldV0Const {
    pub doc: StringMConst<SC_SPEC_DOC_LIMIT>,
    pub name: StringMConst<30>,
    pub type_: ScSpecTypeDefConst,
}

#[cfg(feature = "const")]
impl ScSpecUdtStructFieldV0Const {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_sc_spec_udt_struct_field_v0(self);
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
        w.write_type_sc_spec_udt_struct_field_v0(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`ScSpecUdtStructFieldV0`], mirroring `<ScSpecUdtStructFieldV0 as WriteXdr>::write_xdr`.
    pub const fn write_type_sc_spec_udt_struct_field_v0(
        &mut self,
        v: &ScSpecUdtStructFieldV0Const,
    ) {
        self.write_var_opaque(v.doc.as_slice());
        self.write_var_opaque(v.name.as_slice());
        self.write_type_sc_spec_type_def(&v.type_);
    }

    /// Serializes a variable-length array of [`ScSpecUdtStructFieldV0`], mirroring `<VecM<ScSpecUdtStructFieldV0, MAX> as WriteXdr>::write_xdr`.
    pub const fn write_type_vec_sc_spec_udt_struct_field_v0<const MAX: u32>(
        &mut self,
        v: &VecMConst<ScSpecUdtStructFieldV0Const, MAX>,
    ) {
        let s = v.as_slice();
        let len = s.len();
        self.write_len(len);
        let mut i = 0usize;
        while i < len {
            self.write_type_sc_spec_udt_struct_field_v0(&s[i]);
            i += 1;
        }
    }
}
