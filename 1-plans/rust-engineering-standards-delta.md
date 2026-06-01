# Rust Engineering Standards Delta

Date: 2026-05-31

Scope:

- Compare `kernel-oss` against the Rust engineering standards in `specifications/engineering-standards/7-specifications/rust`.
- Track actionable deltas before implementation work begins.

Baseline:

- `cargo test` passes: 294 unit tests and 11 doctests.
- No source changes were made during the initial audit.

## Applicable Standards

Most relevant to the current kernel:

- Rust value object specification
- Rust construction specification
- Rust accessor and state surface specification
- Rust gateway specification
- Rust macro usage specification
- Rust testing standard
- Kernel error handling standard
- Rust documentation specification

Mostly out of scope until this crate adds those layers:

- use case specifications
- domain event specifications
- decorators
- service runtime
- benchmark workload and review specifications

## Initial Delta

1. Public state surfaces
   - Many bounded public types expose public fields.
   - Action: add callable accessors first, then decide whether field privacy is a breaking-release change.
   - Status: callable accessors added while preserving public fields for compatibility. Verified with `cargo fmt --check` and `cargo test`.

2. Construction naming
   - Several fallible constructors use `new(...) -> Result`.
   - Several fallible builders use `build(...) -> Result`.
   - Action: add `try_new` / `try_build` surfaces, then migrate call sites.

3. Builder finalization
   - Some `try_build` methods borrow `self` instead of consuming the builder.
   - Action: migrate builders to consuming `self` where compatible with existing usage.

4. Gateway contracts
   - Current gateway contracts use operation-specific methods and type aliases.
   - Standards expect `*GW` traits, explicit request/response types, and one `execute(request)` method.
   - Action: handle after value-object and construction cleanup because this is likely the most disruptive API change.

5. Test standard
   - Tests pass but contain widespread `.unwrap()` usage.
   - Many test names do not end in `_success`, `_error`, `_success_async`, or `_error_async`.
   - Action: migrate tests module by module using project test macros and `.expect(...)` for setup.

6. Production unwrap/expect usage
   - Production code still has a few `.unwrap()` / `.expect(...)` sites.
   - Action: replace recoverable paths with explicit bounded error handling.

7. Documentation
   - `cargo rustdoc --lib -- -D missing_docs` fails broadly.
   - Rustdoc reports broken intra-doc links.
   - Action: add public API docs incrementally by module after API shape decisions settle.

8. Clippy
   - `cargo clippy --all-targets -- -D warnings` fails.
   - Action: address mechanical clippy issues after API decisions that may remove some code.

## Suggested Order

1. Add missing accessors while keeping public fields temporarily.
2. Add foundational shared traits and implement them on the obvious kernel types.
3. Add `try_new` / `try_build` aliases beside existing APIs.
4. Remove production `.unwrap()` / `.expect(...)` where behavior can stay unchanged.
5. Migrate tests to standard names and no `.unwrap()`.
6. Make public field privacy decisions.
7. Refactor gateway contracts.
8. Bring docs and clippy to enforceable state.

## Specification Check, Shared Traits

The Rust specifications already define several of the shared trait shapes we want in the kernel.

Already specified:

- `Value`
  - Source: `17-rust-value-object-specification.md`
  - Shape: `pub trait Value { type ValueType: ?Sized; fn value(&self) -> &Self::ValueType; }`
  - Status: the spec already says Rust value objects should normally implement this shared trait.

- `Entity`
  - Source: `18-rust-entity-specification.md`
  - Shape: `pub trait Entity { type IdType; fn id(&self) -> &Self::IdType; }`
  - Status: the spec already says Rust entities should normally implement this shared trait.

- `UseCase`
  - Source: `01-rust-use-case-specification.md`
  - Shape: associated `Request` and `Response` types with either synchronous `execute(request: Request) -> Result<Response, Error>` or asynchronous `execute(request: Request) -> ResponseFuture<Response>`.
  - Status: the spec now defines shared `UseCase` and `AsyncUseCase` traits. Domain-specific `*UC` traits are marker supertraits over the shared trait with concrete associated type bindings.

- `Gateway`
  - Source: `02-rust-gateway-specification.md`
  - Shape: associated `Request` and `Response` types with either synchronous `execute(request: Request) -> Result<Response, Error>` or asynchronous `execute(request: Request) -> ResponseFuture<Response>`.
  - Status: the spec now defines shared `Gateway` and `AsyncGateway` traits. Domain-specific `*GW` traits are marker supertraits over the shared trait with concrete associated type bindings.

