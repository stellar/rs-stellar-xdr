use clap::{Args, ValueEnum};

use crate::cli::Channel;

#[derive(Args, Debug, Clone)]
#[command()]
pub struct Cmd {
    // Output format
    #[arg(long, value_enum, default_value_t)]
    output: OutputFormat,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, ValueEnum)]
pub enum OutputFormat {
    Plain,
    Json,
    JsonFormatted,
}

impl Default for OutputFormat {
    fn default() -> Self {
        Self::Plain
    }
}

impl Cmd {
    pub fn run(&self, channel: &Channel) {
        let types = Self::types(channel);
        match self.output {
            OutputFormat::Plain => {
                for t in types {
                    println!("{t}");
                }
            }
            OutputFormat::Json => {
                println!("{}", serde_json::to_string(&types).unwrap());
            }
            OutputFormat::JsonFormatted => {
                println!("{}", serde_json::to_string_pretty(&types).unwrap());
            }
        }
    }

    fn types(channel: &Channel) -> Vec<&'static str> {
        let types: &[&str] = match channel {
            Channel::Curr => &crate::curr::TypeVariant::VARIANTS_STR,
            Channel::Next => &crate::next::TypeVariant::VARIANTS_STR,
        };
        let mut types: Vec<&'static str> = types.to_vec();
        types.sort_unstable();
        types
    }
}
