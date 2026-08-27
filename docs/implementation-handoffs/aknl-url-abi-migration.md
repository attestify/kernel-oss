# URL ABI migration freeze

Current public fields become private finalized state. `URL::new` is removed at
the breaking cutover and replaced by `try_new`. Owned `value()` becomes borrowed;
`Line` is no longer public. Empty string and port-zero sentinels become
absence-aware `Option` components. Owned query access becomes the borrowed,
ordered pair slice. Derived Debug becomes the approved redacted implementation.

RepositoryLink migrates to the redacted Display contract at P5. Legacy behavior
is compared with the accepted locked `url` 2.5.7 oracle; implementation
conformance is governed by WHATWG and approved WPT/Unicode inputs. Final release
cannot retain the external dependency.
