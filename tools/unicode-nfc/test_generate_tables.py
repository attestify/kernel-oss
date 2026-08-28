"""Offline NFC generator coverage for the permanent custody contract.

The controlling generation and coverage requirements are documented in
``docs/nfc-text.md`` and ``docs/testing.md``.
"""

from __future__ import annotations

import hashlib
import importlib.util
from pathlib import Path
import tempfile
import unittest


GENERATOR_PATH = Path(__file__).with_name("generate_tables.py")
SPECIFICATION = importlib.util.spec_from_file_location("generate_tables", GENERATOR_PATH)
assert SPECIFICATION is not None and SPECIFICATION.loader is not None
GENERATOR = importlib.util.module_from_spec(SPECIFICATION)
SPECIFICATION.loader.exec_module(GENERATOR)


def unicode_record(scalar: str, combining_class: str, decomposition: str) -> str:
    """Return a minimal structurally valid UnicodeData record."""
    fields = [scalar, "TEST", "Lo", combining_class, "L", decomposition, "", "", "", "N", "", "", "", "", ""]
    return ";".join(fields) + "\n"


class GeneratorTests(unittest.TestCase):
    """Committed-source, checksum, census, and reproduction qualification."""

    def write_fixture(self, directory: Path) -> tuple[Path, Path, Path]:
        """Create exact temporary source data and its checksum manifest."""
        input_directory = directory / "inputs"
        input_directory.mkdir()
        unicode_data = input_directory / "UnicodeData-17.0.0.txt"
        unicode_data.write_text(
            unicode_record("0041", "0", "")
            + unicode_record("00C5", "0", "0041 030A")
            + unicode_record("212B", "0", "00C5")
            + unicode_record("00A0", "0", "<noBreak> 0020")
            + unicode_record("030A", "230", ""),
            encoding="utf-8",
        )
        properties = input_directory / "DerivedNormalizationProps-17.0.0.txt"
        properties.write_text("00C5 ; Full_Composition_Exclusion\n", encoding="utf-8")
        normalization_test = input_directory / "NormalizationTest-17.0.0.txt"
        normalization_test.write_text("# fixture\n", encoding="utf-8")
        license_path = directory / "Unicode-3.0.txt"
        license_path.write_text("Unicode license fixture\n", encoding="utf-8")
        checksum_path = directory / "checksums.sha256"
        files = {
            "UnicodeData-17.0.0.txt": unicode_data,
            "DerivedNormalizationProps-17.0.0.txt": properties,
            "NormalizationTest-17.0.0.txt": normalization_test,
            "Unicode-3.0.txt": license_path,
        }
        checksum_path.write_text(
            "".join(
                f"{hashlib.sha256(path.read_bytes()).hexdigest()}  {name}\n"
                for name, path in files.items()
            ),
            encoding="utf-8",
        )
        return input_directory, license_path, checksum_path

    def test_checksum_mismatch_error(self) -> None:
        """Rejects a Unicode source whose committed checksum does not match."""
        with tempfile.TemporaryDirectory() as temporary_directory:
            input_directory, license_path, checksum_path = self.write_fixture(Path(temporary_directory))
            (input_directory / "UnicodeData-17.0.0.txt").write_text("changed\n", encoding="utf-8")
            with self.assertRaisesRegex(ValueError, "checksum mismatch"):
                GENERATOR.generate(input_directory, license_path, checksum_path)

    def test_malformed_unicode_data_error(self) -> None:
        """Rejects malformed Unicode data instead of generating partial tables."""
        with tempfile.TemporaryDirectory() as temporary_directory:
            input_directory, license_path, checksum_path = self.write_fixture(Path(temporary_directory))
            unicode_data = input_directory / "UnicodeData-17.0.0.txt"
            unicode_data.write_text("not;a;unicode;record\n", encoding="utf-8")
            checksum_path.write_text(
                "".join(
                    f"{hashlib.sha256(path.read_bytes()).hexdigest()}  {name}\n"
                    for name, path in {
                        "UnicodeData-17.0.0.txt": unicode_data,
                        "DerivedNormalizationProps-17.0.0.txt": input_directory
                        / "DerivedNormalizationProps-17.0.0.txt",
                        "NormalizationTest-17.0.0.txt": input_directory / "NormalizationTest-17.0.0.txt",
                        "Unicode-3.0.txt": license_path,
                    }.items()
                ),
                encoding="utf-8",
            )
            with self.assertRaisesRegex(ValueError, "malformed UnicodeData"):
                GENERATOR.generate(input_directory, license_path, checksum_path)

    def test_all_verified_input_checksums_are_rendered_success(self) -> None:
        """Renders every verified source checksum into the generated header."""
        with tempfile.TemporaryDirectory() as temporary_directory:
            input_directory, license_path, checksum_path = self.write_fixture(Path(temporary_directory))
            rendered = GENERATOR.generate(input_directory, license_path, checksum_path)
            for name, digest in GENERATOR.read_checksums(checksum_path).items():
                self.assertIn(f"// {name} SHA-256: {digest}", rendered)

    def test_canonical_only_exclusion_and_repeat_generation_success(self) -> None:
        """Proves canonical-only exclusions and byte-identical regeneration."""
        with tempfile.TemporaryDirectory() as temporary_directory:
            input_directory, license_path, checksum_path = self.write_fixture(Path(temporary_directory))
            first = GENERATOR.generate(input_directory, license_path, checksum_path)
            second = GENERATOR.generate(input_directory, license_path, checksum_path)
            self.assertEqual(first, second)
            self.assertIn("(0x212B, &[0x00C5])", first)
            self.assertNotIn("0x00A0, &[", first)
            self.assertNotIn("(0x0041, 0x030A, 0x00C5)", first)

    def test_committed_unicode_sources_have_expected_nfc_census_success(self) -> None:
        """Verifies the real committed Unicode 17.0 inputs and generated NFC-table
        census without altering generator behavior or fetching source data.
        """
        input_directory = GENERATOR.DEFAULT_INPUT_DIRECTORY
        unicode_data = input_directory / "UnicodeData-17.0.0.txt"
        properties = input_directory / "DerivedNormalizationProps-17.0.0.txt"

        unicode_data_records = unicode_data.read_text(encoding="utf-8").splitlines()
        property_records = [
            line
            for line in properties.read_text(encoding="utf-8").splitlines()
            if line.split("#", 1)[0].strip()
        ]
        combining_classes, decompositions = GENERATOR.parse_unicode_data(unicode_data)
        exclusions = GENERATOR.parse_full_composition_exclusions(properties)
        compositions = GENERATOR.composition_pairs(decompositions, exclusions)
        exclusion_records = [
            line
            for line in property_records
            if line.split(";", 1)[1].split("#", 1)[0].strip()
            == "Full_Composition_Exclusion"
        ]

        self.assertEqual(len(unicode_data_records), 40_575)
        self.assertEqual(len(property_records), 16_172)
        self.assertEqual(len(exclusion_records), 74)
        self.assertEqual(len(exclusions), 1_120)
        self.assertEqual(len(combining_classes), 968)
        self.assertEqual(len(decompositions), 2_081)
        self.assertEqual(len(compositions), 961)


if __name__ == "__main__":
    unittest.main()
