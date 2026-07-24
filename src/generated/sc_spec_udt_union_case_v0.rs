#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ScSpecUdtUnionCaseV0 is an XDR Union defined as:
///
/// ```text
/// union SCSpecUDTUnionCaseV0 switch (SCSpecUDTUnionCaseV0Kind kind)
/// {
/// case SC_SPEC_UDT_UNION_CASE_VOID_V0:
///     SCSpecUDTUnionCaseVoidV0 voidCase;
/// case SC_SPEC_UDT_UNION_CASE_TUPLE_V0:
///     SCSpecUDTUnionCaseTupleV0 tupleCase;
/// };
/// ```
///
// union with discriminant ScSpecUdtUnionCaseV0Kind
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
pub enum ScSpecUdtUnionCaseV0 {
    VoidV0(ScSpecUdtUnionCaseVoidV0),
    TupleV0(ScSpecUdtUnionCaseTupleV0),
}

#[cfg(feature = "alloc")]
impl Default for ScSpecUdtUnionCaseV0 {
    fn default() -> Self {
        Self::VoidV0(ScSpecUdtUnionCaseVoidV0::default())
    }
}

impl ScSpecUdtUnionCaseV0 {
    const _VARIANTS: &[ScSpecUdtUnionCaseV0Kind] = &[
        ScSpecUdtUnionCaseV0Kind::VoidV0,
        ScSpecUdtUnionCaseV0Kind::TupleV0,
    ];
    pub const VARIANTS: [ScSpecUdtUnionCaseV0Kind; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &["VoidV0", "TupleV0"];
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
            Self::VoidV0(_) => "VoidV0",
            Self::TupleV0(_) => "TupleV0",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> ScSpecUdtUnionCaseV0Kind {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::VoidV0(_) => ScSpecUdtUnionCaseV0Kind::VoidV0,
            Self::TupleV0(_) => ScSpecUdtUnionCaseV0Kind::TupleV0,
        }
    }

    #[must_use]
    pub const fn variants() -> [ScSpecUdtUnionCaseV0Kind; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for ScSpecUdtUnionCaseV0 {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<ScSpecUdtUnionCaseV0Kind> for ScSpecUdtUnionCaseV0 {
    #[must_use]
    fn discriminant(&self) -> ScSpecUdtUnionCaseV0Kind {
        Self::discriminant(self)
    }
}

impl Variants<ScSpecUdtUnionCaseV0Kind> for ScSpecUdtUnionCaseV0 {
    fn variants() -> slice::Iter<'static, ScSpecUdtUnionCaseV0Kind> {
        Self::VARIANTS.iter()
    }
}

impl Union<ScSpecUdtUnionCaseV0Kind> for ScSpecUdtUnionCaseV0 {}

impl ReadXdr for ScSpecUdtUnionCaseV0 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: ScSpecUdtUnionCaseV0Kind = <ScSpecUdtUnionCaseV0Kind as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                ScSpecUdtUnionCaseV0Kind::VoidV0 => {
                    Self::VoidV0(ScSpecUdtUnionCaseVoidV0::read_xdr(r)?)
                }
                ScSpecUdtUnionCaseV0Kind::TupleV0 => {
                    Self::TupleV0(ScSpecUdtUnionCaseTupleV0::read_xdr(r)?)
                }
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl ScSpecUdtUnionCaseV0 {
    /// Serialize this value as XDR into a [`ConstWriter`] using only const
    /// operations. This is the const implementation underlying `to_xdr`.
    #[cfg(feature = "std")]
    pub const fn const_write_xdr(&self, w: &mut ConstWriter) {
        w.enter_depth();
        let d = self.discriminant();
        d.const_write_xdr(w);
        #[allow(clippy::match_same_arms)]
        match self {
            Self::VoidV0(v) => {
                v.const_write_xdr(w);
            }
            Self::TupleV0(v) => {
                v.const_write_xdr(w);
            }
        }
        w.leave_depth();
    }
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::to_xdr_array`] at compile time.
    #[cfg(feature = "std")]
    #[must_use]
    pub const fn xdr_len(&self) -> usize {
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
    /// operations.
    ///
    /// `N` must equal [`Self::xdr_len`]. It is intended for callers, such as a
    /// proc-macro, that compute the length with `xdr_len` and pass it as `N`;
    /// `to_xdr_array` itself does not need to call `xdr_len`.
    ///
    /// # Panics
    ///
    /// Panics if `N` does not equal the value's [`Self::xdr_len`].
    #[cfg(feature = "std")]
    #[must_use]
    pub const fn to_xdr_array<const N: usize>(&self) -> [u8; N] {
        let limits = Limits {
            depth: u32::MAX,
            len: usize::MAX,
        };
        let mut buf = [0u8; N];
        let mut w = ConstWriter::new(&mut buf, &limits);
        self.const_write_xdr(&mut w);
        assert!(
            w.position() == N,
            "to_xdr_array: N does not equal the XDR-encoded length"
        );
        buf
    }
}

impl WriteXdr for ScSpecUdtUnionCaseV0 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        write_xdr_via_const(self, w, Self::const_write_xdr)
    }
}
