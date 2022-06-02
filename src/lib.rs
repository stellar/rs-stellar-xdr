#![forbid(unsafe_code)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "next"))]
mod curr;
#[cfg(not(feature = "next"))]
pub use curr::*;

#[cfg(feature = "next")]
mod next;
#[cfg(feature = "next")]
pub use next::*;
