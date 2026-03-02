pub mod preprocess;

use clap::{Args, Subcommand};

use crate::cli::Channel;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Preprocess(#[from] preprocess::Error),
}

#[derive(Args, Debug, Clone)]
#[command()]
pub struct Cmd {
    #[command(subcommand)]
    pub sub: Sub,
}

#[derive(Subcommand, Clone, Debug)]
pub enum Sub {
    /// Preprocess XDR .x files by evaluating #ifdef/#ifndef/#else/#endif directives
    Preprocess(preprocess::Cmd),
}

impl Cmd {
    /// Run the CLIs xfile command.
    ///
    /// ## Errors
    ///
    /// If the sub-command is configured with state that is invalid.
    pub fn run(&self, _channel: &Channel) -> Result<(), Error> {
        match &self.sub {
            Sub::Preprocess(c) => c.run()?,
        }
        Ok(())
    }
}
