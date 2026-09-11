#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ScSpecEntry is a borrowing equivalent of [`ScSpecEntry`](super::super::ScSpecEntry)
/// over `'static` data, for const XDR encoding.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(clippy::large_enum_variant)]
pub enum ScSpecEntry {
    FunctionV0(ScSpecFunctionV0),
    UdtStructV0(ScSpecUdtStructV0),
    UdtUnionV0(ScSpecUdtUnionV0),
    UdtEnumV0(ScSpecUdtEnumV0),
    UdtErrorEnumV0(ScSpecUdtErrorEnumV0),
    EventV0(ScSpecEventV0),
}

impl ScSpecEntry {
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

impl ScSpecEntry {
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
    /// operations. This is the const counterpart to
    /// [`WriteXdr::to_xdr`](super::super::WriteXdr::to_xdr).
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

impl ConstWriter<'_> {
    /// Serializes a [`ScSpecEntry`], mirroring `<ScSpecEntry as WriteXdr>::write_xdr`.
    pub const fn write_type_sc_spec_entry(&mut self, v: &ScSpecEntry) {
        let d = v.discriminant();
        self.write_type_sc_spec_entry_kind(&d);
        #[allow(clippy::match_same_arms)]
        match v {
            ScSpecEntry::FunctionV0(value) => {
                self.write_type_sc_spec_function_v0(value);
            }
            ScSpecEntry::UdtStructV0(value) => {
                self.write_type_sc_spec_udt_struct_v0(value);
            }
            ScSpecEntry::UdtUnionV0(value) => {
                self.write_type_sc_spec_udt_union_v0(value);
            }
            ScSpecEntry::UdtEnumV0(value) => {
                self.write_type_sc_spec_udt_enum_v0(value);
            }
            ScSpecEntry::UdtErrorEnumV0(value) => {
                self.write_type_sc_spec_udt_error_enum_v0(value);
            }
            ScSpecEntry::EventV0(value) => {
                self.write_type_sc_spec_event_v0(value);
            }
        }
    }
}
