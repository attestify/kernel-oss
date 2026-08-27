# URL ABI migration freeze

| Current surface | Target and caller action | Impact / phase |
|---|---|---|
| public URL fields | private state; use accessors | source break, P5 |
| `URL::new` | `URL::try_new` | rename/fallible, P5 |
| `value()` | borrowed sensitive `value()` | remove clone, P5 |
| `line()` | no replacement | removed, P5 |
| `scheme()` | retained borrowed | canonical value, P5 |
| `host()` | `Option<&str>` | empty sentinel removed, P5 |
| `port()` | `Option<u16>` | zero sentinel removed, P5 |
| `path()` | retained borrowed | canonical path, P5 |
| `query_string()` | `query()` | absent versus empty, P5 |
| `queries()` | `query_pairs()` | borrowed ordered slice, P5 |
| `query_pairs()` | retained borrowed slice | no allocation, P5 |
| `query_count()` | `query_pairs().len()` | convenience removed, P5 |
| `fragment()` | `Option<&str>` | absent versus empty, P5 |
| Clone | retained | canonical value, P5 |
| PartialEq/Eq | retained | complete canonical equality, P5 |
| Hash | retained | equality-consistent hash, P5 |
| Debug | manual redacted Debug | security change, P5 |
| URL Display | absent | compile failure, P5 |
| RepositoryLink construction | target URL facade | migrate builder internals, P5 |
| RepositoryLink errors | preserve P1A Kind/Audience; redact text | security, P5 |
| RepositoryLink Display | `RepositoryLink([REDACTED])` | security, P5 |
| external `url` | remove manifest/lock/tree/product use | final cutover, P6 |

New target-only APIs are `username()`, `password()`, `has_opaque_path()`,
`try_resolve()`, and `mutator()`/`URLMutator`.
