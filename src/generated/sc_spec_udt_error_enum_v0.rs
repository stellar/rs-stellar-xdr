#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ScSpecUdtErrorEnumV0 is an XDR Struct defined as:
///
/// ```text
/// struct SCSpecUDTErrorEnumV0
/// {
///     string doc<SC_SPEC_DOC_LIMIT>;
///     string lib<80>;
///     string name<60>;
///     opaque id[SC_SPEC_TYPE_ID_LEN];
///     SCSpecUDTErrorEnumCaseV0 cases<>;
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
pub struct ScSpecUdtErrorEnumV0 {
    pub doc: StringM<1024>,
    pub lib: StringM<80>,
    pub name: StringM<60>,
    pub id: [u8; 8],
    pub cases: VecM<ScSpecUdtErrorEnumCaseV0>,
}

impl ReadXdr for ScSpecUdtErrorEnumV0 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                doc: StringM::<1024>::read_xdr(r)?,
                lib: StringM::<80>::read_xdr(r)?,
                name: StringM::<60>::read_xdr(r)?,
                id: <[u8; 8]>::read_xdr(r)?,
                cases: VecM::<ScSpecUdtErrorEnumCaseV0>::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ScSpecUdtErrorEnumV0 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.doc.write_xdr(w)?;
            self.lib.write_xdr(w)?;
            self.name.write_xdr(w)?;
            self.id.write_xdr(w)?;
            self.cases.write_xdr(w)?;
            Ok(())
        })
    }
}

/// ScSpecUdtErrorEnumV0View is a borrowing equivalent of [`ScSpecUdtErrorEnumV0`], usable in
/// const contexts and convertible to the owned type via [`From`]/[`Into`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScSpecUdtErrorEnumV0View<'a> {
    pub doc: StringMView<'a, 1024>,
    pub lib: StringMView<'a, 80>,
    pub name: StringMView<'a, 60>,
    pub id: [u8; 8],
    pub cases: VecMView<'a, ScSpecUdtErrorEnumCaseV0View<'a>>,
}

#[cfg(feature = "alloc")]
impl From<&ScSpecUdtErrorEnumV0View<'_>> for ScSpecUdtErrorEnumV0 {
    #[must_use]
    fn from(v: &ScSpecUdtErrorEnumV0View<'_>) -> Self {
        Self {
            doc: v.doc.to_stringm(),
            lib: v.lib.to_stringm(),
            name: v.name.to_stringm(),
            id: v.id,
            cases: v.cases.to_vecm_from(),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<ScSpecUdtErrorEnumV0View<'_>> for ScSpecUdtErrorEnumV0 {
    #[must_use]
    fn from(v: ScSpecUdtErrorEnumV0View<'_>) -> Self {
        Self::from(&v)
    }
}

impl WriteXdr for ScSpecUdtErrorEnumV0View<'_> {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.doc.write_xdr(w)?;
            self.lib.write_xdr(w)?;
            self.name.write_xdr(w)?;
            self.id.write_xdr(w)?;
            self.cases.write_xdr(w)?;
            Ok(())
        })
    }
}
#[cfg(feature = "const")]
impl ScSpecUdtErrorEnumV0View<'_> {
    /// Serialize this value as XDR into a [`ConstWriter`] using only const
    /// operations. This is the const counterpart to the owned type's
    /// [`WriteXdr::write_xdr`].
    pub const fn const_write_xdr(&self, w: &mut ConstWriter) {
        w.enter_depth();
        w.write_len_prefixed(self.doc.as_slice());
        w.write_len_prefixed(self.lib.as_slice());
        w.write_len_prefixed(self.name.as_slice());
        w.write_fixed_opaque(&self.id);
        {
            w.enter_depth();
            let __s0 = self.cases.as_slice();
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
