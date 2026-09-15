use arbitrary::Unstructured;
use clap::{Args, ValueEnum};
use std::{
    ffi::{OsStr, OsString},
    fs,
    io::{stdin, stdout, Read, Write},
    str::FromStr,
};

use crate::cli::util;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("unknown type {0}, choose one of {1:?}")]
    UnknownType(String, &'static [&'static str]),
    #[error("error reading file: {0}")]
    ReadFile(#[from] std::io::Error),
    #[error("error generating XDR: {0}")]
    WriteXdr(#[from] crate::Error),
    #[error("error generating JSON: {0}")]
    GenerateJson(#[from] serde_json::Error),
    #[error("error generating arbitrary value: {0}")]
    Arbitrary(#[from] arbitrary::Error),
    #[error("type doesn't have a text representation, use 'json' as output")]
    TextUnsupported,
    #[error("could not generate a value whose JSON contains all of {hints:?} within {attempts} attempts")]
    HintNotFound { hints: Vec<String>, attempts: u64 },
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

    /// Keep generating values until the value's JSON representation contains
    /// this string. Useful for hinting toward a particular union/enum variant
    /// or sub-value. Repeat the flag to require several substrings; all of
    /// them must be present in the same value.
    #[arg(long)]
    pub hint: Vec<String>,

    /// Maximum number of generation attempts when --hint is set before giving
    /// up.
    #[arg(long, default_value_t = 20_000)]
    pub hint_attempts: u64,

    /// Drive generation with the bytes in this file, or stdin if '-', instead
    /// of with fresh randomness. Bytes are consumed the same way the fuzz
    /// targets consume them, so a fuzz corpus entry piped in here produces the
    /// value that entry stands for.
    #[arg(long, conflicts_with = "hint")]
    pub entropy: Option<OsString>,
}

#[derive(Default, Clone, Copy, Debug, Eq, Hash, PartialEq, ValueEnum)]
pub enum OutputFormat {
    Single,
    #[default]
    SingleBase64,
    // TODO: Stream,
    // TODO: StreamBase64,
    // TODO: StreamFramed,
    Json,
    JsonFormatted,
    Text,
}

// TODO: Remove run_x macro, it exists only to reduce the diff from when curr/next
// channels existed and each had their own run_curr/run_next invocation.
macro_rules! run_x {
    ($f:ident) => {
        fn $f(&self) -> Result<(), Error> {
            use crate::WriteXdr;
            let r#type = crate::TypeVariant::from_str(&self.r#type).map_err(|_| {
                Error::UnknownType(self.r#type.clone(), &crate::TypeVariant::VARIANTS_STR)
            })?;
            let (v, hint_json) = self.generate(r#type)?;
            match self.output_format {
                OutputFormat::Single => {
                    let l = crate::Limits::none();
                    stdout().write_all(&v.to_xdr(l)?)?
                }
                OutputFormat::SingleBase64 => {
                    let l = crate::Limits::none();
                    println!("{}", v.to_xdr_base64(l)?)
                }
                OutputFormat::Json => match hint_json {
                    Some(json) => println!("{json}"),
                    None => println!("{}", serde_json::to_string(&v)?),
                },
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
    pub fn run(&self) -> Result<(), Error> {
        self.run_inner()
    }

    run_x!(run_inner);

    /// Generate a single arbitrary value of the given type from fresh random
    /// bytes.
    fn generate_one(type_: crate::TypeVariant) -> Result<crate::Type, Error> {
        let r = rand::random::<[u8; 10_240]>();
        let mut u = Unstructured::new(&r);
        Ok(crate::Type::arbitrary(type_, &mut u)?)
    }

    /// Read the bytes that drive generation, from a file or from stdin if the
    /// path is `-`.
    fn read_entropy(path: &OsStr) -> Result<Vec<u8>, Error> {
        if path == "-" {
            let mut bytes = Vec::new();
            stdin().read_to_end(&mut bytes)?;
            Ok(bytes)
        } else {
            Ok(fs::read(path)?)
        }
    }

    /// Generate an arbitrary value of the given type.
    ///
    /// If `entropy` is configured the value is built from those bytes rather
    /// than from randomness, which makes the output a function of the input
    /// alone. Hints are meaningless there — the bytes decide the value — and
    /// the two flags conflict.
    ///
    /// If any `hint`s are configured, values are generated repeatedly (up to
    /// `hint_attempts` times) until one whose JSON representation contains all
    /// of the hints is found. In that case the matching value's compact JSON,
    /// already computed to test the hints, is returned alongside the value so
    /// callers can reuse it instead of serializing again. When no hints are
    /// configured no JSON is computed and `None` is returned.
    ///
    /// An attempt that fails to generate or serialize is treated as unlucky
    /// rather than fatal: it is skipped and the search continues with fresh
    /// randomness, so a single bad draw doesn't abort the whole search.
    fn generate(&self, type_: crate::TypeVariant) -> Result<(crate::Type, Option<String>), Error> {
        if let Some(path) = &self.entropy {
            let bytes = Self::read_entropy(path)?;
            let mut u = Unstructured::new(&bytes);
            return Ok((crate::Type::arbitrary(type_, &mut u)?, None));
        }
        if self.hint.is_empty() {
            return Ok((Self::generate_one(type_)?, None));
        }
        for _ in 0..self.hint_attempts {
            let Ok(v) = Self::generate_one(type_) else {
                continue;
            };
            let Ok(json) = serde_json::to_string(&v) else {
                continue;
            };
            if self.hint.iter().all(|hint| json.contains(hint)) {
                return Ok((v, Some(json)));
            }
        }
        Err(Error::HintNotFound {
            hints: self.hint.clone(),
            attempts: self.hint_attempts,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The same bytes must always produce the same value, otherwise piping a
    /// corpus entry in wouldn't show what that entry stands for.
    #[test]
    fn entropy_is_deterministic() {
        let file = std::env::temp_dir().join("stellar-xdr-arbitrary-entropy");
        fs::write(&file, (0u8..=255).collect::<Vec<u8>>()).unwrap();
        let cmd = Cmd {
            r#type: "TimeBounds".to_string(),
            output_format: OutputFormat::Json,
            hint: vec![],
            hint_attempts: 1,
            entropy: Some(file.clone().into_os_string()),
        };
        let type_ = crate::TypeVariant::from_str("TimeBounds").unwrap();
        let (first, json) = cmd.generate(type_).unwrap();
        assert!(json.is_none());
        let (second, _) = cmd.generate(type_).unwrap();
        assert_eq!(
            serde_json::to_string(&first).unwrap(),
            serde_json::to_string(&second).unwrap(),
        );
        fs::remove_file(&file).unwrap();
    }

    #[test]
    fn hint_matches() {
        let cmd = Cmd {
            r#type: "TimeBounds".to_string(),
            output_format: OutputFormat::Json,
            hint: vec!["min_time".into()],
            hint_attempts: 1000,
            entropy: None,
        };
        let type_ = crate::TypeVariant::from_str("TimeBounds").unwrap();
        let (v, json) = cmd.generate(type_).unwrap();
        let json = json.expect("a hinted search returns the computed json");
        assert!(json.contains("min_time"));
        assert_eq!(json, serde_json::to_string(&v).unwrap());
    }

    #[test]
    fn all_hints_match() {
        let cmd = Cmd {
            r#type: "TimeBounds".to_string(),
            output_format: OutputFormat::Json,
            hint: vec!["min_time".into(), "max_time".into()],
            hint_attempts: 1000,
            entropy: None,
        };
        let type_ = crate::TypeVariant::from_str("TimeBounds").unwrap();
        let (_v, json) = cmd.generate(type_).unwrap();
        let json = json.expect("a hinted search returns the computed json");
        assert!(json.contains("min_time"));
        assert!(json.contains("max_time"));
    }

    #[test]
    fn hint_not_found() {
        let cmd = Cmd {
            r#type: "TimeBounds".to_string(),
            output_format: OutputFormat::Json,
            hint: vec!["zzz_does_not_exist_xyzzy".into()],
            hint_attempts: 5,
            entropy: None,
        };
        let type_ = crate::TypeVariant::from_str("TimeBounds").unwrap();
        assert!(matches!(
            cmd.generate(type_),
            Err(Error::HintNotFound { .. })
        ));
    }

    #[test]
    fn not_all_hints_found() {
        let cmd = Cmd {
            r#type: "TimeBounds".to_string(),
            output_format: OutputFormat::Json,
            hint: vec!["min_time".into(), "zzz_does_not_exist_xyzzy".into()],
            hint_attempts: 5,
            entropy: None,
        };
        let type_ = crate::TypeVariant::from_str("TimeBounds").unwrap();
        assert!(matches!(
            cmd.generate(type_),
            Err(Error::HintNotFound { .. })
        ));
    }

    #[test]
    fn no_hint_generates_once() {
        let cmd = Cmd {
            r#type: "TimeBounds".to_string(),
            output_format: OutputFormat::Json,
            hint: vec![],
            hint_attempts: 1,
            entropy: None,
        };
        let type_ = crate::TypeVariant::from_str("TimeBounds").unwrap();
        let (_v, json) = cmd.generate(type_).unwrap();
        assert!(json.is_none());
    }
}
