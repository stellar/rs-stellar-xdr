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

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, ValueEnum)]
pub enum InputFormat {
    Json,
}

impl Default for InputFormat {
    fn default() -> Self {
        Self::Json
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, ValueEnum)]
pub enum OutputFormat {
    Single,
    SingleBase64,
    Stream,
    // TODO: StreamBase64,
    // TODO: StreamFramed,
}

impl Default for OutputFormat {
    fn default() -> Self {
        Self::SingleBase64
    }
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
                            let t = crate::Type::from_json(r#type, f)?;
                            let l = crate::Limits::none();
                            stdout()
                                .write_all(&t.to_xdr(l)?)
                                .map_err(Error::WriteOutput)?;
                        }
                        OutputFormat::SingleBase64 => {
                            let t = crate::Type::from_json(r#type, f)?;
                            let l = crate::Limits::none();
                            writeln!(stdout(), "{}", t.to_xdr_base64(l)?)
                                .map_err(Error::WriteOutput)?
                        }
                        OutputFormat::Stream => {
                            let mut de =
                                serde_json::Deserializer::new(serde_json::de::IoRead::new(f));
                            loop {
                                let t = match crate::Type::deserialize_json(r#type, &mut de) {
                                    Ok(t) => t,
                                    Err(crate::Error::Json(ref inner)) if inner.is_eof() => {
                                        break;
                                    }
                                    Err(e) => Err(e)?,
                                };
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
