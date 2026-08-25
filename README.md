# opto-sync-interfaces

Transport-neutral schemas, events, configuration, and generated cross-language
contracts for Opto Sync.

## Status

Implemented contract source; registry publication remains deliberately
disabled. `provenance.json` records the exact `opto-sync-clients` commit, tree,
ordered source files, and SHA-256 digests used for the initial extraction. The
canonical ingest schema is byte-identical to its reviewed source at commit
`1d22a98fbef4888e36ca1f78b72d469f74f61721`.

## Repository layout

- `schemas/` contains stable-ID JSON Schema authorities.
- `src/lib.rs` is the type-only Rust representation.
- `generated/` contains type-only TypeScript, Dart, Kotlin, Swift, Java, and C
  representations. These packages contain declarations, not transports,
  persistence, validation engines, or merge bodies.
- `fixtures/` carries deterministic positive and negative wire examples.
- `scripts/verify_contracts.py` locks schema bytes, provenance, language
  coverage, and the no-runtime-implementation boundary.
- `tests/contract.rs` proves Rust wire names and fixture outcomes.

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

## Validation

```sh
python3 scripts/verify_contracts.py
cargo fmt --all -- --check
cargo test --all-targets --locked
cargo clippy --all-targets --locked -- -D warnings
tsc --noEmit --strict generated/typescript/index.ts
dart analyze generated/dart/lib/opto_sync_interfaces.dart
swiftc -typecheck generated/swift/Sources/OptoSyncInterfaces/Contracts.swift
javac generated/java/src/main/java/dev/optosync/interfaces/Contracts.java
clang -std=c17 -Wall -Wextra -Werror -fsyntax-only -x c \
  generated/c/include/opto_sync_interfaces.h
```

Kotlin syntax is CI-validated with the pinned Kotlin compiler. Publication
stays disabled until clean-room consumers resolve an immutable lock and pass
cross-version compatibility tests.
