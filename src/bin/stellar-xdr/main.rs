use clap::Error;
use std::env;
use stellar_xdr::cli;

fn main() {
    if let Err(e) = cli::run(env::args_os()) {
        match e {
            cli::Error::Clap(e) => e.exit(),
            cli::Error::Types(_)
            | cli::Error::Guess(_)
            | cli::Error::Decode(_)
            | cli::Error::Encode(_)
            | cli::Error::Compare(_) => {
                Error::raw(clap::error::ErrorKind::ValueValidation, e).exit()
            }
        }
    }
}
