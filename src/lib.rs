#![forbid(unsafe_code)]
#![cfg_attr(not(feature = "std"), no_std)]

// TODO: Concatenate these into a single string, and also include XDR_VERSION.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const GIT_REVISION: &str = env!("GIT_REVISION");
#[must_use]
pub const fn version() -> &'static [&'static str] {
    &[VERSION, GIT_REVISION, XDR_VERSION]
}

#[cfg(not(feature = "next"))]
mod version_curr;
#[cfg(not(feature = "next"))]
pub use version_curr::*;

#[cfg(feature = "next")]
mod version_next;
#[cfg(feature = "next")]
pub use version_next::*;

#[cfg(not(feature = "next"))]
mod curr;
#[cfg(not(feature = "next"))]
pub use curr::*;

#[cfg(feature = "next")]
mod next;
#[cfg(feature = "next")]
pub use next::*;

#[cfg(feature = "next")]
mod scval_conversions;
#[cfg(feature = "next")]
pub use scval_conversions::*;

#[cfg(feature = "next")]
mod scval_validations;
#[cfg(feature = "next")]
pub use scval_validations::*;

#[cfg(all(feature = "alloc", feature = "next"))]
mod scmap;
#[cfg(all(feature = "alloc", feature = "next"))]
pub use scmap::*;
