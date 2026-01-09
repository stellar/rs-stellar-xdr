# XDR Generator Rust - Project Handoff Summary

## Overview

This project is rewriting XDR-to-Rust code generation from Ruby to Rust. The goal is to produce byte-for-byte identical output to the current Ruby implementation.

## Current Status

**Working:** The generator successfully parses all XDR files and produces valid Rust code.

**Type Count:** 477 types generated - matches the original exactly!

**Completed:** Anonymous union/inline struct extraction is now working.

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

## Completed Fixes

### 1. Anonymous Union/Inline Struct Extraction (FIXED)

The generator now correctly extracts:
- **Anonymous unions in struct members:** `struct Foo { union switch (...) { ... } ext; }` → extracts `FooExt`
- **Inline structs in union arms:** `union Foo { case X: struct { ... } bar; }` → extracts `FooBar`
- **Anonymous unions in union arms:** `union Foo { case X: union switch (...) { ... } tr; }` → extracts `FooTr`
- **Nested types:** Correctly handles deeply nested anonymous types with proper naming

**Implementation details (in parser.rs):**
- Added `root_parent` field to track the current top-level type being parsed
- Added `extracted_definitions` to collect nested type definitions
- Uses look-ahead parsing for inline structs to get field name before parsing body
- Added `Type::AnonymousUnion` variant to AST for deferred extraction

## Remaining Issues to Fix

### 1. Attribute Formatting Differences

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

1. ~~**Fix anonymous union extraction** - DONE~~
2. **Add header.rs embedding** - Copy content from Ruby generator's header.rs
3. **Match attribute formatting** - Multi-line attributes need to match exactly
4. **Verify exact output match** - Run diff and fix remaining differences
5. **Update Makefile** - Replace Ruby command with Rust binary
