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
   - Standards expect `*GW` traits, explicit request builder/request/response types, and one `execute(...)` method.
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
  - Shape: associated `RequestBuilder`, `Request`, and `Response` types with `execute(...) -> Result<Response, Error>`.
  - Status: the spec defines the abstract shared shape, while examples still emphasize domain-specific seam traits such as `RegisterUserUC`.

- `Gateway`
  - Source: `02-rust-gateway-specification.md`
  - Shape: associated `RequestBuilder`, `Request`, and `Response` types with `execute(...) -> Result<Response, Error>`.
  - Status: the spec defines the abstract shared shape, while examples still emphasize domain-specific seam traits such as `RegisterUserGW`.

Possible specification gaps to resolve:

- The use case and gateway specs show a reusable abstract trait shape, but do not state where a shared trait should live in the kernel crate.
- The use case and gateway specs say request builders build verified request types through `try_build()`, but they do not define a shared `TryBuild<T>` or `RequestBuilder` trait that can be used as a bound.
- If we want generic trait parameters instead of associated types, the use case and gateway specs should be updated. The current specs use associated types.

Engineering direction for the kernel:

- Prefer associated types for `UseCase` and `Gateway` unless a concrete need appears for one type to implement the same trait repeatedly with different request/response pairs.
- Keep domain-specific `*UC` and `*GW` seam traits aligned with the shared kernel traits.
- Add shared kernel traits first, then implement them gradually.

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
   - Proposed use case location: `src/core/traits/mod.rs` or a new use-case-oriented module if one is introduced.
   - Proposed gateway location: `src/gateways/mod.rs` or `src/gateways/traits.rs`.
   - Shape: associated `RequestBuilder`, `Request`, and `Response`, with `execute`.
   - Do not retrofit all current gateways immediately; first add the common trait definitions and then evaluate existing gateway traits one at a time.

4. Decide whether to add a shared `TryBuild<T>` trait.
   - This is not explicitly specified today.
   - It would let `UseCase` and `Gateway` encode that their `RequestBuilder` produces their `Request`.
   - If we add it, update the Rust specifications first or alongside the kernel implementation.

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
