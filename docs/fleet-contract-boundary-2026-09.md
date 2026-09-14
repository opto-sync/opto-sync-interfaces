# Fleet contract dependency boundary

Opto Sync owns synchronization, offline/online reconciliation, entity-sync, and product configuration contracts. Fleet-generic semantic primitives may be composed from `oresoftware/ores-interfaces`; Opto Sync-specific contracts remain authoritative here.

## Fleet dependency rules

- Keep Opto Sync domain contracts local to this org, including sync protocol semantics and product persistence models.
- Reuse fleet-generic IDs, envelopes, request metadata, and generic NATS/Redis semantic contracts from `oresoftware/ores-interfaces` when appropriate.
- Kubernetes, JetStream, Redis cluster wiring, service discovery, and environment topology belong in `oresoftware/k8s-libs-and-shared-defs`.
- Coordination requiring locks, leases, or fencing goes through `oresoftware/ores-locks-and-leases`.
- TypeSpec and JSON Schema Draft 2020-12 are independent authored authorities admitted by `oresoftware/typespec-json-schema-validator`.
- `oresoftware/ores-cli` validates repo/org policy and cross-repo compatibility using exact zed-pkg resolved versions; it is not a runtime dependency.
- `.zpkg.toml` expresses intent; `.zpkg.lock` is execution truth. Tip-of-default-branch checks are separate canaries.

## Anti-cycle rule

Shared primitives/tooling may flow into Opto Sync, but shared fleet repositories must not depend on Opto Sync runtime implementations.