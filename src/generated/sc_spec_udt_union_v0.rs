#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ScSpecUdtUnionV0 is an XDR Struct defined as:
///
/// ```text
/// struct SCSpecUDTUnionV0
/// {
///     string doc<SC_SPEC_DOC_LIMIT>;
///     string lib<80>;
///     string name<60>;
///     SCSpecUDTUnionCaseV0 cases<>;
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
pub struct ScSpecUdtUnionV0 {
    pub doc: StringM<1024>,
    pub lib: StringM<80>,
    pub name: StringM<60>,
    pub cases: VecM<ScSpecUdtUnionCaseV0>,
}

impl ReadXdr for ScSpecUdtUnionV0 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                doc: StringM::<1024>::read_xdr(r)?,
                lib: StringM::<80>::read_xdr(r)?,
                name: StringM::<60>::read_xdr(r)?,
                cases: VecM::<ScSpecUdtUnionCaseV0>::read_xdr(r)?,
            })
        })
    }
}

impl ScSpecUdtUnionV0 {
    /// Serialize this value as XDR into a [`ConstWriter`] using only const
    /// operations. This is the const implementation underlying `to_xdr`.
    #[cfg(feature = "std")]
    pub const fn const_to_xdr(&self, w: &mut ConstWriter) {
        w.enter_depth();
        w.write_len_prefixed(self.doc.0.as_slice());
        w.write_len_prefixed(self.lib.0.as_slice());
        w.write_len_prefixed(self.name.0.as_slice());
        {
            w.enter_depth();
            let __s0 = self.cases.0.as_slice();
            let __len0 = __s0.len();
            w.write_length_prefix(__len0);
            let mut __i0 = 0usize;
            while __i0 < __len0 {
                __s0[__i0].const_to_xdr(w);
                __i0 += 1;
            }
            w.leave_depth();
        }
        w.leave_depth();
    }
}

impl WriteXdr for ScSpecUdtUnionV0 {
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

    #[cfg(feature = "std")]
    fn to_xdr(&self, limits: Limits) -> Result<Vec<u8>, Error> {
        to_xdr_via_const(self, &limits, Self::const_to_xdr)
    }
}
