use clap::{Args, ValueEnum};
use std::error::Error;

use crate::Channel;

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
    pub fn run(&self, channel: &Channel) -> Result<(), Box<dyn Error>> {
        let types = Self::types(channel);
        match self.output {
            OutputFormat::Plain => {
                for t in types {
                    println!("{t}");
                }
            }
            OutputFormat::Json => {
                println!("{}", serde_json::to_string(&types)?);
            }
            OutputFormat::JsonFormatted => {
                println!("{}", serde_json::to_string_pretty(&types)?);
            }
        }
        Ok(())
    }

    fn types(channel: &Channel) -> Vec<&'static str> {
        let types: &[&str] = match channel {
            Channel::Curr => &stellar_xdr::curr::TypeVariant::VARIANTS_STR,
            Channel::Next => &stellar_xdr::next::TypeVariant::VARIANTS_STR,
        };
        let mut types: Vec<&'static str> = types.to_vec();
        types.sort_unstable();
        types
    }
}
