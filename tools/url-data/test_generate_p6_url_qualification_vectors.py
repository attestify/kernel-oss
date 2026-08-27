#!/usr/bin/env python3
"""Unit tests for deterministic P6 URL qualification vector generation."""

from __future__ import annotations

import importlib.util
import json
from pathlib import Path
import unittest


SCRIPT = Path(__file__).with_name("generate_p6_url_qualification_vectors.py")
SPEC = importlib.util.spec_from_file_location("p6_vectors", SCRIPT)
assert SPEC is not None and SPEC.loader is not None
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)


class GenerateP6UrlQualificationVectorsTests(unittest.TestCase):
    """Exercises the committed, dependency-free P6 vector renderer."""

    def test_oracle_count_success(self) -> None:
        """Preserves all 190 frozen oracle records."""
        data = json.loads(MODULE.ORACLE.read_text(encoding="utf-8"))
        self.assertEqual(len(data["records"]), 190)
        self.assertIn("ORACLE_CASE_COUNT: usize = 190", MODULE.render())

    def test_setter_count_success(self) -> None:
        """Preserves every non-comment pinned WPT setter row."""
        data = json.loads(MODULE.SETTERS.read_text(encoding="utf-8"))
        count = sum(len(value) for key, value in data.items() if key != "comment" and isinstance(value, list))
        self.assertEqual(count, 278)
        self.assertIn("SETTER_SOURCE_CASE_COUNT: usize = 278", MODULE.render())

    def test_committed_output_is_reproducible_success(self) -> None:
        """Requires the committed generated Rust source to match the renderer."""
        self.assertEqual(MODULE.OUTPUT.read_text(encoding="utf-8"), MODULE.render())

    def test_benchmark_corpus_success(self) -> None:
        """Preserves all pinned WPT object-row inputs for throughput qualification."""
        rows = json.loads(MODULE.URLTESTDATA.read_text(encoding="utf-8"))
        self.assertEqual(sum(isinstance(row, dict) for row in rows), 891)
        self.assertEqual(MODULE.BENCHMARK_OUTPUT.read_text(encoding="utf-8"), MODULE.render_benchmark_corpus())


if __name__ == "__main__":
    unittest.main()
