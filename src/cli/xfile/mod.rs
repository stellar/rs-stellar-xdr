pub mod ast;
pub mod generate_rust;
pub mod preprocess;

use clap::{Args, Subcommand};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Preprocess(#[from] preprocess::Error),
    #[error("{0}")]
    GenerateRust(#[from] generate_rust::Error),
    #[error("{0}")]
    Ast(#[from] ast::Error),
}

#[derive(Args, Debug, Clone)]
#[command()]
pub struct Cmd {
    #[command(subcommand)]
    pub sub: Sub,
}

#[derive(Subcommand, Clone, Debug)]
pub enum Sub {
    /// Preprocess XDR .x files by evaluating #ifdef/#ifndef/#elif/#else/#endif directives
    Preprocess(preprocess::Cmd),
    /// Generate Rust code from XDR .x files
    GenerateRust(generate_rust::Cmd),
    /// Emit parsed XDR definitions as a JSON AST
    Ast(ast::Cmd),
}

impl Cmd {
    /// Run the CLIs xfile command.
    ///
    /// ## Errors
    ///
    /// If the sub-command is configured with state that is invalid.
    pub fn run(&self) -> Result<(), Error> {
        match &self.sub {
            Sub::Preprocess(c) => c.run()?,
            Sub::GenerateRust(c) => c.run()?,
            Sub::Ast(c) => c.run()?,
        }
        Ok(())
    }
}
