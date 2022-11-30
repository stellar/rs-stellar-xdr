mod dec;
mod version;

use std::error::Error;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug, Clone)]
#[command(
    author,
    version,
    about,
    long_about = None,
    disable_help_subcommand = true,
    disable_version_flag = true,
)]
struct Root {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand, Debug, Clone)]
enum Cmd {
    /// Decode XDR
    Dec(dec::Cmd),
    /// Version
    Version(version::Cmd),
}

fn main() -> Result<(), Box<dyn Error>> {
    let root = Root::parse();

    match root.cmd {
        Cmd::Dec(c) => c.run()?,
        Cmd::Version(_) => version::Cmd::run(),
    }

    Ok(())
}
