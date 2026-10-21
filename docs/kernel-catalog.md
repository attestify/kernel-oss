# Kernel OSS Catalog

This catalog is the human-readable reuse index for `kernel-oss`. Use it before
creating a new domain or application type so existing kernel primitives, value
objects, request objects, response types, and seam traits are reused instead of
duplicated.

The catalog is intentionally curated. Rustdoc remains the API reference; this
file answers a different question: "Does the kernel already have the thing I am
about to model?"

## How To Use This Catalog

Search by role, invariant, or module path:

- If the type represents one bounded value, check [Value Objects](#value-objects).
- If the type has stable identity and identity-based equality, check [Entities](#entities).
- If the type crosses a use case or gateway boundary, check [Requests And Responses](#requests-and-responses).
- If behavior crosses an application boundary, check [Use Case Seams](#use-case-seams) and [Gateway Seams](#gateway-seams).
- For typed event delivery, check [Usecase Event Handler](#usecase-event-handler).
- For operational logging, check [Operational Logging](#operational-logging).
- If an API looks old, check [Compatibility And Deprecations](#compatibility-and-deprecations).

When a matching type exists, prefer importing it from `kernel_oss` instead of
creating a duplicate in a domain crate. If the existing type almost fits but has
the wrong invariant, create a domain-specific type and document why the kernel
type was not reused.

## Shared Roles

| Role | Shared Shape | Use When |
| --- | --- | --- |
| Value object | `values::Value` | A type exposes one canonical bounded value. |
| Entity | `entity::Entity` | A type has stable identity and should compare by identity. |
| No-input use case | `usecase::VoidUseCase` / `usecase::AsyncVoidUseCase` | A use case has no request object. |
| Request-bearing use case | `usecase::UseCase` / `usecase::AsyncUseCase` | A use case accepts a finalized request. |
| No-input gateway | `gateway::VoidGateway` / `gateway::AsyncVoidGateway` | A gateway has no request object. |
| Request-bearing gateway | `gateway::Gateway` / `gateway::AsyncGateway` | A gateway accepts a finalized request. |
| Async response | `response::ResponseFuture` | Async seams return a boxed future from a normal `execute` method. |
| Typed event delivery | `usecase_event_handler::UsecaseEventHandler` | A Handler consumes a finalized parent Response or Event synchronously. |
| Operational logging | `logging::Logger` / `logging::NoOpLogger` | Synchronous borrowed-message calls; explicit no-op when output is intentionally absent (0.6.0). |
| Bounded error | `error::Error` | Public fallible APIs need classified user/system errors. |

Request builders are construction collaborators. They should build finalized
request objects before callers cross use case or gateway seams.

## Errors

| Type | Module Path | Construction | Reuse Guidance |
| --- | --- | --- | --- |
| `Error` | `kernel_oss::error::Error` | `new`, `for_user`, `for_system` | Use as the normal `Result<Response, Error>` failure value. These three direct constructors are the deliberate small/infallible Kernel exception; there is no `ErrorBuilder`. Initial construction has no provenance input. |
| `ErrorProvenanceNamespace` | `kernel_oss::error::ErrorProvenanceNamespace` | `try_new` | Use for a bounded lowercase semantic-owner namespace; do not use it for product data or diagnostics. |
| `ErrorProvenanceCode` | `kernel_oss::error::ErrorProvenanceCode` | `try_new` | Use for a bounded lowercase semantic code owned by the descriptor's namespace. |
| `ErrorProvenanceSchemaVersion` | `kernel_oss::error::ErrorProvenanceSchemaVersion` | `try_new` | Use for a nonzero descriptor schema version. |
| `ErrorProvenanceDescriptor` | `kernel_oss::error::ErrorProvenanceDescriptor` | `new` | Compose already-valid bounded values; no cross-field validation exists. |
| `ErrorProvenance` | `kernel_oss::error::ErrorProvenance` | Error transitions only | Inspect current and optional origin descriptors; consumers cannot construct a chain. |

Attach provenance only with `Error::try_with_attached_provenance` when it is
absent. Derive once with `Error::try_with_derived_provenance` when only a
current descriptor is present. Clear it with `Error::without_provenance` when a
caller must return the same outer error without semantic provenance. This is not
a general metadata, transport-diagnostic, callback, or source-chain API.

Those operations immutably reconstruct a new `Error`; they are not builder
finalization, mutation, or generic field setters. A semantic owner creates its
validated immutable definition set through `try_new()` and exposes named
descriptor accessors. Kernel supplies no product descriptor meaning.

## Value Objects

### Foundational Values

| Type | Module Path | Canonical Value | Construction | Reuse Guidance |
| --- | --- | --- | --- | --- |
| `ULID` | `kernel_oss::ulid::ULID` | `u128` | `from_parts`, `from_string`, `nil`, `from_bytes` | Use as the default stable identity value. Planned move: prefer `kernel_oss::values::ulid::ULID` after the value-module migration. |
| `UTCTimestamp` | `kernel_oss::values::datetime::utc_timestamp::UTCTimestamp` | nanoseconds/milliseconds/seconds accessors | `builder()` | Use for bounded UTC timestamps. |
| `StartTime` | `kernel_oss::values::datetime::start_time::StartTime` | `u128` milliseconds | `now`, `from`, `try_from` | Use for process or operation start times. |
| `Line` | `kernel_oss::values::text::line::Line` | `str` | `try_from` | Use for bounded single-line text. |
| `Block` | `kernel_oss::values::text::block::Block` | `str` | `try_from` | Use for bounded multi-line text blocks. |
| `NfcText` | `kernel_oss::values::text::NfcText` | Unicode 17.0 NFC `str` | `new` | Use when a complete string must be held as canonical NFC text; the shared normalization engine remains private. |
| `URL` | `kernel_oss::values::uri::url::URL` | canonical WHATWG URL | `try_new` | Use for dependency-free parsing, resolution, borrowed component access, and immutable mutation. See [URL Value and Architecture](url.md). |

### File And Directory Values

| Type | Module Path | Canonical Value | Construction | Reuse Guidance |
| --- | --- | --- | --- | --- |
| `FilePath` | `kernel_oss::values::specification::file_path::FilePath` | `str` | `from`, `try_from` | Use for validated file path text in requests and specification values. |
| `FileName` | `kernel_oss::values::file_system::file_name::FileName` | `str` | `builder()` | Use for bounded file names. |
| `DirectoryName` | `kernel_oss::values::directory::name::DirectoryName` | `str` | `try_from` | Use for bounded directory names. |
| `DirectoryList` | `kernel_oss::values::directory::directory_list::DirectoryList` | named path list | `try_from_vec`, `try_from_hashmap`, `try_add`, `try_merge` | Use for expected directory structures, not OS-specific path retrieval. |

### Namespace Values

| Type | Module Path | Canonical Value | Construction | Reuse Guidance |
| --- | --- | --- | --- | --- |
| `NRN` | `kernel_oss::values::nrn::nrn::NRN` | `str` | `try_from` | Use for Attestify namespace resource names. |
| `NID` | `kernel_oss::values::nrn::nrn::NID` | `str` | `new` | Use for namespace identifiers. |
| `NSS` | `kernel_oss::values::nrn::nrn::NSS` | `str` | `new` | Use for namespace-specific resource strings. |
| `NapeNID` | `kernel_oss::values::nrn::nrn::NapeNID` | enum | `new` | Use for kernel-supported NRN namespace identifiers. |

### Specification Values

| Type | Module Path | Canonical Value | Construction | Reuse Guidance |
| --- | --- | --- | --- | --- |
| `APIVersion` | `kernel_oss::values::specification::api_version::APIVersion` | major/minor/patch | `new`, `parse`, `FromStr` | Use for specification API versions. |
| `Name` | `kernel_oss::values::specification::name::Name` | `str` | `try_from` | Use for bounded specification names. |
| `Description` | `kernel_oss::values::specification::description::Description` | `str` | `try_from` | Use for bounded long descriptions. |
| `ShortDescription` | `kernel_oss::values::specification::short_description::ShortDescription` | `str` | `try_from` | Use for bounded short descriptions. |
| `SubjectId` | `kernel_oss::values::specification::subject_id::SubjectId` | `str` | `new` | Use for bounded subject identifiers. |
| `Subject` | `kernel_oss::values::specification::subject::Subject` | `NRN` plus `SubjectId` | `new` | Use for specification subjects. |
| `MetaData` | `kernel_oss::values::specification::metadata::MetaData` | name/description pairs | `try_from_vec`, `try_add`, `try_merge` | Use for bounded metadata collections. |
| `Kind` | `kernel_oss::values::specification::kind::Kind` | enum | `new` | Use for specification kind values. |
| `Outcome` | `kernel_oss::values::specification::outcome::Outcome` | enum | `try_from` | Use for pass/fail/error/inconclusive outcomes. |
| `Procedure` | `kernel_oss::values::specification::procedure::Procedure` | repository and directory | `new` | Use for specification procedure locations. |
| `RepositoryLink` | `kernel_oss::values::specification::repository_link::RepositoryLink` | parsed URL | `builder()` | Use for repository URLs with allowed/default schemes. |

### Assurance Specification Objects

These types model assurance procedure/report data. Reuse them when working with
the existing Attestify assurance specification shape. Do not copy them into a
domain crate unless the target domain intentionally owns a different schema.

| Area | Representative Types | Module Path |
| --- | --- | --- |
| Assurance procedure | `Action`, `ActionBuilder`, `Activity`, `Activities`, `Artifact`, `Artifacts`, `Procedure` | `kernel_oss::values::specification::assurance_procedure::*` |
| Assurance report | `Action`, `ActionBuilder`, `Activity`, `Activities`, `AdditionalInformation`, `SignedFile`, `Summary` | `kernel_oss::values::specification::assurance_report::*` |
| Versioned specs | `AssuranceProcedure`, `AssuranceProcedureBuilder`, `AssuranceReportV1`, `Builder` | `kernel_oss::values::specification::v1_0_0::*` |
| Spec traits | `AssuranceProcedure`, `AssuranceReport` | `kernel_oss::values::specification::traits::*` |

## Entities

The kernel currently provides the shared entity role trait:

| Trait | Module Path | Contract |
| --- | --- | --- |
| `Entity` | `kernel_oss::entity::Entity` | Exposes a stable identity through `id()`. |

Kernel examples demonstrate the expected pattern:

- Use `ULID` directly as the entity identity unless a domain-specific identity
  wrapper has a real invariant.
- Compare entities by stable identity, not by every field.

## Requests And Responses

| Type | Module Path | Role | Builder | Used By |
| --- | --- | --- | --- | --- |
| `RetrieveDirectoryPathRequest` | `kernel_oss::gateway::retrieve_directory_path::RetrieveDirectoryPathRequest` | Gateway request | `builder()` | `RetrieveDirectoryPathGW`, `AsyncRetrieveDirectoryPathGW` |
| `FileDataRequest` | `kernel_oss::gateway::file_data::FileDataRequest` | Gateway request | `builder()` | `FileDataGW`, `AsyncFileDataGW` |
| `WriteLogEntryRequest` | `kernel_oss::gateway::write_log_entry::WriteLogEntryRequest` | Gateway request | `builder()` | `WriteLogEntryGW`, `AsyncWriteLogEntryGW` |
| `LogLevel` | `kernel_oss::gateway::write_log_entry::LogLevel` | Request value | enum | `WriteLogEntryRequest` |
| `()` | Rust unit | No-payload success response | none | Use when success has no meaningful payload. |

For new request types:

- Use a named request object instead of primitive request parameters at the seam.
- Expose bounded accessors.
- Keep raw-input validation in the builder.
- Pass only finalized requests into `execute`.

## Use Case Seams

The kernel owns the shared use case roles, not concrete domain use cases.

| Trait | Module Path | Execution Model | Request | Response |
| --- | --- | --- | --- | --- |
| `VoidUseCase` | `kernel_oss::usecase::VoidUseCase` | sync | none | associated `Response` |
| `UseCase` | `kernel_oss::usecase::UseCase` | sync | associated `Request` | associated `Response` |
| `AsyncVoidUseCase` | `kernel_oss::usecase::AsyncVoidUseCase` | async | none | `ResponseFuture<Response>` |
| `AsyncUseCase` | `kernel_oss::usecase::AsyncUseCase` | async | associated `Request` | `ResponseFuture<Response>` |

Domain crates should define their own `*UC` marker traits as thin supertraits
over these shared roles.

## Usecase Event Handler

`kernel_oss::usecase_event_handler::UsecaseEventHandler` defines synchronous
typed delivery: `handle(&self, event: Self::Event) -> Result<(), Error>`.
The Handler is `Send + Sync`; its required associated `Event` has no additional
universal bound. Domain marker traits fix the exact finalized input, which may
be a parent Response or a dedicated Event. This role is distinct from Usecase
execution and Gateway capabilities.

`kernel_oss::usecase_event_handler::NoOpUsecaseEventHandler<Input>` provides
explicit `new()` and `Default` for `Input: Send + Sync`. It consumes and drops
its input before returning `Ok(())`, retains no invocation state and performs
no output. A destructor panic is not converted to success or Kernel Error.
Import `UsecaseEventHandler` for method lookup; there is no inherent `handle`.

The Domain crate owning a marker may bind the Kernel no-op only where the
parent contract permits it. Application explicitly selects and injects the
Handler; required delivery cannot use a no-op. Default construction is not an
implicit dependency fallback. Unit success from the no-op establishes no
boundary output. Parent event meaning, timing and failure policy remain
outside Kernel. Two markers sharing an input remain separate capabilities.

## Operational Logging

Introduced in 0.6.0 (release tag pending). See the
[0.6.0 changelog](../CHANGELOG.md#060--unreleased) for compatibility and migration.

| Type | Public path | Contract |
| --- | --- | --- |
| `Logger` | `kernel_oss::logging::Logger` | `Send + Sync`; required synchronous `info`, `warn`, `error`, each borrowing `&str` and returning `()`. |
| `NoOpLogger` | `kernel_oss::logging::NoOpLogger` | Infallible `new()` and `Default`; no output, retained input or invocation state. |

These are operational logging APIs, not Gateways. Unit return is not a delivery
acknowledgment. See [Logging](logging.md) for safety, concrete implementation
obligations and compatibility limits. Abandoned original messages are discarded;
emergency diagnostics report logger health only and never replay payloads.
The [diagnostic policy](logging.md#concrete-implementation-obligations)
owns routing, permitted fields, episode accounting and timing. NoOpLogger has no
sink failures and emits no diagnostics.

## Gateway Seams

### Shared Gateway Roles

| Trait | Module Path | Execution Model | Request | Response |
| --- | --- | --- | --- | --- |
| `VoidGateway` | `kernel_oss::gateway::VoidGateway` | sync | none | associated `Response` |
| `Gateway` | `kernel_oss::gateway::Gateway` | sync | associated `Request` | associated `Response` |
| `AsyncVoidGateway` | `kernel_oss::gateway::AsyncVoidGateway` | async | none | `ResponseFuture<Response>` |
| `AsyncGateway` | `kernel_oss::gateway::AsyncGateway` | async | associated `Request` | `ResponseFuture<Response>` |

### Kernel Gateway Marker Seams

| Trait | Module Path | Execution Model | Request | Response | Use When |
| --- | --- | --- | --- | --- | --- |
| `NewIdentityGW` | `kernel_oss::gateway::new_identity::NewIdentityGW` | sync | none | `ULID` | A caller needs a newly generated identity. |
| `AsyncNewIdentityGW` | `kernel_oss::gateway::new_identity::AsyncNewIdentityGW` | async | none | `ULID` | Async identity generation. |
| `CurrentUTCTimestampGW` | `kernel_oss::gateway::current_utc_timestamp::CurrentUTCTimestampGW` | sync | none | `UTCTimestamp` | A caller needs the current UTC timestamp. |
| `AsyncCurrentUTCTimestampGW` | `kernel_oss::gateway::current_utc_timestamp::AsyncCurrentUTCTimestampGW` | async | none | `UTCTimestamp` | Async timestamp retrieval. |
| `RetrieveDirectoryPathGW` | `kernel_oss::gateway::retrieve_directory_path::RetrieveDirectoryPathGW` | sync | `RetrieveDirectoryPathRequest` | `String` | A caller needs a directory path by key. |
| `AsyncRetrieveDirectoryPathGW` | `kernel_oss::gateway::retrieve_directory_path::AsyncRetrieveDirectoryPathGW` | async | `RetrieveDirectoryPathRequest` | `String` | Async directory path retrieval. |
| `FileDataGW` | `kernel_oss::gateway::file_data::FileDataGW` | sync | `FileDataRequest` | `Vec<u8>` | A caller needs file data by path. |
| `AsyncFileDataGW` | `kernel_oss::gateway::file_data::AsyncFileDataGW` | async | `FileDataRequest` | `Vec<u8>` | Async file data retrieval. |
| `WriteLogEntryGW` | `kernel_oss::gateway::write_log_entry::WriteLogEntryGW` | sync | `WriteLogEntryRequest` | `()` | Compatibility request/Result log-entry writing. |
| `AsyncWriteLogEntryGW` | `kernel_oss::gateway::write_log_entry::AsyncWriteLogEntryGW` | async | `WriteLogEntryRequest` | `()` | Compatibility async request/Result log-entry writing. |

## Compatibility And Deprecations

Deprecated APIs remain temporarily available for source compatibility. Prefer
the replacement APIs in new code.

| Deprecated API | Replacement |
| --- | --- |
| `gateway::identity::IdentityGateway` | `gateway::new_identity::NewIdentityGW` |
| `gateway::utc_timestamp::UTCTimestampGateway` | `gateway::current_utc_timestamp::CurrentUTCTimestampGW` |
| `gateway::directory_list::RetrieveDirectoryPath` | `gateway::retrieve_directory_path::RetrieveDirectoryPathGW` |
| `gateway::file_data_gateway::FileDataGateway` | `gateway::file_data::FileDataGW` |
| `gateway::logger::Logger` | `logging::Logger` (0.6.0; signature adaptation required) |

## Planned Moves / Do Not Copy

| Area | Current Status | Guidance |
| --- | --- | --- |
| `ULID` module | Currently `kernel_oss::ulid::ULID` | Planned move to `kernel_oss::values::ulid::ULID`; keep compatibility re-export. |
| `src::algorithms::signature_algorithm` | Still in kernel | Future move to a cryptographic domain project. Do not copy into new domains. |
| `src::algorithms::os_home_directory` | Still in kernel | Future kernel driver seam plus driver implementation. Do not model as a generic algorithm in new domains. |
| Public fields on old bounded types | Kept for compatibility | Prefer accessors in new code. Field privacy is a future breaking-change decision. |
| Fallible `new` / `build` surfaces | Still present in old APIs | Prefer `try_new` / `try_build` for new APIs and examples. |

## Examples

| Example | Teaches |
| --- | --- |
| `examples/value_object_and_entity.rs` | Implementing `Value`, implementing `Entity`, using `ULID` as direct identity. |
| `examples/gateway_usecase_composition.rs` | Sync request-bearing use case plus no-input gateway composition. |
| `examples/async_gateway_usecase_composition.rs` | Async `ResponseFuture` composition without public async trait methods. |
| `examples/unit_success_payload.rs` | `Response = ()` with explicit `Result<(), Error>` behavior. |
