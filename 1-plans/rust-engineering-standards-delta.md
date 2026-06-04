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

9. Algorithms domain boundary
   - `src/algorithms` currently lives inside the kernel crate.
   - This behavior should be extracted into its own algorithms domain project instead of remaining in the kernel.
   - Action: create a separate domain project for algorithms, then migrate the existing algorithm behavior into standards-aligned value objects, entities, use cases, and gateways as appropriate.
   - Compatibility caution: if current algorithm APIs are consumed externally, introduce replacement APIs side by side and deprecate old kernel APIs before removal.
   - Status: planned; no code changes started for this extraction.

10. ULID value-object module boundary
    - `src/ulid` currently lives as a top-level kernel module.
    - `ULID` is a bounded value object and already implements the shared `Value` trait.
    - Action: move `ULID` into the `values` module tree, likely under `src/values/ulid`.
    - Compatibility caution: keep the existing `kernel_oss::ulid::ULID` path as a temporary re-export or compatibility shim before removal.
    - Status: planned; no code changes started for this move.

## Suggested Order

1. Add missing accessors while keeping public fields temporarily.
2. Add foundational shared traits and implement them on the obvious kernel types.
3. Move `ULID` into the `values` module tree with a compatibility re-export.
4. Add `try_new` / `try_build` aliases beside existing APIs.
5. Remove production `.unwrap()` / `.expect(...)` where behavior can stay unchanged.
6. Migrate tests to standard names and no `.unwrap()`.
7. Make public field privacy decisions.
8. Refactor gateway contracts.
9. Plan and start extraction of `src/algorithms` into its own algorithms domain project.
10. Bring docs and clippy to enforceable state.

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
- Use `Response = ()` for use cases or gateways that succeed without a payload; execution still returns `Result<(), Error>`.
- Do not create an empty response object unless the empty response has real domain meaning.

## Next Task Set, Foundational Shared Traits

1. Add a shared `Value` trait.
   - Proposed location: `src/values/value.rs`, re-exported from `src/values/mod.rs`.
   - Shape: associated `ValueType: ?Sized` and `value(&self) -> &Self::ValueType`.
   - First implementation candidates: `Name`, `Description`, `ShortDescription`, `SubjectId`, `FilePath`, `FileName`, `NID`, `NSS`, `Line`, and `Block`.
   - Caution: some current types have inherent `value()` methods that return owned values. Decide whether to preserve those while implementing the trait through fully-qualified calls, or adjust the public API in a compatibility-safe sequence.
   - Status: complete for the first implementation set, plus `ULID`, `DirectoryName`, and `NRN`.
   - Follow-up: move `ULID` from the top-level `src/ulid` module into the `values` module tree, while keeping a temporary compatibility export for existing callers.

2. Add a shared `Entity` trait.
   - Proposed location: `src/core/traits/mod.rs` or `src/core/traits/entity.rs`.
   - Shape: associated `IdType` and `id(&self) -> &Self::IdType`.
   - First implementation candidates should wait until we identify which kernel types are truly entities rather than value objects.
   - Status: shared trait added to `src/core/traits/mod.rs`; first usage is compiler-checked through the Rust examples.

3. Add shared synchronous `UseCase` and `Gateway` traits.
   - Current use case location after file move: `src/usecase/mod.rs`.
   - Current gateway location after file move: `src/gateway/mod.rs`.
   - Shape: associated `Request` and `Response`, with `execute(request: Request)`.
   - Sync traits: `UseCase` and `Gateway`.
   - Async traits: `AsyncUseCase` and `AsyncGateway`.
   - Async future alias: shared `ResponseFuture` in `src/core/traits/mod.rs`.
   - Status: shared trait layout compiles and formats. No-payload success responses are verified with `Response = ()`.

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

1. Complete: keep `UseCase` / `AsyncUseCase` in `src/usecase/mod.rs` for now.
2. Complete: keep `Gateway` / `AsyncGateway` in `src/gateway/mod.rs` for now.
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
7. Create an algorithms extraction plan:
   - identify every public API under `src/algorithms`
   - classify each algorithm type as a value object, entity, use case, gateway, or implementation detail
   - create a separate algorithms domain project
   - move domain behavior into standards-aligned `*UC` and `*GW` seams where behavior crosses a boundary
   - keep compatibility shims or deprecations in the kernel until consumers can migrate
