use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command()]
pub struct Cmd;

impl Cmd {
    pub fn run() {
        let v = stellar_xdr::VERSION;
        println!(
            "stellar-xdr {} ({})
xdr (+curr): {}
xdr (+next): {}",
            v.pkg,
            v.rev.unwrap_or("unknown git revision"),
            v.xdr_curr,
            v.xdr_next
        );
    }
}
