#[allow(unused_imports)]
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
#[cfg_eval::cfg_eval]
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
