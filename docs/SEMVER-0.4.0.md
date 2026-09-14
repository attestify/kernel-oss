# Kernel OSS 0.4.0 semver report

## Public API change

`Error` gains private optional provenance state and therefore direct external
struct-literal construction is source-incompatible. The supported constructors
and existing audience, kind, message, display, and accessor behavior remain
available. `Error::new`, `Error::for_user`, and `Error::for_system` remain the
deliberate direct-construction exception for this small, infallible value; no
`ErrorBuilder`, builder setter, generic field setter, or initial provenance
input is introduced.

The release adds five public bounded provenance types and the following Error
methods:

- `try_with_attached_provenance`
- `try_with_derived_provenance`
- `without_provenance`
- `provenance`
- `has_current_provenance`

Every direct constructor starts with absent provenance. Attach, derive, and
clear return reconstructed immutable Errors rather than mutating an existing
Error or finalizing a builder.

## Intentional trait change

`Error` retains `Clone`, `Debug`, `PartialEq`, and `Eq`. Equality includes all
provenance state. `Hash` is intentionally removed so no caller can use a hash
identity that omits current or origin provenance.

## Compatibility boundary

This is a pre-1.0 minor-version source evolution. Consumers that construct
`Error` with a struct literal or use it as a hash key must migrate to the
supported constructors/accessors or an explicit caller-owned key. This report
does not claim downstream consumer compatibility or publication status.
