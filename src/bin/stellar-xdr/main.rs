use stellar_xdr::cli;

fn main() {
    if let Err(e) = cli::run() {
        Root::command()
            .error(clap::error::ErrorKind::ValueValidation, e)
            .exit()
    }
}
