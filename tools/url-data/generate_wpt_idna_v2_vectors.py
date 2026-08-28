#!/usr/bin/env python3
"""Generate private WPT IdnaTestV2 vectors from the approved JSON fixture."""

from __future__ import annotations

import argparse
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
INPUT = ROOT / "tests/data/url/wpt/IdnaTestV2.json"
OUTPUT = ROOT / "src/values/uri/url/engine/idna/wpt_idna_v2_vectors.rs"


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
    raw = json.loads(INPUT.read_text(encoding="utf-8"))
    cases = [item for item in raw if isinstance(item, dict)]
    skipped = [item for item in cases if any(0xD800 <= ord(character) <= 0xDFFF for character in item["input"])]
    cases = [item for item in cases if item not in skipped]
    lines = ["//! Generated from WPT IdnaTestV2.json; do not edit.", f"pub(super) const WPT_IDNA_V2_CASE_COUNT: usize = {len(cases)};", f"pub(super) const WPT_IDNA_V2_ILL_FORMED_SKIPPED_COUNT: usize = {len(skipped)};", "#[rustfmt::skip]", "pub(super) static WPT_IDNA_V2_CASES: &[(&str, Option<&str>)] = &["]
    for case in cases:
        expected = "None" if case["output"] is None else f"Some({rust(case['output'])})"
        lines.extend(["    (", f"        {rust(case['input'])},", f"        {expected},", "    ),"])
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
            raise SystemExit("generated WPT IdnaTestV2 vectors differ")
    else:
        args.output.write_text(generated, encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
