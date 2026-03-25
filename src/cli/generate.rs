pub mod arbitrary;
pub mod default;

use clap::{Args, Subcommand};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Default(#[from] default::Error),
    #[error("{0}")]
    Arbitrary(#[from] arbitrary::Error),
}

/// Generate XDR values
#[derive(Args, Debug, Clone)]
#[command()]
pub struct Cmd {
    #[command(subcommand)]
    pub sub: Sub,
}

#[derive(Subcommand, Clone, Debug)]
pub enum Sub {
    Default(default::Cmd),
    Arbitrary(arbitrary::Cmd),
}

impl Cmd {
    /// Run the CLIs generate command.
    ///
    /// ## Errors
    ///
    /// If the sub-command panics.
    ///
    /// ## Panics
    ///
    /// If the sub-command panics.
    pub fn run(&self) -> Result<(), Error> {
        match &self.sub {
            Sub::Default(c) => c.run()?,
            Sub::Arbitrary(c) => c.run()?,
        }
        Ok(())
    }
}
