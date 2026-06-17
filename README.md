# Attestify Kernel OSS

Shared Rust kernel types and seam traits for Attestify.

This crate provides:

- bounded error and value object support
- shared entity, response future, use case, and gateway traits
- sync and async execution seams
- compatibility modules for older gateway APIs while consumers migrate

## Shared Seams

Use cases:

- `usecase::VoidUseCase`
- `usecase::UseCase`
- `usecase::AsyncVoidUseCase`
- `usecase::AsyncUseCase`

Gateways:

- `gateway::VoidGateway`
- `gateway::Gateway`
- `gateway::AsyncVoidGateway`
- `gateway::AsyncGateway`

Supporting roles:

- `entity::Entity`
- `response::ResponseFuture`
- `values::Value`

## Gateway Migrations

The standards-aligned gateway seams live beside legacy compatibility APIs.

- Use `gateway::new_identity::NewIdentityGW` instead of `gateway::identity::IdentityGateway`.
- Use `gateway::current_utc_timestamp::CurrentUTCTimestampGW` instead of `gateway::utc_timestamp::UTCTimestampGateway`.
- Use `gateway::retrieve_directory_path::RetrieveDirectoryPathGW` instead of `gateway::directory_list::RetrieveDirectoryPath`.
- Use `gateway::file_data::FileDataGW` instead of `gateway::file_data_gateway::FileDataGateway`.
- Use `gateway::write_log_entry::WriteLogEntryGW` instead of `gateway::logger::Logger`.

Async marker seams are available for the same foundational capabilities when async callers need them.

### Log Entry Design

Logging is represented as one gateway capability: write a log entry. The request carries:

- `LogLevel`: error, warning, info, or debug
- `message`: the primary event or operation being logged
- `error`: optional structured failure context
- `context`: optional additional diagnostic detail

`message` is required even when `error` is present. The message describes the log event, while the optional `Error` carries bounded failure context for that event. This keeps info/debug logs from needing fake errors and keeps error logs from overloading `Error::message` as the entire log line.

## Examples

Run the checked examples from the repository root:

```text
cargo run --example value_object_and_entity
cargo run --example gateway_usecase_composition
cargo run --example async_gateway_usecase_composition
cargo run --example unit_success_payload
```

## Catalog

Use [`docs/kernel-catalog.md`](docs/kernel-catalog.md) before creating new
domain or application types. The catalog indexes reusable kernel value objects,
entity/use-case/gateway role traits, request and response types, gateway seams,
compatibility migrations, and planned moves so consumers can avoid duplicating
kernel behavior.

Use [`docs/documentation-audit.md`](docs/documentation-audit.md) to track the
remaining rustdoc cleanup needed before strict documentation enforcement.

Use [`docs/test-audit.md`](docs/test-audit.md) to track the current test
surface, the logical-path audit, and documentation gaps for each object.

Use [`docs/module-audit.md`](docs/module-audit.md) to track module-organization
deviations against the Rust module standard.

Use [`docs/clippy-audit.md`](docs/clippy-audit.md) to review the current
clippy backlog, grouped by category and file.

## Verification

```text
cargo fmt --all --check
cargo test
cargo check --examples
```
