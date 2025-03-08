use clap::{Args, ValueEnum};
use schemars::gen::SchemaSettings;

use crate::cli::Channel;
use std::path::PathBuf;

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

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, ValueEnum)]
pub enum OutputFormat {
    JsonSchemaDraft7,
    JsonSchemaDraft201909,
}

impl Default for OutputFormat {
    fn default() -> Self {
        Self::JsonSchemaDraft201909
    }
}

macro_rules! run_x {
    ($f:ident, $m:ident) => {
        fn $f(&self) -> Result<(), Error> {
            for t in crate::$m::TypeVariant::VARIANTS {
                let settings = match self.output {
                    OutputFormat::JsonSchemaDraft7 => SchemaSettings::draft07(),
                    OutputFormat::JsonSchemaDraft201909 => SchemaSettings::draft2019_09(),
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
    pub fn run(&self, channel: &Channel) -> Result<(), Error> {
        match channel {
            Channel::Curr => self.run_curr()?,
            Channel::Next => self.run_next()?,
        }
        Ok(())
    }

    run_x!(run_curr, curr);
    run_x!(run_next, next);
}
