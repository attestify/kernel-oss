# Operational Logger design

Design document identity: `kernel-oss-operational-logger@0.1.0`.
**0.1.0 is this design document revision, not a Kernel crate version or Git tag.**
The API is introduced in Kernel OSS 0.6.0.
Owner: independently governed Attestify Kernel OSS.
[Consumer contract](README.md) defines public behavior;
[source layout](source-layout.md) maps the implementation and tests.

## Purpose

Provide a shared operational logging interface without an Application dependency,
and an explicitly constructed no-op whose absence of effects is predictable.
This design describes the shared operational logging seam and its explicit no-op;
it does not introduce a Domain Gateway, Usecase or Value.

## Scope

### In scope

One synchronous operational seam, its explicit no-op, canonical public exposure,
ordinary consumer verification, compatibility-preserving guidance and documentation
of concrete implementation obligations. The accepted [consumer contract](README.md)
owns externally visible behavior.

### Out of scope

No production sinks, decorators, consumer registration, async Logger,
initialization hook, global installation, worker,
queue, health API, schema type, flush/shutdown, new Error, or API removal.
Release versioning and publication are separate from this design.

## Participants and boundaries

| Participant | Responsibility |
| --- | --- |
| Kernel OSS | Own canonical Logger declaration and stateless NoOpLogger |
| Consumer | Supply safe borrowed messages; explicitly select concrete implementation |
| No-op | Ignore each message and return unit without effects |
| Real concrete logger owner | Own synchronization, preparation, delivery and adopted safety/failure policy; no implementation supplied here |
| Application/composition | Own registration and injection; preserve core Usecase/Gateway boundaries |
| Legacy consumers | Retain existing Logger and logging Gateway behavior unchanged |

## Behavioral flow

1. Consumer explicitly constructs no-op through new or Default; no configuration
   or external operation occurs. Default delegates to new.
2. Consumer invokes one required logging method on an implementation directly or
   through a shared trait object. No shared initialization is invoked.
3. The no-op ignores borrowed text and returns unit, retaining nothing. Repeated
   and concurrent calls have the same behavior.
4. Real implementations own their effects/failure policy. Return is not a delivery
   receipt. This component neither implements nor qualifies production delivery.

## Logical paths

Error contract is N/A for every no-op path: no Kernel Kind, Audience or message
mapping exists. Unit is a return value, not a finalized business Response.

| Path ID | Initial state and trigger | Input or decision | Ordered calls and maximum count | Expected result | Error contract | Final outcome | State change | Verification targets |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| LOG-P01 | No object; explicit construction | new | One constructor | Usable no-op | N/A | Stateless object | None externally | LOG-T01 |
| LOG-P02 | No object; default construction | Default | One default, one new delegation | Equivalent no-op | N/A | Stateless object | None externally | LOG-T01 |
| LOG-P03 | No-op ready; info | Borrowed text | One call, zero sink calls | Unit | N/A | Normal return | None | LOG-T02, LOG-T07 |
| LOG-P04 | No-op ready; warn | Borrowed text | One call, zero sink calls | Unit | N/A | Normal return | None | LOG-T03, LOG-T07 |
| LOG-P05 | No-op ready; error | Borrowed text | One call, zero sink calls | Unit | N/A | Normal return | None | LOG-T04, LOG-T07 |
| LOG-P06 | Concrete implementation shared; selected method | Exact text and operation | One dynamic dispatch per call | Selected method observes exact text | Concrete-owned; no Result channel | Unit on normal return | Concrete-owned | LOG-T05 |
| LOG-P07 | Shared no-op; finite repeat/concurrent calls | Any valid borrowed text | One method per invocation; no background calls | Unit per call | N/A | Independent normal returns | No accumulation | LOG-T06, LOG-T07 |
| LOG-P08 | Consumer defines invalid implementation | Missing method, wrong result, unsupported hook or missing bounds | Compiler checking only | Reject intended obligation | Compiler diagnostics, not Kernel Error | No executable invalid binding | None | LOG-T08 |

## Contract

