//! CLI entry point for emitting XDR definitions as a JSON AST.
//!
//! This is a thin wrapper over the [`generator_definitions_json::generate`]
//! library function. It exists so `xdr-definitions-json/xdr.json` can be
//! generated without first building `stellar-xdr`. The same functionality is
//! also available as `stellar-xdr xfile ast`.

use clap::Parser;
use std::path::PathBuf;

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

    /// Feature set for cfg resolution. Items whose cfg evaluates to true
    /// under this feature set are kept; others are dropped. Accepts
    /// comma-separated values or may be repeated (e.g. --feature curr,next
    /// or --feature curr --feature next).
    ///
    /// By default the feature set is empty: items gated on any feature are
    /// dropped (`not(feature = "X")` still evaluates to true with an empty
    /// set and is kept).
    #[arg(long, value_delimiter = ',')]
    feature: Vec<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    generator_definitions_json::generate(&args.input, &args.output, &args.feature)?;
    eprintln!("JSON AST: {}", args.output.display());
    Ok(())
}
