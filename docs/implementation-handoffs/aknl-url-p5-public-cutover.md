# AKNL-URL-P5 public URL cutover

P5 connects the accepted private P2–P4 URL engine to the frozen P1B public
contract. `URL` now owns a finalized private canonical record and exposes only
borrowed, absence-aware accessors. `try_new` and `try_resolve` enter the same
private parser/resolver, and `URLMutator` gathers copied component replacements
before one final parse-and-validation reconstruction.

`URL::value()` returns the complete canonical serialization and is explicitly
sensitive. Automatic `Debug` is exactly `URL([REDACTED])`; `URL` intentionally
has no `Display`. Public parsing, resolution, and mutation failures are
`InvalidInput` / `User` errors with the frozen redacted messages.

RepositoryLink now consumes the public facade, keeps its builder outcomes, and
uses exact redacted `Display` and `Debug`. Procedure obtains its persisted
canonical repository value through `valid_repo.url().value().to_owned()` rather
than automatic formatting. The external `url` dependency remains declared and
locked only until the separately authorized P6 removal; no P5 source uses it.

The public facade does not expose parser, record, host, IDNA, NFC, or external
crate types. It makes no layout, FFI, or binary-ABI promise.
