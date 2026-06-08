//! CLI entry point for the XDR code generator.
//!
//! This is a thin wrapper over the [`xdr_generator_rust::generate`] library
//! function. It exists so `src/generated.rs` can be regenerated without first
//! building `stellar-xdr` (which depends on the generated code). The same
//! functionality is also available as `stellar-xdr xfile generate-rust`.

use clap::Parser;
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
    xdr_generator_rust::generate(
        &args.input,
        &args.output,
        &args.custom_default,
        &args.custom_str,
        &args.no_display_fromstr,
    )?;
    eprintln!("Generated: {}", args.output.display());
    Ok(())
}
