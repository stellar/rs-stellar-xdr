# xdr-generator

The Ruby script in this directory generates Rust code for the curr and next
XDR. The script uses [xdrgen] with a custom generator.

## Usage

From the root directory of the repository run:

```
make clean generate
```

This scripts entry point, [generate.rb], will be called.

[xdrgen]: https://github.com/stellar/xdrgen
[generate.rb]: ./generate.rb
