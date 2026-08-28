#!/usr/bin/env python3
"""Generate private WHATWG WPT ToASCII vectors from the approved fixture."""

from __future__ import annotations

import argparse
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
INPUT = ROOT / "tests/data/url/wpt/toascii.json"
OUTPUT = ROOT / "src/values/uri/url/engine/idna/wpt_vectors.rs"


def rust(value: str) -> str:
    chunks: list[str] = []
    for character in value:
        if character == "\\": chunks.append("\\\\")
        elif character == '"': chunks.append('\\"')
        elif character == "\n": chunks.append("\\n")
        elif character == "\r": chunks.append("\\r")
        elif character == "\t": chunks.append("\\t")
        elif 0x20 <= ord(character) <= 0x7E: chunks.append(character)
        else: chunks.append(f"\\u{{{ord(character):x}}}")
    return '"' + "".join(chunks) + '"'


def render() -> str:
    data = json.loads(INPUT.read_text(encoding="utf-8"))
    cases = [item for item in data if isinstance(item, dict) and "input" in item and "output" in item]
    skipped = len([item for item in cases if any(0xD800 <= ord(character) <= 0xDFFF for character in item["input"])])
    cases = [item for item in cases if not any(0xD800 <= ord(character) <= 0xDFFF for character in item["input"])]
    lines = ["//! Generated from WPT toascii.json; do not edit.", f"pub(super) const WPT_TOASCII_CASE_COUNT: usize = {len(cases)};", f"pub(super) const WPT_TOASCII_ILL_FORMED_SKIPPED_COUNT: usize = {skipped};", "#[rustfmt::skip]", "pub(super) static WPT_TOASCII_CASES: &[(&str, Option<&str>, bool)] = &["]
    for case in cases:
        expected = "None" if case["output"] is None else f"Some({rust(case['output'])})"
        lines.extend(["    (", f"        {rust(case['input'])},", f"        {expected},", f"        {str(bool(case.get('urlStandardOnly'))).lower()},", "    ),"])
    lines.extend(["];", ""])
    return "\n".join(lines)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true")
    parser.add_argument("--output", type=Path, default=OUTPUT)
    args = parser.parse_args()
    generated = render()
    if args.check:
        if not args.output.is_file() or args.output.read_text(encoding="utf-8") != generated:
            raise SystemExit("generated WPT vectors differ")
    else:
        args.output.write_text(generated, encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
