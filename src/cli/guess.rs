use std::{
    cmp,
    fs::File,
    io::{self, stdin, Read},
    path::PathBuf,
};

use clap::{Args, ValueEnum};

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
    /// File to decode, or stdin if omitted
    #[arg()]
    file: Option<PathBuf>,

    // Input format of the XDR
    #[arg(long, value_enum, default_value_t)]
    input: InputFormat,

    // Output format
    #[arg(long, value_enum, default_value_t)]
    output: OutputFormat,

    /// Certainty as an arbitrary value
    #[arg(long, default_value = "2")]
    certainty: usize,
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
            let mut f =
                crate::$m::Limited::new(ResetRead::new(self.file()?), crate::$m::Limits::none());
            'variants: for v in crate::$m::TypeVariant::VARIANTS {
                f.inner.reset();
                let count: usize = match self.input {
                    InputFormat::Single => crate::$m::Type::read_xdr_to_end(v, &mut f)
                        .ok()
                        .map(|_| 1)
                        .unwrap_or_default(),
                    InputFormat::SingleBase64 => crate::$m::Type::read_xdr_base64_to_end(v, &mut f)
                        .ok()
                        .map(|_| 1)
                        .unwrap_or_default(),
                    InputFormat::Stream => {
                        let iter = crate::$m::Type::read_xdr_iter(v, &mut f).take(self.certainty);
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
                        let iter =
                            crate::$m::Type::read_xdr_base64_iter(v, &mut f).take(self.certainty);
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
                        let iter =
                            crate::$m::Type::read_xdr_framed_iter(v, &mut f).take(self.certainty);
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
                }
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

    fn file(&self) -> Result<Box<dyn Read>, Error> {
        if let Some(f) = &self.file {
            Ok(Box::new(File::open(f)?))
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
