#!/usr/bin/env python3
"""Generate dependency-free private success vectors from approved Unicode data."""

from __future__ import annotations

import argparse
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
INPUT = ROOT / "test_data/url/unicode/IdnaTestV2.txt"
OUTPUT = ROOT / "src/values/uri/url/engine/idna/fixture_vectors.rs"
ESCAPE = re.compile(r"\\(?:u([0-9A-Fa-f]{4})|x\{([0-9A-Fa-f]+)\})")


def decode(value: str) -> str:
    def replacement(match: re.Match[str]) -> str:
        return chr(int(match.group(1) or match.group(2), 16))
    return ESCAPE.sub(replacement, value)


def field_value(value: str) -> str:
    return "" if value == '\"\"' else decode(value)


def requires_relaxed_error(unicode_status: str, ascii_status: str) -> bool:
    # UTS #46's blank ToASCII status inherits the ToUnicode status. Explicit
    # ToASCII status, including [], takes precedence.
    status = ascii_status if ascii_status else unicode_status
    tokens = [token.strip() for token in status.strip("[]").split(",") if token.strip()]
    return any(token not in {"A4_1", "A4_2", "V2", "V3", "U1"} for token in tokens)


def has_to_unicode_error(status: str) -> bool:
    tokens = [token.strip() for token in status.strip("[]").split(",") if token.strip()]
    return any(token != "U1" for token in tokens)


def rust(value: str) -> str:
    chunks: list[str] = []
    for character in value:
        if character == "\\":
            chunks.append("\\\\")
        elif character == '"':
            chunks.append('\\"')
        elif character == "\n":
            chunks.append("\\n")
        elif character == "\r":
            chunks.append("\\r")
        elif character == "\t":
            chunks.append("\\t")
        elif 0x20 <= ord(character) <= 0x7E:
            chunks.append(character)
        else:
            chunks.append(f"\\u{{{ord(character):x}}}")
    return '"' + "".join(chunks) + '"'


def vectors() -> tuple[list[tuple[str, str]], list[str], list[tuple[str, str, bool]], int]:
    result: list[tuple[str, str]] = []
    error_cases: list[str] = []
    to_unicode_cases: list[tuple[str, str, bool]] = []
    skipped = 0
    to_unicode_only = 0
    for raw in INPUT.read_text(encoding="utf-8").splitlines():
        line = raw.split("#", 1)[0].strip()
        if not line or ";" not in line:
            continue
        fields = [field.strip() for field in line.split(";")]
        if len(fields) < 7:
            continue
        source, unicode_value, unicode_status, ascii_value, ascii_status = fields[:5]
        source = field_value(source)
        unicode_value = field_value(unicode_value) if unicode_value else source
        expected = field_value(ascii_value) if ascii_value else unicode_value
        if any(
            0xD800 <= ord(character) <= 0xDFFF
            for character in source + unicode_value + expected
        ):
            skipped += 1
            continue
        if requires_relaxed_error(unicode_status, ascii_status):
            # These records require a private failure category rather than an
            # externally observable UTS #46 status token.
            error_cases.append(source)
            continue
        if unicode_status:
            to_unicode_only += 1
            to_unicode_cases.append((source, unicode_value, has_to_unicode_error(unicode_status)))
            continue
        result.append((source, expected))
    return result, error_cases, to_unicode_cases, skipped


def render() -> str:
    cases, error_cases, to_unicode_cases, skipped = vectors()
    lines = [
        "//! Generated from Unicode 17.0.0 IdnaTestV2.txt; do not edit.",
        f"pub(super) const UTS46_SUCCESS_CASE_COUNT: usize = {len(cases)};",
        f"pub(super) const UTS46_ERROR_CASE_COUNT: usize = {len(error_cases)};",
        f"pub(super) const UTS46_ILL_FORMED_SKIPPED_COUNT: usize = {skipped};",
        f"pub(super) const UTS46_TO_UNICODE_ONLY_COUNT: usize = {len(to_unicode_cases)};",
        "#[rustfmt::skip]",
        "pub(super) static UTS46_SUCCESS_CASES: &[(&str, &str)] = &[",
    ]
    for source, expected in cases:
        lines.extend(["    (", f"        {rust(source)},", f"        {rust(expected)},", "    ),"])
    lines.extend(["];", "", "#[rustfmt::skip]", "pub(super) static UTS46_ERROR_CASES: &[&str] = &["])
    lines.extend(f"    {rust(source)}," for source in error_cases)
    lines.extend(
        [
            "];",
            "",
            "#[rustfmt::skip]",
            "pub(super) static UTS46_TO_UNICODE_CASES: &[(&str, &str, bool)] = &[",
        ]
    )
    for source, expected, has_error in to_unicode_cases:
        lines.extend(
            [
                "    (",
                f"        {rust(source)},",
                f"        {rust(expected)},",
                f"        {str(has_error).lower()},",
                "    ),",
            ]
        )
    lines.extend(["];", ""])
    return "\n".join(lines)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true")
    parser.add_argument("--output", type=Path, default=OUTPUT)
    args = parser.parse_args()
    output = render()
    if args.check:
        if not args.output.is_file() or args.output.read_text(encoding="utf-8") != output:
            raise SystemExit("generated IDNA fixture vectors differ")
    else:
        args.output.write_text(output, encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