8. Create a ULID module move plan:
   - move the current `src/ulid` implementation into `src/values/ulid`
   - update internal imports to use the new value-object module path
   - keep `kernel_oss::ulid::ULID` temporarily available as a compatibility re-export
   - add deprecation guidance if the compatibility path remains as a public module
   - update examples and documentation to prefer the new `values::ulid::ULID` path

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
- Keep the shared use case traits in `src/usecase/mod.rs` for now.
- Keep the shared gateway traits in `src/gateway/mod.rs` for now.
- Request-bearing use case and gateway `execute` functions receive `Request` directly. They no longer expose or accept `RequestBuilder`.
- True no-input seams use dedicated `VoidUseCase` / `AsyncVoidUseCase` and `VoidGateway` / `AsyncVoidGateway` traits rather than placeholder request objects.
- Use `Response = ()` for no-payload success use cases and gateways; the seam still returns `Result<(), Error>`.
- Add `gateway::new_identity::NewIdentityGW` as the first side-by-side standards-aligned gateway retrofit.
- Model `NewIdentityGW` as a synchronous gateway seam for requesting a newly generated identity.
- Use `ULID` directly as the `NewIdentityGW` success response because the gateway returns one existing bounded Kernel value object.
- Mark the legacy `gateway::identity::IdentityGateway` trait deprecated with a note pointing to `gateway::new_identity::NewIdentityGW`.

Verification:

- `cargo fmt --check`
- `cargo test`

Result:

- 305 unit tests passed.
- 11 doctests passed.

### 2026-05-31, Examples Progress

Started Cargo-checked Rust examples under `examples/`.

Completed:

- Added `examples/value_object_and_entity.rs` to show:
  - value objects implementing `Value`
  - value return types using fundamental Rust values (`str`, `u128`)
  - an entity implementing the shared `Entity` trait with `ULID` as the direct entity identity
- Added `examples/gateway_usecase_composition.rs` to show:
  - a dedicated no-input gateway seam through `VoidGateway`
  - a domain-specific `*GW` marker seam over the shared `Gateway` trait
  - a domain-specific `*UC` marker seam over the shared `UseCase` trait
  - request builder usage outside the use case boundary
  - `ULID` used directly as the entity identity without a user-specific identity wrapper
  - direct bounded response return instead of wrapping one bounded entity in a ceremony response object
- Documented the examples using the Rust documentation standard:
  - module-level example purpose and run command
  - type-level role documentation
  - argument and return documentation for constructors, builders, and seam methods
  - explicit `# Errors` sections for fallible example paths

Verification:

- `cargo fmt --check`
- `cargo check --examples`
- `cargo rustdoc --examples`

### 2026-06-01, Example Standards Audit Actions

Status: complete.

Actions:

1. Complete: replace derived entity equality with identity-only equality for example entities.
2. Complete: add `Display` for the example `EmailAddress` value object.
3. Complete: expose `RegisterUserRequest::builder()` as the non-void request construction entrypoint.
4. Complete: add a bounded accessor on `RegisterUserRequest` and avoid using a consuming extractor as the primary request surface.
5. Complete: move request-builder validation into `validate_*` helper methods.
6. Complete: align request-builder error text with the required deterministic message pattern.
7. Complete: add builder-first construction for the `RegisterUser` use case implementation.
8. Complete: add builder-first construction for the `StaticNewIdentityGateway` gateway implementation.
9. Complete: update example documentation to reflect the standard-aligned construction paths and entity equality semantics.
10. Complete: update execution calls to use fully qualified shared-trait calls with inline marker-seam casts where seam visibility matters.
11. Complete: re-run example verification with formatting, example checks, rustdoc, and example execution.

Verification:

- `cargo fmt --check`
- `cargo check --examples`
- `cargo rustdoc --examples`
- `cargo run --example value_object_and_entity`
- `cargo run --example gateway_usecase_composition`

Result:

- All commands passed.
- `cargo rustdoc --examples` still reports existing broken intra-doc link warnings from library docs outside these examples.

### 2026-06-01, Request Builder Primitive Input Direction

Status: complete.

Actions:

