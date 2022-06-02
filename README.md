# rs-stellar-xdr
Rust SDK for Stellar XDR.

**This repository contains code that is in early development, incomplete, not tested, and not recommended for use. The API is unstable, experimental, and is receiving breaking changes frequently.**

## Usage

Include in your toml:

```toml
stellar-xdr = { git = "https://github.com/stellar/rs-stellar-xdr", rev = "..." }
```

The crate has two features and three tiers of functionality:

1. `std` – The std feature provides all functionality (types, encode, decode), and is the default feature set.
2. `alloc` – The alloc feature uses `Box` and `Vec` types for recursive references and arrays, and is automatically enabled if the std feature is enabled. The default global allocator is used. Support for a custom allocator will be added in [#39]. No encode or decode capability exists, only types. Encode and decode capability will be added in [#46].
3. If std or alloc are not enabled recursive and array types requires static lifetime values. No encode or decode capability exists. Encode and decode capability will be added in [#47].

The crate has one extra feature, `next`, that if enabled exposes the generated types for the next Stellar protocol version still in development.

[#39]: https://github.com/stellar/rs-stellar-xdr/issues/39
[#46]: https://github.com/stellar/rs-stellar-xdr/issues/46
[#47]: https://github.com/stellar/rs-stellar-xdr/issues/47
