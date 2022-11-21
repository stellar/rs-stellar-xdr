#![cfg(feature = "cli")]

use std::{
    error::Error,
    fs::File,
    io::{stdin, ErrorKind, Read},
    path::PathBuf,
};

use clap::{builder::TypedValueParser, Parser, ValueEnum};
use stellar_xdr::{ReadXdr, Type, TypeVariant};

#[derive(Parser, Debug, Clone)]
#[command(
    author,
    version,
    about,
    long_about = None,
    disable_help_subcommand = true,
    disable_version_flag = true,
)]
struct Config {
    /// Files to stream
    #[arg()]
    files: Vec<PathBuf>,

    /// XDR type to decode
    #[clap(long, value_enum, value_parser
        = clap::builder::PossibleValuesParser::new(TypeVariant::VARIANTS_STR).map(|s| s.parse::<TypeVariant>().unwrap()))]
    r#type: TypeVariant,

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
        Self::SingleBase64
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, ValueEnum)]
pub enum OutputFormat {
    Json,
    JsonFormatted,
}

impl Default for OutputFormat {
    fn default() -> Self {
        Self::JsonFormatted
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::parse();

    let mut files: Vec<Box<dyn Read>> = if config.files.is_empty() {
        vec![Box::new(stdin())]
    } else {
        config
            .files
            .iter()
            .map(File::open)
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .map(|f| -> Box<dyn Read> { Box::new(f) })
            .collect()
    };

    let out = match config.output {
        OutputFormat::Json => |t| {
            let j = serde_json::to_string(&t)?;
            println!("{}", j);
            Ok::<_, Box<dyn Error>>(())
        },
        OutputFormat::JsonFormatted => |t| {
            let j = serde_json::to_string_pretty(&t)?;
            println!("{}", j);
            Ok::<_, Box<dyn Error>>(())
        },
    };

    for f in &mut files {
        match config.input {
            InputFormat::SingleBase64 => {
                let t = Type::read_xdr_base64_to_end(config.r#type, f)?;
                out(t)?;
            }
            InputFormat::Single => {
                let t = Type::read_xdr(config.r#type, f)?;
                out(t)?;
            }
            InputFormat::Stream => todo!(),
            InputFormat::StreamBase64 => todo!(),
            InputFormat::StreamFramed => loop {
                match u32::read_xdr(f) {
                    Ok(_) => (),
                    Err(stellar_xdr::Error::Io(e)) if e.kind() == ErrorKind::UnexpectedEof => break,
                    Err(e) => Err(e)?,
                };
                let t = Type::read_xdr(config.r#type, f)?;
                out(t)?;
            },
        };
    }

    Ok(())
}
