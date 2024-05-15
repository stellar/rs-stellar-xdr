pub mod list;

use clap::{Args, Subcommand};

use crate::cli::Channel;

#[derive(Args, Debug, Clone)]
#[command()]
pub struct Cmd {
    #[command(subcommand)]
    sub: Sub,
}

#[derive(Subcommand, Clone, Debug)]
enum Sub {
    List(list::Cmd),
}

impl Cmd {
    /// Run the CLIs types command.
    ///
    /// ## Panics
    ///
    /// If the lists sub-command panics.
    pub fn run(&self, channel: &Channel) {
        match &self.sub {
            Sub::List(c) => c.run(channel),
        }
    }
}
