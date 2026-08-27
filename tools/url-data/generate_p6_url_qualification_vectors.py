#!/usr/bin/env python3
"""Generate deterministic P6 URL facade qualification vectors from pinned inputs."""

from __future__ import annotations

import argparse
import csv
import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
ORACLE = ROOT / "test_data/url/oracle/locked-url-2.5.7.json"
INVENTORY = ROOT / "test_data/url/oracle/case-inventory.tsv"
SETTERS = ROOT / "test_data/url/wpt/setters_tests.json"
OUTPUT = ROOT / "src/values/uri/url/p6_vectors.rs"
BENCHMARK_OUTPUT = ROOT / "tools/url-data/url_benchmark_corpus.rs"
URLTESTDATA = ROOT / "test_data/url/wpt/urltestdata.json"


def rust(value: str) -> str:
    """Render a scalar-value Rust string literal."""
    parts: list[str] = ['"']
    for character in value:
        if character == "\\":
            parts.append("\\\\")
        elif character == '"':
            parts.append('\\"')
        elif character == "\n":
            parts.append("\\n")
        elif character == "\r":
            parts.append("\\r")
        elif character == "\t":
            parts.append("\\t")
        elif 0x20 <= ord(character) <= 0x7E:
            parts.append(character)
        else:
            parts.append(f"\\u{{{ord(character):x}}}")
    parts.append('"')
    return "".join(parts)


def optional(value: object) -> str:
    return "None" if value is None else f"Some({rust(str(value))})"


def optional_bool(value: object) -> str:
    return "None" if value is None else f"Some({str(bool(value)).lower()})"


def optional_u16(value: object) -> str:
    return "None" if value is None else f"Some({int(value)})"


def public_host(expected: dict[str, object]) -> object:
    """Map url's legacy host sentinel to the frozen Option-based public ABI."""
    value = expected.get("serialization")
    if expected.get("host_kind") == "NONE" and isinstance(value, str) and "://" not in value:
        return None
    if expected.get("host_kind") == "IPV6" and isinstance(value, str):
        authority = value.split("://", 1)[1].split("/", 1)[0].split("?", 1)[0].split("#", 1)[0]
        bracketed = authority.rsplit("@", 1)[-1]
        if bracketed.startswith("["):
            return bracketed.split("]", 1)[0] + "]"
    return expected.get("host")


def decode_input(record: dict[str, object]) -> str:
    return bytes.fromhex(str(record["input_utf8_hex"])).decode("utf-8")


def usv(value: str) -> str:
    """Apply the JavaScript USVString conversion used by the pinned WPT runner."""
    units = value.encode("utf-16", "surrogatepass")
    return units.decode("utf-16", "replace")


def inventory_rows() -> dict[str, dict[str, str]]:
    with INVENTORY.open(encoding="utf-8", newline="") as source:
        return {row["id"]: row for row in csv.DictReader(source, delimiter="\t")}


def setter_classification(attribute: str, href: str, expected_href: str) -> str:
    """Classify browser-only href replacement separately from mutator mappings."""
    if attribute == "href":
        return "BrowserOnlyHref"
    if attribute == "pathname" and "://" not in href:
        return "BrowserOnlyOpaquePath"
    return "Mapped"