The accepted Rust constraint is exactly `Logger: Send + Sync` with required
`info(&self, message: &str)`, `warn(&self, message: &str)` and
`error(&self, message: &str)`, each returning unit. The trait is object-safe.
No inherited behavior or common preparation lifecycle. NoOpLogger is stateless;
new and Default are infallible. Empty/whitespace/Unicode text remains valid.
No-op stores no input/resource/invocation state, performs no allocation or I/O,
and generates no failure diagnostic. The trait's borrow alone does not prove
that arbitrary real implementations retain nothing.

Invariants: canonical identity, no Application/runtime dependency, no hidden
initialization or sink, no false acknowledgment, preserved legacy behavior,
and accurate safety-policy boundaries. Legacy signature adaptation is explicit;
no automatic conversion is supplied. Shared module/layout choices are realization
concerns, not new behavioral contracts.

## Diagrams

Not applicable: this stateless three-operation abstraction has no persisted
model, topology, protocol or internal state transition requiring a separate
view. Participants and logical-path tables completely describe this component.
Concrete sink episode diagrams would falsely suggest an implementation in scope.
No separate diagram is needed for the seam and no-op; the participants and
logical paths above describe their behavior.

## Failure and boundary behavior

No-op has no invalid construction or runtime failure branch to invent. Empty,
whitespace, Unicode, repeated and concurrent messages remain no-effects unit
calls. The caller may mutate/drop its source text after return.

Arbitrary implementation panic/allocation failure is not translated. Normal
return does not prove delivery, and synchronous does not mean nonblocking.
Concrete sink failures, partial delivery, eager zero-count failure, diagnostic
route exhaustion, summary timing and recovery remain exactly as described by
consumer-contract sections Concrete implementation obligations and Episodes.
Here they are documentation fidelity obligations, not claims of executable
routing, timing or operator notification. No-op is absence of logging, not failure.

## Verification targets

| Target ID | Kind | Claim | Required evidence |
| --- | --- | --- | --- |
| LOG-T01 | path | new/Default construct equivalent usable no-op | Public construction tests and delegation source review |
| LOG-T02 | path | info returns unit without effects | Actual no-op calls and silent subprocess |
| LOG-T03 | path | warn returns unit without effects | Actual no-op calls and silent subprocess |
| LOG-T04 | path | error returns unit without effects | Actual no-op calls and silent subprocess |
| LOG-T05 | path | Trait-object dispatch preserves operation and text | Recording concrete consumer through actual public trait; Arc coercion |
| LOG-T06 | path | Repeated/concurrent no-op calls retain no invocation state | Shared calls plus source inspection |
| LOG-T07 | invariant | No-op has no allocation, I/O, retention or hidden state; source borrow may expire | Source review, input lifetime use and no-output observation; tests alone do not prove no copying |
| LOG-T08 | invariant | Exact required synchronous Send+Sync shape, no hook/default bodies/Result | Positive control and intended negative compiler failures; signature review |
| LOG-T09 | invariant | Canonical public exports, independent OSS build | External imports and dependency/build review |
| LOG-T10 | invariant | Legacy Logger and all WriteLogEntry behavior retained | Existing tests, exact public-signature/validation/error/adaptor comparison |
| LOG-T11 | invariant | Docs faithfully state no receipt/no panic or latency guarantee and accepted concrete policy | Document-to-approved-contract review, no backend claims |
| LOG-T12 | invariant | Existing Error/provenance, Usecase/Gateway/Handler APIs unaffected | Full baseline, scoped diff/public API preservation review |

## Traceability

Governing design: `kernel-oss-operational-logger@0.1.0` (document only).
Source annotations use this identity and the targets above. Source/test paths
and Layout IDs are maintained in the [source layout](source-layout.md).

| Consumer requirement | Owned targets | Bounded contribution |
| --- | --- | --- |
| [Public contract](README.md#public-contract) | LOG-T05, LOG-T08, LOG-T09 | Exact operational seam and independent ownership |
| [Construction and use](README.md#construction-and-use) | LOG-T01–LOG-T07 | Explicit no-op and observable unit/no-effects behavior |
| [Concrete implementation obligations](README.md#concrete-implementation-obligations) | LOG-T11 | Faithful documentation only; actual sinks and qualification external |
| [Compatibility and migration](README.md#compatibility-and-migration) | LOG-T10, LOG-T12 | Retain executable compatibility and explain source migration |

The consumer contract owns the diagnostic policy. Concrete implementations and
consuming applications own their separate integration and qualification; this
library design does not establish those outcomes.
