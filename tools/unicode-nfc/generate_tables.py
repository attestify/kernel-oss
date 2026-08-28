#!/usr/bin/env python3
"""Generate the private Unicode 17.0 NFC tables without network access.

The generator accepts only committed official Unicode source files. It verifies
their checksums before deriving canonical decomposition, non-zero canonical
combining class, and permitted canonical composition data.
"""

from __future__ import annotations

import argparse
import hashlib
from pathlib import Path
import sys


ROOT = Path(__file__).resolve().parents[2]
DEFAULT_INPUT_DIRECTORY = ROOT / "tools/unicode-nfc/data"
DEFAULT_NORMALIZATION_TEST = (
    ROOT / "tests/data/nfc/unicode/NormalizationTest-17.0.0.txt"
)
DEFAULT_LICENSE = ROOT / "third-party/licenses/Unicode-3.0.txt"
DEFAULT_CHECKSUMS = Path(__file__).with_name("checksums.sha256")
DEFAULT_OUTPUT = ROOT / "src/values/text/nfc_text/normalization/tables.rs"
UNICODE_VERSION = "17.0.0"


def read_checksums(path: Path) -> dict[str, str]:
    """Return the expected SHA-256 values from one strict checksum file."""
    expected: dict[str, str] = {}
    for line_number, line in enumerate(path.read_text(encoding="utf-8").splitlines(), start=1):
        if not line:
            continue
        fields = line.split()
        if len(fields) != 2 or len(fields[0]) != 64:
            raise ValueError(f"malformed checksum record at line {line_number}")
        digest, name = fields
        if any(character not in "0123456789abcdef" for character in digest):
            raise ValueError(f"malformed checksum digest at line {line_number}")
        if name in expected:
            raise ValueError(f"duplicate checksum file name: {name}")
        expected[name] = digest
    return expected


def verify_inputs(
    input_directory: Path,
    license_path: Path,
    checksum_path: Path,
    normalization_test_path: Path | None = None,
) -> dict[str, str]:
    """Verify and return checksums for all required offline source files."""
    expected = read_checksums(checksum_path)
    paths = {
        "UnicodeData-17.0.0.txt": input_directory / "UnicodeData-17.0.0.txt",
        "DerivedNormalizationProps-17.0.0.txt": input_directory
        / "DerivedNormalizationProps-17.0.0.txt",
        "NormalizationTest-17.0.0.txt": normalization_test_path
        or input_directory / "NormalizationTest-17.0.0.txt",
        "Unicode-3.0.txt": license_path,
    }
    if set(expected) != set(paths):
        raise ValueError("checksum file does not name exactly the required Unicode inputs")

    observed: dict[str, str] = {}
    for name, path in paths.items():
        if not path.is_file():
            raise FileNotFoundError(f"missing required Unicode input: {path}")
        digest = hashlib.sha256(path.read_bytes()).hexdigest()
        if digest != expected[name]:
            raise ValueError(f"checksum mismatch for {name}")
        observed[name] = digest
    return observed


def parse_unicode_data(path: Path) -> tuple[dict[int, int], dict[int, tuple[int, ...]]]:
    """Read only NFC-relevant combining classes and canonical decompositions."""
    combining_classes: dict[int, int] = {}
    decompositions: dict[int, tuple[int, ...]] = {}
    for line_number, line in enumerate(path.read_text(encoding="utf-8").splitlines(), start=1):
        fields = line.split(";")
        if len(fields) != 15:
            raise ValueError(f"malformed UnicodeData record at line {line_number}")
        try:
            scalar = int(fields[0], 16)
            combining_class = int(fields[3])
        except ValueError as error:
            raise ValueError(f"invalid scalar or combining class at line {line_number}") from error
        if not 0 <= scalar <= 0x10FFFF:
            raise ValueError(f"invalid Unicode scalar at line {line_number}")
        # UnicodeData includes surrogate-code-point records. They are not Rust
        # Unicode scalars and cannot contribute to NFC runtime tables.
        if 0xD800 <= scalar <= 0xDFFF:
            continue
        if not 0 <= combining_class <= 255:
            raise ValueError(f"invalid canonical combining class at line {line_number}")
        if combining_class:
            combining_classes[scalar] = combining_class

        decomposition = fields[5].strip()
        if decomposition and not decomposition.startswith("<"):
            try:
                mapping = tuple(int(part, 16) for part in decomposition.split())
            except ValueError as error:
                raise ValueError(f"invalid decomposition at line {line_number}") from error
            if not mapping or any(
                scalar_value > 0x10FFFF or 0xD800 <= scalar_value <= 0xDFFF
                for scalar_value in mapping
            ):
                raise ValueError(f"invalid canonical decomposition at line {line_number}")
            decompositions[scalar] = mapping
    return combining_classes, decompositions


