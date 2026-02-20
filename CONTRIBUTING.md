# Contributing

## XDR Generators

This repository has two XDR-to-Rust code generators:

1. **Ruby generator** (`xdr-generator/`) - The original generator using xdrgen. Outputs to `src/*/generated.rs`.

2. **Rust generator** (`xdr-generator-rust/`) - A new generator written in Rust. Outputs to `src/*/generated-rust.rs`.

The plan is to migrate to the Rust generator once it has been validated to produce identical output. Currently both generators run during `make generate` and the outputs are compared to ensure they match.

## How to Regenerate From XDR
To regenerate types from XDR definitions:

1. Update XDR definitions

   ```console
   git submodule update --init --remote
   ```

   The `--init` flag is only required for the first time setting up the local
   project. `--remote` flag will make sure to fetch the latest changes from
   from the remote-tracking branches `curr` and `next` at [stellar/stellar-xdr].

   If you have multiple remotes specified in the submodules (e.g. one
   *tracking `stellar/stellar-xdr`, the other tracking `your-fork/stellar-xdr`),
   make sure the remote that tracks [stellar/stellar-xdr] match with what's
   specifies in the `.git/config` or `.gitsubmodules` (with `.git/config` taking
   precedence. If neither file specifies it, then `origin` is used).

2. Recompile and test

   ```console
   make clean generate
   ```

   When the regenerated types are ready to be merged, make sure to commit the regenerated code file `src/curr/generated.rs`, `src/next/generated.rs`, the version string file `xdr/curr-version`, `xdr/next-version`, as well as the submodule files `xdr/curr`, `xdr/next`.
