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
//!
//! [SEP-51]: https://github.com/stellar/stellar-protocol/blob/master/ecosystem/sep-0051.md
