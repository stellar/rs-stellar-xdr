#![forbid(unsafe_code)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docs, feature(doc_cfg))]
// TODO: Remove these clippy doc comment allows after improving the
// auto-generated docs.
#![allow(clippy::tabs_in_doc_comments)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(unused_attributes)]

//! Library and CLI containing types and functionality for working with Stellar
//! XDR.
//!
//! Types are generated from XDR definitions hosted at [stellar/stellar-xdr]
//! using the Rust XDR generator in `xdr-generator-rust/`.
//!
//! [stellar/stellar-xdr]: https://github.com/stellar/stellar-xdr
//!
//! ## Usage
//!
//! ### Library
//! To use the library, include in your toml:
//!
//! ```toml
//! stellar-xdr = { version = "...", default-features = true, features = [] }
//! ```
//!
//! #### Features
//!
//! The crate has several features, tiers of functionality, and ancillary
//! functionality.
//!
//! Default features: `std`.
//!
//! Tiers of functionality:
//!
//! 1. `std` – The std feature provides all functionality (types, encode,
//! decode), and is the default feature set.
//! 2. `alloc` – The alloc feature uses `Box` and `Vec` types for recursive
//! references and arrays, and is automatically enabled if the std feature is
//! enabled. The default global allocator is used. Support for a custom
//! allocator will be added in [#39]. No encode or decode capability exists,
//! only types. Encode and decode capability will be added in [#46].
//! 3. If std or alloc are not enabled recursive and array types requires static
//! lifetime values. No encode or decode capability exists. Encode and decode
//! capability will be added in [#47].
//!
//! [#39]: https://github.com/stellar/rs-stellar-xdr/issues/39
//! [#46]: https://github.com/stellar/rs-stellar-xdr/issues/46
//! [#47]: https://github.com/stellar/rs-stellar-xdr/issues/47
//!
//! Ancillary functionality:
//!
//! 1. `type_enum` – Enables the generated dynamic `Type` and `TypeVariant`
//! APIs for runtime-selected XDR decoding, encoding, schema generation.
//! 2. `base64` – Enables support for base64 encoding and decoding.
//! 3. `serde` – Enables support for serializing and deserializing types with
//! the serde crate.
//! 4. `serde_json` – Enables support for built-in functionality specifically
//! for serde_json. Often not required to use the types with serde_json, and
//! only necessary to use utility functions that depend on serde_json.
//! 5. `arbitrary` – Enables support for interop with the arbitrary crate.
//! 6. `hex` – Enables support for hex in string representations of some types.
//! Automatically enabled when serde is enabled.
//! 7. `schemars` – Enables support for JSON Schema generation. (Experimental)
//!
//! Features marked experimental may disappear at anytime, see breaking changes
//! at anytime, or and may be minimal implementations instead of complete.
//!
//! ### CLI
//!
//! To use the CLI:
//!
//! ```console
//! cargo install --locked stellar-xdr --version ... --features cli
//! ```
//!
//! #### Examples
//!
//! Parse a `TransactionEnvelope`:
//! ```console
//! stellar-xdr decode --type TransactionEnvelope << -
//! AAAAA...
//! -
//! ```
//!
//! Parse a `ScSpecEntry` stream from a contract:
//! ```console
//! stellar-xdr decode --type ScSpecEntry --input stream-base64 --output json-formatted << -
//! AAAAA...
//! -
//! ```
//!
//! Parse a `BucketEntry` framed stream from a bucket file:
//! ```console
//! stellar-xdr decode --type BucketEntry --input stream-framed --output json-formatted bucket.xdr
//! ```

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Version<'a> {
    pub pkg: &'a str,
    pub rev: &'a str,
    pub xdr: &'a str,
}
pub const VERSION: Version = Version {
    pkg: env!("CARGO_PKG_VERSION"),
    rev: env!("GIT_REVISION"),
    xdr: include_str!("../xdr-version"),
};

#[cfg(feature = "schemars")]
pub mod schemars;

#[allow(clippy::empty_line_after_doc_comments)]
#[allow(clippy::derivable_impls, clippy::len_zero)]
mod generated;
mod ledgerkey;
pub use generated::*;

mod default;
mod jsonschema;
mod str;

mod scval_conversions;
pub use scval_conversions::*;
mod account_conversions;
mod transaction_conversions;

mod scval_validations;
pub use scval_validations::*;

#[cfg(feature = "alloc")]
mod scmap;

mod tx_auths;
mod tx_hash;

#[cfg(feature = "cli")]
pub mod cli;

#[cfg(feature = "alloc")]
pub(crate) mod num256;

#[cfg(feature = "alloc")]
pub(crate) mod num128;
