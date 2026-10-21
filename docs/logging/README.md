# Operational Logger consumer contract

[Documentation index](../README.md)

Consumer-contract document revision: **0.1.0**.
**This is a document revision, not the Kernel crate version or a Git release tag.**
The operational Logger is introduced in Kernel OSS **0.6.0**. Kernel OSS owns
this shared contract; consumers own dependency selection and composition.

## Construction and use

`NoOpLogger::new()` and `NoOpLogger::default()` are infallible, equivalent
construction paths. No builder or configuration is needed. Each of the three
methods ignores its borrowed message, emits nothing, retains no input or invocation
state and returns normally with `()`. Empty, whitespace and Unicode messages are
valid. Repeated and concurrent sharing has the same behavior. No-op is deliberate
absence of logging, not a degraded sink; it emits no health diagnostics.

Usage for the 0.6.0 API (also exercised by public rustdoc):

```rust
use kernel_oss::logging::{Logger, NoOpLogger};
use std::sync::Arc;

let logger: Arc<dyn Logger> = Arc::new(NoOpLogger::new());
logger.info("operation completed");
logger.warn("");
logger.error("operation unavailable");
NoOpLogger::default().info("discarded");
```

Import `Logger` for method lookup; there are no inherent logging methods.
Kernel does not register or resolve a default dependency. Application owns its
separate omission-selects-no-op policy and instance registration. Concrete
loggers belong on decorators or separate composition components, not as retained
dependencies in core Usecases/Gateways. Decorators preserve the original bounded
Response/unit/Error and provenance. Logs never replace business output or
required event delivery.

## Public contract

Consumers use `kernel_oss::logging::{Logger, NoOpLogger}` without depending on
Application. Logger is an operational dependency, separate from Gateway,
Usecase execution and business-event delivery.

```rust
pub trait Logger: Send + Sync {
    fn info(&self, message: &str);
    fn warn(&self, message: &str);
    fn error(&self, message: &str);
}
```

All three methods are required, synchronous, borrow their message and return
unit. There are no default method bodies or shared initialization hooks, extension
hooks, callbacks, preparation proxies, `debug` method, Error parameter, Result,
async Logger or implicit global installation. The interface supports `&dyn Logger`
and `Arc<dyn Logger>`. Its borrowed input does not by itself prevent a concrete
implementation from making a copy; the no-op's retention guarantee below is
stronger than the trait signature alone.

A normal return is not a receipt or proof that the primary destination received
the message. No arbitrary-implementation panic immunity, allocation-failure
immunity, nonblocking behavior or hard real-time bound is promised.

## Concrete implementation obligations

Kernel supplies no file/network/console sink, adapter, worker, queue, flush/shutdown
facility or universal health API. A concrete implementation owns synchronization,
safe concurrent preparation, data sanitization, recovery and finite supported
wait/retry policies. Messages must be safe for their intended operational audience;
secrets, credentials and prohibited sensitive data must not be logged.

Concrete file/network builders may offer eager or lazy preparation. Invalid
configuration may fail validation. Ordinary destination unavailability is handled
internally: eager preparation returns a usable degraded logger; lazy preparation
occurs during actual logging. Console/no-op need no timing option. Kernel has no
shared preparation enum or hook. A caller receiving unit does not receive a
log-delivery Error.

The following diagnostic policy is part of this Kernel consumer contract. These
are obligations for concrete implementations, not additional Logger methods.
Kernel supplies the neutral seam and no-op; it does not supply a production sink.

### Health diagnostics, never payload fallback

When a bounded primary operation fails, abandon the original message. Never copy
or replay its message/fields into an emergency destination, diagnostic queue or
last-error buffer. No original payload backlog or fallback queue is supplied.

| Primary/diagnostic condition | Action |
| --- | --- |
| File/network failure; stderr appropriate and captured for operators | Attempt a direct health-only stderr diagnostic. |
| stderr is primary, unavailable or inappropriate | Attempt an explicitly configured independent platform diagnostic facility if present. |
| Selected stderr diagnostic write fails | Attempt the configured independent platform facility once if present. |
| Platform attempt fails, or no eligible route exists | Update bounded diagnostic-failure state and stop. |

