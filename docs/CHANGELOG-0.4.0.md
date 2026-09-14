# Kernel OSS 0.4.0 changelog

## Added

- Immutable bounded error-provenance values:
  `ErrorProvenanceNamespace`, `ErrorProvenanceCode`,
  `ErrorProvenanceSchemaVersion`, `ErrorProvenanceDescriptor`, and
  `ErrorProvenance`.
- `Error` reconstruction methods for attaching one current descriptor,
  deriving one new current descriptor with one origin, inspecting provenance,
  and clearing provenance.
- Public Rustdoc, executable doctests, and an `error_provenance` example for
  the bounded provenance path.

## Changed

- `Error` equality now explicitly includes its optional current and origin
  provenance state.
- `Error` intentionally no longer implements `Hash`; a hash identity that
  omitted provenance would be semantically incomplete.
- `Error` retains its normal `Result<Response, Error>` failure carriage. No
  response wrapper or alternate failure channel is introduced.

## Excluded

This source change adds no registries, arbitrary metadata, dynamic payloads,
source chains, HTTP values, vendor diagnostics, SQLSTATE values, or
product-specific descriptors.