1. Complete: update `RegisterUserRequestBuilder` so the primary request-side setter accepts raw string-like email input.
2. Complete: keep an explicit convenience setter for callers that already have a valid `EmailAddress`.
3. Complete: validate missing, empty, and whitespace-only raw email input in the request builder.
4. Complete: finalize raw email input into `EmailAddress` during `try_build()`.
5. Complete: propagate `EmailAddress` construction errors from `try_build()` so invalid request input returns a bounded user-facing error before the use case seam is crossed.
6. Complete: update the example call site to pass raw email text through `RegisterUserRequest::builder().email_address(...)`.
7. Complete: re-run example verification.

Verification:

- `cargo fmt --check`
- `cargo check --examples`
- `cargo rustdoc --examples`
- `cargo run --example gateway_usecase_composition`

Result:

- All commands passed.
- `cargo rustdoc --examples` still reports existing broken intra-doc link warnings from library docs outside these examples.

### 2026-06-01, Multi-Input Builder Standards Update

Status: complete.

Updated Rust standards:

- `12-rust-builder-pattern.md`
  - Added an optional recommended internal enum pattern for one logical builder field that accepts multiple input forms.
  - Added naming guidance for primitive-first, valid-value-first, and response-builder variants.
  - Standardized alternate setter names on `raw_<field>` for primitive input and `valid_<field>` for already valid Kernel or Domain objects.
- `01-rust-use-case-specification.md`
  - Clarified that use case request builders should usually expose primitive input setters first for caller-provided application-edge input.
  - Clarified that valid value-object setters may be provided as explicitly named alternatives.
  - Clarified custom use case response builder input-form guidance, with already valid Kernel or Domain objects as the preferred primary setter form.
  - Clarified that examples and tests should prefer fully qualified `UseCase::execute` / `AsyncUseCase::execute` calls with inline `as &dyn *UC` casts when making seam execution explicit.
  - Clarified that sync and async use case implementations put `execute` on the shared role impl and use an empty domain marker `*UC` impl.
  - Documented why bare `dyn *UC` locals do not compile and why omitting the marker cast can leave the marker trait unused from the compiler's perspective.
- `02-rust-gateway-specification.md`
  - Clarified that gateway request builders should usually prefer already valid value objects or entities, while allowing explicit primitive alternatives.
  - Clarified that gateway response builders may make primitive provider data and already valid values first-class when translating into bounded response contracts.
  - Clarified that examples and tests should prefer fully qualified `Gateway::execute` / `AsyncGateway::execute` calls with inline `as &dyn *GW` casts when making seam execution explicit.
  - Clarified that sync and async gateway implementations put `execute` on the shared role impl and use an empty domain marker `*GW` impl.
  - Documented why bare `dyn *GW` locals do not compile and why omitting the marker cast can leave the marker trait unused from the compiler's perspective.

Verification:

- `cargo fmt --check`
- `cargo check --examples`
- `cargo run --example value_object_and_entity`
- `cargo run --example gateway_usecase_composition`
- `cargo rustdoc --examples`
- `git diff --check`
- `git -C ../../specifications/engineering-standards diff --check`

Result:

- All commands passed.
- `cargo rustdoc --examples` still reports existing broken intra-doc link warnings from library docs outside these examples.

### 2026-06-01, Async Gateway/Use Case Example

Status: complete.

Added `examples/async_gateway_usecase_composition.rs` to show:

- shared `AsyncUseCase` and `AsyncGateway` traits with explicit `ResponseFuture` return values
- domain-specific `*UC` and `*GW` marker seams over the shared async traits
- normal `execute(...)` methods that return boxed response futures without exposing public `async fn`
- async use case composition over an async gateway
- fully qualified `AsyncUseCase::execute` usage in `main` with an inline `as &dyn LoadRegisteredUserUC` marker-seam cast and without a boxed use case trait object
- fully qualified `AsyncGateway::execute` usage inside the use case implementation with an inline `as &dyn FindRegisteredUserGW` marker-seam cast
- use case request construction from primitive input before crossing the use case seam
- gateway request construction from an already valid value object inside the use case
- direct entity response return with `ULID` as the entity identity
- example-only `run_ready` support isolated in an `example_runtime` module with guidance to replace it with an application runtime

Verification:

- `cargo fmt`
- `cargo fmt --check`
- `cargo check --examples`
- `cargo run --example async_gateway_usecase_composition`
- `cargo rustdoc --examples`
- `rg -n "async fn" examples/async_gateway_usecase_composition.rs src/usecase/mod.rs src/gateway/mod.rs`
- `git diff --check`

Result:

- All commands passed.
- The `async fn` search returned no matches.
- `cargo rustdoc --examples` still reports existing broken intra-doc link warnings from library docs outside these examples.

### 2026-06-01, Current Decision Checkpoint

Status: recorded.

Decisions:

1. Shared seam traits stay in their current module roots for now:
   - `VoidUseCase`, `UseCase`, `AsyncVoidUseCase`, and `AsyncUseCase` remain in `src/usecase/mod.rs`
   - `VoidGateway`, `Gateway`, `AsyncVoidGateway`, and `AsyncGateway` remain in `src/gateway/mod.rs`
   - `ResponseFuture` remains shared from `src/core/traits/mod.rs`
2. Domain-specific `*UC` and `*GW` traits remain empty marker supertraits over the shared role traits.
3. Concrete implementations put `execute` on the shared role impl:
   - sync use cases implement `UseCase`
   - async use cases implement `AsyncUseCase`
   - sync gateways implement `Gateway`
   - async gateways implement `AsyncGateway`
4. Examples and standards should call seams with fully qualified shared-trait execution and inline marker casts when demonstrating the domain seam:
   - `UseCase::execute(&use_case as &dyn SomeUC, request)`
   - `AsyncUseCase::execute(&use_case as &dyn SomeUC, request)`
   - `Gateway::execute(&gateway as &dyn SomeGW, request)`
   - `AsyncGateway::execute(&gateway as &dyn SomeGW, request)`
5. The inline `as &dyn ...` cast is intentional because it makes the marker seam compiler-visible without introducing a separate variable or heap allocation.
6. Async standalone examples may include an isolated example runner such as `example_runtime::run_ready`, but production application code should replace it with the real runtime or executor.
7. Composition examples should stay focused on the primary flow and avoid secondary call paths that read like tests; optional builder patterns belong in standards or focused examples.
8. Use case and gateway implementations should store only dependency links expressed through traits. Request data belongs in request objects, response data belongs in response payloads, and error information belongs in `Error`.
9. Composition examples should show dependency wiring explicitly at the composition point. Do not hide gateway and use case builder construction behind example-local helpers such as `build_use_case(...)`.

Next implementation direction:

1. Complete: add the unit success payload example using `Response = ()`.
2. After examples, continue side-by-side gateway retrofits one gateway at a time.
3. Keep the broader `ULID` module move and algorithms extraction as separate follow-up plans.

### 2026-06-01, Unit Success Payload Example

Status: complete.

Added `examples/unit_success_payload.rs` to show:

- `Response = ()` as a no-payload success payload, not a void execution response
- sync use case and gateway seams that still return `Result<(), Error>`
- explicit `Ok(())` on successful use case and gateway execution
- explicit `Err(Error)` on gateway failure
- failure propagation from gateway to use case through `Result<(), Error>`
- named request types and builders for non-void requests
- gateway success and failure modeled through substituted dependency implementations, not request or error state stored on the gateway
- explicit inline gateway and use case builder composition without an example-local `build_use_case(...)` helper
- fully qualified `UseCase::execute` and `Gateway::execute` calls with inline `as &dyn *UC` / `as &dyn *GW` marker-seam casts

Updated Rust standards:

- `01-rust-use-case-specification.md`
  - Added `()` as an allowed success payload when a use case succeeds without returned data.
  - Clarified that this still means `Result<(), Error>` for sync use cases and a future resolving to `Result<(), Error>` for async use cases.
  - Clarified that empty custom response objects should not be created solely to avoid `Response = ()`.
  - Clarified that this does not change the named unit-like request rule for void requests.
  - Added a state ownership rule: use case fields are dependency links only; request, response, and error information must not be stored on the use case implementation.
- `02-rust-gateway-specification.md`
  - Added the same unit success payload guidance for gateways.
  - Added a gateway no-payload success pattern returning `Ok(())`.
  - Added a state ownership rule: gateway fields are dependency links only; request, response, and error information must not be stored on the gateway implementation.
  - Updated gateway dependency examples to use trait-object dependency links such as `Arc<dyn AuthProviderClient>`.

