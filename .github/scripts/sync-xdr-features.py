#!/usr/bin/env python3
"""Keep the XDR-origin feature flags in Cargo.toml and src/lib.rs in sync with
what the generated Rust code actually references.

The XDR code generator emits `#[cfg(feature = "<name>")]` for every feature gate
present in the upstream .x files (e.g. `cap_0083`). Each such gate must be:

  1. declared in Cargo.toml under the "# Features from the XDR" block, otherwise
     the crate fails to compile (the Makefile builds with `-Dwarnings`, so rustc's
     "unexpected cfg condition value" warning becomes a hard error); and
  2. listed in the `VERSION.features` array in src/lib.rs so a build reports the
     gate when it is enabled.

This script enforces a single rule across both registries:

    XDR-origin features = (features referenced in src/generated*)
                        - (features declared elsewhere in [features])

That means it both **adds** new gates (e.g. `cap_0083` appearing upstream) and
**removes** obsolete ones (e.g. `cap_0071` once upstream inlines it and the
generator stops emitting it). Dependency features (`serde`, `alloc`, `cli`, …)
declared outside the XDR-origin block are left untouched. Idempotent.

The script depends on the existing structure of Cargo.toml and src/lib.rs (the
"# Features from the XDR" marker and the `VERSION.features` array). If that
structure ever changes, it fails with a clear message and a non-zero exit rather
than an opaque traceback, so the workflow surfaces the cause.

Set SYNC_ROOT to point at a repo checkout other than the default (used by tests).
Pass --check to print the planned changes without writing.
"""
import os
import pathlib
import re
import sys
from typing import NoReturn

ROOT = pathlib.Path(os.environ.get("SYNC_ROOT", pathlib.Path(__file__).resolve().parents[2]))
DRY = "--check" in sys.argv

FEATURE_RE = re.compile(r'feature\s*=\s*"([^"]+)"')


def die(msg: str) -> NoReturn:
    print(f"sync-xdr-features: error: {msg}", file=sys.stderr)
    sys.exit(1)


def referenced_features() -> set[str]:
    feats: set[str] = set()
    paths = [ROOT / "src" / "generated.rs", *(ROOT / "src" / "generated").rglob("*.rs")]
    for p in paths:
        if p.exists():
            feats.update(FEATURE_RE.findall(p.read_text()))
    return feats


def parse_cargo(cargo: str) -> tuple[int, int, list[str], set[str]]:
    """Locate the XDR-origin block and the set of all declared features."""
    marker = "# Features from the XDR\n"
    if marker not in cargo:
        die(f"could not find the {marker.strip()!r} marker in Cargo.toml")
    start = cargo.index(marker) + len(marker)
    end = cargo.find("\n\n", start)
    if end == -1:
        die("could not find the blank line ending the '# Features from the XDR' block in Cargo.toml")
    xdr_names: list[str] = []
    for line in cargo[start:end].splitlines():
        if not line.strip():
            continue
        m = re.match(r"([A-Za-z0-9_-]+)\s*=", line)
        if not m:
            die(f"unexpected line in '# Features from the XDR' block: {line!r}")
        xdr_names.append(m.group(1))

    features_block = re.search(r"(?ms)^\[features\]\s*\n(.*?)(?=^\[|\Z)", cargo)
    if not features_block:
        die("could not find a [features] section in Cargo.toml")
    all_declared = set(re.findall(r"(?m)^([A-Za-z0-9_-]+)\s*=", features_block.group(1)))
    return start, end, xdr_names, all_declared


def rewrite_cargo(cargo: str, start: int, end: int, new_names: list[str]) -> str:
    return cargo[:start] + "\n".join(f"{n} = []" for n in new_names) + cargo[end:]


def rewrite_lib(lib: str, new_set: set[str], added_sorted: list[str]) -> str:
    arr = re.search(r"(?ms)(features:\s*&\[\n)(.*?)(\n[ \t]*\],)", lib)
    if not arr:
        die("could not find the `VERSION.features` array in src/lib.rs")
    head, body, tail = arr.group(1), arr.group(2), arr.group(3)
    present = re.findall(r'#\[cfg\(feature = "([^"]+)"\)\]', body)
    surviving = [n for n in present if n in new_set]
    new_order = surviving + [n for n in added_sorted if n not in present]
    indent = (re.match(r"\s*", body).group(0) if body.strip() else "") or "        "
    new_body = "\n".join(f'{indent}#[cfg(feature = "{n}")]\n{indent}"{n}",' for n in new_order)
    return lib[: arr.start()] + head + new_body + tail + lib[arr.end():]


def main() -> int:
    cargo_path = ROOT / "Cargo.toml"
    lib_path = ROOT / "src" / "lib.rs"
    for p in (cargo_path, lib_path):
        if not p.exists():
            die(f"{p} does not exist")
    cargo = cargo_path.read_text()
    lib = lib_path.read_text()

    start, end, xdr_names, all_declared = parse_cargo(cargo)
    non_xdr_declared = all_declared - set(xdr_names)
    referenced = referenced_features()
    new_xdr_set = referenced - non_xdr_declared

    removed = sorted(set(xdr_names) - new_xdr_set)
    added = sorted(new_xdr_set - set(xdr_names))

    if not removed and not added:
        print("XDR feature flags already in sync.")
        return 0
    if removed:
        print("Removing obsolete XDR feature flags:", ", ".join(removed))
    if added:
        print("Registering new XDR feature flags:", ", ".join(added))

    surviving = [n for n in xdr_names if n in new_xdr_set]
    new_xdr_names = surviving + added
    new_cargo = rewrite_cargo(cargo, start, end, new_xdr_names)
    new_lib = rewrite_lib(lib, new_xdr_set, added)

    if DRY:
        print("\n--- Cargo.toml [features] (new) ---")
        m = re.search(r"(?ms)^\[features\].*?(?=\n\n\[)", new_cargo)
        print(m.group(0) if m else new_cargo)
        print("\n--- src/lib.rs VERSION.features (new) ---")
        m = re.search(r"(?ms)features:\s*&\[.*?\],", new_lib)
        print(m.group(0) if m else "(not found)")
        return 0

    cargo_path.write_text(new_cargo)
    lib_path.write_text(new_lib)
    return 0


if __name__ == "__main__":
    sys.exit(main())
