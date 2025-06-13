use arbitrary::Unstructured;
use clap::{Args, ValueEnum};
use std::{
    io::{stdout, Write},
    str::FromStr,
};

use crate::cli::{util, Channel};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("unknown type {0}, choose one of {1:?}")]
    UnknownType(String, &'static [&'static str]),
    #[error("error reading file: {0}")]
    ReadFile(#[from] std::io::Error),
    #[error("error generating XDR: {0}")]
    WriteXdrCurr(crate::curr::Error),
    #[error("error generating XDR: {0}")]
    WriteXdrNext(crate::next::Error),
    #[error("error generating JSON: {0}")]
    GenerateJson(#[from] serde_json::Error),
    #[error("error generating arbitrary value: {0}")]
    Arbitrary(#[from] arbitrary::Error),
    #[error("type doesn't have a text representation, use 'json' as output")]
    TextUnsupported,
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
            | crate::curr::Error::Arbitrary(_)
            | crate::curr::Error::Json(_) => Error::WriteXdrCurr(e),
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
            | crate::next::Error::Arbitrary(_)
            | crate::next::Error::Json(_) => Error::WriteXdrNext(e),
        }
    }
}

/// Generate arbitrary XDR values
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

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, ValueEnum)]
pub enum OutputFormat {
    Single,
    SingleBase64,
    // TODO: Stream,
    // TODO: StreamBase64,
    // TODO: StreamFramed,
    Json,
    JsonFormatted,
    Text,
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
            use rand::{Rng, SeedableRng};
            let r#type = crate::$m::TypeVariant::from_str(&self.r#type).map_err(|_| {
                Error::UnknownType(self.r#type.clone(), &crate::$m::TypeVariant::VARIANTS_STR)
            })?;
            let mut rng = rand::rngs::StdRng::from_os_rng();
            let mut bytes = vec![0; 1024 * 10];
            let mut iter = bytes.chunks_exact_mut(4);
            while let Some([a0, b0, c0, d0]) = iter.next() {
                let [a1, b1, c1, d1] = rng.random::<u32>().to_be_bytes();
                *a0 = a1;
                *b0 = b1;
                *c0 = c1;
                *d0 = d1;
            }
            for elem in iter.into_remainder() {
                *elem = rng.random::<u8>();
            }
            let mut u = Unstructured::new(&bytes);
            let v = crate::$m::Type::arbitrary(r#type, &mut u)?;
            match self.output_format {
                OutputFormat::Single => {
                    let l = crate::$m::Limits::none();
                    stdout().write_all(&v.to_xdr(l)?)?
                }
                OutputFormat::SingleBase64 => {
                    let l = crate::$m::Limits::none();
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
    /// Run the CLIs generate arbitrary command.
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
