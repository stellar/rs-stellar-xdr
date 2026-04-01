//! CLI entry point for the XDR code generator.

mod generator;
mod naming;
mod options;
mod output;
mod types;

#[cfg(test)]
mod tests;

use clap::Parser;
use generator::RustGenerator;
use options::RustOptions;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::PathBuf;
use xdr_parser::ast::{Definition, UnionCaseValue, XdrSpec};

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

    /// Feature to enable for cfg resolution (restricted to --json-ast).
    /// When specified, cfg expressions are evaluated and filtered before emission.
    /// Without this flag, all definitions are included with cfg annotations intact.
    #[arg(long)]
    feature: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    if args.output.is_none() && args.json_ast.is_none() {
        return Err("at least one of --output or --json-ast must be specified".into());
    }

    if args.feature.is_some() && args.json_ast.is_none() {
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
        let resolved_features = if let Some(ref feature) = args.feature {
            let feature_lower = feature.to_lowercase();
            let features: HashSet<String> =
                std::iter::once(feature_lower.clone()).collect();
            filter_spec(&mut ir_spec, &features)?;
            vec![feature_lower]
        } else {
            Vec::new()
        };

        // Check for duplicate definition names. The computed properties are keyed
        // by name, so duplicates (from cfg-gated redefinitions) would silently
        // produce wrong results. After cfg resolution, duplicates should not exist.
        let mut seen = HashSet::new();
        for def in ir_spec.all_definitions() {
            if !seen.insert(def.name().to_string()) {
                return Err(format!(
                    "duplicate definition {:?} in XDR spec; the JSON IR does not support \
                     cfg-gated redefinition of the same type name",
                    def.name()
                ).into());
            }
        }

        let type_info = xdr_parser::types::TypeInfo::build(&ir_spec, &|name| name.to_string());
        let computed = type_info.compute_properties();
        let ir = serde_json::json!({
            "resolved_features": &resolved_features,
            "spec": &ir_spec,
            "computed": &computed,
        });
        let json = serde_json::to_string_pretty(&ir)?;
        fs::write(json_path, json)?;
        eprintln!(
            "JSON AST: {} ({} types)",
            json_path.display(),
            computed.types.len(),
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
        generator.generate_to_file(&spec, output)?;
        eprintln!("Generated: {}", output.display());
    }

    Ok(())
}

/// Filter an XDR spec in-place: remove definitions and sub-elements whose cfg
/// evaluates to false, then clear all remaining cfg annotations.
/// Validates that unions still cover all discriminant values after filtering.
fn filter_spec(spec: &mut XdrSpec, features: &HashSet<String>) -> Result<(), Box<dyn std::error::Error>> {
    // Filter top-level definitions.
    filter_definitions(&mut spec.definitions, features);

    // Filter definitions inside namespaces.
    for ns in &mut spec.namespaces {
        if !ns.namespaces.is_empty() {
            return Err("nested namespaces are not supported in the JSON IR".into());
        }
        filter_definitions(&mut ns.definitions, features);
    }

    // Validate union arm coverage: every surviving enum discriminant value
    // must be covered by a surviving union arm. The parser rejects default
    // arms, so all cases must be explicit.
    validate_union_coverage(spec)?;

    Ok(())
}

/// Validate that every union whose discriminant is an enum covers all
/// surviving enum member values after cfg filtering.
fn validate_union_coverage(spec: &XdrSpec) -> Result<(), Box<dyn std::error::Error>> {
    let enums: HashMap<&str, HashMap<&str, i32>> = spec
        .all_definitions()
        .filter_map(|def| match def {
            Definition::Enum(e) => Some((
                e.name.as_str(),
                e.members.iter().map(|m| (m.name.as_str(), m.value)).collect(),
            )),
            _ => None,
        })
        .collect();

    for def in spec.all_definitions() {
        let Definition::Union(u) = def else { continue };
        let xdr_parser::ast::Type::Ident { ident: ref disc_type } = u.discriminant.type_ else { continue };
        let Some(members) = enums.get(disc_type.as_str()) else { continue };

        let covered: HashSet<i32> = u.arms.iter()
            .flat_map(|arm| &arm.cases)
            .filter_map(|case| match &case.value {
                UnionCaseValue::Literal { literal } => Some(*literal),
                UnionCaseValue::Ident { ident } => members.get(ident.as_str()).copied(),
            })
            .collect();

        let missing: Vec<i32> = members.values()
            .copied()
            .filter(|v| !covered.contains(v))
            .collect();
        if !missing.is_empty() {
            return Err(format!(
                "union {:?} does not cover discriminant values {:?} \
                 after cfg filtering (enum {:?})",
                u.name, missing, disc_type
            ).into());
        }
    }

    Ok(())
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
            _ => {}
        }
        def.set_cfg(None);
    }
}
