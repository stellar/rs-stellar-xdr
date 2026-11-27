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

    #[test]
    fn huge_length_in_single_file_is_rejected_by_input_size() {
        // A ScBytes value whose length prefix declares 0xFFFFFFF0 bytes but that
        // is followed by no data.
        let path = std::env::temp_dir().join(format!(
            "stellar-xdr-cli-test-{:016x}",
            rand::random::<u64>()
        ));
        std::fs::write(&path, [0xFF, 0xFF, 0xFF, 0xF0]).unwrap();
        let r = Cmd {
            input: vec![path.clone().into()],
            r#type: "ScBytes".to_string(),
            input_format: InputFormat::Single,
            output_format: OutputFormat::Json,
        }
        .run();
        std::fs::remove_file(path).unwrap();
        assert!(
            matches!(r, Err(Error::ReadXdr(crate::Error::LengthLimitExceeded))),
            "{r:?}"
        );
    }

    #[test]
    fn huge_length_in_single_base64_file_is_rejected_by_input_size() {
        // A ScBytes value whose length prefix declares 0xFFFFFFF0 bytes but that
        // is followed by no data, base64 encoded.
        let path = std::env::temp_dir().join(format!(
            "stellar-xdr-cli-test-{:016x}",
            rand::random::<u64>()
        ));
        std::fs::write(&path, "////8A==").unwrap();
        let r = Cmd {
            input: vec![path.clone().into()],
            r#type: "ScBytes".to_string(),
            input_format: InputFormat::SingleBase64,
            output_format: OutputFormat::Json,
        }
        .run();
        std::fs::remove_file(path).unwrap();
        assert!(
            matches!(r, Err(Error::ReadXdr(crate::Error::LengthLimitExceeded))),
            "{r:?}"
        );
    }

    #[test]
    fn huge_length_in_stream_file_is_rejected_by_input_size() {
        // A ScBytes value whose length prefix declares 0xFFFFFFF0 bytes but that
        // is followed by no data.
        let path = std::env::temp_dir().join(format!(
            "stellar-xdr-cli-test-{:016x}",
            rand::random::<u64>()
        ));
        std::fs::write(&path, [0xFF, 0xFF, 0xFF, 0xF0]).unwrap();
        let r = Cmd {
            input: vec![path.clone().into()],
            r#type: "ScBytes".to_string(),
            input_format: InputFormat::Stream,
            output_format: OutputFormat::Json,
        }
        .run();
        std::fs::remove_file(path).unwrap();
        assert!(
            matches!(r, Err(Error::ReadXdr(crate::Error::LengthLimitExceeded))),
            "{r:?}"
        );
    }

    #[test]
    fn huge_length_in_stream_base64_file_is_rejected_by_input_size() {
        // A ScBytes value whose length prefix declares 0xFFFFFFF0 bytes but that
        // is followed by no data, base64 encoded.
        let path = std::env::temp_dir().join(format!(
            "stellar-xdr-cli-test-{:016x}",
            rand::random::<u64>()
        ));
        std::fs::write(&path, "////8A==").unwrap();
        let r = Cmd {
            input: vec![path.clone().into()],
            r#type: "ScBytes".to_string(),
            input_format: InputFormat::StreamBase64,
            output_format: OutputFormat::Json,
        }
        .run();
        std::fs::remove_file(path).unwrap();
        assert!(
            matches!(r, Err(Error::ReadXdr(crate::Error::LengthLimitExceeded))),
            "{r:?}"
        );
    }

    #[test]
    fn huge_length_in_stream_framed_file_is_rejected_by_input_size() {
        // A ScBytes value whose length prefix declares 0xFFFFFFF0 bytes but that
        // is followed by no data, preceded by a frame header for a last fragment.
        let path = std::env::temp_dir().join(format!(
            "stellar-xdr-cli-test-{:016x}",
            rand::random::<u64>()
        ));
        std::fs::write(&path, [0x80, 0x00, 0x00, 0x04, 0xFF, 0xFF, 0xFF, 0xF0]).unwrap();
        let r = Cmd {
            input: vec![path.clone().into()],
            r#type: "ScBytes".to_string(),
            input_format: InputFormat::StreamFramed,
            output_format: OutputFormat::Json,
        }
        .run();
        std::fs::remove_file(path).unwrap();
        assert!(
            matches!(r, Err(Error::ReadXdr(crate::Error::LengthLimitExceeded))),
            "{r:?}"
        );
    }

    #[test]
    fn huge_length_in_single_base64_inline_is_rejected_by_input_size() {
        // A ScBytes value whose length prefix declares 0xFFFFFFF0 bytes but that
        // is followed by no data, base64 encoded.
        let r = Cmd {
            input: vec!["////8A==".into()],
            r#type: "ScBytes".to_string(),
            input_format: InputFormat::SingleBase64,
            output_format: OutputFormat::Json,
        }
        .run();
        assert!(
            matches!(r, Err(Error::ReadXdr(crate::Error::LengthLimitExceeded))),
            "{r:?}"
        );
    }

    #[test]
    fn huge_length_in_stream_base64_inline_is_rejected_by_input_size() {
        // A ScBytes value whose length prefix declares 0xFFFFFFF0 bytes but that
        // is followed by no data, base64 encoded.
        let r = Cmd {
            input: vec!["////8A==".into()],
            r#type: "ScBytes".to_string(),
            input_format: InputFormat::StreamBase64,
            output_format: OutputFormat::Json,
        }
        .run();
        assert!(
            matches!(r, Err(Error::ReadXdr(crate::Error::LengthLimitExceeded))),
            "{r:?}"
        );
    }

    #[test]
    fn valid_single_file_is_accepted_by_input_size() {
        // A ScBytes value of 5 bytes.
        let path = std::env::temp_dir().join(format!(
            "stellar-xdr-cli-test-{:016x}",
            rand::random::<u64>()
        ));
        std::fs::write(
            &path,
            [
                0x00, 0x00, 0x00, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x00, 0x00, 0x00,
            ],
        )
        .unwrap();
        let r = Cmd {
            input: vec![path.clone().into()],
            r#type: "ScBytes".to_string(),
            input_format: InputFormat::Single,
            output_format: OutputFormat::Json,
        }
        .run();
        std::fs::remove_file(path).unwrap();
        r.unwrap();
    }

    #[test]
    fn valid_single_base64_file_is_accepted_by_input_size() {
        // A ScBytes value of 5 bytes, base64 encoded.
        let path = std::env::temp_dir().join(format!(
            "stellar-xdr-cli-test-{:016x}",
            rand::random::<u64>()
        ));
        std::fs::write(&path, "AAAABQUFBQUFAAAA").unwrap();
        let r = Cmd {
            input: vec![path.clone().into()],
            r#type: "ScBytes".to_string(),
            input_format: InputFormat::SingleBase64,
            output_format: OutputFormat::Json,
        }
        .run();
        std::fs::remove_file(path).unwrap();
        r.unwrap();
    }

    #[test]
    fn valid_stream_file_is_accepted_by_input_size() {
        // A ScBytes value of 5 bytes.
        let path = std::env::temp_dir().join(format!(
            "stellar-xdr-cli-test-{:016x}",
            rand::random::<u64>()
        ));
        std::fs::write(
            &path,
            [
                0x00, 0x00, 0x00, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x00, 0x00, 0x00,
            ],
        )
        .unwrap();
        let r = Cmd {
            input: vec![path.clone().into()],
            r#type: "ScBytes".to_string(),
            input_format: InputFormat::Stream,
            output_format: OutputFormat::Json,
        }
        .run();
        std::fs::remove_file(path).unwrap();
        r.unwrap();
    }

    #[test]
    fn valid_stream_base64_file_is_accepted_by_input_size() {
        // A ScBytes value of 5 bytes, base64 encoded.
        let path = std::env::temp_dir().join(format!(
            "stellar-xdr-cli-test-{:016x}",
            rand::random::<u64>()
        ));
        std::fs::write(&path, "AAAABQUFBQUFAAAA").unwrap();
        let r = Cmd {
            input: vec![path.clone().into()],
            r#type: "ScBytes".to_string(),
            input_format: InputFormat::StreamBase64,
            output_format: OutputFormat::Json,
        }
        .run();
        std::fs::remove_file(path).unwrap();
        r.unwrap();
    }

    #[test]
    fn valid_stream_framed_file_is_accepted_by_input_size() {
        // A ScBytes value of 5 bytes, preceded by a frame header for a last fragment.
        let path = std::env::temp_dir().join(format!(
            "stellar-xdr-cli-test-{:016x}",
            rand::random::<u64>()
        ));
        std::fs::write(
            &path,
            [
                0x80, 0x00, 0x00, 0x0C, 0x00, 0x00, 0x00, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x00,
                0x00, 0x00,
            ],
        )
        .unwrap();
        let r = Cmd {
            input: vec![path.clone().into()],
            r#type: "ScBytes".to_string(),
            input_format: InputFormat::StreamFramed,
            output_format: OutputFormat::Json,
        }
        .run();
        std::fs::remove_file(path).unwrap();
        r.unwrap();
    }

    #[test]
    fn valid_single_base64_inline_is_accepted_by_input_size() {
        // A ScBytes value of 5 bytes, base64 encoded.
        let r = Cmd {
            input: vec!["AAAABQUFBQUFAAAA".into()],
            r#type: "ScBytes".to_string(),
            input_format: InputFormat::SingleBase64,
            output_format: OutputFormat::Json,
        }
        .run();
        r.unwrap();
    }

    #[test]
    fn valid_stream_base64_inline_is_accepted_by_input_size() {
        // A ScBytes value of 5 bytes, base64 encoded.
        let r = Cmd {
            input: vec!["AAAABQUFBQUFAAAA".into()],
            r#type: "ScBytes".to_string(),
            input_format: InputFormat::StreamBase64,
            output_format: OutputFormat::Json,
        }
        .run();
        r.unwrap();
    }

    #[test]
    fn multi_item_stream_file_is_accepted_by_input_size() {
        // A stream of ScBytes values of 1, 5, and 9 bytes, that together must fit
        // within the length limit derived from the size of the input.
        let path = std::env::temp_dir().join(format!(
            "stellar-xdr-cli-test-{:016x}",
            rand::random::<u64>()
        ));
        std::fs::write(
            &path,
            [
                0x00, 0x00, 0x00, 0x01, 0x01, 0x00, 0x00, 0x00, //
                0x00, 0x00, 0x00, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x00, 0x00, 0x00, //
                0x00, 0x00, 0x00, 0x09, 0x09, 0x09, 0x09, 0x09, 0x09, 0x09, 0x09, 0x09, 0x09, 0x00,
                0x00, 0x00,
            ],
        )
        .unwrap();
        let r = Cmd {
            input: vec![path.clone().into()],
            r#type: "ScBytes".to_string(),
            input_format: InputFormat::Stream,
            output_format: OutputFormat::Json,
        }
        .run();
        std::fs::remove_file(path).unwrap();
        r.unwrap();
    }

    #[test]
    fn multi_item_stream_base64_file_is_accepted_by_input_size() {
        // A stream of ScBytes values of 1, 5, and 9 bytes, that together must fit
        // within the length limit derived from the size of the input, base64 encoded.
        let path = std::env::temp_dir().join(format!(
            "stellar-xdr-cli-test-{:016x}",
            rand::random::<u64>()
        ));
        std::fs::write(&path, "AAAAAQEAAAAAAAAFBQUFBQUAAAAAAAAJCQkJCQkJCQkJAAAA").unwrap();
        let r = Cmd {
            input: vec![path.clone().into()],
            r#type: "ScBytes".to_string(),
            input_format: InputFormat::StreamBase64,
            output_format: OutputFormat::Json,
        }
        .run();
        std::fs::remove_file(path).unwrap();
        r.unwrap();
    }

    #[test]
    fn multi_item_stream_framed_file_is_accepted_by_input_size() {
        // A stream of ScBytes values of 1, 5, and 9 bytes, that together must fit
        // within the length limit derived from the size of the input, each
        // preceded by a frame header for a last fragment.
        let path = std::env::temp_dir().join(format!(
            "stellar-xdr-cli-test-{:016x}",
            rand::random::<u64>()
        ));
        std::fs::write(
            &path,
            [
                0x80, 0x00, 0x00, 0x08, //
                0x00, 0x00, 0x00, 0x01, 0x01, 0x00, 0x00, 0x00, //
                0x80, 0x00, 0x00, 0x0C, //
                0x00, 0x00, 0x00, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x00, 0x00, 0x00, //
                0x80, 0x00, 0x00, 0x10, //
                0x00, 0x00, 0x00, 0x09, 0x09, 0x09, 0x09, 0x09, 0x09, 0x09, 0x09, 0x09, 0x09, 0x00,
                0x00, 0x00,
            ],
        )
        .unwrap();
        let r = Cmd {
            input: vec![path.clone().into()],
            r#type: "ScBytes".to_string(),
            input_format: InputFormat::StreamFramed,
            output_format: OutputFormat::Json,
        }
        .run();
        std::fs::remove_file(path).unwrap();
        r.unwrap();
    }
}
