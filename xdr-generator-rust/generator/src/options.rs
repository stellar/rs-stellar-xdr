use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

/// Rust-specific configuration options for code generation.
#[derive(Debug, Clone, Default)]
pub struct RustOptions {
    /// Types that have custom Default implementations (skip derive(Default))
    pub custom_default_impl: HashSet<String>,
    /// Types that have custom FromStr/Display implementations (use SerializeDisplay)
    pub custom_str_impl: HashSet<String>,
    /// Types that should NOT have Display/FromStr/schemars generated
    pub no_display_fromstr: HashSet<String>,
}

#[derive(Debug, Default, serde::Deserialize)]
struct Config {
    #[serde(default)]
    custom_default: Vec<String>,
    #[serde(default)]
    custom_str: Vec<String>,
    #[serde(default)]
    no_display_fromstr: Vec<String>,
}

impl RustOptions {
    /// Load options from a TOML config file.
    pub fn from_config_file(path: &PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config: Config = basic_toml::from_str(&content)?;
        Ok(Self {
            custom_default_impl: config.custom_default.into_iter().collect(),
            custom_str_impl: config.custom_str.into_iter().collect(),
            no_display_fromstr: config.no_display_fromstr.into_iter().collect(),
        })
    }
}