def parse_full_composition_exclusions(path: Path) -> set[int]:
    """Read the Full_Composition_Exclusion property from Unicode data."""
    exclusions: set[int] = set()
    for line_number, raw_line in enumerate(path.read_text(encoding="utf-8").splitlines(), start=1):
        line = raw_line.split("#", 1)[0].strip()
        if not line:
            continue
        fields = [field.strip() for field in line.split(";")]
        if len(fields) < 2:
            raise ValueError(f"malformed normalization-property record at line {line_number}")
        if fields[1] != "Full_Composition_Exclusion":
            continue
        if len(fields) != 2:
            raise ValueError(f"malformed composition-exclusion record at line {line_number}")
        try:
            bounds = [int(part, 16) for part in fields[0].split("..")]
        except ValueError as error:
            raise ValueError(f"invalid composition-exclusion scalar at line {line_number}") from error
        if len(bounds) == 1:
            start = end = bounds[0]
        elif len(bounds) == 2:
            start, end = bounds
        else:
            raise ValueError(f"invalid composition-exclusion range at line {line_number}")
        if start > end or start < 0 or end > 0x10FFFF:
            raise ValueError(f"invalid composition-exclusion range at line {line_number}")
        exclusions.update(range(start, end + 1))
    return exclusions


def composition_pairs(
    decompositions: dict[int, tuple[int, ...]], exclusions: set[int]
) -> dict[tuple[int, int], int]:
    """Return only canonical pairs permitted by Full_Composition_Exclusion."""
    pairs: dict[tuple[int, int], int] = {}
    for composed, mapping in decompositions.items():
        if len(mapping) != 2 or composed in exclusions:
            continue
        pair = (mapping[0], mapping[1])
        if pair in pairs:
            raise ValueError(f"duplicate canonical composition pair: {pair!r}")
        pairs[pair] = composed
    return pairs


def render_tables(
    checksums: dict[str, str],
    combining_classes: dict[int, int],
    decompositions: dict[int, tuple[int, ...]],
    compositions: dict[tuple[int, int], int],
) -> str:
    """Render deterministic passive Rust data used solely by the private engine."""
    lines = [
        "// SPDX-License-Identifier: Unicode-3.0",
        "// Generated by tools/unicode-nfc/generate_tables.py; DO NOT EDIT.",
        f"// Unicode version: {UNICODE_VERSION}.",
        f"// UnicodeData-17.0.0.txt SHA-256: {checksums['UnicodeData-17.0.0.txt']}",
        "// DerivedNormalizationProps-17.0.0.txt SHA-256: "
        f"{checksums['DerivedNormalizationProps-17.0.0.txt']}",
        "// NormalizationTest-17.0.0.txt SHA-256: "
        f"{checksums['NormalizationTest-17.0.0.txt']}",
        "// Unicode-3.0.txt SHA-256: " f"{checksums['Unicode-3.0.txt']}",
        "",
        "pub(super) static CANONICAL_COMBINING_CLASSES: &[(u32, u8)] = &[",
    ]
    lines.extend(f"    (0x{scalar:04X}, {combining_class})," for scalar, combining_class in sorted(combining_classes.items()))
    lines.extend(["];", "", "pub(super) static CANONICAL_DECOMPOSITIONS: &[(u32, &[u32])] = &["])
    for scalar, mapping in sorted(decompositions.items()):
        rendered_mapping = ", ".join(f"0x{part:04X}" for part in mapping)
        lines.append(f"    (0x{scalar:04X}, &[{rendered_mapping}]),")
    lines.extend(["];", "", "pub(super) static CANONICAL_COMPOSITIONS: &[(u32, u32, u32)] = &["])
    for (first, second), composed in sorted(compositions.items()):
        lines.append(f"    (0x{first:04X}, 0x{second:04X}, 0x{composed:04X}),")
    lines.extend(["];", ""])
    return "\n".join(lines)


def generate(
    input_directory: Path,
    license_path: Path,
    checksum_path: Path,
    normalization_test_path: Path | None = None,
) -> str:
    """Verify inputs and produce the deterministic NFC table text."""
    checksums = verify_inputs(
        input_directory,
        license_path,
        checksum_path,
        normalization_test_path,
    )
    combining_classes, decompositions = parse_unicode_data(input_directory / "UnicodeData-17.0.0.txt")
    exclusions = parse_full_composition_exclusions(
        input_directory / "DerivedNormalizationProps-17.0.0.txt"
    )
    compositions = composition_pairs(decompositions, exclusions)
    return render_tables(checksums, combining_classes, decompositions, compositions)


def main(argv: list[str]) -> int:
    """Write generated tables, or verify that the checked-in output is exact."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true", help="fail if output differs")
    parser.add_argument("--input-dir", type=Path, default=DEFAULT_INPUT_DIRECTORY)
    parser.add_argument(
        "--normalization-test", type=Path, default=DEFAULT_NORMALIZATION_TEST
    )
    parser.add_argument("--license", type=Path, default=DEFAULT_LICENSE)
    parser.add_argument("--checksums", type=Path, default=DEFAULT_CHECKSUMS)
    parser.add_argument("--output", type=Path, default=DEFAULT_OUTPUT)
    args = parser.parse_args(argv)

    tables = generate(
        args.input_dir,
        args.license,
        args.checksums,
        args.normalization_test,
    )
    if args.check:
        if not args.output.is_file() or args.output.read_text(encoding="utf-8") != tables:
            print("generated NFC tables differ; run tools/unicode-nfc/generate_tables.py", file=sys.stderr)
            return 1
        return 0

    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(tables, encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
