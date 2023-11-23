use clap::Error;
use std::env;
use stellar_xdr::cli;

fn main() {
    if let Err(e) = cli::run(env::args_os()) {
        // TODO: Don't treat the 'help' case as an error.
        // TODO: Remove Box<dyn Error> and replace with concrete errors.
        Error::raw(clap::error::ErrorKind::ValueValidation, e).exit();
    }
}
