# XDR Generator Rust — Improvements

## 1. Deduplicate anonymous union extraction logic
Extract the repeated anonymous union extraction code in `parser.rs` (appears in `parse_member()` and twice in `parse_union_arm()`) into a single helper method like `extract_anonymous_union_if_needed()`.

## 2. Unify `base_rust_type_ref` and `base_rust_type_ref_with_info`
Merge the two nearly identical functions in `types.rs` into one that takes an optional `TypeInfo` reference to resolve const-named sizes.

## 3. Replace fragile string slicing in `rust_read_call_type`
Replace hardcoded index slicing (`&ref_type[3..]`, `&ref_type[11..]`) with `strip_prefix`/`strip_suffix` for robustness.

## 4. Simplify inline struct parsing in `parse_union_arm`
Refactor the two-pass lookahead for inline structs to be clearer, with comments documenting the XDR patterns that require it.

## 5. Improve `fix_parent_relationships` with documentation and tests
Add doc comments and inline tests explaining which XDR patterns trigger the post-processing parent fixup.

## 6. Return errors from `resolve_enum_value` instead of silent fallback
Change `resolve_enum_value` to return `Result<i32, ParseError>` so unresolvable references surface as errors rather than silently returning 0.

## 7. Derive const type from AST instead of hardcoding `u64`
Note: Only if the Ruby generator also uses `u64`. If so, document why. Otherwise, derive from AST.

## 8. Add line/column info to `ParseError`
Use the byte spans from `SpannedToken` to include position context in parse errors.

## 9. Reduce duplication in type reference formatting (minor)
Look for other small dedup opportunities in `types.rs` formatting functions.
