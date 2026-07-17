use std::path::PathBuf;

use clap::Args;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("error generating Rust code: {0}")]
    Generate(Box<dyn std::error::Error>),
}

/// Generate Rust code from XDR .x files
#[derive(Args, Debug, Clone)]
#[command()]
pub struct Cmd {
    /// Input XDR files
    #[arg(long, required = true)]
    pub input: Vec<PathBuf>,

    /// Output module file (e.g. src/generated.rs)
    #[arg(long)]
    pub output: PathBuf,

    /// Types with a custom Default implementation (skip derive(Default))
    #[arg(long, value_delimiter = ',')]
    pub custom_default: Vec<String>,

    /// Types with a custom FromStr/Display implementation (use SerializeDisplay)
    #[arg(long, value_delimiter = ',')]
    pub custom_str: Vec<String>,

    /// Types that should NOT have Display/FromStr/schemars generated
    #[arg(long, value_delimiter = ',')]
    pub no_display_fromstr: Vec<String>,
}

impl Cmd {
    /// Run the CLIs xfile generate-rust command.
    ///
    /// ## Errors
    ///
    /// If the input files cannot be read or parsed, or the generated output
    /// cannot be written.
    pub fn run(&self) -> Result<(), Error> {
        xdr_generator_rust::generate(
            &self.input,
            &self.output,
            &self.custom_default,
            &self.custom_str,
            &self.no_display_fromstr,
        )
        .map_err(Error::Generate)
    }
}