Independence concerns the actual failure path: a platform adapter must not route
back to the failed sink. No recursive logger call or unlimited fallback chain.
The implementation/deployment selects eligible routes; Application installs no
facility. Operators must have documented access to state or diagnostics. Counters
are not notification, and diagnostic delivery is not guaranteed when all routes
fail.

Diagnostic content is closed:

- `code`: `LOGGER_UNAVAILABLE`, `LOGGER_FAILURE_SUMMARY`, `LOGGER_RECOVERED`.
- `sink`: `file`, `network`, `stderr`, `other`; never a configured name/address.
- `operation` on failure/summary: `open`, `connect`, `write`, `flush`.
- `reason` on failure/summary: `permission_denied`, `timeout`,
  `connection_refused`, `storage_full`, `unknown`.
- `dropped`: nonnegative saturating count of messages locally abandoned during
  this episode.

Map structured error kinds/codes; do not parse or display arbitrary error text.
Unknown stays unknown. The latest categorized operation/reason may be retained
for summaries. Neither diagnostic content nor retained state may contain original
messages/fields, Request/Response/Event data, credentials, secrets, paths, URLs,
addresses, connection strings, user-provided sink names, configuration, raw errors,
debug output or stack traces. State is limited to fixed enums, counters and
timing/state metadata. Transport metadata must not introduce excluded data.

### Episodes, concurrency and bounded execution

1. First primary failure starts one degraded episode and attempts one immediate
   `LOGGER_UNAVAILABLE` notice. Count an abandoned message before the notice.
   Eager preparation failure has `dropped=0`. Start the monotonic summary window
   at the diagnostic attempt.
2. Each call abandoned after its bounded primary delivery attempt increments the
   count once. A partial file write or uncertain network acknowledgment may have
   had effects: local abandonment does not prove zero delivered bytes.
3. Aggregate repeated failures. Attempt at most one `LOGGER_FAILURE_SUMMARY`
   per 60 seconds per concrete sink instance. Advance the window even if
   diagnostics fail. Later failing calls may trigger a due summary; no timer,
   heartbeat or background thread is required.
4. The first subsequent successful actual primary write attempts one
   `LOGGER_RECOVERED` with the episode's cumulative count, then resets the episode
   regardless of diagnostic success. Open/connect success alone is not recovery.
5. A later failure starts a new episode. Immediate transition notices can flap;
   the summary interval is not a universal transition-rate cap. Stronger rate
   policies require separate contract treatment.

The concrete implementation synchronizes these transitions safely across callers
and keeps a separate bounded/saturating diagnostic-failure count for unsuccessful
emergency attempts. Diagnostic state does not retain original messages; abandoned
original messages are not replayed. Future primary
buffering policies require separate review and never authorize payload replay
through emergency routes.

Ordinary diagnostic I/O failures are handled internally with error-checked writes.
Finite supported policies must cover primary and diagnostic waits/retries,
including relevant DNS/TLS and queue backpressure. Unlimited retries do not
constitute a bounded call. Direct synchronous stderr lock/write alone does not
prove portable finite latency: concrete owners must document and verify supported
bounds or resolve that limitation before claiming bounded-call qualification.
No-op evidence establishes none of this concrete-sink behavior.

## Compatibility and migration

`gateway::logger::Logger` retains its signatures, behavior and existing
deprecation. Its replacement guidance points to the new operational Logger. The new API uses `warn`, has no `debug`, and accepts no Error or
context argument. Migration requires explicit signature adaptation and safe
message selection; there is no automatic conversion.

`WriteLogEntryGW`, `AsyncWriteLogEntryGW`, their Request builder, validation,
adapters, Result and async behavior remain supported compatibility surfaces.
They are not the new operational Logger. No additional executable API is removed
or deprecated. Error/provenance and all existing Usecase/Gateway/Handler contracts
remain unchanged. These are Rust source contracts, not stable layout/FFI/ABI
promises. Rebuild consumers against an actual immutable release; do not assume
similar names from different crate identities are interchangeable.

## Design and references

- [Logger design](design.md): ownership, logical paths, invariants and verification targets.
- [Source layout](source-layout.md): canonical exports, source responsibilities and test homes.
- [Test architecture](../testing.md): ordinary-interface and compiler-suite verification.

The diagnostic policy above is self-contained. Concrete implementations own its
execution and verification; the no-op has no sink failure or diagnostic behavior.
