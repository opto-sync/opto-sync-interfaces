# opto-sync-interfaces

Transport-neutral schemas, events, configuration, and generated cross-language
contracts for Opto Sync.

## Status

Bootstrap repository. No package or registry publication is enabled yet. The
first implementation pull request must import reviewed contracts from their
current source, preserve provenance, and add deterministic compatibility tests
before this repository becomes authoritative.

## Ownership boundary

This repository owns only language-neutral public contracts:

- versioned JSON Schemas for operations, causal envelopes, conflicts,
  checkpoints, connectivity, retry state, queue state, and observability;
- deterministic fixtures for valid, invalid, forward-compatible, and
  backward-compatible payloads;
- code-generation inputs and generated interface packages for supported
  languages;
- compatibility and breaking-change classification for those contracts.

This repository does **not** own merge algorithms, storage adapters, network
transports, background workers, application orchestration, or deployment
infrastructure. Those remain in `syncer.c`, `syncer.rs`,
`opto-sync-clients`, and their owning applications.

## Dependency direction

```text
opto-sync-interfaces
        ^
        |
opto-sync-lib
        ^
        |
opto-sync-clients
        ^
        |
opto-sync-cli / applications
```

Interfaces must never depend on a sync engine or client implementation.
Generated packages must be reproducible from committed schemas, and consumers
must pin an immutable interface revision or released package.

## First implementation gates

- Record source repository, commit, tree, and per-file digests for imported
  contracts.
- Define schema IDs, compatibility policy, and deterministic code generation.
- Prove generated Dart, TypeScript, Rust, Kotlin, Swift, Java, and C-facing
  representations accept and reject the same canonical fixtures.
- Add negative tests for unknown versions, malformed envelopes, duplicate
  identifiers, invalid causal metadata, and unsafe unbounded fields.
- Keep publication disabled until clean-room consumers resolve an immutable
  lock and pass cross-version compatibility tests.
