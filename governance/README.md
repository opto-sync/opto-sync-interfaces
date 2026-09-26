# Governance

This directory defines how Opto Sync interface changes are admitted and promoted. It does not create a third contract authority.

## Promotion policy

A contract change is promotable only when all required checks run against the exact candidate head and actually execute green. The following are distinct non-green states: queued, skipped, zero-step, stale-head, billing-blocked, admission-blocked, missing-run, and historical-only evidence.

Contract changes must also:

- classify compatibility before merge;
- keep TypeSpec and Draft 2020-12 JSON Schema as independent authored peers;
- fail closed on peer-authority disagreement through TJSV;
- preserve wire names or provide an explicit compatibility/migration contract;
- include acceptance and rejection fixtures for new constraints;
- avoid real-user payloads, credentials, or production topology in fixtures;
- identify which generated artifacts are evidence only.

## Strong-claim rule

Do not upgrade evidence into claims the implementation has not earned. In particular, a bounded idempotency/checkpoint model is not proof of CRDT/OT semantics, total order, causal consistency, cross-runtime convergence, durable storage recovery, or production availability.

## Mixed-version changes

When old and new peers can interoperate, state the supported adjacent-version matrix. When they cannot, require an explicit protocol/schema refusal; silent field loss or truncation is a governance failure.
