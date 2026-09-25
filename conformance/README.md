# Conformance

Conformance turns contract claims into executable, falsifiable checks. A passing declaration-only diff is not enough: each check must exercise a known-good model and at least one intentionally broken control.

## Current bounded model

`model-checker` exhaustively explores the bounded mutation/checkpoint state machine and proves within that bound that:

- applying the same `(client_id, mutation_id)` twice is idempotent;
- the applied count equals the number of unique applied mutation identities;
- checkpoints never regress;
- checkpoints never advance beyond durably applied mutations.

The executable also runs two negative controls: duplicate delivery incorrectly increments the applied count, and checkpoint advancement is allowed beyond applied state. Both must be rejected or the checker itself fails.

Run it with:

```sh
cargo run --locked --manifest-path conformance/model-checker/Cargo.toml
```

This model does **not** by itself prove distributed causal consistency, CRDT/OT semantics, production storage durability, or cross-runtime convergence. Those claims require their own state machines and executable evidence.

## Evidence rules

Only an exact-head, actually executed, stepful green run is conformance evidence. Queued, skipped, zero-step, stale-head, billing/admission-blocked, missing-run, and historical-only results are not green evidence.
