# URL ABI migration freeze

| Current surface | Target / caller action | Compatibility and phase |
| --- | --- | --- |
| public fields | private finalized state | source breaking; P5 |
| `URL::new`, `value`, `line` | `try_new`, borrowed sensitive `value`, no `line` | source/ownership break; P5 |
| scheme/host/port/path/query/fragment accessors | borrowed accessors; optional values use `Option` | zero/empty sentinels removed; P5 |
| query_string/queries/query_count | `query` and borrowed ordered `query_pairs` | query representation break; P5 |
| Clone, Eq, Hash, Debug | retained canonical traits; redacted Debug | semantic redaction change; P5 |
| URL Display | absent | compile fail; P5 |
| RepositoryLink construction/errors/Display | target URL and redacted message/Display | preserve P1A Kind/Audience; P5 |
| external `url` dependency | removed only at final cutover | manifest/lock/tree/product removal; P6 |

RepositoryLink migrates to the redacted Display contract at P5. Legacy behavior
is compared with the accepted locked `url` 2.5.7 oracle; implementation
conformance is governed by WHATWG and approved WPT/Unicode inputs. Final release
cannot retain the external dependency.
