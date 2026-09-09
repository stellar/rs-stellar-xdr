#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ScSpecEntry is an XDR Union defined as:
///
/// ```text
/// union SCSpecEntry switch (SCSpecEntryKind kind)
/// {
/// case SC_SPEC_ENTRY_FUNCTION_V0:
///     SCSpecFunctionV0 functionV0;
/// case SC_SPEC_ENTRY_UDT_STRUCT_V0:
///     SCSpecUDTStructV0 udtStructV0;
/// case SC_SPEC_ENTRY_UDT_UNION_V0:
///     SCSpecUDTUnionV0 udtUnionV0;
/// case SC_SPEC_ENTRY_UDT_ENUM_V0:
///     SCSpecUDTEnumV0 udtEnumV0;
/// case SC_SPEC_ENTRY_UDT_ERROR_ENUM_V0:
///     SCSpecUDTErrorEnumV0 udtErrorEnumV0;
/// case SC_SPEC_ENTRY_EVENT_V0:
///     SCSpecEventV0 eventV0;
/// };
/// ```
///
// union with discriminant ScSpecEntryKind
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
pub enum ScSpecEntry {
    FunctionV0(ScSpecFunctionV0),
    UdtStructV0(ScSpecUdtStructV0),
    UdtUnionV0(ScSpecUdtUnionV0),
    UdtEnumV0(ScSpecUdtEnumV0),
    UdtErrorEnumV0(ScSpecUdtErrorEnumV0),
    EventV0(ScSpecEventV0),
}

#[cfg(feature = "alloc")]
impl Default for ScSpecEntry {
    fn default() -> Self {
        Self::FunctionV0(ScSpecFunctionV0::default())
    }
}

impl ScSpecEntry {
    const _VARIANTS: &[ScSpecEntryKind] = &[
        ScSpecEntryKind::FunctionV0,
        ScSpecEntryKind::UdtStructV0,
        ScSpecEntryKind::UdtUnionV0,
        ScSpecEntryKind::UdtEnumV0,
        ScSpecEntryKind::UdtErrorEnumV0,
        ScSpecEntryKind::EventV0,
    ];
    pub const VARIANTS: [ScSpecEntryKind; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &[
        "FunctionV0",
        "UdtStructV0",
        "UdtUnionV0",
        "UdtEnumV0",
        "UdtErrorEnumV0",
        "EventV0",
    ];
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
            Self::FunctionV0(_) => "FunctionV0",
            Self::UdtStructV0(_) => "UdtStructV0",
            Self::UdtUnionV0(_) => "UdtUnionV0",
            Self::UdtEnumV0(_) => "UdtEnumV0",
            Self::UdtErrorEnumV0(_) => "UdtErrorEnumV0",
            Self::EventV0(_) => "EventV0",
        }
    }

    #[must_use]
    pub const fn discriminant(&self) -> ScSpecEntryKind {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::FunctionV0(_) => ScSpecEntryKind::FunctionV0,
            Self::UdtStructV0(_) => ScSpecEntryKind::UdtStructV0,
            Self::UdtUnionV0(_) => ScSpecEntryKind::UdtUnionV0,
            Self::UdtEnumV0(_) => ScSpecEntryKind::UdtEnumV0,
            Self::UdtErrorEnumV0(_) => ScSpecEntryKind::UdtErrorEnumV0,
            Self::EventV0(_) => ScSpecEntryKind::EventV0,
        }
    }

    #[must_use]
    pub const fn variants() -> [ScSpecEntryKind; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for ScSpecEntry {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Discriminant<ScSpecEntryKind> for ScSpecEntry {
    #[must_use]
    fn discriminant(&self) -> ScSpecEntryKind {
        Self::discriminant(self)
    }
}

impl Variants<ScSpecEntryKind> for ScSpecEntry {
    fn variants() -> slice::Iter<'static, ScSpecEntryKind> {
        Self::VARIANTS.iter()
    }
}

impl Union<ScSpecEntryKind> for ScSpecEntry {}

impl ReadXdr for ScSpecEntry {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let dv: ScSpecEntryKind = <ScSpecEntryKind as ReadXdr>::read_xdr(r)?;
            #[allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants)]
            let v = match dv {
                ScSpecEntryKind::FunctionV0 => Self::FunctionV0(ScSpecFunctionV0::read_xdr(r)?),
                ScSpecEntryKind::UdtStructV0 => Self::UdtStructV0(ScSpecUdtStructV0::read_xdr(r)?),
                ScSpecEntryKind::UdtUnionV0 => Self::UdtUnionV0(ScSpecUdtUnionV0::read_xdr(r)?),
                ScSpecEntryKind::UdtEnumV0 => Self::UdtEnumV0(ScSpecUdtEnumV0::read_xdr(r)?),
                ScSpecEntryKind::UdtErrorEnumV0 => {
                    Self::UdtErrorEnumV0(ScSpecUdtErrorEnumV0::read_xdr(r)?)
                }
                ScSpecEntryKind::EventV0 => Self::EventV0(ScSpecEventV0::read_xdr(r)?),
                #[allow(unreachable_patterns)]
                _ => return Err(Error::Invalid),
            };
            Ok(v)
        })
    }
}

