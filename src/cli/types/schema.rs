use clap::{Args, ValueEnum};
use schemars::gen::SchemaSettings;

use crate::cli::Channel;

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
    /// XDR type to decode
    #[arg(long)]
    r#type: String,

    // Output format
    #[arg(long, value_enum, default_value_t)]
    output: OutputFormat,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, ValueEnum)]
pub enum OutputFormat {
    JsonSchemaDraft7,
}

impl Default for OutputFormat {
    fn default() -> Self {
        Self::JsonSchemaDraft7
    }
}

macro_rules! run_x_case_for_type {
    ($type:tt, $m:tt, $type_name:expr, $output:expr) => {
        if $type_name == stringify!($type) {
            match $output {
                OutputFormat::JsonSchemaDraft7 => {
                    let settings = SchemaSettings::draft07();
                    let generator = settings.into_generator();
                    let schema = generator.into_root_schema_for::<crate::$m::$type>();
                    println!("{}", serde_json::to_string_pretty(&schema)?);
                }
            }
        }
    };
}

macro_rules! run_x {
    ($f:ident, $m:ident) => {
        fn $f(&self) -> Result<(), Error> {
            use std::str::FromStr;
            let _ = crate::$m::TypeVariant::from_str(&self.r#type).map_err(|_| {
                Error::UnknownType(self.r#type.clone(), &crate::$m::TypeVariant::VARIANTS_STR)
            })?;
            crate::$m::call_macro_with_each_type! { run_x_case_for_type, $m, {&self.r#type}, {self.output} }
            Ok(())
        }
    };
}

impl Cmd {
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
