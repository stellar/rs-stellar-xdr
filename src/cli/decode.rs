use crate::cli::{skip_whitespace::SkipWhitespace, Channel};
use clap::{Args, ValueEnum};
use serde::Serialize;
use std::ffi::OsString;
use std::io::Cursor;
use std::path::Path;
use std::{
    fmt::Debug,
    fs::File,
    io::{stdin, Read},
    str::FromStr,
};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("unknown type {0}, choose one of {1:?}")]
    UnknownType(String, &'static [&'static str]),
    #[error("error decoding XDR: {0}")]
    ReadXdrCurr(#[from] crate::curr::Error),
    #[error("error decoding XDR: {0}")]
    ReadXdrNext(#[from] crate::next::Error),
    #[error("error reading file: {0}")]
    ReadFile(#[from] std::io::Error),
    #[error("error generating JSON: {0}")]
    GenerateJson(#[from] serde_json::Error),
}

#[derive(Args, Debug, Clone)]
#[command()]
pub struct Cmd {
    /// XDR or files containing XDR to decode, or stdin if empty
    #[arg()]
    pub input: Vec<OsString>,

    /// XDR type to decode
    #[arg(long)]
    pub r#type: String,

    // Input format of the XDR
    #[arg(long = "input", value_enum, default_value_t)]
    pub input_format: InputFormat,

    // Output format
    #[arg(long = "output", value_enum, default_value_t)]
    pub output_format: OutputFormat,
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
    RustDebug,
    RustDebugFormatted,
}

impl Default for OutputFormat {
    fn default() -> Self {
        Self::Json
    }
}

macro_rules! run_x {
    ($f:ident, $m:ident) => {
        fn $f(&self) -> Result<(), Error> {
            let mut input = self.parse_input()?;
            let r#type = crate::$m::TypeVariant::from_str(&self.r#type).map_err(|_| {
                Error::UnknownType(self.r#type.clone(), &crate::$m::TypeVariant::VARIANTS_STR)
            })?;
            for f in &mut input {
                match self.input_format {
                    InputFormat::Single => {
                        let mut l = crate::$m::Limited::new(f, crate::$m::Limits::none());
                        let t = crate::$m::Type::read_xdr_to_end(r#type, &mut l)?;
                        self.out(&t)?;
                    }
                    InputFormat::SingleBase64 => {
                        let sw = SkipWhitespace::new(f);
                        let mut l = crate::$m::Limited::new(sw, crate::$m::Limits::none());
                        let t = crate::$m::Type::read_xdr_base64_to_end(r#type, &mut l)?;
                        self.out(&t)?;
                    }
                    InputFormat::Stream => {
                        let mut l = crate::$m::Limited::new(f, crate::$m::Limits::none());
                        for t in crate::$m::Type::read_xdr_iter(r#type, &mut l) {
                            self.out(&t?)?;
                        }
                    }
                    InputFormat::StreamBase64 => {
                        let sw = SkipWhitespace::new(f);
                        let mut l = crate::$m::Limited::new(sw, crate::$m::Limits::none());
                        for t in crate::$m::Type::read_xdr_base64_iter(r#type, &mut l) {
                            self.out(&t?)?;
                        }
                    }
                    InputFormat::StreamFramed => {
                        let mut l = crate::$m::Limited::new(f, crate::$m::Limits::none());
                        for t in crate::$m::Type::read_xdr_framed_iter(r#type, &mut l) {
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
    /// Run the CLIs decode command.
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

    fn parse_input(&self) -> Result<Vec<Box<dyn Read>>, Error> {
        if self.input.is_empty() {
            Ok(vec![Box::new(stdin())])
        } else {
            Ok(self
                .input
                .iter()
                .map(|input| {
                    let exist = Path::new(input).try_exists();
                    if let Ok(true) = exist {
                        let file = File::open(input)?;
                        Ok::<Box<dyn Read>, Error>(Box::new(file) as Box<dyn Read>)
                    } else {
                        Ok(Box::new(Cursor::new(input.clone().into_encoded_bytes()))
                            as Box<dyn Read>)
                    }
                })
                .collect::<Result<Vec<_>, _>>()?
                .into_iter()
                .collect())
        }
    }

    fn out(&self, v: &(impl Serialize + Debug)) -> Result<(), Error> {
        match self.output_format {
            OutputFormat::Json => println!("{}", serde_json::to_string(v)?),
            OutputFormat::JsonFormatted => println!("{}", serde_json::to_string_pretty(v)?),
            OutputFormat::RustDebug => println!("{v:?}"),
            OutputFormat::RustDebugFormatted => println!("{v:#?}"),
        }
        Ok(())
    }
}
