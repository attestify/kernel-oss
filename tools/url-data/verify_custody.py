#!/usr/bin/env python3
"""Dependency-free verification for approved URL P1A custody files."""
import hashlib
from pathlib import Path

root = Path(__file__).resolve().parents[2] / "test_data" / "url"
for line in (root / "CHECKSUMS.sha256").read_text().splitlines():
    digest, relative = line.split("  ", 1)
    actual = hashlib.sha256((root / relative).read_bytes()).hexdigest()
    if actual != digest:
        raise SystemExit(f"hash mismatch: {relative}")
print("verified", len((root / "CHECKSUMS.sha256").read_text().splitlines()), "approved artifacts")
