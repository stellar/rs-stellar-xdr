mod list;

use std::error::Error;

use clap::{Args, Subcommand};

use crate::cli::Channel;

#[derive(Args, Debug, Clone)]
#[command()]
pub struct Cmd {
    #[command(subcommand)]
    sub: Sub,
}

#[derive(Subcommand, Clone, Debug)]
pub enum Sub {
    List(list::Cmd),
}

impl Cmd {
    pub fn run(&self, channel: &Channel) -> Result<(), Box<dyn Error>> {
        match &self.sub {
            Sub::List(c) => c.run(channel)?,
        }
        Ok(())
    }
}
