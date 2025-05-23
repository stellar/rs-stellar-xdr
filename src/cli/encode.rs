use std::ffi::OsString;
use std::{
    io::{stdout, Write},
    str::FromStr,
};

use clap::{Args, ValueEnum};

use crate::cli::{util, Channel};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("unknown type {0}, choose one of {1:?}")]
    UnknownType(String, &'static [&'static str]),
    #[error("error decoding JSON: {0}")]
    ReadJsonCurr(crate::curr::Error),
    #[error("error decoding JSON: {0}")]
    ReadJsonNext(crate::next::Error),
    #[error("error reading file: {0}")]
    ReadFile(#[from] std::io::Error),
    #[error("error generating XDR: {0}")]
    WriteXdrCurr(crate::curr::Error),
    #[error("error generating XDR: {0}")]
    WriteXdrNext(crate::next::Error),
}

impl From<crate::curr::Error> for Error {
    fn from(e: crate::curr::Error) -> Self {
        match e {
            crate::curr::Error::Invalid
            | crate::curr::Error::Unsupported
            | crate::curr::Error::LengthExceedsMax
            | crate::curr::Error::LengthMismatch
            | crate::curr::Error::NonZeroPadding
            | crate::curr::Error::Utf8Error(_)
            | crate::curr::Error::InvalidHex
            | crate::curr::Error::Io(_)
            | crate::curr::Error::DepthLimitExceeded
            | crate::curr::Error::LengthLimitExceeded
            | crate::curr::Error::Arbitrary(_) => Error::WriteXdrCurr(e),
            crate::curr::Error::Json(_) => Error::ReadJsonCurr(e),
        }
    }
}

impl From<crate::next::Error> for Error {
    fn from(e: crate::next::Error) -> Self {
        match e {
            crate::next::Error::Invalid
            | crate::next::Error::Unsupported
            | crate::next::Error::LengthExceedsMax
            | crate::next::Error::LengthMismatch
            | crate::next::Error::NonZeroPadding
            | crate::next::Error::Utf8Error(_)
            | crate::next::Error::InvalidHex
            | crate::next::Error::Io(_)
            | crate::next::Error::DepthLimitExceeded
            | crate::next::Error::LengthLimitExceeded
            | crate::next::Error::Arbitrary(_) => Error::WriteXdrNext(e),
            crate::next::Error::Json(_) => Error::ReadJsonNext(e),
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

macro_rules! run_x {
    ($f:ident, $m:ident) => {
        fn $f(&self) -> Result<(), Error> {
            use crate::$m::WriteXdr;
            let mut input = util::parse_input::<Error>(&self.input)?;
            let r#type = crate::$m::TypeVariant::from_str(&self.r#type).map_err(|_| {
                Error::UnknownType(self.r#type.clone(), &crate::$m::TypeVariant::VARIANTS_STR)
            })?;
            for f in &mut input {
                match self.input_format {
                    InputFormat::Json => match self.output_format {
                        OutputFormat::Single => {
                            let t = crate::$m::Type::from_json(r#type, f)?;
                            let l = crate::$m::Limits::none();
                            stdout().write_all(&t.to_xdr(l)?)?
                        }
                        OutputFormat::SingleBase64 => {
                            let t = crate::$m::Type::from_json(r#type, f)?;
                            let l = crate::$m::Limits::none();
                            println!("{}", t.to_xdr_base64(l)?)
                        }
                        OutputFormat::Stream => {
                            let mut de =
                                serde_json::Deserializer::new(serde_json::de::IoRead::new(f));
                            loop {
                                let t = match crate::$m::Type::deserialize_json(r#type, &mut de) {
                                    Ok(t) => t,
                                    Err(crate::$m::Error::Json(ref inner)) if inner.is_eof() => {
                                        break;
                                    }
                                    Err(e) => Err(e)?,
                                };
                                let l = crate::$m::Limits::none();
                                stdout().write_all(&t.to_xdr(l)?)?
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
