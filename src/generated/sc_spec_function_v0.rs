#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ScSpecFunctionV0 is an XDR Struct defined as:
///
/// ```text
/// struct SCSpecFunctionV0
/// {
///     string doc<SC_SPEC_DOC_LIMIT>;
///     SCSymbol name;
///     SCSpecFunctionInputV0 inputs<>;
///     SCSpecTypeDef outputs<1>;
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
pub struct ScSpecFunctionV0 {
    pub doc: StringM<1024>,
    pub name: ScSymbol,
    pub inputs: VecM<ScSpecFunctionInputV0>,
    pub outputs: VecM<ScSpecTypeDef, 1>,
}

impl ReadXdr for ScSpecFunctionV0 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                doc: StringM::<1024>::read_xdr(r)?,
                name: ScSymbol::read_xdr(r)?,
                inputs: VecM::<ScSpecFunctionInputV0>::read_xdr(r)?,
                outputs: VecM::<ScSpecTypeDef, 1>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ScSpecFunctionV0 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.doc.write_xdr(w)?;
            self.name.write_xdr(w)?;
            self.inputs.write_xdr(w)?;
            self.outputs.write_xdr(w)?;
            Ok(())
        })
    }
}

/// ScSpecFunctionV0View is a borrowing equivalent of [`ScSpecFunctionV0`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScSpecFunctionV0View<'a> {
    pub doc: StringMView<'a, 1024>,
    pub name: ScSymbolView<'a>,
    pub inputs: VecMView<'a, ScSpecFunctionInputV0View<'a>>,
    pub outputs: VecMView<'a, ScSpecTypeDefView<'a>, 1>,
}

#[cfg(feature = "alloc")]
impl From<&ScSpecFunctionV0View<'_>> for ScSpecFunctionV0 {
    #[must_use]
    fn from(v: &ScSpecFunctionV0View<'_>) -> Self {
        Self {
            doc: v.doc.to_stringm(),
            name: (&v.name).into(),
            inputs: v.inputs.to_vecm_from(),
            outputs: v.outputs.to_vecm_from(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<ScSpecFunctionV0View<'_>> for ScSpecFunctionV0 {
    #[must_use]
    fn from(v: ScSpecFunctionV0View<'_>) -> Self {
        Self::from(&v)
    }
}

impl WriteXdr for ScSpecFunctionV0View<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.doc.write_xdr(w)?;
            self.name.write_xdr(w)?;
            self.inputs.write_xdr(w)?;
            self.outputs.write_xdr(w)?;
            Ok(())
        })
    }
}
#[cfg(feature = "const")]
impl ScSpecFunctionV0View<'_> {
    /// Serialize this value as XDR into a [`ConstWriter`] using only const
    /// operations. This is the const counterpart to the owned type's
    /// [`WriteXdr::write_xdr`].
    pub const fn const_write_xdr(&self, w: &mut ConstWriter) {
        w.enter_depth();
        w.write_len_prefixed(self.doc.as_slice());
        self.name.const_write_xdr(w);
        {
            w.enter_depth();
            let __s0 = self.inputs.as_slice();
            let __len0 = __s0.len();
            w.write_length_prefix(__len0);
            let mut __i0 = 0usize;
            while __i0 < __len0 {
                __s0[__i0].const_write_xdr(w);
                __i0 += 1;
            }
            w.leave_depth();
        }
        {
            w.enter_depth();
            let __s0 = self.outputs.as_slice();
            let __len0 = __s0.len();
            w.write_length_prefix(__len0);
            let mut __i0 = 0usize;
            while __i0 < __len0 {
                __s0[__i0].const_write_xdr(w);
                __i0 += 1;
            }
            w.leave_depth();
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
