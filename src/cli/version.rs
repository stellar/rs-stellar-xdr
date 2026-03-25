use clap::Parser;

use crate::VERSION;

#[derive(Parser, Debug, Clone)]
#[command()]
pub struct Cmd;

impl Cmd {
    pub fn run() {
        let v = VERSION;
        println!("stellar-xdr {} ({})\nxdr: {}", v.pkg, v.rev, v.xdr);
    }
}
