//! Emit parsed XDR definitions as a JSON AST for downstream code generators.
//!
//! This library parses XDR `.x` definition files, filters them against a
//! feature set, and writes the resulting definitions as JSON. The primary
//! entry point is [`generate`].

mod ir;

#[cfg(test)]
mod tests;

use std::collections::HashSet;
use std::fs;
use std::path::Path;
use xdr_parser::ast::{Definition, XdrSpec};

/// Generate a JSON AST from the given XDR input files.
///
/// The `input` files are read and sorted by path, parsed into a single XDR
/// spec, filtered against `features` (cfg resolution), and written to `output`
/// as pretty-printed JSON.
///
/// `features` is the feature set for cfg resolution: items whose cfg evaluates
/// to true under this set are kept; others are dropped. Feature names are
/// lower-cased before evaluation. An empty set drops items gated on any
/// feature (`not(feature = "X")` still evaluates to true and is kept).
///
/// # Errors
///
/// Returns an error if any input file cannot be read, the XDR cannot be
/// parsed, the JSON cannot be serialized, or the output cannot be written.
pub fn generate(
    input: &[impl AsRef<Path>],
    output: impl AsRef<Path>,
    features: &[String],
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
    let mut spec = xdr_parser::parser::parse_files(&file_refs)?;

    let features: HashSet<String> = features.iter().map(|f| f.to_lowercase()).collect();
    let mut resolved_features: Vec<String> = features.iter().cloned().collect();
    resolved_features.sort();
    filter_spec(&mut spec, &features);

    let output_ir = ir::build(&spec, resolved_features);
    let json = serde_json::to_string_pretty(&output_ir)?;
    fs::write(output, json)?;

    Ok(())
}

/// Filter an XDR spec in-place: remove definitions and sub-elements whose cfg
/// evaluates to false under the given feature set, then clear all remaining
/// cfg annotations.
fn filter_spec(spec: &mut XdrSpec, features: &HashSet<String>) {
    filter_definitions(&mut spec.definitions, features);
    for ns in &mut spec.namespaces {
        filter_definitions(&mut ns.definitions, features);
    }
}

/// Filter a list of definitions: remove those whose cfg is false, filter
/// sub-elements (union arms, enum members), and clear remaining cfg annotations.
fn filter_definitions(defs: &mut Vec<Definition>, features: &HashSet<String>) {
    defs.retain(|d| d.cfg().map_or(true, |cfg| cfg.evaluate(features)));

    for def in defs.iter_mut() {
        match def {
            Definition::Union(u) => {
                u.arms.retain_mut(|arm| {
                    let keep = arm.cfg.as_ref().map_or(true, |cfg| cfg.evaluate(features));
                    arm.cfg = None;
                    keep
                });
            }
            Definition::Enum(e) => {
                e.members.retain_mut(|m| {
                    let keep = m.cfg.as_ref().map_or(true, |cfg| cfg.evaluate(features));
                    m.cfg = None;
                    keep
                });
            }
            Definition::Struct(_) | Definition::Typedef(_) | Definition::Const(_) => {}
        }
        def.set_cfg(None);
    }
}
