# Opto Sync interfaces instructions

- Keep this repository declaration-only. Merge algorithms, validators,
  transports, persistence, schedulers, and telemetry exporters belong in their
  implementation repositories.
- Preserve `provenance.json` and the byte-identical canonical schema. Contract
  changes must record an immutable source revision and classify compatibility.
- Keep Rust, TypeScript, Dart, Kotlin, Swift, Java, and C-facing declarations
  synchronized before changing a public wire name or enum value.
- Never include credentials, record payload fixtures from real users, database
  URLs, or production topology.
- Run `python3 scripts/verify_contracts.py`, Rust formatting/tests/Clippy, and
  every locally available language syntax check before pushing.
