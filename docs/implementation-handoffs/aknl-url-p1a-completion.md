# AKNL URL P1A candidate

P1A adds approved pinned WPT/Unicode custody files, dependency-free custody verification, a locked-2.5.7 black-box oracle dataset, and package inclusion entries. URL/IDNA implementation, source replacement, ABI, diagnostics, P1B-P6, and final dependency removal remain out of scope.

The final cutover gate requires removing `url` from Cargo.toml, Cargo.lock, and cargo tree; removing its exclusive transitives and all product type use/source; and independently qualifying the OSS implementation against approved conformance suites.
