#![forbid(unsafe_code)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "next"))]
mod curr;
mod scval_validations;
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