def render_oracle() -> list[str]:
    records = json.loads(ORACLE.read_text(encoding="utf-8"))["records"]
    inventory = inventory_rows()
    lines = [
        "pub(super) struct OracleVector {",
        "    pub(super) id: &'static str,",
        "    pub(super) group: &'static str,",
        "    pub(super) input: &'static str,",
        "    pub(super) operation: &'static str,",
        "    pub(super) expected_success: bool,",
        "    pub(super) expected_value: Option<&'static str>,",
        "    pub(super) expected_scheme: Option<&'static str>,",
        "    pub(super) expected_username: Option<&'static str>,",
        "    pub(super) expected_password_present: Option<bool>,",
        "    pub(super) expected_host: Option<&'static str>,",
        "    pub(super) expected_port: Option<u16>,",
        "    pub(super) expected_path: Option<&'static str>,",
        "    pub(super) expected_query: Option<&'static str>,",
        "    pub(super) expected_query_pairs: &'static [(&'static str, &'static str)],",
        "    pub(super) expected_fragment: Option<&'static str>,",
        "    pub(super) allowed_schemes: &'static [&'static str],",
        "    pub(super) default_scheme: Option<&'static str>,",
        "    pub(super) allowed_schemes_supplied: bool,",
        "    pub(super) default_scheme_supplied: bool,",
        "    pub(super) repo_link_supplied: bool,",
        "    pub(super) repository_expected_success: Option<bool>,",
        "    pub(super) repository_expected_kind: Option<&'static str>,",
        "    pub(super) repository_expected_audience: Option<&'static str>,",
        "    pub(super) comparison_class: Option<&'static str>,",
        "}",
        "",
        f"pub(super) const ORACLE_CASE_COUNT: usize = {len(records)};",
        "#[rustfmt::skip]",
        "pub(super) static ORACLE_VECTORS: &[OracleVector] = &[",
    ]
    for record in records:
        expected = record["url257"]
        repository = record.get("repository_link")
        row = inventory[str(record["id"])]
        schemes = [item for item in row["allowed_schemes"].split(",") if item]
        scheme_literals = ", ".join(rust(item) for item in schemes)
        pair_literals = ", ".join(
            f"({rust(str(pair[0]))}, {rust(str(pair[1]))})"
            for pair in expected.get("query_pairs", [])
        )
        success = bool(expected["success"])
        lines.extend(
            [
                "    OracleVector {",
                f"        id: {rust(str(record['id']))},",
                f"        group: {rust(str(record['group']))},",
                f"        input: {rust(decode_input(record))},",
                f"        operation: {rust(row['operation'])},",
                f"        expected_success: {str(success).lower()},",
                f"        expected_value: {optional(expected.get('serialization'))},",
                f"        expected_scheme: {optional(expected.get('scheme'))},",
                f"        expected_username: {optional(expected.get('username'))},",
                f"        expected_password_present: {optional_bool(expected.get('password_present') if success else None)},",
                f"        expected_host: {optional(public_host(expected))},",
                f"        expected_port: {optional_u16(expected.get('explicit_port'))},",
                f"        expected_path: {optional(expected.get('path'))},",
                f"        expected_query: {optional(expected.get('raw_query'))},",
                f"        expected_query_pairs: &[{pair_literals}],",
                f"        expected_fragment: {optional(expected.get('fragment'))},",
                f"        allowed_schemes: &[{scheme_literals}],",
                f"        default_scheme: {optional(row['default_scheme'] if row['default_scheme_supplied'] == 'true' else None)},",
                f"        allowed_schemes_supplied: {row['allowed_schemes_supplied'].lower()},",
                f"        default_scheme_supplied: {row['default_scheme_supplied'].lower()},",
                f"        repo_link_supplied: {row['repo_link_supplied'].lower()},",
                f"        repository_expected_success: {optional_bool(repository.get('success') if repository else None)},",
                f"        repository_expected_kind: {optional(repository.get('error_kind') if repository and not repository.get('success') else None)},",
                f"        repository_expected_audience: {optional(repository.get('error_audience') if repository and not repository.get('success') else None)},",
                f"        comparison_class: {optional(row['comparison_class_id'] or None)},",
                "    },",
            ]
        )
    lines.append("];" )
    return lines


