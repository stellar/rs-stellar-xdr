use std::collections::{HashMap, HashSet};

/// Rust-specific configuration options for code generation.
#[derive(Debug, Clone, Default)]
pub struct RustOptions {
    /// Types that have custom Default implementations (skip derive(Default))
    pub custom_default_impl: HashSet<String>,
    /// Types that have custom FromStr/Display implementations (use `SerializeDisplay`)
    pub custom_str_impl: HashSet<String>,
    /// Types that should NOT have Display/FromStr/schemars generated
    pub no_display_fromstr: HashSet<String>,
    /// Minimum lengths for variable-length opaque struct members, keyed by
    /// `Type.member`, for minimums the XDR definitions do not express.
    pub min_len: HashMap<String, u32>,
}
