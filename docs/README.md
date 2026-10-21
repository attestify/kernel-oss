# Kernel OSS documentation

Start with the [crate README](../README.md) for installation and a quick start,
or the [Kernel catalog](kernel-catalog.md) to find a reusable public type.
These guides explain how to use the library and its safety boundaries. Rustdoc
provides the detailed API reference; generate it with
`cargo doc --no-deps --locked --offline`.

## Consumer guides

| Area | Guide | What it explains |
| --- | --- | --- |
| Logging | [Operational logging](logging/README.md) | Construct and share a Logger, choose a no-op, implement the interface safely and migrate legacy callers. |
| Text | [NFC text](values/text/README.md) | Construct canonical text and understand normalization behavior. |
| URLs | [URL value](values/uri/url/README.md) | Parse, resolve and update URLs; handle errors and sensitive values. |
| Shared types and seams | [Kernel catalog](kernel-catalog.md) | Find Errors, Values, Usecases, Gateways and typed Event Handlers. |
| Upgrading | [Changelog](../CHANGELOG.md) | Release changes, compatibility and migration guidance. |

## Design and contributor references

- [Logging design](logging/design.md) explains behavior and ownership boundaries.
- [Logging source layout](logging/source-layout.md) maps implementation and tests.
- [Test architecture](testing.md) explains verification and fixture custody.
- [Repository README standard](readme-standard.md) governs the root README.
- [Library documentation convention](#appendix-a-library-documentation-convention)
  governs feature guides and their navigation.

## Appendix A: Library documentation convention

### Purpose and scope

This is the local Kernel OSS convention for maintaining library documentation.
It governs documentation organization and authoring, not software architecture
or release approval. The root README remains governed by the
[repository README standard](readme-standard.md). Other projects do not adopt
this convention merely by depending on Kernel OSS.

Write for someone using the public library without knowledge of its development
history. Explain the task they can accomplish, how to call the API, and what
behavior and limitations they can rely on. Keep design and contributor details
separate from the initial usage path, with links when useful.

### Organize by public API area

Use directories that follow recognizable public module paths where useful.
Mirror the consumer's API structure, not every private source directory or file.
A private helper, test module or implementation leaf does not need its own guide.
Group related types when readers use them together.

Each documented area has a `README.md` landing page. Add focused sibling guides
only when the material would make the landing page difficult to use. A small
area needs only its README; do not create empty directories or placeholder pages.
Use lowercase descriptive filenames for supporting pages, such as `design.md`
or `migration.md`. Keep filenames version-neutral; release history belongs in
the root [CHANGELOG.md](../CHANGELOG.md).

The current area layout follows this organization:

```text
docs/
  README.md                 Documentation index and this convention
  kernel-catalog.md         Cross-library public API discovery
  logging/
    README.md               Using Logger and NoOpLogger
    design.md               Ownership and behavior rationale
    source-layout.md        Maintainer-oriented source map
  values/
    text/
      README.md             Text guidance, including NfcText
      design.md             Normalization architecture and data custody
    uri/
      url/
        README.md           Using the public URL value
        design.md           Private architecture and data custody
        architecture.puml   Architecture diagram source
```

The public Logger is re-exported from `logging`; guides do not mirror its private
`seam` or `no_op_logger` directories. Shared topics such as test architecture
remain at the documentation root when no one API area owns them.

### Content of an area landing page

Use the following reading order, omitting genuinely inapplicable sections:

1. Purpose and when to use the API, including important alternatives or limits.
2. Exact public import paths and a minimal working usage example.
3. Construction and configuration, including defaults and required choices.
4. Normal behavior: inputs, outputs, ownership and relevant state or side effects.
5. Failure and safety behavior: errors, sensitive data, concurrency and limits
   that affect the caller. State when a successful return is not a delivery receipt.
6. Compatibility or migration guidance when existing consumers need it.
7. Links to deeper guides, design and API reference.

Introduce only concepts needed to use the area. Do not fill in an inapplicable
section with ceremony. Avoid repeating installation instructions in every guide;
link to the root README. Explain availability accurately for unreleased APIs.

### Examples and sources of truth

Use real public imports and supported construction paths. Examples must include
necessary trait imports and meaningful error handling. Show safe, deterministic
inputs without credentials or production data. Never present a compatibility API
as the preferred new interface without explaining why.

Compile Rust examples through rustdoc, a checked example or an equivalent test.
When a Markdown example is not automatically extracted, identify and check its
matching executable example or verify the snippet explicitly. Markdown/link
validation alone does not establish that code compiles.

Rustdoc owns detailed symbol reference. Area guides own usage narratives and
consumer obligations. Design pages explain rationale and boundaries. Link to
existing authoritative explanations instead of copying independently editable
policies into multiple pages. Keep all required consumer guidance available in
the OSS repository and package.

Do not put approval records, delivery tasks, acceptance ledgers or completed
implementation plans into consumer guides. Maintainer references may describe
source structure and verification, but must be identified as such and must not
be required to understand a basic usage example.

### Navigation and packaging

Link every area landing page from this index, directly or through a parent area.
Link supporting pages from their owning landing page and provide a path back.
The root README links this index; the catalog links the relevant consumer guide.
Use relative links and stable heading anchors that work in both the repository
and packaged source. Ship nested Markdown guides and referenced diagrams through
Cargo's package include list; check the package file list after moving files.

### Updating documentation with a change

For each public API or behavior change:

- Update the owning area guide and affected rustdoc, examples and design guidance.
- Update this index and catalog when public areas, paths or navigation change.
- Record release and migration implications in the root changelog.
- Check the examples against the actual source and dependency state.
- Validate changed Markdown, links, anchors and whitespace, including new files.
- Verify packaged documentation includes every referenced local guide or diagram.

For a documentation move, inventory inbound links in source comments, examples,
README, catalog and other guides before removing the old path. Update those links
and the moved page's relative links together. Preserve useful historical migration
instructions; do not relabel old behavior as a new release change.

### Existing documents

The existing Logging, text and URL guides use the area layout above. Apply this
layout to new guides and keep shared references at the documentation root. This convention does not require a bulk source or
documentation reorganization, and it does not create a documentation folder for
every source module.