Resolved specification gaps:

- The use case and gateway specs now state the shared trait direction and preserve domain-specific `*UC` / `*GW` marker seams.
- The async seam rule now requires a normal `fn execute(...) -> ResponseFuture<...>` signature rather than public `async fn`.
- The specs now state that sync and async execution are separate traits, not two methods on one trait.
- The standards now say every public bounded object should clearly satisfy an architectural role.
- Request builders are now treated as external construction concerns. Shared use case and gateway seams receive a finalized `Request`; they do not expose or accept a `RequestBuilder`.

Remaining specification gaps to resolve:

- The kernel implementation needs to settle the final module locations and exported names for shared use case, gateway, and response-future traits after the recent file move.

Engineering direction for the kernel:

- Prefer associated types for `UseCase` and `Gateway` unless a concrete need appears for one type to implement the same trait repeatedly with different request/response pairs.
- Keep domain-specific `*UC` and `*GW` seam traits aligned with the shared kernel traits.
- Keep request builders outside use case and gateway execution. Builders may construct verified request objects, but `execute` receives only the finalized request.
- Add shared kernel traits first, then implement them gradually.
- Use `Response = ()` for void-style use cases or gateways that succeed without a payload.
- Do not create an empty response object unless the empty response has real domain meaning.

## Next Task Set, Foundational Shared Traits

1. Add a shared `Value` trait.
   - Proposed location: `src/values/value.rs`, re-exported from `src/values/mod.rs`.
   - Shape: associated `ValueType: ?Sized` and `value(&self) -> &Self::ValueType`.
   - First implementation candidates: `Name`, `Description`, `ShortDescription`, `SubjectId`, `FilePath`, `FileName`, `NID`, `NSS`, `Line`, and `Block`.
   - Caution: some current types have inherent `value()` methods that return owned values. Decide whether to preserve those while implementing the trait through fully-qualified calls, or adjust the public API in a compatibility-safe sequence.
   - Status: complete for the first implementation set, plus `ULID`, `DirectoryName`, and `NRN`.

2. Add a shared `Entity` trait.
   - Proposed location: `src/core/traits/mod.rs` or `src/core/traits/entity.rs`.
   - Shape: associated `IdType` and `id(&self) -> &Self::IdType`.
   - First implementation candidates should wait until we identify which kernel types are truly entities rather than value objects.

3. Add shared synchronous `UseCase` and `Gateway` traits.
   - Current use case location after file move: `src/usecase/mod.rs`.
   - Current gateway location after file move: `src/gateway/mod.rs`.
   - Shape: associated `Request` and `Response`, with `execute(request: Request)`.
   - Sync traits: `UseCase` and `Gateway`.
   - Async traits: `AsyncUseCase` and `AsyncGateway`.
   - Async future alias: shared `ResponseFuture` in `src/core/traits/mod.rs`.
   - Status: shared trait layout compiles and formats. Void-style responses are verified with `Response = ()`.

4. Keep request builders as construction-only collaborators.
   - Builders may expose `try_build()` and produce verified request types.
   - Shared `UseCase`, `AsyncUseCase`, `Gateway`, and `AsyncGateway` traits must not accept builder types.
   - Callers build the request before crossing the use case or gateway seam.

## Temporary Stop, 2026-05-31

Stop here before retrofitting existing value objects, entities, use cases, or gateways.

Current checkpoint:

- Engineering standards were updated for:
  - associated-type `Value`
  - primitive/fundamental value return guidance
  - shared sync/async use case traits
  - shared sync/async gateway traits
  - request-only `execute(request)` seams, with builders kept outside execution
  - marker-supertrait `*UC` and `*GW` seams
  - explicit `ResponseFuture` async seams instead of public `async fn`
  - public bounded object architectural role classification
- Kernel implementation work completed or started:
  - callable accessors were added while preserving public fields
  - shared `Value` trait was added and implemented for the first value-object set
  - shared use case and gateway trait work was started
  - a new `src/usecase` directory was introduced
  - use case and gateway traits were moved into `src/usecase/mod.rs` and `src/gateway/mod.rs`

Current caution:

- Do not start retrofitting existing gateway traits until the shared trait module layout is verified.
- Shared trait module layout is now verified, but gateway retrofits should still be handled one at a time.
- Existing gateway APIs are already in use by consumers.
- Do not replace or remove existing public gateway traits/type aliases in this pass.
- Introduce updated shared-trait-compatible APIs side by side, then mark the old APIs with `#[deprecated(...)]` so consumers get compiler warnings and can migrate intentionally.

Pickup steps:

1. Decide whether `UseCase` / `AsyncUseCase` should remain in `src/usecase/mod.rs` or move to `src/usecase/use_case.rs` with re-exports from `src/usecase/mod.rs`.
2. Decide whether `Gateway` / `AsyncGateway` should remain in `src/gateway/mod.rs` or move to `src/gateway/gateway.rs` with re-exports from `src/gateway/mod.rs`.
3. Define the compatibility migration pattern before touching any existing gateway:
   - keep the current public API in place for source compatibility
   - add the new shared-trait-compatible API in a separate module or path
   - allow the new API to reuse the same semantic name in that separate module/path
   - mark the old API with `#[deprecated(note = "...")]`
   - make the deprecation note point to the replacement module/path
   - add tests that prove the new API works before deprecating the old one
4. Evaluate existing gateways one at a time:
   - `IdentityGateway` - first side-by-side retrofit complete as `NewIdentityGW` in `src/gateway/new_identity`
   - `UTCTimestampGateway`
   - `FileDataGateway`
   - `RetrieveDirectoryPath`
   - `Logger`
5. For each existing gateway, add the new API side by side before marking the old API deprecated.
6. Do not remove deprecated APIs until a later breaking-release plan explicitly schedules removal.

Compatibility migration rule:

```rust
#[deprecated(
    note = "Use gateway::new_identity::NewIdentityGW, which implements the shared Gateway seam."
)]
pub trait IdentityGateway {
    // existing operation-specific API remains available temporarily
}
```

Replacement APIs should live in a separate module or path so consumers can switch imports explicitly without losing the old API immediately.

### 2026-05-31, Pickup Progress

Verified and cleaned the shared use case and gateway trait layout.

Decisions:

- Collapse separate `UCResponseFuture` and `GWResponseFuture` aliases into one shared `ResponseFuture`.
- Keep `UseCase` and `AsyncUseCase` in `src/usecase/mod.rs` for now.
- Keep `Gateway` and `AsyncGateway` in `src/gateway/mod.rs` for now.
- Use case and gateway `execute` functions receive `Request` directly. They no longer expose or accept `RequestBuilder`.
- Use `Response = ()` for void-style use cases and gateways.
- Add `gateway::new_identity::NewIdentityGW` as the first side-by-side standards-aligned gateway retrofit.
- Model `NewIdentityGW` as a synchronous gateway seam for requesting a newly generated identity.
- Model `NewIdentityGatewayRequest` as a void-by-construction request object with no request builder.
- Use `ULID` directly as the `NewIdentityGW` success response because the gateway returns one existing bounded Kernel value object.
- Mark the legacy `gateway::identity::IdentityGateway` trait deprecated with a note pointing to `gateway::new_identity::NewIdentityGW`.

Verification:

- `cargo fmt --check`
- `cargo test`

Result:

- 300 unit tests passed.
- 11 doctests passed.

## Completed Work

### 2026-05-31, Accessor Surface

Added callable accessors for bounded public types that still expose public fields.

Compatibility decision:

- Public fields remain public for now.
- Existing methods such as `URL::value()` and `URL::queries()` remain unchanged.
- Borrowed companion accessors were added where that improves the read-only access surface without breaking callers.

Verification:

- `cargo fmt --check`
- `cargo test`

### 2026-05-31, Shared Value Trait

Added a shared `Value` trait for value objects.

Decision:

- Use an associated `ValueType` instead of a generic trait parameter.
- Allow `ValueType: ?Sized` so string-backed value objects can expose `str` instead of forcing callers through `String`.
- Return `&Self::ValueType` so value access does not allocate or clone by default.

Reasoning:

- A value object should have one canonical bounded value contract.
- Associated types encode that one canonical contract directly on the implementation.
- A generic trait such as `Value<T>` is useful when one type intentionally implements the same trait for multiple exposed value forms, but that weakens the default value-object contract and can create ambiguous downstream bounds.

Verification:

- `cargo fmt --check`
- `cargo test`
