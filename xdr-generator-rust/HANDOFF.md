# XDR Generator Rust - Project Handoff Summary

## Overview

This project is rewriting XDR-to-Rust code generation from Ruby to Rust. The goal is to produce byte-for-byte identical output to the current Ruby implementation.

## Current Status

**Working:** The generator successfully parses all XDR files and produces valid Rust code.

**Type Count Gap:** 419 types generated vs 477 in the original (missing ~58 types).

## Architecture

```
xdr-generator-rust/
├── Cargo.toml
├── src/
│   ├── lib.rs              # Module declarations
│   ├── main.rs             # CLI entry point
│   ├── ast.rs              # AST types for XDR definitions
│   ├── lexer.rs            # XDR tokenizer
│   ├── parser.rs           # Recursive descent XDR parser
│   ├── generator.rs        # Prepares data for templates
│   └── types.rs            # Type resolution, name conversion, attributes
└── templates/              # Askama templates
    ├── generated.rs.jinja  # Main output file
    ├── struct.rs.jinja     # Struct definition
    ├── enum.rs.jinja       # Enum definition
    ├── union.rs.jinja      # Union definition
    ├── typedef_alias.rs.jinja    # Simple type alias
    ├── typedef_newtype.rs.jinja  # Newtype struct
    └── type_enum.rs.jinja        # TypeVariant/Type enum
```

## Key Issues to Fix

### 1. Missing Nested/Anonymous Types (~58 missing)

The Ruby generator extracts anonymous union definitions and gives them generated names. For example:

```xdr
struct AccountEntry {
    // ...
    union switch (int v) {
        case 0: void;
        case 1: AccountEntryExtensionV1 ext;
    } ext;
};
```

Generates both `AccountEntry` AND `AccountEntryExt` as separate types. Our parser sees the anonymous union but doesn't extract it as a standalone type.

**Missing types include:**
- `AccountEntryExt`, `AccountEntryExtensionV1Ext`, `AccountEntryExtensionV2Ext`
- `AuthenticatedMessageV0`, `ClaimantV0`, `ContractEventV0`, `ContractEventBody`
- `LedgerKeyAccount`, `LedgerKeyClaimableBalance`, `LedgerKeyContractCode`, etc.
- `FeeBumpTransactionExt`, `FeeBumpTransactionInnerTx`
- `InnerTransactionResultExt`, `InnerTransactionResultResult`
- And more...

**Solution approach:**
1. When parsing a struct member that contains an anonymous union, extract it as a separate definition
2. Generate a name for it (e.g., field name + parent struct name pattern)
3. Replace the inline union with a reference to the extracted type

### 2. Attribute Formatting Differences

The Ruby generator formats multi-line attributes differently:
```rust
// Ruby (original):
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]

// Rust generator (current):
    #[cfg_attr(all(feature = "serde", feature = "alloc"), serde_as(as = "NumberOrString"))]
```

This is a minor formatting difference but needs to match exactly for byte-for-byte compatibility.

### 3. Header Content

The header.rs content (4000+ lines of support code) needs to be embedded. Currently using a placeholder.

## Completed Work

1. **Lexer (lexer.rs):** Tokenizes XDR files correctly
   - Handles keywords, identifiers, literals
   - Handles comments (// and /* */)
   - Handles % preprocessor directives (skips them)

2. **Parser (parser.rs):** Parses XDR into AST
   - Parses structs, enums, unions, typedefs, consts
   - Handles namespaces
   - Handles optional types (`Type*` syntax)
   - Handles built-in type aliases (`uint64` → `UnsignedHyper`, etc.)
   - Handles array/vararray suffixes

3. **Type System (types.rs):**
   - Rust type name conversion (UpperCamelCase)
   - Rust field name conversion (snake_case) with reserved word escaping
   - Type reference generation
   - Cyclic type detection (for Box wrapping)
   - Serde field attributes for i64/u64

4. **Generator (generator.rs):**
   - Generates struct, enum, union, typedef outputs
   - Computes derive attributes based on type characteristics
   - Handles custom_default_impl and custom_str_impl options (currently empty sets)
   - Distinguishes between type aliases (Uint64, Int64, etc.) and newtypes (Duration, TimePoint, etc.)

5. **Templates (templates/):**
   - All definition types have templates
   - ReadXdr/WriteXdr implementations
   - From/AsRef/TryFrom conversions
   - Debug/Display/FromStr for fixed opaque types

## How to Build and Test

```bash
# Build
cd xdr-generator-rust
cargo build --release

# Generate
./target/release/xdr-generator-rust \
  --input "../xdr/curr/Stellar-contract-config-setting.x" \
  --input "../xdr/curr/Stellar-contract-env-meta.x" \
  --input "../xdr/curr/Stellar-contract-meta.x" \
  --input "../xdr/curr/Stellar-contract-spec.x" \
  --input "../xdr/curr/Stellar-contract.x" \
  --input "../xdr/curr/Stellar-exporter.x" \
  --input "../xdr/curr/Stellar-internal.x" \
  --input "../xdr/curr/Stellar-ledger-entries.x" \
  --input "../xdr/curr/Stellar-ledger.x" \
  --input "../xdr/curr/Stellar-overlay.x" \
  --input "../xdr/curr/Stellar-transaction.x" \
  --input "../xdr/curr/Stellar-types.x" \
  --output /tmp/claude/new_generated.rs

# Compare type counts
grep -c "^pub struct\|^pub enum\|^pub type\|^pub const" /tmp/claude/new_generated.rs
grep -c "^pub struct\|^pub enum\|^pub type\|^pub const" ../src/curr/generated.rs

# Compare type names (to find missing types)
grep "^pub struct\|^pub enum\|^pub type" /tmp/claude/new_generated.rs | sed 's/pub struct //' | sed 's/pub enum //' | sed 's/pub type //' | sed 's/ .*//' | sort > /tmp/claude/new_types.txt
grep "^pub struct\|^pub enum\|^pub type" ../src/curr/generated.rs | sed 's/pub struct //' | sed 's/pub enum //' | sed 's/pub type //' | sed 's/ .*//' | sort > /tmp/claude/old_types.txt
comm -23 /tmp/claude/old_types.txt /tmp/claude/new_types.txt  # Shows missing types
```

## Files Reference

- **Ruby generator:** `../xdr-generator/generator/generator.rb` (~1280 lines)
- **Ruby header:** `../xdr-generator/generator/header.rs` (~4000 lines)
- **Original output:** `../src/curr/generated.rs` (~70K lines)
- **Plan document:** `/Users/leighmcculloch/.claude/plans/curious-skipping-lantern.md`

## Next Steps

1. **Fix anonymous union extraction** - Most important, accounts for most missing types
2. **Add header.rs embedding** - Copy content from Ruby generator's header.rs
3. **Match attribute formatting** - Multi-line attributes need to match exactly
4. **Verify exact output match** - Run diff and fix remaining differences
5. **Update Makefile** - Replace Ruby command with Rust binary
