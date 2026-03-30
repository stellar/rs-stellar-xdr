use clap::{Args, ValueEnum};
use std::{
    io::{stdout, Write},
    str::FromStr,
};

use crate::cli::util;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("unknown type {0}, choose one of {1:?}")]
    UnknownType(String, &'static [&'static str]),
    #[error("error reading file: {0}")]
    ReadFile(#[from] std::io::Error),
    #[error("error generating XDR: {0}")]
    WriteXdr(#[from] crate::Error),
    #[error("error generating JSON: {0}")]
    GenerateJson(#[from] serde_json::Error),
    #[error("type doesn't have a text representation, use 'json' as output")]
    TextUnsupported,
}

/// Generate default XDR values
#[derive(Args, Debug, Clone)]
#[command()]
pub struct Cmd {
    /// XDR type to generate
    #[arg(long)]
    pub r#type: String,

    // Output format to encode to
    #[arg(long = "output", value_enum, default_value_t)]
    pub output_format: OutputFormat,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, ValueEnum)]
pub enum OutputFormat {
    Single,
    #[default]
    SingleBase64,
    // TODO: Stream,
    // TODO: StreamBase64,
    // TODO: StreamFramed,
    Json,
    JsonFormatted,
    Text,
}

// TODO: Remove run_x macro, it exists only to reduce the diff from when curr/next
// channels existed and each had their own run_curr/run_next invocation.
macro_rules! run_x {
    ($f:ident) => {
        fn $f(&self) -> Result<(), Error> {
            use crate::WriteXdr;
            let r#type = crate::TypeVariant::from_str(&self.r#type).map_err(|_| {
                Error::UnknownType(self.r#type.clone(), &crate::TypeVariant::VARIANTS_STR)
            })?;
            let v = crate::Type::default(r#type);
            match self.output_format {
                OutputFormat::Single => {
                    let l = crate::Limits::none();
                    stdout().write_all(&v.to_xdr(l)?)?
                }
                OutputFormat::SingleBase64 => {
                    let l = crate::Limits::none();
                    println!("{}", v.to_xdr_base64(l)?)
                }
                OutputFormat::Json => {
                    println!("{}", serde_json::to_string(&v)?);
                }
                OutputFormat::JsonFormatted => {
                    println!("{}", serde_json::to_string_pretty(&v)?);
                }
                OutputFormat::Text => {
                    let v = serde_json::to_value(v)?;
                    let text = util::serde_json_value_to_text(v).ok_or(Error::TextUnsupported)?;
                    println!("{text}")
                }
            }
            Ok(())
        }
    };
}

impl Cmd {
    /// Run the CLIs generate zero command.
    ///
    /// ## Errors
    ///
    /// If the command is configured with state that is invalid.
    pub fn run(&self) -> Result<(), Error> {
        self.run_inner()
    }

    run_x!(run_inner);
}
