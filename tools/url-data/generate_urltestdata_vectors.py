#!/usr/bin/env python3
"""Generate deterministic private Rust vectors from the pinned URL WPT fixture."""

from __future__ import annotations

import argparse
import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
SOURCE = ROOT / "test_data/url/wpt/urltestdata.json"
TARGET = ROOT / "src/values/uri/url/engine/parser/wpt_vectors.rs"


def usv(value: str) -> str:
    """Apply JavaScript USVString conversion to JSON's scalar/surrogate string."""
    units = value.encode("utf-16", "surrogatepass")
    return units.decode("utf-16", "replace")


def rust_string(value: str) -> str:
    """Emit one Rust string literal without retaining unpaired surrogate syntax."""
    escaped: list[str] = ['"']
    for character in usv(value):
        match character:
            case '\\':
                escaped.append("\\\\")
            case '"':
                escaped.append('\\"')
            case '\\n':
                escaped.append("\\n")
            case '\\r':
                escaped.append("\\r")
            case '\\t':
                escaped.append("\\t")
            case _ if ord(character) < 0x20 or ord(character) == 0x7F:
                escaped.append(f"\\u{{{ord(character):x}}}")
            case _ if ord(character) > 0x7E:
                escaped.append(f"\\u{{{ord(character):x}}}")
            case _:
                escaped.append(character)
    escaped.append('"')
    return "".join(escaped)


def optional(value: object) -> str:
    return "None" if value is None else f"Some({rust_string(str(value))})"


def render() -> str:
    rows = json.loads(SOURCE.read_text(encoding="utf-8"))
    objects = [row for row in rows if isinstance(row, dict)]
    lines = [
        "//! Generated from pinned WPT url/resources/urltestdata.json; do not edit.",
        "//! Generator: tools/url-data/generate_urltestdata_vectors.py.",
        "",
        "pub(super) struct Vector {",
        "    pub(super) input: &'static str,",
        "    pub(super) base: Option<&'static str>,",
        "    pub(super) failure: bool,",
        "    pub(super) href: Option<&'static str>,",
        "    pub(super) protocol: Option<&'static str>,",
        "    pub(super) username: Option<&'static str>,",
        "    pub(super) password: Option<&'static str>,",
        "    pub(super) host: Option<&'static str>,",
        "    pub(super) hostname: Option<&'static str>,",
        "    pub(super) port: Option<&'static str>,",
        "    pub(super) pathname: Option<&'static str>,",
        "    pub(super) search: Option<&'static str>,",
        "    pub(super) hash: Option<&'static str>,",
        "}",
        "",
        "pub(super) static VECTORS: &[Vector] = &[",
    ]
    for row in objects:
        failure = bool(row.get("failure", False))
        lines.extend(
            [
                "    Vector {",
                f"        input: {rust_string(str(row.get('input', '')))},",
                f"        base: {optional(row.get('base'))},",
                f"        failure: {'true' if failure else 'false'},",
            ]
        )
        for field in (
            "href",
            "protocol",
            "username",
            "password",
            "host",
            "hostname",
            "port",
            "pathname",
            "search",
            "hash",
        ):
            lines.append(f"        {field}: {optional(row.get(field))},")
        lines.append("    },")
    lines.extend(["] ;", ""])
    return "\n".join(lines).replace("] ;", "];" ) + "\n"


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true")
    arguments = parser.parse_args()
    rendered = render()
    if arguments.check:
        return 0 if TARGET.read_text(encoding="utf-8") == rendered else 1
    TARGET.write_text(rendered, encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