def render_setters() -> list[str]:
    fixture = json.loads(SETTERS.read_text(encoding="utf-8"))
    cases = [
        (attribute, case)
        for attribute, values in fixture.items()
        if attribute != "comment" and isinstance(values, list)
        for case in values
    ]
    lines = [
        "pub(super) enum SetterAttribute {",
        "    Scheme,",
        "    Username,",
        "    Password,",
        "    Host,",
        "    Hostname,",
        "    Port,",
        "    Path,",
        "    Query,",
        "    Fragment,",
        "    BrowserOnlyHref,",
        "}",
        "",
        "pub(super) enum SetterClassification {",
        "    Mapped,",
        "    BrowserOnlyHref,",
        "    BrowserOnlyOpaquePath,",
        "}",
        "",
        "pub(super) struct SetterVector {",
        "    pub(super) attribute: SetterAttribute,",
        "    pub(super) classification: SetterClassification,",
        "    pub(super) href: &'static str,",
        "    pub(super) new_value: &'static str,",
        "    pub(super) expected_href: &'static str,",
        "}",
        "",
        f"pub(super) const SETTER_SOURCE_CASE_COUNT: usize = {len(cases)};",
        f"pub(super) const SETTER_BROWSER_ONLY_CASE_COUNT: usize = {sum(1 for attribute, case in cases if setter_classification(attribute, str(case['href']), str(case['expected']['href'])) != 'Mapped')};",
        "#[rustfmt::skip]",
        "pub(super) static SETTER_VECTORS: &[SetterVector] = &[",
    ]
    names = {
        "protocol": "Scheme",
        "username": "Username",
        "password": "Password",
        "host": "Host",
        "hostname": "Hostname",
        "port": "Port",
        "pathname": "Path",
        "search": "Query",
        "hash": "Fragment",
        "href": "BrowserOnlyHref",
    }
    for attribute, case in cases:
        href = str(case["href"])
        expected_href = str(case["expected"]["href"])
        classification = setter_classification(attribute, href, expected_href)
        lines.extend(
            [
                "    SetterVector {",
                f"        attribute: SetterAttribute::{names[attribute]},",
                f"        classification: SetterClassification::{classification},",
                f"        href: {rust(href)},",
                f"        new_value: {rust(str(case['new_value']))},",
                f"        expected_href: {rust(expected_href)},",
                "    },",
            ]
        )
    lines.append("];" )
    return lines


def render() -> str:
    header = [
        "//! Generated P6 public-URL qualification vectors; do not edit.",
        "//! Inputs: P1A locked-url-2.5.7 oracle and pinned WPT setters_tests.json.",
        "//! Generator: tools/url-data/generate_p6_url_qualification_vectors.py.",
        "",
    ]
    return "\n".join(header + render_oracle() + [""] + render_setters() + [""])


def render_benchmark_corpus() -> str:
    """Render the 891 object-row inputs for the dependency-free throughput workload."""
    rows = json.loads(URLTESTDATA.read_text(encoding="utf-8"))
    inputs = [usv(str(row.get("input", ""))) for row in rows if isinstance(row, dict)]
    lines = [
        "// Generated pinned WPT URL input corpus for the P6 performance workload; do not edit.",
        "// Generator: tools/url-data/generate_p6_url_qualification_vectors.py.",
        f"pub const WPT_URLTESTDATA_CASE_COUNT: usize = {len(inputs)};",
        "#[rustfmt::skip]",
        "pub static WPT_URLTESTDATA_INPUTS: &[&str] = &[",
    ]
    lines.extend(f"    {rust(value)}," for value in inputs)
    lines.extend(["];", ""])
    return "\n".join(lines)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true")
    arguments = parser.parse_args()
    generated = render()
    benchmark = render_benchmark_corpus()
    if arguments.check:
        return 0 if (
            OUTPUT.is_file()
            and OUTPUT.read_text(encoding="utf-8") == generated
            and BENCHMARK_OUTPUT.is_file()
            and BENCHMARK_OUTPUT.read_text(encoding="utf-8") == benchmark
        ) else 1
    OUTPUT.write_text(generated, encoding="utf-8")
    BENCHMARK_OUTPUT.write_text(benchmark, encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
