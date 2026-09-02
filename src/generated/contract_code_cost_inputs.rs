#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ContractCodeCostInputs is an XDR Struct defined as:
///
/// ```text
/// struct ContractCodeCostInputs {
///     ExtensionPoint ext;
///     uint32 nInstructions;
///     uint32 nFunctions;
///     uint32 nGlobals;
///     uint32 nTableEntries;
///     uint32 nTypes;
///     uint32 nDataSegments;
///     uint32 nElemSegments;
///     uint32 nImports;
///     uint32 nExports;
///     uint32 nDataSegmentBytes;
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
pub struct ContractCodeCostInputs {
    pub ext: ExtensionPoint,
    pub n_instructions: u32,
    pub n_functions: u32,
    pub n_globals: u32,
    pub n_table_entries: u32,
    pub n_types: u32,
    pub n_data_segments: u32,
    pub n_elem_segments: u32,
    pub n_imports: u32,
    pub n_exports: u32,
    pub n_data_segment_bytes: u32,
}

impl ReadXdr for ContractCodeCostInputs {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                ext: ExtensionPoint::read_xdr(r)?,
                n_instructions: u32::read_xdr(r)?,
                n_functions: u32::read_xdr(r)?,
                n_globals: u32::read_xdr(r)?,
                n_table_entries: u32::read_xdr(r)?,
                n_types: u32::read_xdr(r)?,
                n_data_segments: u32::read_xdr(r)?,
                n_elem_segments: u32::read_xdr(r)?,
                n_imports: u32::read_xdr(r)?,
                n_exports: u32::read_xdr(r)?,
                n_data_segment_bytes: u32::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ContractCodeCostInputs {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.ext.write_xdr(w)?;
            self.n_instructions.write_xdr(w)?;
            self.n_functions.write_xdr(w)?;
            self.n_globals.write_xdr(w)?;
            self.n_table_entries.write_xdr(w)?;
            self.n_types.write_xdr(w)?;
            self.n_data_segments.write_xdr(w)?;
            self.n_elem_segments.write_xdr(w)?;
            self.n_imports.write_xdr(w)?;
            self.n_exports.write_xdr(w)?;
            self.n_data_segment_bytes.write_xdr(w)?;
            Ok(())
        })
    }
}

#[cfg(feature = "const")]
impl ContractCodeCostInputs {
    /// The exact XDR-encoded length of this value, in bytes.
    ///
    /// Evaluable in a const context, so a caller (such as a proc-macro) can
    /// size a buffer for [`Self::const_to_xdr`] at compile time.
    #[must_use]
    pub const fn const_xdr_len(&self) -> usize {
        let mut empty: [u8; 0] = [];
        let mut w = ConstWriter::new(&mut empty);
        w.write_type_contract_code_cost_inputs(self);
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
        w.write_type_contract_code_cost_inputs(self);
        assert!(
            w.len() == N,
            "const_to_xdr: N does not equal the XDR-encoded length"
        );
        buf
    }
}

#[cfg(feature = "const")]
impl ConstWriter<'_> {
    /// Serializes a [`ContractCodeCostInputs`], mirroring `<ContractCodeCostInputs as WriteXdr>::write_xdr`.
    pub const fn write_type_contract_code_cost_inputs(&mut self, v: &ContractCodeCostInputs) {
        self.write_type_extension_point(&v.ext);
        self.write_u32(v.n_instructions);
        self.write_u32(v.n_functions);
        self.write_u32(v.n_globals);
        self.write_u32(v.n_table_entries);
        self.write_u32(v.n_types);
        self.write_u32(v.n_data_segments);
        self.write_u32(v.n_elem_segments);
        self.write_u32(v.n_imports);
        self.write_u32(v.n_exports);
        self.write_u32(v.n_data_segment_bytes);
    }
}
