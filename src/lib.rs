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
    xdr_curr: env!("XDR_CURR_VERSION"),
    xdr_next: env!("XDR_NEXT_VERSION"),
};

pub mod curr;
pub mod next;

#[cfg(not(feature = "next"))]
pub use curr::*;

#[cfg(feature = "next")]
pub use next::*;
