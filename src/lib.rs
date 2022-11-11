#![forbid(unsafe_code)]
#![cfg_attr(not(feature = "std"), no_std)]

const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
const GIT_REVISION: &str = env!("GIT_REVISION");
pub const VERSION: &str =
    const_format::formatcp!("{PKG_VERSION} ({GIT_REVISION}) (xdr: {XDR_VERSION})");

#[cfg(not(feature = "next"))]
mod curr_xdr_version;
#[cfg(not(feature = "next"))]
use curr_xdr_version::XDR_VERSION;

#[cfg(feature = "next")]
mod next_xdr_version;
#[cfg(feature = "next")]
use next_xdr_version::XDR_VERSION;

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
