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
    xdr: if cfg!(all(feature = "curr", feature = "next")) {
        "curr,next"
    } else if cfg!(feature = "curr") {
        "curr"
    } else if cfg!(feature = "next") {
        "next"
    } else {
        ""
    },
    xdr_curr: env!("XDR_CURR_VERSION"),
    xdr_next: env!("XDR_NEXT_VERSION"),
};

#[cfg(feature = "curr")]
pub mod curr;
#[cfg(all(feature = "curr", not(feature = "next")))]
pub use curr::*;

#[cfg(feature = "next")]
pub mod next;
#[cfg(all(not(feature = "curr"), feature = "next"))]
pub use next::*;
