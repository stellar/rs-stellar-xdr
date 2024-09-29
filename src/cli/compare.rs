use std::{fmt::Debug, fs::File, path::PathBuf, str::FromStr};

use clap::{Args, ValueEnum};

use crate::cli::{skip_whitespace::SkipWhitespace, Channel};

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
}

#[derive(Args, Debug, Clone)]
#[command()]
pub struct Cmd {
    /// First XDR to decode and compare with the second file
    #[arg()]
    file1: PathBuf,

    /// Second XDR to decode and compare with the second file
    #[arg()]
    file2: PathBuf,

    /// XDR type of both inputs
    #[arg(long)]
    r#type: String,

    // Input format of the XDR
    #[arg(long, value_enum, default_value_t)]
    input: InputFormat,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, ValueEnum)]
pub enum InputFormat {
    Single,
    SingleBase64,
}

impl Default for InputFormat {
    fn default() -> Self {
        Self::SingleBase64
    }
}

macro_rules! run_x {
    ($f:ident, $m:ident) => {
        fn $f(&self) -> Result<(), Error> {
            let f1 = File::open(&self.file1)?;
            let f2 = File::open(&self.file2)?;
            let r#type = crate::$m::TypeVariant::from_str(&self.r#type).map_err(|_| {
                Error::UnknownType(self.r#type.clone(), &crate::$m::TypeVariant::VARIANTS_STR)
            })?;
            let (t1, t2) = match self.input {
                InputFormat::Single => {
                    let t1 = {
                        let mut l1 = crate::$m::Limited::new(f1, crate::$m::Limits::none());
                        crate::$m::Type::read_xdr_to_end(r#type, &mut l1)?
                    };
                    let t2 = {
                        let mut l = crate::$m::Limited::new(f2, crate::$m::Limits::none());
                        crate::$m::Type::read_xdr_to_end(r#type, &mut l)?
                    };
                    (t1, t2)
                }
                InputFormat::SingleBase64 => {
                    let t1 = {
                        let sw = SkipWhitespace::new(f1);
                        let mut l = crate::$m::Limited::new(sw, crate::$m::Limits::none());
                        crate::$m::Type::read_xdr_base64_to_end(r#type, &mut l)?
                    };
                    let t2 = {
                        let sw = SkipWhitespace::new(f2);
                        let mut l = crate::$m::Limited::new(sw, crate::$m::Limits::none());
                        crate::$m::Type::read_xdr_base64_to_end(r#type, &mut l)?
                    };
                    (t1, t2)
                }
            };
            let cmp = t1.cmp(&t2) as i8;
            println!("{cmp}");
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
}
