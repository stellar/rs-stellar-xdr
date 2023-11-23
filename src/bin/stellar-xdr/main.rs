use clap::Error;
use stellar_xdr::cli;

fn main() {
    if let Err(e) = cli::run() {
        Error::raw(clap::error::ErrorKind::ValueValidation, e).exit();
    }
}
