# Contributing

## XDR Generator

This repository has an XDR-to-Rust code generator (`xdr-generator-rust/`) that generates Rust types from XDR definitions. It outputs to `src/generated.rs`.

## How to Regenerate From XDR
To regenerate types from XDR definitions:

1. Update XDR definitions

   ```console
   git submodule update --init --remote
   ```

   The `--init` flag is only required for the first time setting up the local
   project. `--remote` flag will make sure to fetch the latest changes from
   the remote-tracking branch `main` at [stellar/stellar-xdr].

2. Recompile and test

   ```console
   make clean generate
   ```

   When the regenerated types are ready to be merged, make sure to commit the regenerated code file `src/generated.rs`, the version string file `xdr-version`, as well as the submodule file `xdr`.
