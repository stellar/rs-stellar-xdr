//! The crate's integration tests, as one test binary.
//!
//! Each module below was its own file in `tests/`, and so its own binary that
//! linked the whole crate. Building them as modules of a single binary links
//! the crate once instead of once per file.

// Only `const_writer` uses these values, so the module is gated the same way it
// is, to keep it out of builds that do not compile it.
#[cfg(all(feature = "const", feature = "std"))]
mod common;

mod account_conversions;
mod arbitrary;
mod const_types;
mod const_writer;
mod default;
mod ledgerkey_to_key;
mod serde;
mod serde_ints;
mod serde_tx;
mod serde_tx_schema;
mod str;
mod stringm;
mod tx_auths;
mod tx_base64_skip_whitespace;
mod tx_debug_display;
mod tx_hash;
mod tx_prot18;
mod tx_read_edge_cases;
mod tx_small;
mod vecm;
mod version;
