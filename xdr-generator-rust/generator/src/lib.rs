//! XDR to Rust code generator.
//!
//! This library parses XDR `.x` definition files and generates Rust source
//! code from them. The primary entry point is [`generate`], which reads the
//! input files, parses them, and writes the generated module file plus a
//! directory of per-type files.

mod generator;
mod naming;
mod options;
mod output;
mod types;

#[cfg(test)]
mod tests;

use std::collections::HashSet;
use std::fs;
use std::path::Path;

pub use generator::RustGenerator;
pub use options::RustOptions;

/// Generate Rust code from the given XDR input files.
///
/// The `input` files are read and sorted by path, parsed into a single XDR
/// spec, and then rendered to `output` (the module file, e.g.
/// `src/generated.rs`) plus a sibling directory derived from the module file
/// name (e.g. `src/generated/`) containing the per-type files.
///
/// `custom_default`, `custom_str`, and `no_display_fromstr` are lists of type
/// names controlling, respectively, which types skip `derive(Default)`, which
/// types use a custom `FromStr`/`Display` implementation, and which types
/// should not have `Display`/`FromStr`/`schemars` generated.
///
/// # Errors
///
/// Returns an error if any input file cannot be read, the XDR cannot be
/// parsed, or the generated output cannot be written.
pub fn generate(
    input: &[impl AsRef<Path>],
    output: impl AsRef<Path>,
    custom_default: &[String],
    custom_str: &[String],
    no_display_fromstr: &[String],
) -> Result<(), Box<dyn std::error::Error>> {
    let output = output.as_ref();

    // Read all input files and sort by filename.
    let mut files: Vec<(&Path, String)> = Vec::new();
    for path in input {
        let path = path.as_ref();
        let content = fs::read_to_string(path)?;
        files.push((path, content));
    }
    files.sort_by(|a, b| a.0.cmp(b.0));

    // Build file list for the parser.
    let file_refs: Vec<(&str, &str)> = files
        .iter()
        .map(|(path, content)| (path.to_str().unwrap_or(""), content.as_str()))
        .collect();

    // Parse the XDR files.
    let spec = xdr_parser::parser::parse_files(&file_refs)?;

    let options = RustOptions {
        custom_default_impl: custom_default.iter().cloned().collect::<HashSet<_>>(),
        custom_str_impl: custom_str.iter().cloned().collect::<HashSet<_>>(),
        no_display_fromstr: no_display_fromstr.iter().cloned().collect::<HashSet<_>>(),
    };

    let generator = RustGenerator::new(&spec, options);

    // Derive the per-type directory from the module file path:
    // e.g. src/generated.rs -> src/generated/
    let output_dir = output.with_extension("");
    generator.generate_to_dir(&spec, output, &output_dir)?;

    Ok(())
}
