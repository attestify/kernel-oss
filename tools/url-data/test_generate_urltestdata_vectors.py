#!/usr/bin/env python3
"""Unit tests for the private URL WPT vector generator."""

import importlib.util
import unittest
from pathlib import Path


SCRIPT = Path(__file__).with_name("generate_urltestdata_vectors.py")
SPEC = importlib.util.spec_from_file_location("generate_urltestdata_vectors", SCRIPT)
MODULE = importlib.util.module_from_spec(SPEC)
assert SPEC and SPEC.loader
SPEC.loader.exec_module(MODULE)


class GeneratorTests(unittest.TestCase):
    """Verifies USV conversion and deterministic vector rendering."""

    def test_unpaired_surrogate_becomes_replacement_success(self) -> None:
        self.assertEqual(MODULE.usv("a" + chr(0xD800) + "b"), "a\ufffdb")

    def test_non_ascii_becomes_rust_scalar_escape_success(self) -> None:
        self.assertEqual(MODULE.rust_string("é"), '"\\u{e9}"')

    def test_render_is_deterministic_success(self) -> None:
        self.assertEqual(MODULE.render(), MODULE.render())


if __name__ == "__main__":
    unittest.main()
