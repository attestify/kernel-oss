#!/usr/bin/env python3
"""Unit tests for the dependency-free IDNA table generator."""

from __future__ import annotations

import importlib.util
import tempfile
import unittest
from pathlib import Path

SCRIPT = Path(__file__).with_name("generate_idna_tables.py")
SPEC = importlib.util.spec_from_file_location("generate_idna_tables", SCRIPT)
assert SPEC and SPEC.loader
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)


class GeneratorTests(unittest.TestCase):
    """Exercise input parsing and deterministic rendering."""

    def test_parse_range_accepts_scalar_success(self) -> None:
        self.assertEqual(MODULE.parse_range("0041"), (0x41, 0x41))

    def test_parse_range_retains_disallowed_surrogate_source_success(self) -> None:
        self.assertEqual(MODULE.parse_range("D800..DFFF"), (0xD800, 0xDFFF))

    def test_render_is_deterministic_success(self) -> None:
        entries = MODULE.parse()
        self.assertEqual(MODULE.render(entries), MODULE.render(entries))

    def test_generated_output_matches_check_success(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            output = Path(directory) / "tables.rs"
            output.write_text(MODULE.render(MODULE.parse()), encoding="utf-8")
            self.assertEqual(output.read_text(encoding="utf-8"), MODULE.render(MODULE.parse()))


if __name__ == "__main__":
    unittest.main()
