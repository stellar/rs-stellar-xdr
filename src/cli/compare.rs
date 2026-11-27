use std::{
    fmt::Debug,
    fs::File,
    io::{stdout, Write},
    path::PathBuf,
    str::FromStr,
};

use clap::{Args, ValueEnum};

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
}

/// Compare two XDR values with each other
///
/// Outputs:
///   `-1` when the left XDR value is less than the right XDR value,
///   `0` when the left XDR value is equal to the right XDR value,
///   `1` when the left XDR value is greater than the right XDR value
#[derive(Args, Debug, Clone)]
#[command()]
pub struct Cmd {
    /// XDR file to decode and compare with the right value
    #[arg()]
    pub left: PathBuf,

    /// XDR file to decode and compare with the left value
    #[arg()]
    pub right: PathBuf,

    /// XDR type of both inputs
    #[arg(long)]
    pub r#type: String,

    // Input format of the XDR
    #[arg(long, value_enum, default_value_t)]
    pub input: InputFormat,
}

#[derive(Default, Clone, Copy, Debug, Eq, Hash, PartialEq, ValueEnum)]
pub enum InputFormat {
    Single,
    #[default]
    SingleBase64,
}

// TODO: Remove run_x macro, it exists only to reduce the diff from when curr/next
// channels existed and each had their own run_curr/run_next invocation.
macro_rules! run_x {
    ($f:ident) => {
        fn $f(&self) -> Result<(), Error> {
            let f1 = File::open(&self.left).map_err(Error::ReadFile)?;
            let f2 = File::open(&self.right).map_err(Error::ReadFile)?;
            let l1 = util::file_limits(&f1);
            let l2 = util::file_limits(&f2);
            let r#type = crate::TypeVariant::from_str(&self.r#type).map_err(|_| {
                Error::UnknownType(self.r#type.clone(), &crate::TypeVariant::VARIANTS_STR)
            })?;
            let (t1, t2) = match self.input {
                InputFormat::Single => {
                    let t1 = {
                        let mut l1 = crate::Limited::new(f1, l1);
                        crate::Type::read_xdr_to_end(r#type, &mut l1)?
                    };
                    let t2 = {
                        let mut l = crate::Limited::new(f2, l2);
                        crate::Type::read_xdr_to_end(r#type, &mut l)?
                    };
                    (t1, t2)
                }
                InputFormat::SingleBase64 => {
                    let t1 = {
                        let mut l = crate::Limited::new(f1, l1);
                        crate::Type::read_xdr_base64_to_end(r#type, &mut l)?
                    };
                    let t2 = {
                        let mut l = crate::Limited::new(f2, l2);
                        crate::Type::read_xdr_base64_to_end(r#type, &mut l)?
                    };
                    (t1, t2)
                }
            };
            let cmp = t1.cmp(&t2) as i8;
            writeln!(stdout(), "{cmp}").map_err(Error::WriteOutput)?;
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn huge_length_in_file_is_rejected_by_input_size() {
        // A ScBytes value whose length prefix declares 0xFFFFFFF0 bytes but
        // that is followed by no data.
        let path = std::env::temp_dir().join(format!(
            "stellar-xdr-cli-test-{:016x}",
            rand::random::<u64>()
        ));
        std::fs::write(&path, [0xFF, 0xFF, 0xFF, 0xF0]).unwrap();
        let r = Cmd {
            left: path.clone(),
            right: path.clone(),
            r#type: "ScBytes".to_string(),
            input: InputFormat::Single,
        }
        .run();
        std::fs::remove_file(path).unwrap();
        assert!(
            matches!(r, Err(Error::ReadXdr(crate::Error::LengthLimitExceeded))),
            "{r:?}"
        );
    }

    #[test]
    fn valid_single_file_is_accepted_by_input_size() {
        // ScBytes values of 1 and 5 bytes, that exactly fill the length limit
        // derived from the size of each file.
        let left = std::env::temp_dir().join(format!(
            "stellar-xdr-cli-test-{:016x}",
            rand::random::<u64>()
        ));
        std::fs::write(&left, [0x00, 0x00, 0x00, 0x01, 0x01, 0x00, 0x00, 0x00]).unwrap();
        let right = std::env::temp_dir().join(format!(
            "stellar-xdr-cli-test-{:016x}",
            rand::random::<u64>()
        ));
        std::fs::write(
            &right,
            [
                0x00, 0x00, 0x00, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x00, 0x00, 0x00,
            ],
        )
        .unwrap();
        let r = Cmd {
            left: left.clone(),
            right: right.clone(),
            r#type: "ScBytes".to_string(),
            input: InputFormat::Single,
        }
        .run();
        std::fs::remove_file(left).unwrap();
        std::fs::remove_file(right).unwrap();
        r.unwrap();
    }

    #[test]
    fn valid_single_base64_file_is_accepted_by_input_size() {
        // ScBytes values of 1 and 5 bytes base64 encoded, that fit within the
        // length limit derived from the size of each file.
        let left = std::env::temp_dir().join(format!(
            "stellar-xdr-cli-test-{:016x}",
            rand::random::<u64>()
        ));
        std::fs::write(&left, "AAAAAQEAAAA=").unwrap();
        let right = std::env::temp_dir().join(format!(
            "stellar-xdr-cli-test-{:016x}",
            rand::random::<u64>()
        ));
        std::fs::write(&right, "AAAABQUFBQUFAAAA").unwrap();
        let r = Cmd {
            left: left.clone(),
            right: right.clone(),
            r#type: "ScBytes".to_string(),
            input: InputFormat::SingleBase64,
        }
        .run();
        std::fs::remove_file(left).unwrap();
        std::fs::remove_file(right).unwrap();
        r.unwrap();
    }
}
