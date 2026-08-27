#!/usr/bin/env python3
"""Generate passive Unicode 17 UTS #46 mapping data for the private IDNA engine."""

from __future__ import annotations

import argparse
import hashlib
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
INPUT = ROOT / "test_data/url/unicode/IdnaMappingTable.txt"
JOINING = ROOT / "test_data/url/unicode/DerivedJoiningType.txt"
BIDI = ROOT / "test_data/url/unicode/DerivedBidiClass.txt"
CATEGORY = ROOT / "test_data/url/unicode/DerivedGeneralCategory.txt"
OUTPUT = ROOT / "src/values/uri/url/engine/idna/tables.rs"
EXPECTED = "87f05505dc026fdb2bff16132bdc68a8014675836882a9a2b1844540ad3be382"
STATUS = {
    "valid": "Valid",
    "mapped": "Mapped",
    "ignored": "Ignored",
    "deviation": "Deviation",
    "disallowed": "Disallowed",
}


def hash_file(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def rust_string(value: str) -> str:
    return '"' + value.replace('\\', '\\\\').replace('"', '\\"') + '"'


def parse_range(token: str) -> tuple[int, int]:
    values = token.split("..")
    if len(values) == 1:
        start = end = int(values[0], 16)
    elif len(values) == 2:
        start, end = (int(value, 16) for value in values)
    else:
        raise ValueError(f"invalid scalar range: {token}")
    # UTS #46 deliberately records the UTF-16 surrogate range as disallowed.
    # Rust input cannot contain one, but retaining this source range preserves
    # complete table coverage and makes the generated table auditable.
    if start > end or end > 0x10FFFF:
        raise ValueError(f"invalid scalar range: {token}")
    return start, end


def parse() -> list[tuple[int, int, str, str]]:
    entries: list[tuple[int, int, str, str]] = []
    prior_end = -1
    for raw in INPUT.read_text(encoding="utf-8").splitlines():
        line = raw.split("#", 1)[0].strip()
        if not line or line.startswith("@missing:"):
            continue
        fields = [field.strip() for field in line.split(";")]
        if len(fields) < 2 or fields[1] not in STATUS:
            raise ValueError(f"malformed or unsupported row: {raw}")
        start, end = parse_range(fields[0])
        if start <= prior_end:
            raise ValueError(f"overlapping row: {raw}")
        mapping = ""
        if len(fields) > 2 and fields[2]:
            mapping = "".join(chr(int(value, 16)) for value in fields[2].split())
        if fields[1] == "mapped" and not mapping:
            raise ValueError(f"mapped row without mapping: {raw}")
        entries.append((start, end, STATUS[fields[1]], mapping))
        prior_end = end
    if not entries:
        raise ValueError("no UTS #46 entries")
    return entries


def parse_property(path: Path) -> list[tuple[int, int, str]]:
    entries: list[tuple[int, int, str]] = []
    for raw in path.read_text(encoding="utf-8").splitlines():
        line = raw.split("#", 1)[0].strip()
        if not line or line.startswith("@missing:"):
            continue
        fields = [field.strip() for field in line.split(";")]
        if len(fields) != 2 or not fields[1].replace("_", "").isalpha():
            raise ValueError(f"malformed property row in {path.name}: {raw}")
        start, end = parse_range(fields[0])
        entries.append((start, end, fields[1]))
    return entries


def render(entries: list[tuple[int, int, str, str]]) -> str:
    joining = parse_property(JOINING)
    bidi = parse_property(BIDI)
    category = parse_property(CATEGORY)
    lines = [
        "//! Generated from Unicode 17.0.0 IdnaMappingTable.txt; do not edit.",
        "//! Source SHA-256: " + hash_file(INPUT),
        "",
        "#[derive(Clone, Copy, Eq, PartialEq)]",
        "pub(super) enum Status {",
        "    Valid,",
        "    Mapped,",
        "    Ignored,",
        "    Deviation,",
        "    Disallowed,",
        "}",
        "",
        "pub(super) struct Entry {",
        "    pub(super) start: u32,",
        "    pub(super) end: u32,",
        "    pub(super) status: Status,",
        "    pub(super) mapping: &'static str,",
        "}",
        "pub(super) struct Property {",
        "    pub(super) start: u32,",
        "    pub(super) end: u32,",
        "    pub(super) value: &'static str,",
        "}",
        "",
        "pub(super) static MAPPING: &[Entry] = &[",
    ]
    lines.extend(
        f"    Entry {{\n        start: 0x{start:04X},\n        end: 0x{end:04X},\n        status: Status::{status},\n        mapping: {rust_string(mapping)},\n    }},"
        for start, end, status, mapping in entries
    )
    lines.extend(["];", "", "pub(super) static JOINING: &[Property] = &["])
    lines.extend(f"    Property {{\n        start: 0x{start:04X},\n        end: 0x{end:04X},\n        value: \"{value}\",\n    }}," for start, end, value in joining)
    lines.extend(["];", "", "pub(super) static BIDI: &[Property] = &["])
    lines.extend(f"    Property {{\n        start: 0x{start:04X},\n        end: 0x{end:04X},\n        value: \"{value}\",\n    }}," for start, end, value in bidi)
    lines.extend(["];", "", "pub(super) static CATEGORY: &[Property] = &["])
    lines.extend(f"    Property {{\n        start: 0x{start:04X},\n        end: 0x{end:04X},\n        value: \"{value}\",\n    }}," for start, end, value in category)
    lines.extend(["];", ""])
    return "\n".join(lines)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true")
    parser.add_argument("--output", type=Path, default=OUTPUT)
    args = parser.parse_args()
    if hash_file(INPUT) != EXPECTED:
        raise SystemExit("IdnaMappingTable.txt SHA-256 does not match the approved input")
    if "Version: 17.0.0" not in INPUT.read_text(encoding="utf-8"):
        raise SystemExit("IdnaMappingTable.txt is not Unicode 17.0.0")
    for path in (JOINING, BIDI, CATEGORY):
        if "17.0.0" not in path.read_text(encoding="utf-8"):
            raise SystemExit(f"{path.name} is not Unicode 17.0.0")
    generated = render(parse())
    if args.check:
        if not args.output.is_file() or args.output.read_text(encoding="utf-8") != generated:
            raise SystemExit("generated IDNA tables differ")
    else:
        args.output.write_text(generated, encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
