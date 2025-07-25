pub mod list;
pub mod schema;
pub mod schema_files;

use clap::{Args, Subcommand};

use crate::cli::Channel;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    SchemaError(#[from] schema::Error),
    #[error("{0}")]
    SchemaFilesError(#[from] schema_files::Error),
}

#[derive(Args, Debug, Clone)]
#[command()]
pub struct Cmd {
    #[command(subcommand)]
    pub sub: Sub,
}

#[derive(Subcommand, Clone, Debug)]
pub enum Sub {
    List(list::Cmd),
    Schema(schema::Cmd),
    SchemaFiles(schema_files::Cmd),
}

impl Cmd {
    /// Run the CLIs types command.
    ///
    /// ## Errors
    ///
    /// If the sub-command panics.
    ///
    /// ## Panics
    ///
    /// If the sub-command panics.
    pub fn run(&self, channel: &Channel) -> Result<(), Error> {
        match &self.sub {
            Sub::List(c) => c.run(channel),
            Sub::Schema(c) => c.run(channel)?,
            Sub::SchemaFiles(c) => c.run(channel)?,
        }
        Ok(())
    }
}
