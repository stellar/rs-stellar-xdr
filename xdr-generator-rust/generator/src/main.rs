//! CLI entry point for the XDR code generator.

mod generator;
mod ir;
mod naming;
mod options;
mod output;
mod types;

#[cfg(test)]
mod tests;

use clap::Parser;
use generator::RustGenerator;
use options::RustOptions;
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;
use xdr_parser::ast::{Definition, XdrSpec};

/// XDR code generator.
#[derive(Parser, Debug)]
#[command(name = "xdr-generator")]
#[command(about = "Generate code from XDR definitions")]
struct Args {
    /// Input XDR files
    #[arg(short, long, required = true)]
    input: Vec<PathBuf>,

    /// Output module file (e.g. src/generated.rs)
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Output the parsed XDR AST as JSON (for consumption by other code generators)
    #[arg(long)]
    json_ast: Option<PathBuf>,

    /// Types with custom Default implementation (skip derive(Default))
    #[arg(long, value_delimiter = ',')]
    custom_default: Vec<String>,

    /// Types with custom FromStr/Display implementation (use SerializeDisplay)
    #[arg(long, value_delimiter = ',')]
    custom_str: Vec<String>,

    /// Types that should NOT have Display/FromStr/schemars generated
    #[arg(long, value_delimiter = ',')]
    no_display_fromstr: Vec<String>,

    /// Features to enable for cfg resolution (restricted to --json-ast).
    /// When specified, cfg expressions are evaluated and filtered before emission.
    /// Without this flag, all definitions are included with cfg annotations intact.
    #[arg(long, value_delimiter = ',')]
    feature: Vec<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    if args.output.is_none() && args.json_ast.is_none() {
        return Err("at least one of --output or --json-ast must be specified".into());
    }

    if !args.feature.is_empty() && args.json_ast.is_none() {
        return Err("--feature can only be used with --json-ast".into());
    }

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
    let spec = xdr_parser::parser::parse_files(&file_refs)?;

    // Output JSON AST if requested
    if let Some(json_path) = &args.json_ast {
        let mut ir_spec = spec.clone();
        let resolved_features: Vec<String> = args.feature.iter()
            .map(|f| f.to_lowercase())
            .collect();
        if !resolved_features.is_empty() {
            let features: HashSet<String> = resolved_features.iter().cloned().collect();
            filter_spec(&mut ir_spec, &features);
        }

        let type_info = xdr_parser::types::TypeInfo::build(&ir_spec, &|name| name.to_string());
        let computed = type_info.compute_properties();
        let definitions = ir::build_definitions(&ir_spec, &computed);

        let output = ir::IR {
            version: 1,
            files: ir_spec.files.iter().map(|f| ir::XdrFile {
                name: f.name.clone(),
                sha256: f.sha256.clone(),
            }).collect(),
            resolved_features,
            definitions,
        };
        let json = serde_json::to_string_pretty(&output)?;
        fs::write(json_path, json)?;
        eprintln!(
            "JSON AST: {} ({} definitions)",
            json_path.display(),
            output.definitions.len(),
        );
    }

    // Generate Rust code if output path provided
    if let Some(output) = &args.output {
        let options = RustOptions {
            custom_default_impl: args.custom_default.into_iter().collect::<HashSet<_>>(),
            custom_str_impl: args.custom_str.into_iter().collect::<HashSet<_>>(),
            no_display_fromstr: args.no_display_fromstr.into_iter().collect::<HashSet<_>>(),
        };

        let generator = RustGenerator::new(&spec, options);
        let output_dir = output.with_extension("");
        generator.generate_to_dir(&spec, output, &output_dir)?;
        eprintln!("Generated: {}", output.display());
    }

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
