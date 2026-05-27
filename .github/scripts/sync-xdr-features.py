#!/usr/bin/env python3
"""Register XDR-origin feature flags in Cargo.toml and src/lib.rs.

The XDR code generator emits `#[cfg(feature = "<name>")]` for every feature
gate present in the upstream .x files (e.g. `cap_0083`). Each such gate must be:

  1. declared in Cargo.toml under the "# Features from the XDR" block, otherwise
     the crate fails to compile (the Makefile builds with `-Dwarnings`, so rustc's
     "unexpected cfg condition value" warning becomes a hard error); and
  2. listed in the `VERSION.features` array in src/lib.rs so a build reports the
     gate when it is enabled.

A feature referenced by the generated code but not declared in Cargo.toml is, by
definition, a new XDR gate (dependency features like `serde`/`alloc` are always
already declared). This script finds those and adds them to both places. It is
idempotent: running it when nothing is missing is a no-op.

Set SYNC_ROOT to point at a repo checkout other than the default (used by tests).
Pass --check to print the planned changes without writing.
"""
import os
import pathlib
import re
import sys

ROOT = pathlib.Path(os.environ.get("SYNC_ROOT", pathlib.Path(__file__).resolve().parents[2]))
DRY = "--check" in sys.argv

FEATURE_RE = re.compile(r'feature\s*=\s*"([^"]+)"')


def referenced_features() -> set[str]:
    feats: set[str] = set()
    paths = [ROOT / "src" / "generated.rs", *(ROOT / "src" / "generated").rglob("*.rs")]
    for p in paths:
        if p.exists():
            feats.update(FEATURE_RE.findall(p.read_text()))
    return feats


def declared_features(cargo: str) -> set[str]:
    block = re.search(r"(?ms)^\[features\]\s*\n(.*?)(?=^\[|\Z)", cargo)
    return set(re.findall(r"(?m)^([A-Za-z0-9_-]+)\s*=", block.group(1))) if block else set()


def add_to_cargo(cargo: str, missing: list[str]) -> str:
    marker = "# Features from the XDR\n"
    start = cargo.index(marker) + len(marker)
    end = cargo.index("\n\n", start)
    existing = [l for l in cargo[start:end].splitlines() if l.strip()]
    existing += [f"{f} = []" for f in missing]
    return cargo[:start] + "\n".join(existing) + cargo[end:]


def add_to_lib(lib: str, missing: list[str]) -> str:
    arr = re.search(r"(?ms)(features:\s*&\[\n)(.*?)(\n[ \t]*\],)", lib)
    head, body, tail = arr.group(1), arr.group(2), arr.group(3)
    present = re.findall(r'#\[cfg\(feature = "([^"]+)"\)\]', body)
    feats = present + [f for f in missing if f not in present]
    indent = re.match(r"\s*", body).group(0)
    new_body = "\n".join(f'{indent}#[cfg(feature = "{f}")]\n{indent}"{f}",' for f in feats)
    return lib[: arr.start()] + head + new_body + tail + lib[arr.end():]


def main() -> int:
    cargo_path = ROOT / "Cargo.toml"
    lib_path = ROOT / "src" / "lib.rs"
    cargo = cargo_path.read_text()

    missing = sorted(referenced_features() - declared_features(cargo))
    if not missing:
        print("No new XDR feature flags to register.")
        return 0
    print("Registering new XDR feature flags:", ", ".join(missing))

    new_cargo = add_to_cargo(cargo, missing)
    new_lib = add_to_lib(lib_path.read_text(), missing)

    if DRY:
        print("\n--- Cargo.toml [features] (new) ---")
        print(re.search(r"(?ms)^\[features\].*?(?=\n\n\[)", new_cargo).group(0))
        print("\n--- src/lib.rs VERSION.features (new) ---")
        print(re.search(r"(?ms)features:\s*&\[.*?\],", new_lib).group(0))
        return 0

    cargo_path.write_text(new_cargo)
    lib_path.write_text(new_lib)
    return 0


if __name__ == "__main__":
    sys.exit(main())
