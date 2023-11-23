mod decode;
mod encode;
mod guess;
mod types;
mod version;

use clap::{Parser, Subcommand, ValueEnum};
use std::{error::Error, ffi::OsString, fmt::Debug};

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

/// Run the CLI with the given args.
///
/// ## Errors
///
/// If the input cannot be parsed.
pub fn run<I, T>(args: I) -> Result<(), Box<dyn Error>>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    let root = Root::try_parse_from(args)?;
    match root.cmd {
        Cmd::Types(c) => c.run(&root.channel)?,
        Cmd::Guess(c) => c.run(&root.channel)?,
        Cmd::Decode(c) => c.run(&root.channel)?,
        Cmd::Encode(c) => c.run(&root.channel)?,
        Cmd::Version => version::Cmd::run(),
    }
    Ok(())
}
