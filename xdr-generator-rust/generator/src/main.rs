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

    /// Output file
    #[arg(short, long)]
    output: PathBuf,

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

    // Read all input files and sort by filename
    let mut files: Vec<(PathBuf, String)> = Vec::new();
    for path in &args.input {
        let content = fs::read_to_string(path)?;
        files.push((path.clone(), content));
    }
    files.sort_by(|a, b| a.0.cmp(&b.0));

    // Build combined XDR in sorted order
    let mut combined_xdr = String::new();
    for (_, content) in &files {
        combined_xdr.push_str(content);
        combined_xdr.push('\n');
    }

    // Build input_files list
    let input_files: Vec<(String, String)> = files
        .iter()
        .map(|(path, content)| (path.to_string_lossy().to_string(), content.clone()))
        .collect();

    // Parse the combined XDR
    let spec = xdr_parser::parser::parse(&combined_xdr)?;

    let options = RustOptions {
        custom_default_impl: args.custom_default.into_iter().collect::<HashSet<_>>(),
        custom_str_impl: args.custom_str.into_iter().collect::<HashSet<_>>(),
        no_display_fromstr: args.no_display_fromstr.into_iter().collect::<HashSet<_>>(),
    };

    let generator = RustGenerator::new(&spec, options);
    generator.generate_to_file(&spec, &input_files, &args.output)?;

    eprintln!("Generated: {}", args.output.display());

    Ok(())
}
