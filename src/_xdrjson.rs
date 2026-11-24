//! # XDR-JSON exceptions
//!
//! With the `serde` feature, types serialize to and deserialize from XDR-JSON
//! ([SEP-51]), and XDR converted to XDR-JSON converts back to the same XDR,
//! with the following exceptions:
//!
//! 1. [`SignerKeyEd25519SignedPayload`][crate::SignerKeyEd25519SignedPayload]
//! with an empty payload – A signed payload strkey requires a payload of 1 to
//! 64 bytes, but the XDR type permits an empty payload. An empty payload is
//! rendered as `<INVALID:G...:EMPTY_PAYLOAD>`, where `G...` is the ed25519 key
//! as a strkey, which cannot be converted back to XDR. This also applies
//! wherever the type is nested, such as in
//! [`SignerKey::Ed25519SignedPayload`][crate::SignerKey::Ed25519SignedPayload].
//! 2. Deeply nested values – serde_json stops reading JSON nested more than 128
//! levels deep by default. Values nested that deep can be written to XDR-JSON
//! but not read back with the default limit. To read them, enable serde_json's
//! `unbounded_depth` feature, and deserialize with a deserializer that has the
//! recursion limit disabled:
//!
//! ```
//! # #[cfg(feature = "serde")]
//! # fn main() -> Result<(), serde_json::Error> {
//! use serde::Deserialize;
//! use stellar_xdr::ScVal;
//!
//! let depth = 100;
//! let json = format!("{}\"void\"{}", r#"{"vec":["#.repeat(depth), "]}".repeat(depth));
//!
//! let mut de = serde_json::Deserializer::from_str(&json);
//! de.disable_recursion_limit();
//! let v = ScVal::deserialize(&mut de)?;
//! de.end()?;
//! # Ok(())
//! # }
//! # #[cfg(not(feature = "serde"))]
//! # fn main() {}
//! ```
//!
//! With the `type_enum` and `serde_json` features, pass the deserializer to
//! `Type::deserialize_json` to read a type chosen at runtime:
//!
//! ```
//! # #[cfg(all(feature = "type_enum", feature = "serde_json"))]
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! use stellar_xdr::{Type, TypeVariant};
//!
//! let depth = 100;
//! let json = format!("{}\"void\"{}", r#"{"vec":["#.repeat(depth), "]}".repeat(depth));
//!
//! let mut de = serde_json::Deserializer::from_str(&json);
//! de.disable_recursion_limit();
//! let t = Type::deserialize_json(TypeVariant::ScVal, &mut de)?;
//! de.end()?;
//! # Ok(())
//! # }
//! # #[cfg(not(all(feature = "type_enum", feature = "serde_json")))]
//! # fn main() {}
//! ```
//!
//! **Warning:** Without the limit, values nested deeply enough can overflow the
//! stack while being read, and also when being cloned, compared, or dropped.
//!
//! [SEP-51]: https://github.com/stellar/stellar-protocol/blob/master/ecosystem/sep-0051.md