impl WriteXdr for ScSpecEntry {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.discriminant().write_xdr(w)?;
            #[allow(clippy::match_same_arms)]
            match self {
                Self::FunctionV0(v) => v.write_xdr(w)?,
                Self::UdtStructV0(v) => v.write_xdr(w)?,
                Self::UdtUnionV0(v) => v.write_xdr(w)?,
                Self::UdtEnumV0(v) => v.write_xdr(w)?,
                Self::UdtErrorEnumV0(v) => v.write_xdr(w)?,
                Self::EventV0(v) => v.write_xdr(w)?,
            };
            Ok(())
        })
    }
}

/// ScSpecEntryConst is a borrowing equivalent of [`ScSpecEntry`] over `'static`
/// data, for const XDR encoding.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(clippy::large_enum_variant)]
pub enum ScSpecEntryConst {
    FunctionV0(ScSpecFunctionV0Const),
    UdtStructV0(ScSpecUdtStructV0Const),
    UdtUnionV0(ScSpecUdtUnionV0Const),
    UdtEnumV0(ScSpecUdtEnumV0Const),
    UdtErrorEnumV0(ScSpecUdtErrorEnumV0Const),
    EventV0(ScSpecEventV0Const),
}

impl ScSpecEntryConst {
    #[must_use]
    pub const fn discriminant(&self) -> ScSpecEntryKind {
        #[allow(clippy::match_same_arms)]
        match self {
            Self::FunctionV0(_) => ScSpecEntryKind::FunctionV0,
            Self::UdtStructV0(_) => ScSpecEntryKind::UdtStructV0,
            Self::UdtUnionV0(_) => ScSpecEntryKind::UdtUnionV0,
            Self::UdtEnumV0(_) => ScSpecEntryKind::UdtEnumV0,
            Self::UdtErrorEnumV0(_) => ScSpecEntryKind::UdtErrorEnumV0,
            Self::EventV0(_) => ScSpecEntryKind::EventV0,
        }
    }
}

#[cfg(feature = "const")]
impl ScSpecEntryConst {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_sc_spec_entry(self);
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
        w.write_type_sc_spec_entry(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`ScSpecEntry`], mirroring `<ScSpecEntry as WriteXdr>::write_xdr`.
    pub const fn write_type_sc_spec_entry(&mut self, v: &ScSpecEntryConst) {
        let d = v.discriminant();
        self.write_type_sc_spec_entry_kind(&d);
        #[allow(clippy::match_same_arms)]
        match v {
            ScSpecEntryConst::FunctionV0(value) => {
                self.write_type_sc_spec_function_v0(value);
            }
            ScSpecEntryConst::UdtStructV0(value) => {
                self.write_type_sc_spec_udt_struct_v0(value);
            }
            ScSpecEntryConst::UdtUnionV0(value) => {
                self.write_type_sc_spec_udt_union_v0(value);
            }
            ScSpecEntryConst::UdtEnumV0(value) => {
                self.write_type_sc_spec_udt_enum_v0(value);
            }
            ScSpecEntryConst::UdtErrorEnumV0(value) => {
                self.write_type_sc_spec_udt_error_enum_v0(value);
            }
            ScSpecEntryConst::EventV0(value) => {
                self.write_type_sc_spec_event_v0(value);
            }
        }
    }
}
