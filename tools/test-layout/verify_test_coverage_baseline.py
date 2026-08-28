#!/usr/bin/env python3
"""Verify that the Kernel OSS test-layout migration preserves test discovery."""

from __future__ import annotations

import csv
import re
import subprocess
import sys
from collections import Counter
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
BASELINE = ROOT / "tests" / "test-coverage-baseline.tsv"
RUST_TEST_FLOOR = 493
DOCTEST_FLOOR = 30
PYTHON_TEST_FLOOR = 18
TEST_LINE = re.compile(r"^(.+): test$")
PYTHON_TEST = re.compile(r"^\s+def (test_[A-Za-z0-9_]+)\(")


def try_run(command: list[str]) -> str:
    completed = subprocess.run(
        command,
        cwd=ROOT,
        check=False,
        capture_output=True,
        text=True,
    )
    if completed.returncode != 0:
        print(completed.stdout, end="", file=sys.stderr)
        print(completed.stderr, end="", file=sys.stderr)
        raise RuntimeError(
            f"command exited {completed.returncode}: {' '.join(command)}"
        )
    return f"{completed.stdout}\n{completed.stderr}"


def discovered_rust_tests() -> list[str]:
    output = try_run(
        [
            "cargo",
            "test",
            "--all-targets",
            "--locked",
            "--offline",
            "--",
            "--list",
        ]
    )
    return [
        match.group(1)
        for line in output.splitlines()
        if (match := TEST_LINE.match(line)) is not None
    ]


def discovered_doctests() -> list[str]:
    output = try_run(
        ["cargo", "test", "--doc", "--locked", "--offline", "--", "--list"]
    )
    return [
        match.group(1)
        for line in output.splitlines()
        if (match := TEST_LINE.match(line)) is not None
    ]


def discovered_python_tests() -> Counter[str]:
    discovered: Counter[str] = Counter()
    for path in sorted(ROOT.glob("tools/**/test_*.py")):
        for line in path.read_text(encoding="utf-8").splitlines():
            match = PYTHON_TEST.match(line)
            if match is not None:
                discovered[match.group(1)] += 1
    return discovered


def baseline_rows() -> list[dict[str, str]]:
    with BASELINE.open(newline="", encoding="utf-8") as source:
        rows = list(csv.DictReader(source, delimiter="\t"))
    required = {
        "test_id",
        "language",
        "current_name",
        "stable_name",
        "match_mode",
        "current_tier",
        "target_tier",
        "disposition",
    }
    if not rows or set(rows[0]) != required:
        raise ValueError("test coverage baseline has an unexpected schema")
    if len({row["test_id"] for row in rows}) != len(rows):
        raise ValueError("test coverage baseline contains duplicate test IDs")
    return rows


def verify_rust_rows(rows: list[dict[str, str]], discovered: list[str]) -> list[str]:
    failures: list[str] = []
    discovered_names = set(discovered)
    discovered_basenames = Counter(name.rsplit("::", 1)[-1] for name in discovered)

    for row in rows:
        if row["language"] != "rust":
            continue
        if row["match_mode"] == "qualified":
            present = row["current_name"] in discovered_names
        elif row["match_mode"] == "stable_name":
            present = discovered_basenames[row["stable_name"]] == 1
        else:
            failures.append(f"{row['test_id']}: unknown match mode")
            continue
        if not present:
            failures.append(
                f"{row['test_id']}: missing Rust test {row['current_name']}"
            )
    return failures


def verify_python_rows(
    rows: list[dict[str, str]], discovered: Counter[str]
) -> list[str]:
    expected = Counter(
        row["stable_name"] for row in rows if row["language"] == "python"
    )
    return [
        f"missing Python generator test {name}: expected {count}, found {discovered[name]}"
        for name, count in expected.items()
        if discovered[name] < count
    ]


def main() -> int:
    rows = baseline_rows()
    rust_tests = discovered_rust_tests()
    doctests = discovered_doctests()
    python_tests = discovered_python_tests()

    failures = verify_rust_rows(rows, rust_tests)
    failures.extend(verify_python_rows(rows, python_tests))
    if len(rust_tests) < RUST_TEST_FLOOR:
        failures.append(
            f"Rust test floor failed: expected at least {RUST_TEST_FLOOR}, "
            f"found {len(rust_tests)}"
        )
    if len(doctests) < DOCTEST_FLOOR:
        failures.append(
            f"doctest floor failed: expected at least {DOCTEST_FLOOR}, "
            f"found {len(doctests)}"
        )
    if sum(python_tests.values()) < PYTHON_TEST_FLOOR:
        failures.append(
            f"Python test floor failed: expected at least {PYTHON_TEST_FLOOR}, "
            f"found {sum(python_tests.values())}"
        )

    if failures:
        for failure in failures:
            print(f"FAIL: {failure}", file=sys.stderr)
        return 1

    affected_rust = sum(row["language"] == "rust" for row in rows)
    affected_python = sum(row["language"] == "python" for row in rows)
    print(
        "PASS "
        f"rust={len(rust_tests)} "
        f"affected_rust={affected_rust} "
        f"doctests={len(doctests)} "
        f"python={sum(python_tests.values())} "
        f"affected_python={affected_python}"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
