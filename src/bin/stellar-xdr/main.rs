mod decode;
mod types;
mod version;

use std::error::Error;

use clap::{Parser, Subcommand, ValueEnum};

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
enum Cmd {
    /// View information about types
    Types(types::Cmd),
    /// Decode XDR
    Decode(decode::Cmd),
    /// Print version information
    Version,
}

fn main() -> Result<(), Box<dyn Error>> {
    let root = Root::parse();

    match root.cmd {
        Cmd::Types(c) => c.run(&root.channel)?,
        Cmd::Decode(c) => c.run(&root.channel)?,
        Cmd::Version => version::Cmd::run(),
    }

    Ok(())
}
