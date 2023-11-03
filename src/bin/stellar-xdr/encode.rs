use std::{
    fs::File,
    io::{stdin, stdout, Read, Write},
    path::PathBuf,
    str::FromStr,
};

use clap::{Args, ValueEnum};

use crate::Channel;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("unknown type {0}, choose one of {1:?}")]
    UnknownType(String, &'static [&'static str]),
    #[error("error decoding JSON: {0}")]
    ReadJsonCurr(stellar_xdr::curr::Error),
    #[error("error decoding JSON: {0}")]
    ReadJsonNext(stellar_xdr::next::Error),
    #[error("error reading file: {0}")]
    ReadFile(#[from] std::io::Error),
    #[error("error generating XDR: {0}")]
    WriteXdrCurr(stellar_xdr::curr::Error),
    #[error("error generating XDR: {0}")]
    WriteXdrNext(stellar_xdr::next::Error),
}

impl From<stellar_xdr::curr::Error> for Error {
    fn from(e: stellar_xdr::curr::Error) -> Self {
        match e {
            stellar_xdr::curr::Error::Invalid
            | stellar_xdr::curr::Error::Unsupported
            | stellar_xdr::curr::Error::LengthExceedsMax
            | stellar_xdr::curr::Error::LengthMismatch
            | stellar_xdr::curr::Error::NonZeroPadding
            | stellar_xdr::curr::Error::Utf8Error(_)
            | stellar_xdr::curr::Error::InvalidHex
            | stellar_xdr::curr::Error::Io(_)
            | stellar_xdr::curr::Error::DepthLimitExceeded
            | stellar_xdr::curr::Error::LengthLimitExceeded => Error::WriteXdrCurr(e),
            stellar_xdr::curr::Error::Json(_) => Error::ReadJsonCurr(e),
        }
    }
}

impl From<stellar_xdr::next::Error> for Error {
    fn from(e: stellar_xdr::next::Error) -> Self {
        match e {
            stellar_xdr::next::Error::Invalid
            | stellar_xdr::next::Error::Unsupported
            | stellar_xdr::next::Error::LengthExceedsMax
            | stellar_xdr::next::Error::LengthMismatch
            | stellar_xdr::next::Error::NonZeroPadding
            | stellar_xdr::next::Error::Utf8Error(_)
            | stellar_xdr::next::Error::InvalidHex
            | stellar_xdr::next::Error::Io(_)
            | stellar_xdr::next::Error::DepthLimitExceeded
            | stellar_xdr::next::Error::LengthLimitExceeded => Error::WriteXdrNext(e),
            stellar_xdr::next::Error::Json(_) => Error::ReadJsonNext(e),
        }
    }
}

#[derive(Args, Debug, Clone)]
#[command()]
pub struct Cmd {
    /// Files to encode, or stdin if omitted
    #[arg()]
    files: Vec<PathBuf>,

    /// XDR type to encode
    #[arg(long)]
    r#type: String,

    // Input format
    #[arg(long, value_enum, default_value_t)]
    input: InputFormat,

    // Output format to encode to
    #[arg(long, value_enum, default_value_t)]
    output: OutputFormat,
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
}

impl Default for OutputFormat {
    fn default() -> Self {
        Self::SingleBase64
    }
}

macro_rules! run_x {
    ($f:ident, $m:ident) => {
        fn $f(&self) -> Result<(), Error> {
            use stellar_xdr::$m::WriteXdr;
            let mut files = self.files()?;
            let r#type = stellar_xdr::$m::TypeVariant::from_str(&self.r#type).map_err(|_| {
                Error::UnknownType(
                    self.r#type.clone(),
                    &stellar_xdr::$m::TypeVariant::VARIANTS_STR,
                )
            })?;
            for f in &mut files {
                match self.input {
                    InputFormat::Json => {
                        let t = stellar_xdr::$m::Type::read_json(r#type, f)?;
                        let l = stellar_xdr::$m::Limits::none();

                        match self.output {
                            OutputFormat::Single => stdout().write_all(&t.to_xdr(l)?)?,
                            OutputFormat::SingleBase64 => println!("{}", t.to_xdr_base64(l)?),
                        }
                    }
                };
            }
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

    fn files(&self) -> Result<Vec<Box<dyn Read>>, Error> {
        if self.files.is_empty() {
            Ok(vec![Box::new(stdin())])
        } else {
            Ok(self
                .files
                .iter()
                .map(File::open)
                .collect::<Result<Vec<_>, _>>()?
                .into_iter()
                .map(|f| -> Box<dyn Read> { Box::new(f) })
                .collect())
        }
    }
}
