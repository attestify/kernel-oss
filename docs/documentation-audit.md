# Kernel OSS Documentation Audit

Date: 2026-06-17

This audit records the current documentation state after adding the human
catalog. It is scoped to `kernel-oss`.

## Documentation Surfaces Audited

| Surface | Status | Notes |
| --- | --- | --- |
| `README.md` | Updated | Links to `docs/kernel-catalog.md` and remains the short project entry point. |
| `docs/kernel-catalog.md` | Added | Human-readable reuse catalog for values, entities, requests, responses, use cases, gateways, compatibility, and planned moves. |
| `examples/` | Current | Examples teach value/entity, sync use case/gateway, async use case/gateway, and `Response = ()` patterns. |
| Crate root rustdoc | Updated | `src/lib.rs` now explains crate purpose and points to the catalog, README, and examples. |
| Major module rustdoc | Started | Added entry-point docs for `algorithms`, `core`, `entity`, `error`, `gateway`, `response`, `usecase`, and `values`. |
| Intra-doc links | Clean | `cargo rustdoc --lib` runs without broken-link warnings after this pass. |
| Full missing-docs enforcement | Clean | `cargo rustdoc --lib -- -D missing_docs` now passes across the public crate API. |

## Standards Alignment Completed

- Added a catalog that answers whether a reusable kernel type or seam already exists.
- Documented active shared role traits in the catalog.
- Documented current gateway migration replacements in the catalog.
- Documented deprecated APIs separately instead of presenting them as the preferred path.
- Documented planned moves so domain/application crates do not copy temporary kernel APIs.
- Added top-level rustdoc for the crate and major public modules.
- Fixed existing broken intra-doc links reported by `cargo rustdoc --lib`.
- Packaged `/docs/**` so the catalog is included with the crate package.

## Remaining Documentation Updates

1. Catalog maintenance
   - Update `docs/kernel-catalog.md` whenever a public reusable value object, entity, request, response, use case seam, or gateway seam is added, deprecated, moved, or removed.
   - Keep deprecated APIs in the compatibility section, not the preferred-use tables.

2. Ongoing maintenance
   - Keep new public APIs documented as they are added.
   - Keep deprecated compatibility APIs documented with the replacement path and planned removal timing.
   - Keep the handoff and standards-delta notes aligned with the current release decision.

## Recommended Cleanup Order

1. Keep `docs/kernel-catalog.md` current during release work.
2. Keep public docs current for any new or changed export.
3. Keep deprecated compatibility docs aligned with their replacement paths.
4. Maintain `cargo rustdoc --lib -- -D missing_docs` as a release check.
5. Consider stricter rustdoc enforcement in CI if the team wants a hard gate for future additions.

## Verification Commands

Useful documentation checks:

```text
cargo rustdoc --lib
cargo rustdoc --examples
cargo rustdoc --lib -- -D missing_docs
```

Current expected state:

- `cargo rustdoc --lib` passes.
- `cargo rustdoc --lib -- -D missing_docs` passes.
