#![forbid(unsafe_code)]
#![cfg_attr(not(feature = "std"), no_std)]

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Version<'a> {
    pub pkg: &'a str,
    pub rev: &'a str,
    pub xdr: &'a str,
    pub xdr_curr: &'a str,
    pub xdr_next: &'a str,
}

pub const VERSION: Version = Version {
    pkg: env!("CARGO_PKG_VERSION"),
    rev: env!("GIT_REVISION"),
    xdr: if cfg!(feature = "next") {
        "next"
    } else {
        "curr"
    },
    xdr_curr: env!("XDR_CURR_VERSION"),
    xdr_next: env!("XDR_NEXT_VERSION"),
};

#[cfg(not(any(feature = "curr", feature = "next")))]
pub mod curr;
#[cfg(not(any(feature = "curr", feature = "next")))]
pub mod next;

#[cfg(feature = "curr")]
mod curr;
#[cfg(feature = "curr")]
pub use curr::*;

#[cfg(feature = "next")]
mod next;
#[cfg(feature = "next")]
pub use next::*;