Verification:

- `cargo fmt`
- `cargo fmt --check`
- `cargo check --examples`
- `cargo run --example unit_success_payload`
- `cargo rustdoc --examples`
- `git diff --check`
- `git -C ../../specifications/engineering-standards diff --check`
- `rg -n "build_use_case|fn build_.*use_case|build use case" examples ../../specifications/engineering-standards/7-specifications/rust`

### 2026-06-01, Plan/Decision/Action Update

Status: recorded.

Decisions:

1. The `build_use_case(...)` helper shape is not a standards pattern.
2. Standards examples should expose composition directly:
   - build the gateway implementation
   - store it behind the gateway seam trait
   - inject it into the use case builder
   - execute through the fully qualified shared trait call and inline marker-seam cast
3. Example-local helpers may still exist for ordinary fixtures or repeated primitive values, but not for hiding the use case/gateway composition pattern being taught.

Actions completed:

1. Removed `build_use_case(...)` from `examples/unit_success_payload.rs`.
2. Inlined both success and failure composition paths in `main`.
3. Searched examples and Rust standards for `build_use_case`-style helper usage; no matches remain.
4. Re-ran example verification after the cleanup.

Next tasks:

1. Complete: start the next side-by-side gateway retrofit with `UTCTimestampGateway`.
2. Continue later with `FileDataGateway`, `RetrieveDirectoryPath`, and `Logger`, one gateway at a time.

### 2026-06-01, UTC Timestamp Gateway Retrofit

Status: complete.

Decisions:

1. The replacement shared-trait seam is `CurrentUTCTimestampGW`.
2. The seam is no-input and therefore uses `VoidGateway` rather than a placeholder request object.
3. The response remains the existing `UTCTimestamp` value object.
4. The old `UTCTimestampGateway` trait remains in place for compatibility and is deprecated with a note pointing to the new shared `Gateway` seam.
5. Tests for the new gateway seam must follow the Rust testing standard:
   - test function names end in `_success` or `_error`
   - no `.unwrap()`
   - `.expect(...)` is allowed only for fixture setup that is not the behavior under test
   - gateway execution uses the fully qualified shared trait call with an inline `as &dyn *GW` marker-seam cast

Actions completed:

1. Added `src/gateway/current_utc_timestamp/mod.rs`.
2. Added `CurrentUTCTimestampGW: VoidGateway<Response = UTCTimestamp>`.
3. Exported the new module from `src/gateway/mod.rs`.
4. Deprecated the old `UTCTimestampGateway` compatibility trait.
5. Added a standards-aligned unit test in `src/gateway/current_utc_timestamp/tests.rs`.

Verification:

- `cargo fmt`
- `cargo fmt --check`
- `cargo test current_utc_timestamp`
- `cargo check --examples`
- `cargo test`
- `git diff --check`

Next tasks:

1. Start the next side-by-side gateway retrofit.
2. Recommended next candidate: `RetrieveDirectoryPath`, because it is a simple function-type gateway with one primitive input and one primitive response.
3. Continue later with `FileDataGateway` and `Logger`.

### 2026-06-01, Test Documentation and Traceability Standards

Status: complete.

Decisions:

1. Tests are verification artifacts and should carry explicit documentation.
2. Rust test modules should start with `//!` documentation that identifies:
   - bounded unit under test
   - public interfaces verified
   - logical paths covered
   - supplied requirement validation points, or an explicit statement that none are currently supplied
3. Rust test functions should use `///` documentation that identifies:
   - `Requirement validation:` label with a supplied identifier, or a statement that no requirement validation point is currently supplied
   - public interface or seam exercised
   - logical path tested
   - expected observable result
4. Test names remain behavior-focused and must still end in `_success`, `_success_async`, `_error`, or `_error_async`.
5. Requirement validation identifiers should live in documentation, not in Rust function names.
6. Logical paths are extracted from implementation behavior; requirement validation points must be supplied from an external requirement or verification artifact and must not be invented from logical paths.
7. Existing legacy tests were not mass-migrated in this pass; the new traceability shape was applied to the standards-aligned gateway/use case tests touched by this work.

Actions completed:

1. Updated the Rust testing standard with a test documentation and traceability rule.
2. Updated the Rust reference testing template with module-level `//!` docs and per-test `/// Requirement validation:` docs.
3. Updated the Rust reference testing slice with the same traceability shape.
4. Updated the standards-aligned shared gateway, shared use case, new identity gateway, and current UTC timestamp gateway tests with module-level and per-test documentation.
5. Updated the shared gateway/use case sync success tests to use `is_ok!` for the behavior result instead of direct `assert!(result.is_ok())`.
6. Re-audited only test files already present in the uncommitted kernel status and tightened the shared async gateway/use case tests so the `ResponseFuture` resolves to `Ok(())` instead of only checking that a future is returned.
7. Replaced invented placeholder validation identifiers in the uncommitted tests and standards examples with explicit statements that no requirement validation points are currently supplied.

Verification:

- `cargo fmt`
- `cargo fmt --check`
- `cargo test`
- `cargo check --examples`

### 2026-06-01, Updated Restart Checkpoint

Status: recorded.

Current verified baseline:

- The `kernel-oss` worktree is clean on `main`.
- `cargo test` passes with 301 unit tests and 11 doctests.
- `cargo fmt --check` passes.
- `cargo check --examples` passes.
- The adjacent `specifications/engineering-standards` worktree is clean.

Completed since the earlier stop point:

- Shared sync and async seam examples were aligned with the current standards.
- The unit success payload example was added to show `Response = ()` with explicit `Result<(), Error>` behavior.
- The UTC timestamp gateway side-by-side retrofit was completed as `CurrentUTCTimestampGW`.
- Standards-aligned test documentation and traceability rules were applied to the touched seam tests.

Current shared seam baseline:

- `UseCase` and `AsyncUseCase` remain in `src/usecase/mod.rs`.
- `Gateway` and `AsyncGateway` remain in `src/gateway/mod.rs`.
- `ResponseFuture` remains shared from `src/core/traits/mod.rs`.
- Existing compatibility seams now completed:
  - `IdentityGateway` -> `gateway::new_identity::NewIdentityGW`
  - `UTCTimestampGateway` -> `gateway::current_utc_timestamp::CurrentUTCTimestampGW`

Updated pickup order:

1. Start the next side-by-side gateway retrofit with `RetrieveDirectoryPath`.
2. After that, retrofit `FileDataGateway`.
3. Treat `Logger` as a separate design step before implementation because its current shape is a multi-operation trait rather than a single-operation seam.
4. Keep the broader `ULID` module move and `src/algorithms` extraction as separate follow-up plans after the remaining small gateway retrofits.

Reasoning for the next starting point:

- `RetrieveDirectoryPath` is currently only a function type alias with one primitive input and one primitive output.
- It is the smallest remaining gateway candidate and should let the compatibility migration pattern be repeated with low disruption before touching `FileDataGateway` or `Logger`.

Expected `RetrieveDirectoryPath` retrofit shape:

- Add a new side-by-side gateway module, likely `src/gateway/retrieve_directory_path`.
- Introduce a named request object rather than passing a bare `&str`.
- Define a marker seam trait over the shared `Gateway` trait.
- Keep the old `RetrieveDirectoryPath` function alias temporarily for compatibility.
- Add standards-aligned tests before deprecating the old alias.

## Completed Work

### 2026-06-04, Void Seam Direction Change

Status: complete.

Decisions:

1. True no-input use cases and gateways should not use placeholder request objects.
2. The shared kernel seam surface now distinguishes:
   - request-bearing sync seams: `UseCase` and `Gateway`
   - request-bearing async seams: `AsyncUseCase` and `AsyncGateway`
   - no-input sync seams: `VoidUseCase` and `VoidGateway`
   - no-input async seams: `AsyncVoidUseCase` and `AsyncVoidGateway`
3. Domain-specific marker seams for no-input capabilities should inherit from the void shared traits rather than from request-bearing traits with empty request types.
4. This change applies to both kernel implementation and engineering standards guidance.

Actions completed:

