use std::{
    fs::File,
    io::{stdin, Read},
    path::PathBuf,
    str::FromStr,
};

use clap::{Args, ValueEnum};
use serde::Serialize;

use crate::Channel;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("unknown type {0}, choose one of {1:?}")]
    UnknownType(String, &'static [&'static str]),
    #[error("error decoding XDR: {0}")]
    ReadXdrCurr(#[from] stellar_xdr::curr::Error),
    #[error("error decoding XDR: {0}")]
    ReadXdrNext(#[from] stellar_xdr::next::Error),
    #[error("error reading file: {0}")]
    ReadFile(#[from] std::io::Error),
    #[error("error generating JSON: {0}")]
    GenerateJson(#[from] serde_json::Error),
}

#[derive(Args, Debug, Clone)]
#[command()]
pub struct Cmd {
    /// Files to decode, or stdin if omitted
    #[arg()]
    files: Vec<PathBuf>,

    /// XDR type to decode
    #[arg(long)]
    r#type: String,

    // Input format of the XDR
    #[arg(long, value_enum, default_value_t)]
    input: InputFormat,

    // Output format
    #[arg(long, value_enum, default_value_t)]
    output: OutputFormat,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, ValueEnum)]
pub enum InputFormat {
    Single,
    SingleBase64,
    Stream,
    StreamBase64,
    StreamFramed,
}

impl Default for InputFormat {
    fn default() -> Self {
        Self::StreamBase64
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, ValueEnum)]
pub enum OutputFormat {
    Json,
    JsonFormatted,
}

impl Default for OutputFormat {
    fn default() -> Self {
        Self::Json
    }
}

macro_rules! run_x {
    ($f:ident, $m:ident) => {
        fn $f(&self) -> Result<(), Error> {
            let mut files = self.files()?;
            let r#type = stellar_xdr::$m::TypeVariant::from_str(&self.r#type).map_err(|_| {
                Error::UnknownType(
                    self.r#type.clone(),
                    &stellar_xdr::$m::TypeVariant::VARIANTS_STR,
                )
            })?;
            for f in &mut files {
                let mut f = stellar_xdr::$m::DepthLimitedRead::new(
                    f,
                    stellar_xdr::$m::DEFAULT_MAX_DEPTH_LIMIT,
                );
                match self.input {
                    InputFormat::Single => {
                        let t = stellar_xdr::$m::Type::read_xdr_to_end(r#type, &mut f)?;
                        self.out(&t)?;
                    }
                    InputFormat::SingleBase64 => {
                        let t = stellar_xdr::$m::Type::read_xdr_base64_to_end(r#type, &mut f)?;
                        self.out(&t)?;
                    }
                    InputFormat::Stream => {
                        for t in stellar_xdr::$m::Type::read_xdr_iter(r#type, &mut f) {
                            self.out(&t?)?;
                        }
                    }
                    InputFormat::StreamBase64 => {
                        for t in stellar_xdr::$m::Type::read_xdr_base64_iter(r#type, &mut f) {
                            self.out(&t?)?;
                        }
                    }
                    InputFormat::StreamFramed => {
                        for t in stellar_xdr::$m::Type::read_xdr_framed_iter(r#type, &mut f) {
                            self.out(&t?)?;
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

    fn out(&self, v: &impl Serialize) -> Result<(), Error> {
        match self.output {
            OutputFormat::Json => println!("{}", serde_json::to_string(v)?),
            OutputFormat::JsonFormatted => println!("{}", serde_json::to_string_pretty(v)?),
        }
        Ok(())
    }
}
