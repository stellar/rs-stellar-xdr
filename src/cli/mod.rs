mod decode;
mod encode;
mod guess;
mod types;
mod version;

use clap::{Parser, Subcommand, ValueEnum};
use std::{error::Error, fmt::Debug};

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
struct Root {
    /// Channel of XDR to operate on
    #[arg(value_enum, default_value_t)]
    channel: Channel,
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(ValueEnum, Debug, Clone)]
pub(crate) enum Channel {
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
enum Cmd {
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

/// Run the CLI.
///
/// TODO: How to pass in input to run from other CLI?
///
/// ## Errors
///
/// If the input cannot be parsed.
pub fn run() -> Result<(), Box<dyn Error>> {
    let root = Root::try_parse()?;
    match root.cmd {
        Cmd::Types(c) => c.run(&root.channel)?,
        Cmd::Guess(c) => c.run(&root.channel)?,
        Cmd::Decode(c) => c.run(&root.channel)?,
        Cmd::Encode(c) => c.run(&root.channel)?,
        Cmd::Version => version::Cmd::run(),
    }
    Ok(())
}
