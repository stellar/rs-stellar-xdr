use std::ffi::OsString;
use std::{
    io::{stdout, Write},
    str::FromStr,
};

use clap::{Args, ValueEnum};

use crate::cli::util;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("unknown type {0}, choose one of {1:?}")]
    UnknownType(String, &'static [&'static str]),
    #[error("error decoding JSON: {0}")]
    ReadJson(crate::Error),
    #[error("error reading file: {0}")]
    ReadFile(std::io::Error),
    #[error("error writing output: {0}")]
    WriteOutput(std::io::Error),
    #[error("error generating XDR: {0}")]
    WriteXdr(crate::Error),
    #[error("unknown fields in JSON input: {}", .0.join(", "))]
    UnknownFields(Vec<String>),
}

impl From<crate::Error> for Error {
    fn from(e: crate::Error) -> Self {
        match e {
            crate::Error::Invalid
            | crate::Error::Unsupported
            | crate::Error::LengthExceedsMax
            | crate::Error::LengthMismatch
            | crate::Error::NonZeroPadding
            | crate::Error::Utf8Error(_)
            | crate::Error::InvalidHex
            | crate::Error::Io(_)
            | crate::Error::DepthLimitExceeded
            | crate::Error::LengthLimitExceeded
            | crate::Error::Arbitrary(_) => Error::WriteXdr(e),
            crate::Error::Json(_) => Error::ReadJson(e),
        }
    }
}

#[derive(Args, Debug, Clone)]
#[command()]
pub struct Cmd {
    /// XDR or files containing XDR to decode, or stdin if empty
    #[arg()]
    pub input: Vec<OsString>,

    /// XDR type to encode
    #[arg(long)]
    pub r#type: String,

    // Input format
    #[arg(long = "input", value_enum, default_value_t)]
    pub input_format: InputFormat,

    // Output format to encode to
    #[arg(long = "output", value_enum, default_value_t)]
    pub output_format: OutputFormat,
}

#[derive(Default, Clone, Copy, Debug, Eq, Hash, PartialEq, ValueEnum)]
pub enum InputFormat {
    #[default]
    Json,
}

#[derive(Default, Clone, Copy, Debug, Eq, Hash, PartialEq, ValueEnum)]
pub enum OutputFormat {
    Single,
    #[default]
    SingleBase64,
    Stream,
    // TODO: StreamBase64,
    // TODO: StreamFramed,
}

fn strip_root_schema(value: &mut serde_json::Value) {
    if let serde_json::Value::Object(map) = value {
        map.remove("$schema");
    }
}

pub(crate) fn from_json_ignoring_schema(
    r#type: crate::TypeVariant,
    r: impl std::io::Read,
) -> Result<(crate::Type, Vec<String>), Error> {
    let mut value: serde_json::Value = serde_json::from_reader(r).map_err(crate::Error::Json)?;
    strip_root_schema(&mut value);
    let bytes = serde_json::to_vec(&value).map_err(crate::Error::Json)?;
    let mut de = serde_json::Deserializer::from_slice(&bytes);
    let result = crate::Type::deserialize_json_collecting_ignored_fields(r#type, &mut de)?;
    de.end().map_err(crate::Error::Json)?;
    Ok(result)
}

// TODO: Remove run_x macro, it exists only to reduce the diff from when curr/next
// channels existed and each had their own run_curr/run_next invocation.
macro_rules! run_x {
    ($f:ident) => {
        fn $f(&self) -> Result<(), Error> {
            use crate::WriteXdr;
            let mut input = util::parse_input(&self.input).map_err(Error::ReadFile)?;
            let r#type = crate::TypeVariant::from_str(&self.r#type).map_err(|_| {
                Error::UnknownType(self.r#type.clone(), &crate::TypeVariant::VARIANTS_STR)
            })?;
            for f in &mut input {
                match self.input_format {
                    InputFormat::Json => match self.output_format {
                        OutputFormat::Single => {
                            let (t, ignored) = from_json_ignoring_schema(r#type, f)?;
                            check_ignored_fields(&ignored)?;
                            let l = crate::Limits::none();
                            stdout()
                                .write_all(&t.to_xdr(l)?)
                                .map_err(Error::WriteOutput)?;
                        }
                        OutputFormat::SingleBase64 => {
                            let (t, ignored) = from_json_ignoring_schema(r#type, f)?;
                            check_ignored_fields(&ignored)?;
                            let l = crate::Limits::none();
                            writeln!(stdout(), "{}", t.to_xdr_base64(l)?)
                                .map_err(Error::WriteOutput)?
                        }
                        OutputFormat::Stream => {
                            let stream =
                                serde_json::Deserializer::new(serde_json::de::IoRead::new(f))
                                    .into_iter::<serde_json::Value>();
                            for value_res in stream {
                                let mut value = value_res.map_err(crate::Error::Json)?;
                                strip_root_schema(&mut value);
                                let bytes =
                                    serde_json::to_vec(&value).map_err(crate::Error::Json)?;
                                let mut de = serde_json::Deserializer::from_slice(&bytes);
                                let (t, ignored) =
                                    crate::Type::deserialize_json_collecting_ignored_fields(
                                        r#type, &mut de,
                                    )?;
                                check_ignored_fields(&ignored)?;
                                let l = crate::Limits::none();
                                stdout()
                                    .write_all(&t.to_xdr(l)?)
                                    .map_err(Error::WriteOutput)?;
                            }
                        }
                    },
                };
            }
            Ok(())
        }
    };
}

impl Cmd {
    /// Run the CLIs encode command.
    ///
    /// ## Errors
    ///
    /// If the command is configured with state that is invalid.
    pub fn run(&self) -> Result<(), Error> {
        let result = self.run_inner();
        match result {
            Ok(()) => Ok(()),
            Err(Error::WriteOutput(e)) if e.kind() == std::io::ErrorKind::BrokenPipe => Ok(()),
            Err(e) => Err(e),
        }
    }

    run_x!(run_inner);
}

pub(crate) fn check_ignored_fields(ignored: &[String]) -> Result<(), Error> {
    let unknown: Vec<String> = ignored
        .iter()
        .filter(|field| field.as_str() != "$schema")
        .cloned()
        .collect();
    if unknown.is_empty() {
        return Ok(());
    }
    Err(Error::UnknownFields(unknown))
}
