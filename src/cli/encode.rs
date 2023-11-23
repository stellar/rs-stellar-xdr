use std::{
    fs::File,
    io::{stdin, stdout, Read, Write},
    path::PathBuf,
    str::FromStr,
};

use clap::{Args, ValueEnum};

use crate::cli::Channel;

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
            | crate::curr::Error::LengthLimitExceeded => Error::WriteXdrCurr(e),
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
            | crate::next::Error::LengthLimitExceeded => Error::WriteXdrNext(e),
            crate::next::Error::Json(_) => Error::ReadJsonNext(e),
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
            use crate::$m::WriteXdr;
            let mut files = self.files()?;
            let r#type = crate::$m::TypeVariant::from_str(&self.r#type).map_err(|_| {
                Error::UnknownType(self.r#type.clone(), &crate::$m::TypeVariant::VARIANTS_STR)
            })?;
            for f in &mut files {
                match self.input {
                    InputFormat::Json => {
                        let t = crate::$m::Type::read_json(r#type, f)?;
                        let l = crate::$m::Limits::none();

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
