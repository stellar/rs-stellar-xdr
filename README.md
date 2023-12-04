# stellar-xdr

Library and CLI containing types and functionality for working with Stellar
XDR.

Types are generated from XDR definitions hosted at [stellar/stellar-xdr]
using [xdrgen].

[stellar/stellar-xdr]: https://github.com/stellar/stellar-xdr
[xdrgen]: https://github.com/stellar/xdrgen

### Usage

#### Library
To use the library, include in your toml:

```toml
stellar-xdr = { version = "...", default-features = true, features = [] }
```

##### Features

The crate has several features, tiers of functionality, ancillary
functionality, and channels of XDR.

Default features: `std`, `curr`.

Teirs of functionality:

1. `std` – The std feature provides all functionality (types, encode,
decode), and is the default feature set.
2. `alloc` – The alloc feature uses `Box` and `Vec` types for recursive
references and arrays, and is automatically enabled if the std feature is
enabled. The default global allocator is used. Support for a custom
allocator will be added in [#39]. No encode or decode capability exists,
only types. Encode and decode capability will be added in [#46].
3. If std or alloc are not enabled recursive and array types requires static
lifetime values. No encode or decode capability exists. Encode and decode
capability will be added in [#47].

[#39]: https://github.com/stellar/rs-stellar-xdr/issues/39
[#46]: https://github.com/stellar/rs-stellar-xdr/issues/46
[#47]: https://github.com/stellar/rs-stellar-xdr/issues/47

Ancillary functionality:

1. `base64` – Enables support for base64 encoding and decoding.
2. `serde` – Enables support for serializing and deserializing types with
the serde crate.
3. `arbitrary` – Enables support for interop with the arbitrary crate.

Channels of XDR:

- `curr` – XDR types built from the `stellar/stellar-xdr` `curr` branch.
- `next` – XDR types built from the `stellar/stellar-xdr` `next` branch.

If a single channel is enabled the types are available at the root of the
crate. If multiple channels are enabled they are available in modules at
the root of the crate.

#### CLI

To use the CLI:

```console
cargo install --locked stellar-xdr --version ... --features cli
```

##### Examples

Parse a `TransactionEnvelope`:
```console
stellar-xdr decode --type TransactionEnvelope << -
AAAAA...
-
```

Parse a `ScSpecEntry` stream from a contract:
```console
stellar-xdr +next decode --type ScSpecEntry --input stream-base64 --output json-formatted << -
AAAAA...
-
```

Parse a `BucketEntry` framed stream from a bucket file:
```console
stellar-xdr decode --type BucketEntry --input stream-framed --output json-formatted bucket.xdr
```

License: Apache-2.0
