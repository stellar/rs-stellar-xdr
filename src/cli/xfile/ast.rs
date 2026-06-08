use std::path::PathBuf;

use clap::Args;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("error generating JSON AST: {0}")]
    Generate(Box<dyn std::error::Error>),
}

/// Emit parsed XDR definitions as a JSON AST
#[derive(Args, Debug, Clone)]
#[command()]
pub struct Cmd {
    /// Input XDR files
    #[arg(long, required = true)]
    pub input: Vec<PathBuf>,

    /// Output JSON AST path
    #[arg(long)]
    pub output: PathBuf,

    /// Feature set for cfg resolution. Items whose cfg evaluates to true under
    /// this feature set are kept; others are dropped. Accepts comma-separated
    /// values or may be repeated (e.g. --feature curr,next or --feature curr
    /// --feature next).
    #[arg(long, value_delimiter = ',')]
    pub feature: Vec<String>,
}

impl Cmd {
    /// Run the CLIs xfile ast command.
    ///
    /// ## Errors
    ///
    /// If the input files cannot be read or parsed, or the JSON AST cannot be
    /// written.
    pub fn run(&self) -> Result<(), Error> {
        generator_definitions_json::generate(&self.input, &self.output, &self.feature)
            .map_err(Error::Generate)
    }
}
