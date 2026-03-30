use clap::{Args, ValueEnum};
use std::path::PathBuf;

use crate::schemars;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("error generating JSON: {0}")]
    GenerateJson(#[from] serde_json::Error),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

/// Generate JSON schema files for the XDR types, writing a file for each type to the out directory
#[derive(Args, Debug, Clone)]
#[command()]
pub struct Cmd {
    // Out directory to write files to
    #[arg(long)]
    pub out_dir: PathBuf,

    // Output format
    #[arg(long, value_enum, default_value_t)]
    pub output: OutputFormat,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, ValueEnum)]
pub enum OutputFormat {
    #[default]
    JsonSchemaDraft201909,
}

// TODO: Remove run_x macro, it exists only to reduce the diff from when curr/next
// channels existed and each had their own run_curr/run_next invocation.
macro_rules! run_x {
    ($f:ident) => {
        fn $f(&self) -> Result<(), Error> {
            for t in crate::TypeVariant::VARIANTS {
                let settings = match self.output {
                    OutputFormat::JsonSchemaDraft201909 => schemars::settings_draft201909(),
                };
                let generator = settings.into_generator();
                let schema = t.json_schema(generator);
                let path = self.out_dir.join(format!("{}.json", t.name()));
                std::fs::write(&path, serde_json::to_string_pretty(&schema)?)?;
            }
            Ok(())
        }
    };
}

impl Cmd {
    /// # Errors
    /// Fails if the JSON generation fails.
    pub fn run(&self) -> Result<(), Error> {
        self.run_inner()
    }

    run_x!(run_inner);
}
