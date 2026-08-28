#!/usr/bin/env python3
"""Dependency-free verification for pinned URL behavioral and generator data."""
import hashlib
from pathlib import Path

repository = Path(__file__).resolve().parents[2]
roots = (repository / "tests/data/url", repository / "tools/url-data/data")
verified = 0
for root in roots:
    records = (root / "CHECKSUMS.sha256").read_text(encoding="utf-8").splitlines()
    for line in records:
        digest, relative = line.split("  ", 1)
        actual = hashlib.sha256((root / relative).read_bytes()).hexdigest()
        if actual != digest:
            raise SystemExit(f"hash mismatch: {root / relative}")
        verified += 1
print("verified", verified, "approved artifacts")
