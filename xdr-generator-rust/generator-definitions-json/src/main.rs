//! CLI entry point for emitting XDR definitions as a JSON AST.

mod ir;

#[cfg(test)]
mod tests;

use clap::Parser;
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;
use xdr_parser::ast::{Definition, XdrSpec};

/// XDR JSON AST emitter.
#[derive(Parser, Debug)]
#[command(name = "generator-definitions-json")]
#[command(about = "Emit parsed XDR definitions as JSON for downstream generators")]
struct Args {
    /// Input XDR files
    #[arg(short, long, required = true)]
    input: Vec<PathBuf>,

    /// Output JSON AST path
    #[arg(short, long, required = true)]
    output: PathBuf,

    /// Features to enable for cfg resolution.
    /// When specified, cfg expressions are evaluated and filtered before emission.
    /// Without this flag, all definitions are included with cfg annotations intact.
    #[arg(long, value_delimiter = ',')]
    feature: Vec<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Read all input files and sort by filename
    let mut files: Vec<(PathBuf, String)> = Vec::new();
    for path in &args.input {
        let content = fs::read_to_string(path)?;
        files.push((path.clone(), content));
    }
    files.sort_by(|a, b| a.0.cmp(&b.0));

    // Build file list for the parser
    let file_refs: Vec<(&str, &str)> = files
        .iter()
        .map(|(path, content)| (path.to_str().unwrap_or(""), content.as_str()))
        .collect();

    // Parse the XDR files
    let mut spec = xdr_parser::parser::parse_files(&file_refs)?;

    let resolved_features: Vec<String> = args.feature.iter()
        .map(|f| f.to_lowercase())
        .collect();
    if !resolved_features.is_empty() {
        let features: HashSet<String> = resolved_features.iter().cloned().collect();
        filter_spec(&mut spec, &features);
    }

    let output = ir::build(&spec, resolved_features);
    let json = serde_json::to_string_pretty(&output)?;
    fs::write(&args.output, json)?;
    eprintln!(
        "JSON AST: {} ({} definitions)",
        args.output.display(),
        output.definitions.len(),
    );

    Ok(())
}

/// Filter an XDR spec in-place: remove definitions and sub-elements whose cfg
/// evaluates to false, then clear all remaining cfg annotations.
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
