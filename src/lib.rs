#![forbid(unsafe_code)]
#![cfg_attr(not(feature = "std"), no_std)]

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct Version<'a> {
    pub pkg: &'a str,
    pub rev: &'a str,
    pub xdr: &'a str,
}

pub const VERSION: Version = Version {
    pkg: env!("CARGO_PKG_VERSION"),
    rev: env!("GIT_REVISION"),
    xdr: {
        #[cfg(not(feature = "next"))]
        {
            env!("XDR_CURR_VERSION")
        }
        #[cfg(feature = "next")]
        {
            env!("XDR_NEXT_VERSION")
        }
    },
};

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
