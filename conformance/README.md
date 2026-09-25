# Conformance declarations

This repository is declaration-only. `conformance/` records falsifiable properties and required controls that implementation/E2E repositories must execute; validators, merge engines, transports, persistence, schedulers, and telemetry code do not belong here.

## Mutation/checkpoint requirements

`mutation-checkpoint.properties.json` declares the bounded protocol properties that downstream executable lanes must refine against real implementation commits:

- duplicate `(client_id, mutation_id)` delivery is idempotent;
- applied-count state reflects unique applied mutation identities;
- checkpoints never regress;
- a checkpoint cannot acknowledge state beyond durably applied mutations.

A qualifying executable checker must also prove it can fail by rejecting intentionally broken controls for duplicate-counting and checkpoint leap-ahead.

These declarations do **not** establish CRDT/OT semantics, causal consistency, production durability, or cross-runtime convergence. Such claims require implementation-linked execution in `opto-sync-e2e` and/or the owning runtime repository.

Only exact-head, actually executed, stepful green downstream evidence may satisfy these declarations. Queued, skipped, zero-step, stale-head, billing/admission-blocked, missing-run, and historical-only results are not green evidence.
