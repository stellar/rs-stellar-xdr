# stellar-xdr

Library and CLI containing types and functionality for working with Stellar
XDR.

Types are generated from XDR definitions hosted at [stellar/stellar-xdr]
using [xdrgen].

[stellar/stellar-xdr]: https://github.com/stellar/stellar-xdr
[xdrgen]: https://github.com/stellar/xdrgen

## Usage

### Library
To use the library, include in your toml:

```toml
stellar-xdr = { version = "...", default-features = true, features = [] }
```

#### Features

The crate has several features, tiers of functionality, ancillary
functionality, and channels of XDR.

Default features: `std`, `curr`.

Tiers of functionality:

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
3. `serde_json` – Enables support for built-in functionality specifically
for serde_json. Often not required to use the types with serde_json, and
only necessary to use utility functions that depend on serde_json.
4. `arbitrary` – Enables support for interop with the arbitrary crate.
5. `hex` – Enables support for hex in string representations of some types.
Automatically enabled when serde is enabled.
6. `schemars` – Enables support for JSON Schema generation. (Experimental)

Features marked experimental may disappear at anytime, see breaking changes
at anytime, or and may be minimal implementations instead of complete.

Channels of XDR:

- `curr` – XDR types built from the `stellar/stellar-xdr` `curr` branch.
- `next` – XDR types built from the `stellar/stellar-xdr` `next` branch.

If a single channel is enabled the types are available at the root of the
crate. If multiple channels are enabled they are available in modules at
the root of the crate.

#### Sparse Types

Sparse types are decode-only types that extract specific nested fields from
XDR data while skipping over unneeded fields during decoding. This can
significantly improve performance when you only need to access certain
fields from large XDR structures.

For example, `TransactionEnvelopeSparse` extracts only the `operations`
field from transaction envelopes:

```rust
use stellar_xdr::curr::{TransactionEnvelopeSparse, ReadXdr, Limits};

let xdr_data: &[u8] = /* ... */;
let sparse = TransactionEnvelopeSparse::from_xdr(xdr_data, Limits::none())?;

// Access operations without fully decoding the entire envelope
let operations = match sparse {
    TransactionEnvelopeSparse::Tx(e) => e.tx.operations,
    TransactionEnvelopeSparse::TxV0(e) => e.tx.operations,
    TransactionEnvelopeSparse::TxFeeBump(e) => {
        // Access nested operations through fee bump
        // ...
    }
};
```

Sparse types preserve the structure of the original types (enums remain
enums with all variants), but only include the fields along the extraction
paths. All other fields are skipped during decoding.

To configure additional sparse types, modify the `sparse_types` array in
`xdr-generator/generate.rb`:

```ruby
sparse_types: [
  {
    base_type: "TransactionEnvelope",
    name: "TransactionEnvelopeSparse",
    paths: [
      "Tx.tx.operations",
      "TxV0.tx.operations",
      "TxFeeBump.tx.innerTx.Tx.tx.operations",
    ],
  },
]
```

Path format:
- Dot-separated segments for navigating through types
- Use variant names for unions (e.g., `Tx`, `TxV0`)
- Use field names for structs (e.g., `tx`, `operations`)
- Use `[]` suffix for array/Vec traversal (e.g., `tx_processing[].result.hash`)

##### Seekable Sparse Decoding

For file-backed readers or other seekable sources, sparse types also implement
`SeekableReadXdr` which uses `seek` operations instead of reading and discarding
bytes. This is more efficient for large files:

```rust
use stellar_xdr::curr::{TransactionEnvelopeSparse, SeekableReadXdr, Limits};
use std::fs::File;
use std::io::BufReader;

let file = File::open("transaction.xdr")?;
let mut reader = Limited::new(BufReader::new(file), Limits::none());
let sparse = TransactionEnvelopeSparse::seekable_read_xdr(&mut reader)?;
```

Or for in-memory byte slices:

```rust
let sparse = TransactionEnvelopeSparse::from_xdr_seekable(&xdr_data, Limits::none())?;
```

### CLI

To use the CLI:

```console
cargo install --locked stellar-xdr --version ... --features cli
```

#### Examples

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
