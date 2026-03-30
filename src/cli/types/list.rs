use clap::{Args, ValueEnum};

#[derive(Args, Debug, Clone)]
#[command()]
pub struct Cmd {
    // Output format
    #[arg(long, value_enum, default_value_t)]
    pub output: OutputFormat,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, ValueEnum)]
pub enum OutputFormat {
    #[default]
    Plain,
    Json,
    JsonFormatted,
}

impl Cmd {
    /// Run the CLIs types list command.
    ///
    /// ## Panics
    ///
    /// If the list cannot be rendered as JSON.
    pub fn run(&self) {
        let types: &[&str] = &crate::TypeVariant::VARIANTS_STR;
        let mut types: Vec<&'static str> = types.to_vec();
        types.sort_unstable();
        match self.output {
            OutputFormat::Plain => {
                for t in types {
                    println!("{t}");
                }
            }
            OutputFormat::Json => {
                println!("{}", serde_json::to_string(&types).unwrap());
            }
            OutputFormat::JsonFormatted => {
                println!("{}", serde_json::to_string_pretty(&types).unwrap());
            }
        }
    }
}
