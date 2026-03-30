use clap::{Args, ValueEnum};

use crate::schemars;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("unknown type {0}, choose one of {1:?}")]
    UnknownType(String, &'static [&'static str]),
    #[error("error generating JSON: {0}")]
    GenerateJson(#[from] serde_json::Error),
}

#[derive(Args, Debug, Clone)]
#[command()]
pub struct Cmd {
    /// XDR type to generate schema for
    #[arg(long)]
    pub r#type: String,

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
            use std::str::FromStr;
            let r#type = crate::TypeVariant::from_str(&self.r#type).map_err(|_| {
                Error::UnknownType(self.r#type.clone(), &crate::TypeVariant::VARIANTS_STR)
            })?;
            let settings = match self.output {
                OutputFormat::JsonSchemaDraft201909 => schemars::settings_draft201909(),
            };
            let generator = settings.into_generator();
            let schema = r#type.json_schema(generator);
            println!("{}", serde_json::to_string_pretty(&schema)?);
            Ok(())
        }
    };
}

impl Cmd {
    /// # Errors
    /// Fails if the type is unknown or if the JSON generation fails.
    pub fn run(&self) -> Result<(), Error> {
        self.run_inner()
    }

    run_x!(run_inner);
}
