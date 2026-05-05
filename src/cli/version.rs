use clap::Parser;

use crate::VERSION;

#[derive(Parser, Debug, Clone)]
#[command()]
pub struct Cmd;

impl Cmd {
    pub fn run() {
        let v = VERSION;
        let features = match v.features {
            [] => "<none>".to_string(),
            fs => fs.join(", "),
        };
        println!(
            "stellar-xdr {} ({})\nxdr: {}\nfeatures: {}",
            v.pkg, v.rev, v.xdr, features,
        );
    }
}
