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
- If an API looks old, check [Compatibility And Deprecations](#compatibility-and-deprecations).

When a matching type exists, prefer importing it from `kernel_oss` instead of
creating a duplicate in a domain crate. If the existing type almost fits but has
the wrong invariant, create a domain-specific type and document why the kernel
type was not reused.

## Role Standards

| Role | Standard Shape | Use When |
| --- | --- | --- |
| Value object | `values::Value` | A type exposes one canonical bounded value. |
| Entity | `entity::Entity` | A type has stable identity and should compare by identity. |
| No-input use case | `usecase::VoidUseCase` / `usecase::AsyncVoidUseCase` | A use case has no request object. |
| Request-bearing use case | `usecase::UseCase` / `usecase::AsyncUseCase` | A use case accepts a finalized request. |
| No-input gateway | `gateway::VoidGateway` / `gateway::AsyncVoidGateway` | A gateway has no request object. |
| Request-bearing gateway | `gateway::Gateway` / `gateway::AsyncGateway` | A gateway accepts a finalized request. |
| Async response | `response::ResponseFuture` | Async seams return a boxed future from a normal `execute` method. |
| Bounded error | `error::Error` | Public fallible APIs need classified user/system errors. |

Request builders are construction collaborators. They should build finalized
request objects before callers cross use case or gateway seams.

## Value Objects

### Foundational Values

| Type | Module Path | Canonical Value | Construction | Reuse Guidance |
| --- | --- | --- | --- | --- |
| `ULID` | `kernel_oss::ulid::ULID` | `u128` | `from_parts`, `from_string`, `nil`, `from_bytes` | Use as the default stable identity value. Planned move: prefer `kernel_oss::values::ulid::ULID` after the value-module migration. |
| `UTCTimestamp` | `kernel_oss::values::datetime::utc_timestamp::UTCTimestamp` | nanoseconds/milliseconds/seconds accessors | `builder()` | Use for bounded UTC timestamps. |
| `StartTime` | `kernel_oss::values::datetime::start_time::StartTime` | `u128` milliseconds | `now`, `from`, `try_from` | Use for process or operation start times. |
| `Line` | `kernel_oss::values::text::line::Line` | `str` | `try_from` | Use for bounded single-line text. |
| `Block` | `kernel_oss::values::text::block::Block` | `str` | `try_from` | Use for bounded multi-line text blocks. |
| `URL` | `kernel_oss::values::uri::url::URL` | parsed URL parts plus original line | `new` | Use when a URL must be parsed into scheme, host, path, query, and fragment. |

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
| `WriteLogEntryGW` | `kernel_oss::gateway::write_log_entry::WriteLogEntryGW` | sync | `WriteLogEntryRequest` | `()` | A caller needs to write one log entry. |
| `AsyncWriteLogEntryGW` | `kernel_oss::gateway::write_log_entry::AsyncWriteLogEntryGW` | async | `WriteLogEntryRequest` | `()` | Async log-entry writing. |

## Compatibility And Deprecations

Deprecated APIs remain temporarily available for source compatibility. Prefer
the replacement APIs in new code.

| Deprecated API | Replacement |
| --- | --- |
| `gateway::identity::IdentityGateway` | `gateway::new_identity::NewIdentityGW` |
| `gateway::utc_timestamp::UTCTimestampGateway` | `gateway::current_utc_timestamp::CurrentUTCTimestampGW` |
| `gateway::directory_list::RetrieveDirectoryPath` | `gateway::retrieve_directory_path::RetrieveDirectoryPathGW` |
| `gateway::file_data_gateway::FileDataGateway` | `gateway::file_data::FileDataGW` |
| `gateway::logger::Logger` | `gateway::write_log_entry::WriteLogEntryGW` |

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

## Proprietary Catalog Wrapping

`kernel-proprietary` can document its wrapped/re-exported surface the same way
it wraps code:

- Re-exported OSS items should keep their canonical docs in `kernel-oss`.
- The proprietary catalog should link to or compose this catalog.
- Proprietary-only additions should be listed in the proprietary catalog.
- Overrides or stronger internal guidance should be documented as proprietary
  additions, not by duplicating the OSS descriptions.

Recommended shape:

```text
kernel-proprietary catalog =
  kernel-oss catalog
  + proprietary-only value objects, entities, requests, responses, and seams
  + proprietary re-export notes
```
