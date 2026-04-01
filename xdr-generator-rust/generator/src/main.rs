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
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

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
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    if args.output.is_none() && args.json_ast.is_none() {
        return Err("at least one of --output or --json-ast must be specified".into());
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
        // Check for duplicate definition names. The computed properties are keyed
        // by name, so duplicates (from cfg-gated redefinitions) would silently
        // produce wrong results. The JSON IR requires unique names.
        let mut seen = std::collections::HashSet::new();
        for def in spec.all_definitions() {
            if !seen.insert(def.name().to_string()) {
                return Err(format!(
                    "duplicate definition {:?} in XDR spec; the JSON IR does not support \
                     cfg-gated redefinition of the same type name",
                    def.name()
                ).into());
            }
        }

        let type_info = xdr_parser::types::TypeInfo::build(&spec, &|name| name.to_string());
        let computed = type_info.compute_properties();
        let ir = serde_json::json!({
            "spec": &spec,
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
