# Kernel Repository README Standard

## Status and purpose

This document is the local authoring standard for the Kernel OSS repository-root
`README.md`. It governs initial writing and later maintenance of that file.
Area landing pages under `docs/` follow the
[library documentation convention](README.md#appendix-a-library-documentation-convention).

A Kernel README is the durable entry point for a consumer or maintainer who
does not know the repository's implementation history. It must explain what the
crate is, what it owns and which declared dependencies it uses, how to consume it safely,
where deeper reference material lives, and how to verify it.

A README is not a project tracker, approval record, implementation handoff,
test ledger, release evidence packet, or changelog.

## Required audience

Write for these readers, in this order:

1. a Rust consumer deciding whether and how to depend on the crate;
2. a maintainer locating the correct architecture and API references; and
3. a reviewer checking ownership, security, dependency, and verification
   boundaries.

Do not assume that the reader knows internal phase names, team names, planning
directories, or historical decisions.

## Required section order

The repository-root README must use the following information order. Headings may be
specialized when clarity improves, but the content order must remain stable.

1. **Title and summary** — canonical repository/crate name and a two- or
   three-sentence statement of responsibility and non-responsibility.
2. **Dependency or installation** — the supported immutable release coordinate
   and the correct Cargo package name.
3. **Main capabilities** — a curated list of public responsibilities, not a
   dump of every module.
4. **Quick start** — the smallest compiling examples that demonstrate the
   primary consumer path.
5. **Ownership or composition** — what this crate owns and what it
   re-exports or delegates through its declared dependencies.
6. **Security and data handling** — sensitive accessors, redacted formatting,
   trust boundaries, and APIs that intentionally do not exist.
7. **Architecture and reference documentation** — links to durable formal docs,
   the API reference, and any curated catalog.
8. **Examples and verification** — runnable repository examples and the normal
   contributor commands.
9. **Versioning and distribution** — compatibility expectations, immutable tag
   guidance, publication boundary, and license location.

Omit an inapplicable section instead of filling it with ceremony. Do not omit
security, ownership, or versioning information when those concerns are
material.

## Content rules

The README must:

- describe the current released architecture in present tense;
- use public Rust paths exactly as consumers import them;
- distinguish owned implementations from direct re-exports;
- distinguish an explicit sensitive accessor from automatic formatting;
- distinguish Rust source/API compatibility from binary layout or FFI ABI;
- use relative links that work in both the repository and packaged source;
- keep examples deterministic, minimal, and compilable; and
- point detailed behavior, architecture, custody, and maintenance material to
  formal documents under `docs/`.

The README must not contain:

- implementation phase identifiers, approval codes, readiness status, or team
  handoff language;
- commit hashes, tag-object hashes, timestamps, reviewer identities, or command
  evidence;
- plans for work that is already complete;
- raw test counts or benchmark samples that will become stale;
- evidence manifests, source-transfer history, correction rounds, or stop
  conditions;
- unresolved TODO lists masquerading as consumer guidance; or
- internal implementation details that are neither a consumer contract nor a
  durable architectural boundary.

Use Git history and release notes for history. Use issues or plans for future
work. Use source tests and CI for current verification evidence.

## Example requirements

Examples must:

- use the public crate name rather than internal modules;
- show explicit error handling with meaningful `expect` text when brevity
  requires a panic in documentation;
- avoid network, environment, clock, filesystem, or randomness dependence;
- avoid printing sensitive values; and
- be exercised by doctest or an equivalent checked example whenever practical.

Do not demonstrate a deprecated or compatibility API as the preferred path.

## Documentation boundary

Use formal documents under `docs/` for material that is durable but too deep
for the README, including:

- public behavior beyond the quick start;
- component or interaction architecture;
- ownership and dependency direction;
- security constraints and intentional API omissions;
- source/data custody that consumers or maintainers must preserve; and
- deterministic generator or conformance maintenance instructions.

Do not retain completed implementation handoffs alongside formal docs. Promote
the durable contract, then delete the handoff.

## Maintenance triggers

Review the README whenever any of the following changes:

- crate or repository name;
- supported release coordinate;
- public import path or construction API;
- ownership or dependency boundaries;
- sensitive-data or diagnostic behavior;
- preferred seam or deprecated compatibility path;
- verification commands;
- distribution or license boundary; or
- formal documentation location.

## Review checklist

A conforming README answers all of these questions without implementation
history:

- What is this crate responsible for?
- What is it explicitly not responsible for?
- How does a consumer depend on it?
- What are the primary public paths?
- Which crate owns each shared capability?
- Which values or diagnostics are sensitive?
- Where is deeper architecture and API documentation?
- How does a maintainer run the normal checks?
- What compatibility and distribution promises are made?

If the answer to any applicable question is missing, revise the README before
release.
