use std::{
    error::Error,
    fs::File,
    io::{stdin, Read},
    path::PathBuf,
};

use clap::{
    builder::{PossibleValuesParser, TypedValueParser},
    Parser, ValueEnum,
};
use stellar_xdr::{Type, TypeVariant};

#[derive(Parser, Debug, Clone)]
#[command()]
pub struct Cmd {
    /// Files to decode, or stdin if omitted
    #[arg()]
    files: Vec<PathBuf>,

    /// XDR type to decode
    #[clap(long, value_enum, value_parser = PossibleValuesParser::new(TypeVariant::VARIANTS_STR).map(|s| s.parse::<TypeVariant>().unwrap()))]
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
        Self::Json
    }
}

impl Cmd {
    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        let mut files = self.files()?;

        for f in &mut files {
            match self.input {
                InputFormat::Single => {
                    let t = Type::read_xdr_to_end(self.r#type, f)?;
                    self.out(&t)?;
                }
                InputFormat::SingleBase64 => {
                    let t = Type::read_xdr_base64_to_end(self.r#type, f)?;
                    self.out(&t)?;
                }
                InputFormat::Stream => {
                    for t in Type::read_xdr_iter(self.r#type, f) {
                        self.out(&t?)?;
                    }
                }
                InputFormat::StreamBase64 => {
                    for t in Type::read_xdr_base64_iter(self.r#type, f) {
                        self.out(&t?)?;
                    }
                }
                InputFormat::StreamFramed => {
                    for t in Type::read_xdr_framed_iter(self.r#type, f) {
                        self.out(&t?)?;
                    }
                }
            };
        }

        Ok(())
    }

    fn files(&self) -> Result<Vec<Box<dyn Read>>, Box<dyn Error>> {
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

    fn out(&self, v: &Type) -> Result<(), Box<dyn Error>> {
        match self.output {
            OutputFormat::Json => println!("{}", serde_json::to_string(v)?),
            OutputFormat::JsonFormatted => println!("{}", serde_json::to_string_pretty(v)?),
        }
        Ok(())
    }
}
