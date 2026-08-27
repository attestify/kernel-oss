# AKNL-URL-P1B public contract and design freeze

Status: frozen design only. P2 and later implementation remain unauthorized.

## Public contract

`kernel_oss::values::uri::url::URL` is a public value with private finalized
state. Construction is `URL::try_new(&str) -> Result<Self, Error>` and the type
implements `Value<ValueType = str>`. `value()` borrows the complete canonical
serialization and may disclose credentials, query data, or fragments; callers
must treat it as sensitive. Component accessors are borrowed and use `Option`
for absence: password, host, port, query, and fragment. Query pairs are the
borrowed `&[(String, String)]` source-order, duplicate-preserving decoded view.

`try_resolve(&self, reference: &str) -> Result<Self, Error>` uses `self` as the
WHATWG base, returns a new canonical value, mutates nothing, and performs no
I/O, DNS, redirects, networking, or filesystem work. Opaque-base behavior is
the approved WHATWG behavior. `mutator()` returns private-state `URLMutator`;
its consuming component setters take `&str`/`Option<&str>` and only
`try_mutate()` reconstructs and validates a new `URL`.

Approved traits: `Clone`, `PartialEq`, `Eq`, `Hash`, and manual `Debug` exactly
`URL([REDACTED])`. URL has no `Display`, `Copy`, `Default`, ordering, `FromStr`,
public conversion, or serde contract. Equality and hashing use canonical URL
equality. Errors are Kernel-owned redacted errors with exactly: `URL parsing
failed.`, `URL resolution failed.`, or `URL mutation failed.`

No public item may expose a `url`-crate type, engine type/state, IDNA or NFC
type/table, or stable layout/FFI promise. `RepositoryLink` migration at P5 keeps
`Display` but makes it exactly `RepositoryLink([REDACTED])`.

## Private design

One private canonical parse/serialize engine serves construction, resolution,
and mutation. Private modules cover finalized state, WHATWG parser and
serializer, resolver, host/IPv4/IPv6, IDNA/domain, percent/form processing, and
Kernel error translation. IDNA consumes the existing crate-private shared NFC
seam; it creates neither a second normalizer nor NFC tables/generator.

The design requires deterministic no-I/O parsing. Resource bounds and panic
safety must be reviewed against the approved WHATWG algorithms during P2; no
arbitrary input bound is frozen here.

## Compatibility and qualification

Public fields, `Line` exposure, `URL::new`, owned `value`, empty/zero sentinels,
derived Debug, and inherited wording are removed at the approved breaking
cutover. They become private canonical state, `try_new`, borrowed value,
`Option`, redacted Debug, and stable Kernel errors. The external `url`
dependency remains black-box-only until final cutover removes it from manifest,
lock, tree, product code, and package.

P5 manual compile-pass cases cover exact signatures, `Value<str>`, traits,
resolution, chaining, and borrowed pairs. Compile-fail cases cover fields,
`new`, URL Display, absent traits/conversions, mutator internals, and all engine,
IDNA, NFC, and `url` types. P6 measures absolute parsing, resolution, query
access, percent-heavy data, IP hosts, IDNA, invalid rejection, bounded long
inputs, and corpus throughput; numeric limits are selected then.

## Design review

The boundary preserves immutable finalized values and contains mutable requests
inside an owned mutator. Diagnostics are redacted except explicit sensitive
`value()`. A single parse path prevents constructor/resolver/mutator drift.
Future P2 must explicitly assess bounded resource consumption and panic safety.