1. Added `VoidUseCase` and `AsyncVoidUseCase` to `src/usecase/mod.rs`.
2. Added `VoidGateway` and `AsyncVoidGateway` to `src/gateway/mod.rs`.
3. Updated the shared use case and gateway tests to verify both no-input and request-bearing seam variants.
4. Retrofitted `gateway::new_identity::NewIdentityGW` to use `VoidGateway<Response = ULID>`.
5. Retrofitted `gateway::current_utc_timestamp::CurrentUTCTimestampGW` to use `VoidGateway<Response = UTCTimestamp>`.
6. Updated `examples/gateway_usecase_composition.rs` to execute the new identity seam through `VoidGateway::execute(...)`.
7. Updated the local engineering-standards delta notes to reflect the new void seam direction.
8. Updated the engineering standards so true no-input seams use dedicated void traits rather than placeholder request objects.

Verification:

- `cargo fmt --all`
- `cargo test`

Result:

- 305 unit tests passed.
- 11 doctests passed.

### 2026-06-04, Async Kernel No-Input Gateway Marker Seams

Status: complete.

Decisions:

1. Kernel-level foundational gateway seams should normally expose both sync and async marker seam options when the capability can reasonably be consumed from either execution model.
2. The sync and async variants must remain separate traits; do not place both sync and async `execute` methods on the same marker seam trait.
3. `NewIdentityGW` and `CurrentUTCTimestampGW` therefore each need paired async marker seams.

Actions completed:

1. Added `AsyncNewIdentityGW: AsyncVoidGateway<Response = ULID>` in `src/gateway/new_identity/mod.rs`.
2. Added `AsyncCurrentUTCTimestampGW: AsyncVoidGateway<Response = UTCTimestamp>` in `src/gateway/current_utc_timestamp/mod.rs`.
3. Added async standards-aligned tests for both marker seams.
4. Updated the engineering standards to state that Kernel-owned or other foundational shared seams should normally provide paired sync and async seam variants when both execution models are reasonable.

Verification:

- `cargo fmt --all`
- `cargo test`

Result:

- 307 unit tests passed.
- 11 doctests passed.

### 2026-06-04, Updated Restart Checkpoint

Status: recorded.

Current verified baseline:

- The `kernel-oss` worktree contains local uncommitted changes for the shared void seam work, async kernel seam additions, and plan/handoff updates.
- `cargo fmt --all` passes.
- `cargo test` passes with 307 unit tests and 11 doctests.
- The current side-by-side compatibility seams completed in this repo are:
  - `IdentityGateway` -> `gateway::new_identity::NewIdentityGW` and `gateway::new_identity::AsyncNewIdentityGW`
  - `UTCTimestampGateway` -> `gateway::current_utc_timestamp::CurrentUTCTimestampGW` and `gateway::current_utc_timestamp::AsyncCurrentUTCTimestampGW`

Current shared seam baseline:

- `VoidUseCase`, `UseCase`, `AsyncVoidUseCase`, and `AsyncUseCase` remain in `src/usecase/mod.rs`.
- `VoidGateway`, `Gateway`, `AsyncVoidGateway`, and `AsyncGateway` remain in `src/gateway/mod.rs`.
- `ResponseFuture` remains shared from `src/core/traits/mod.rs`.
- True no-input seams should use the dedicated void traits rather than placeholder request objects.

Next recommended implementation order:

1. Retrofit `RetrieveDirectoryPath` in `src/gateway/directory_list/mod.rs` side by side with:
   - a new request-bearing sync seam
   - a paired request-bearing async seam if the foundational capability should remain available to async flows
   - compatibility preservation for the current function alias
2. Retrofit `FileDataGateway` in `src/gateway/file_data_gateway/mod.rs` using the same compatibility pattern.
3. Treat `Logger` in `src/gateway/logger/mod.rs` as a separate design step before implementation because it is a multi-operation trait and does not currently fit the one-command seam model.
4. After the remaining small gateway retrofits, return to:
   - the `ULID` module move into `src/values`
   - the `src/algorithms` extraction plan
   - broader test cleanup, rustdoc, and clippy enforcement work

Recommended `RetrieveDirectoryPath` retrofit shape:

1. Add a side-by-side module such as `src/gateway/retrieve_directory_path`.
2. Define a finalized request object instead of passing bare `&str`.
3. Define marker seams over `Gateway` and `AsyncGateway` with the same request and response contract.
4. Keep the old `RetrieveDirectoryPath` function alias temporarily for compatibility.
5. Add standards-aligned sync and async seam tests before deprecating the old alias.

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
