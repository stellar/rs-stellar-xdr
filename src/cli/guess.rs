use clap::{Args, ValueEnum};
use std::ffi::OsString;
use std::fs::File;
use std::io::{stdin, Cursor};
use std::path::Path;
use std::{
    cmp,
    io::{self, Read},
};

use crate::cli::Channel;

#[derive(thiserror::Error, Debug)]
#[allow(clippy::enum_variant_names)]
pub enum Error {
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
    /// XDR or file containing XDR to decode, or stdin if empty
    #[arg()]
    pub input: Option<OsString>,

    // Input format
    #[arg(long = "input", value_enum, default_value_t)]
    pub input_format: InputFormat,

    // Output format
    #[arg(long = "output", value_enum, default_value_t)]
    pub output_format: OutputFormat,

    /// Certainty as an arbitrary value
    #[arg(long, default_value = "2")]
    pub certainty: usize,
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
        Self::SingleBase64
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, ValueEnum)]
pub enum OutputFormat {
    List,
}

impl Default for OutputFormat {
    fn default() -> Self {
        Self::List
    }
}

macro_rules! run_x {
    ($f:ident, $m:ident) => {
        fn $f(&self) -> Result<(), Error> {
            let mut rr = ResetRead::new(self.input()?);
            let mut guessed = false;
            'variants: for v in crate::$m::TypeVariant::VARIANTS {
                rr.reset();
                let count: usize = match self.input_format {
                    InputFormat::Single => {
                        let mut l = crate::$m::Limited::new(&mut rr, crate::$m::Limits::none());
                        crate::$m::Type::read_xdr_to_end(v, &mut l)
                            .ok()
                            .map(|_| 1)
                            .unwrap_or_default()
                    }
                    InputFormat::SingleBase64 => {
                        let mut l = crate::$m::Limited::new(&mut rr, crate::$m::Limits::none());
                        crate::$m::Type::read_xdr_base64_to_end(v, &mut l)
                            .ok()
                            .map(|_| 1)
                            .unwrap_or_default()
                    }
                    InputFormat::Stream => {
                        let mut l = crate::$m::Limited::new(&mut rr, crate::$m::Limits::none());
                        let iter = crate::$m::Type::read_xdr_iter(v, &mut l);
                        let iter = iter.take(self.certainty);
                        let mut count = 0;
                        for v in iter {
                            match v {
                                Ok(_) => count += 1,
                                Err(_) => continue 'variants,
                            }
                        }
                        count
                    }
                    InputFormat::StreamBase64 => {
                        let mut l = crate::$m::Limited::new(&mut rr, crate::$m::Limits::none());
                        let iter = crate::$m::Type::read_xdr_base64_iter(v, &mut l);
                        let iter = iter.take(self.certainty);
                        let mut count = 0;
                        for v in iter {
                            match v {
                                Ok(_) => count += 1,
                                Err(_) => continue 'variants,
                            }
                        }
                        count
                    }
                    InputFormat::StreamFramed => {
                        let mut l = crate::$m::Limited::new(&mut rr, crate::$m::Limits::none());
                        let iter = crate::$m::Type::read_xdr_framed_iter(v, &mut l);
                        let iter = iter.take(self.certainty);
                        let mut count = 0;
                        for v in iter {
                            match v {
                                Ok(_) => count += 1,
                                Err(_) => continue 'variants,
                            }
                        }
                        count
                    }
                };
                if count > 0 {
                    println!("{}", v.name());
                    guessed = true;
                }
            }
            if (!guessed) {
                std::process::exit(1);
            }
            Ok(())
        }
    };
}

impl Cmd {
    /// Run the CLIs guess command.
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

    fn input(&self) -> Result<Box<dyn Read>, Error> {
        if let Some(input) = &self.input {
            let exist = Path::new(input).try_exists();
            if let Ok(true) = exist {
                Ok(Box::new(File::open(input)?))
            } else {
                Ok(Box::new(Cursor::new(input.clone().into_encoded_bytes())))
            }
        } else {
            Ok(Box::new(stdin()))
        }
    }
}

struct ResetRead<R: Read> {
    read: R,
    buf: Vec<u8>,
    cursor: usize,
}

impl<R: Read> ResetRead<R> {
    fn new(r: R) -> Self {
        Self {
            read: r,
            buf: Vec::new(),
            cursor: 0,
        }
    }

    fn reset(&mut self) {
        self.cursor = 0;
    }
}

impl<R: Read> Read for ResetRead<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        // Read from the buffer first into buf.
        let n = cmp::min(self.buf.len() - self.cursor, buf.len());
        buf[..n].copy_from_slice(&self.buf[self.cursor..self.cursor + n]);
        // Read from the reader and cache the result in the buf if the buf is consumed.
        if n < buf.len() {
            let read_n = self.read.read(buf)?;
            self.buf.extend_from_slice(&buf[n..n + read_n]);
            self.cursor += n + read_n;
            Ok(n + read_n)
        } else {
            self.cursor += n;
            Ok(n)
        }
    }
}

#[cfg(test)]
mod test {
    use std::{
        error,
        io::{Cursor, Read},
    };

    use super::ResetRead;

    #[test]
    fn test_reset_read() -> Result<(), Box<dyn error::Error>> {
        let source: Vec<u8> = (0..8).collect();
        let reader = Cursor::new(source);
        let mut rr = ResetRead::new(reader);

        let mut buf = [0u8; 4];
        let n = rr.read(&mut buf)?;
        assert_eq!(n, 4);
        assert_eq!(buf, [0, 1, 2, 3]);

        let mut buf = [0u8; 4];
        let n = rr.read(&mut buf)?;
        assert_eq!(n, 4);
        assert_eq!(buf, [4, 5, 6, 7]);

        let n = rr.read(&mut buf)?;
        assert_eq!(n, 0);

        rr.reset();
        let mut buf = [0u8; 4];
        let n = rr.read(&mut buf)?;
        assert_eq!(n, 4);
        assert_eq!(buf, [0, 1, 2, 3]);

        Ok(())
    }
}
