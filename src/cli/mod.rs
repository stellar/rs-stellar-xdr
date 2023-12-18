mod decode;
mod encode;
mod guess;
mod types;
mod version;

use clap::{Parser, Subcommand, ValueEnum};
use std::{ffi::OsString, fmt::Debug};

#[derive(Parser, Debug, Clone)]
#[command(
    author,
    version,
    about,
    long_about = None,
    disable_help_subcommand = true,
    disable_version_flag = true,
    disable_colored_help = true,
    infer_subcommands = true,
)]
pub struct Root {
    /// Channel of XDR to operate on
    #[arg(value_enum, default_value_t)]
    channel: Channel,
    #[command(subcommand)]
    cmd: Cmd,
}

impl Root {
    /// Run the CLIs root command.
    ///
    /// ## Errors
    ///
    /// If the root command is configured with state that is invalid.
    pub fn run(&self) -> Result<(), Error> {
        match &self.cmd {
            Cmd::Types(c) => c.run(&self.channel),
            Cmd::Guess(c) => c.run(&self.channel)?,
            Cmd::Decode(c) => c.run(&self.channel)?,
            Cmd::Encode(c) => c.run(&self.channel)?,
            Cmd::Version => version::Cmd::run(),
        }
        Ok(())
    }
}

#[derive(ValueEnum, Debug, Clone)]
pub enum Channel {
    #[value(name = "+curr")]
    Curr,
    #[value(name = "+next")]
    Next,
}

impl Default for Channel {
    fn default() -> Self {
        Self::Curr
    }
}

#[derive(Subcommand, Debug, Clone)]
pub enum Cmd {
    /// View information about types
    Types(types::Cmd),
    /// Guess the XDR type
    Guess(guess::Cmd),
    /// Decode XDR
    Decode(decode::Cmd),
    /// Encode XDR
    Encode(encode::Cmd),
    /// Print version information
    Version,
}

#[derive(thiserror::Error, Debug)]
#[allow(clippy::enum_variant_names)]
pub enum Error {
    #[error("{0}")]
    Clap(#[from] clap::Error),
    #[error("error decoding XDR: {0}")]
    Guess(#[from] guess::Error),
    #[error("error reading file: {0}")]
    Decode(#[from] decode::Error),
    #[error("error reading file: {0}")]
    Encode(#[from] encode::Error),
}

/// Run the CLI with the given args.
///
/// ## Errors
///
/// If the input cannot be parsed.
pub fn run<I, T>(args: I) -> Result<(), Error>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    let root = Root::try_parse_from(args)?;
    root.run()
}
