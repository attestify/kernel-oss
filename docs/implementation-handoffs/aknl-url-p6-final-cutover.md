# AKNL URL P6 final cutover

Kernel OSS 0.3.0 removes the temporary external `url` dependency. `URL`,
`URLMutator`, RepositoryLink, and Procedure use the accepted private P2–P4
WHATWG engine only; Cargo resolves no URL, IDNA, Unicode, percent-encoding, or
form-processing dependency for the product.

Permanent dependency-free qualification consumes the frozen 190-case P1A
compatibility oracle and the pinned WPT setters fixture. The oracle compares
public parse outcomes, canonical values, borrowed components, ordered form
pairs, equality/hash classes, RepositoryLink outcomes, and redacted security
boundaries while preserving the P1B/P5 absence-aware and redacted API contract.
The setters adapter maps 264 representable browser setter cases to immutable
`URLMutator` operations. It classifies 14 browser-only shapes outside that API:
the one `href` replacement and 13 opaque-path pathname cases.

The crate retains the approved WPT and Unicode fixtures, licenses, source
records, deterministic generators, and frozen oracle. The package include scope
keeps current handoffs and documentation while excluding historical raw evidence
directories and metadata junk. No public parser, record, host, IDNA, NFC, or
external URL type is exposed.
