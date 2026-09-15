# Fuzz targets

Differential fuzzing of the const XDR encoder against the owned one.

Each target builds a value in both the owned and the const form from the same
input bytes, then asserts the two encoders produce identical XDR. The
`Arbitrary` impls of the types the const form substitutes — `VecM` for `Vec`,
`BytesM` and `StringM` for `Vec<u8>`, a `&'static` reference for a `Box` —
consume input bytes exactly as the owned ones do, so one input yields the same
value in both forms with no conversion between them.

| Target | Type |
| --- | --- |
| `transaction-envelope` | `TransactionEnvelope` |
| `ledger-close-meta` | `LedgerCloseMeta` |

## Running

```
ASAN_OPTIONS=detect_leaks=0 cargo +nightly fuzz run transaction-envelope -- -rss_limit_mb=0
```

Both options are required rather than optional. Const types hold `&'static`
data, so their `Arbitrary` impls leak every value they build: without them the
run ends in a LeakSanitizer report or an out-of-memory abort that says nothing
about the encoders. Memory grows by roughly 10MB/s at 25k exec/s, so run
bounded campaigns (`-runs=N` or `-max_total_time=N`) and restart, rather than
leaving a target running indefinitely. The corpus carries across restarts in
`fuzz/corpus/<target>/`.
