use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command()]
pub struct Cmd;

impl Cmd {
    pub fn run() {
        println!("{:#?}", stellar_xdr::VERSION);
    }
}
