# URL value

[Documentation index](../../../README.md)

## Public contract

`kernel_oss::values::uri::url::URL` is an immutable, dependency-free WHATWG URL
value. It owns canonical private state and exposes borrowed views of that state.

Construct a URL with the fictional credentials below to illustrate sensitive
component access. Never substitute production credentials in examples:

```rust
use kernel_oss::values::Value;
use kernel_oss::values::uri::url::URL;

let url = URL::try_new("https://user:secret@example.test/a?one=1#section")
    .expect("the example URL is valid");

assert_eq!(url.scheme(), "https");
assert_eq!(url.username(), "user");
assert_eq!(url.password(), Some("secret"));
assert_eq!(url.host(), Some("example.test"));
assert_eq!(url.port(), None);
assert_eq!(url.path(), "/a");
assert_eq!(url.query(), Some("one=1"));
assert_eq!(url.fragment(), Some("section"));
```

Optional accessors distinguish absence from a present empty component. For
example, `None` query means there was no query delimiter, while `Some("")`
means an empty query was present. `query_pairs()` returns a borrowed,
source-ordered, duplicate-preserving decoded form view.

## Resolution

`try_resolve(&self, reference)` treats `self` as the base URL, applies WHATWG
relative-reference rules, and returns a new canonical `URL`. It does not mutate
the base and performs no DNS lookup, redirect, network request, filesystem
access, or runtime I/O.

```rust
use kernel_oss::values::Value;
use kernel_oss::values::uri::url::URL;

let base = URL::try_new("https://example.test/a/b/")
    .expect("the base URL is valid");
let result = base
    .try_resolve("../report")
    .expect("the relative reference resolves");

assert_eq!(result.value(), "https://example.test/a/report");
assert_eq!(base.value(), "https://example.test/a/b/");
```

## Immutable mutation

`mutator()` gathers requested component replacements. Its consuming setters
copy caller input; the last call to the same setter wins. `try_mutate()` applies
the complete replacement set once, validates the result through the canonical
engine, and returns a new URL without changing the original.

Supported setters cover scheme, username, password, host, port, path, query,
and fragment. Passing `None` clears an optional component. Host input is a host
component, not an authority string: it cannot inject credentials, ports, paths,
queries, or fragments.

## Equality, hashing, and errors

Equality and hashing use the complete canonical URL, including credentials,
query, and fragment. Original spelling and private parser state are not part of
identity.

Caller-input failures are bounded Kernel errors:

| Operation | Audience | Kind | Message |
| --- | --- | --- | --- |
| `try_new` | `User` | `InvalidInput` | `URL parsing failed.` |
| `try_resolve` | `User` | `InvalidInput` | `URL resolution failed.` |
| `try_mutate` | `User` | `InvalidInput` | `URL mutation failed.` |

The messages disclose no input, canonical URL, credentials, query, fragment,
upstream wording, or private parser state.

## Sensitive values

`value()` deliberately exposes the complete canonical serialization. It may
contain secrets and must be used only at an explicit authorized boundary.

`Debug` is exactly `URL([REDACTED])`. `URL` intentionally has no `Display`,
`Copy`, `Default`, ordering, `FromStr`, general conversion, or serde contract.
The API promises Rust source behavior, not stable layout, FFI, or binary ABI.

## Design and verification

See [Design and maintenance](design.md) for private architecture, source and data
custody, and verification commands. These details do not change the public API.
