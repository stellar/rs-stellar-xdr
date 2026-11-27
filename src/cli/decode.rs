use std::ffi::OsString;
use std::io::{stdout, Write};
use std::{fmt::Debug, str::FromStr};

use clap::{Args, ValueEnum};
use serde::Serialize;

use crate::cli::util;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("unknown type {0}, choose one of {1:?}")]
    UnknownType(String, &'static [&'static str]),
    #[error("error decoding XDR: {0}")]
    ReadXdr(#[from] crate::Error),
    #[error("error reading file: {0}")]
    ReadFile(std::io::Error),
    #[error("error writing output: {0}")]
    WriteOutput(std::io::Error),
    #[error("error generating JSON: {0}")]
    GenerateJson(#[from] serde_json::Error),
    #[error("type doesn't have a text representation, use 'json' as output")]
    TextUnsupported,
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

#[derive(Default, Clone, Copy, Debug, Eq, Hash, PartialEq, ValueEnum)]
pub enum InputFormat {
    Single,
    SingleBase64,
    Stream,
    #[default]
    StreamBase64,
    StreamFramed,
}

#[derive(Default, Clone, Copy, Debug, Eq, Hash, PartialEq, ValueEnum)]
pub enum OutputFormat {
    #[default]
    Json,
    JsonFormatted,
    Text,
    RustDebug,
    RustDebugFormatted,
}

// TODO: Remove run_x macro, it exists only to reduce the diff from when curr/next
// channels existed and each had their own run_curr/run_next invocation.
macro_rules! run_x {
    ($f:ident) => {
        fn $f(&self) -> Result<(), Error> {
            let mut input = util::parse_input(&self.input).map_err(Error::ReadFile)?;
            let r#type = crate::TypeVariant::from_str(&self.r#type).map_err(|_| {
                Error::UnknownType(self.r#type.clone(), &crate::TypeVariant::VARIANTS_STR)
            })?;
            for (f, limits) in &mut input {
                match self.input_format {
                    InputFormat::Single => {
                        let mut l = crate::Limited::new(f, limits.clone());
                        let t = crate::Type::read_xdr_to_end(r#type, &mut l)?;
                        self.out(&t)?;
                    }
                    InputFormat::SingleBase64 => {
                        let mut l = crate::Limited::new(f, limits.clone());
                        let t = crate::Type::read_xdr_base64_to_end(r#type, &mut l)?;
                        self.out(&t)?;
                    }
                    InputFormat::Stream => {
                        let mut l = crate::Limited::new(f, limits.clone());
                        for t in crate::Type::read_xdr_iter(r#type, &mut l) {
                            self.out(&t?)?;
                        }
                    }
                    InputFormat::StreamBase64 => {
                        let mut l = crate::Limited::new(f, limits.clone());
                        for t in crate::Type::read_xdr_base64_iter(r#type, &mut l) {
                            self.out(&t?)?;
                        }
                    }
                    InputFormat::StreamFramed => {
                        let mut l = crate::Limited::new(f, limits.clone());
                        for t in crate::Type::read_xdr_framed_iter(r#type, &mut l) {
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
    pub fn run(&self) -> Result<(), Error> {
        let result = self.run_inner();
        match result {
            Ok(()) => Ok(()),
            Err(Error::WriteOutput(e)) if e.kind() == std::io::ErrorKind::BrokenPipe => Ok(()),
            Err(e) => Err(e),
        }
    }

    run_x!(run_inner);

    fn out(&self, v: &(impl Serialize + Debug)) -> Result<(), Error> {
        let text = match self.output_format {
            OutputFormat::Json => serde_json::to_string(v)?,
            OutputFormat::JsonFormatted => serde_json::to_string_pretty(v)?,
            OutputFormat::Text => {
                let v = serde_json::to_value(v)?;
                util::serde_json_value_to_text(v).ok_or(Error::TextUnsupported)?
            }
            OutputFormat::RustDebug => format!("{v:?}"),
            OutputFormat::RustDebugFormatted => format!("{v:#?}"),
        };
        writeln!(stdout(), "{text}").map_err(Error::WriteOutput)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{BytesM, Limits, ScBytes, WriteXdr};
    use base64::Engine as _;

    // A ScBytes value whose length prefix declares 0xFFFFFFF0 bytes but that is
    // followed by no data.
    const HUGE_LEN: [u8; 4] = [0xFF, 0xFF, 0xFF, 0xF0];

    fn decode(input: impl Into<OsString>, input_format: InputFormat) -> Result<(), Error> {
        Cmd {
            input: vec![input.into()],
            r#type: "ScBytes".to_string(),
            input_format,
            output_format: OutputFormat::Json,
        }
        .run()
    }

    fn sc_bytes_xdr(len: u8) -> Vec<u8> {
        let v = ScBytes(BytesM::try_from(vec![len; len.into()]).unwrap());
        v.to_xdr(Limits::none()).unwrap()
    }

    fn framed(xdr: &[u8]) -> Vec<u8> {
        let header = 0x8000_0000 | u32::try_from(xdr.len()).unwrap();
        let mut bytes = header.to_be_bytes().to_vec();
        bytes.extend_from_slice(xdr);
        bytes
    }

    fn base64(bytes: &[u8]) -> Vec<u8> {
        base64::engine::general_purpose::STANDARD
            .encode(bytes)
            .into_bytes()
    }

    fn assert_length_limit_exceeded(r: &Result<(), Error>) {
        assert!(
            matches!(r, Err(Error::ReadXdr(crate::Error::LengthLimitExceeded))),
            "{r:?}"
        );
    }

    #[test]
    fn huge_length_in_file_is_rejected_by_input_size() {
        let path = util::temp_file(&HUGE_LEN);
        assert_length_limit_exceeded(&decode(&path, InputFormat::Single));
        assert_length_limit_exceeded(&decode(&path, InputFormat::Stream));
        std::fs::remove_file(path).unwrap();

        let path = util::temp_file(&base64(&HUGE_LEN));
        assert_length_limit_exceeded(&decode(&path, InputFormat::SingleBase64));
        assert_length_limit_exceeded(&decode(&path, InputFormat::StreamBase64));
        std::fs::remove_file(path).unwrap();

        let path = util::temp_file(&framed(&HUGE_LEN));
        assert_length_limit_exceeded(&decode(&path, InputFormat::StreamFramed));
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn huge_length_inline_is_rejected_by_input_size() {
        let input = String::from_utf8(base64(&HUGE_LEN)).unwrap();
        assert_length_limit_exceeded(&decode(input.clone(), InputFormat::SingleBase64));
        assert_length_limit_exceeded(&decode(input, InputFormat::StreamBase64));
    }

    #[test]
    fn decode_each_input_format() {
        let xdr = sc_bytes_xdr(5);
        for (input_format, contents) in [
            (InputFormat::Single, xdr.clone()),
            (InputFormat::SingleBase64, base64(&xdr)),
            (InputFormat::Stream, xdr.clone()),
            (InputFormat::StreamBase64, base64(&xdr)),
            (InputFormat::StreamFramed, framed(&xdr)),
        ] {
            let path = util::temp_file(&contents);
            decode(&path, input_format).unwrap();
            std::fs::remove_file(path).unwrap();
        }
        let inline = String::from_utf8(base64(&xdr)).unwrap();
        decode(inline.clone(), InputFormat::SingleBase64).unwrap();
        decode(inline, InputFormat::StreamBase64).unwrap();
    }

    #[test]
    fn decode_multi_item_stream_fully() {
        // The length limit is shared by all items in a stream, so the items
        // must together fit within the size of the input.
        let items = [sc_bytes_xdr(1), sc_bytes_xdr(5), sc_bytes_xdr(9)];
        let stream = items.concat();
        let framed_stream: Vec<u8> = items.iter().flat_map(|i| framed(i)).collect();
        for (input_format, contents) in [
            (InputFormat::Stream, stream.clone()),
            (InputFormat::StreamBase64, base64(&stream)),
            (InputFormat::StreamFramed, framed_stream),
        ] {
            let path = util::temp_file(&contents);
            decode(&path, input_format).unwrap();
            std::fs::remove_file(path).unwrap();
        }
    }
}
