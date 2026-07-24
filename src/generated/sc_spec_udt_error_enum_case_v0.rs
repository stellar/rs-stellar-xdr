#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ScSpecUdtErrorEnumCaseV0 is an XDR Struct defined as:
///
/// ```text
/// struct SCSpecUDTErrorEnumCaseV0
/// {
///     string doc<SC_SPEC_DOC_LIMIT>;
///     string name<60>;
///     uint32 value;
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
pub struct ScSpecUdtErrorEnumCaseV0 {
    pub doc: StringM<1024>,
    pub name: StringM<60>,
    pub value: u32,
}

impl ReadXdr for ScSpecUdtErrorEnumCaseV0 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                doc: StringM::<1024>::read_xdr(r)?,
                name: StringM::<60>::read_xdr(r)?,
                value: u32::read_xdr(r)?,
            })
        })
    }
}

impl ScSpecUdtErrorEnumCaseV0 {
    /// Serialize this value as XDR into a [`ConstWriter`] using only const
    /// operations. This is the const implementation underlying `to_xdr`.
    #[cfg(feature = "std")]
    pub const fn const_write_xdr(&self, w: &mut ConstWriter) {
        w.enter_depth();
        w.write_len_prefixed(self.doc.0.as_slice());
        w.write_len_prefixed(self.name.0.as_slice());
        w.write_u32(self.value);
        w.leave_depth();
    }
}

impl WriteXdr for ScSpecUdtErrorEnumCaseV0 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.doc.write_xdr(w)?;
            self.name.write_xdr(w)?;
            self.value.write_xdr(w)?;
            Ok(())
        })
    }

    #[cfg(feature = "std")]
    fn to_xdr(&self, limits: Limits) -> Result<Vec<u8>, Error> {
        to_xdr_via_const(self, &limits, Self::const_write_xdr)
    }
}
