# `.opto-sync.toml`

`.opto-sync.toml` is the Opto Sync domain runtime configuration contract. It is not an argv parser and does not replace `.cli-flags.toml`.

Executable option handling stays owned by `flags-2-env`: audit the repository-root `.cli-flags.toml`, parse argv through the official binding, and feed only normalized argv-derived environment overrides into Opto Sync config resolution. Runtime precedence is non-secret argv override, then ambient environment/approved secret delivery, then a non-secret `.opto-sync.toml` default.

Secret bindings are environment/secret-store only. A `secret = true` binding may not have a plaintext default and must never be accepted from argv. Missing required bindings, duplicate names/keys, invalid coercion, unresolved client/server references, disabled strict mode/audit, unsafe flags contract paths, and role/mode inconsistencies fail startup without echoing rejected values.

## Independent authorities

The canonical parsed object has two independent human-authored authorities:

- `contracts/opto-sync-config/opto-sync-config.tsp`
- `contracts/opto-sync-config/opto-sync-config.schema.json` (Draft 2020-12)

`ORESoftware/typespec-json-schema-validator` admits the pair fail-closed. Generated JSON Schema and receipts are evidence, never an authored third authority.

## Same repository client/server support

`mode` is `client`, `server`, or `hybrid`. Hybrid repositories keep one env-binding inventory and explicit client/server projections. Client code must not require server-only database/NATS/Redis credentials, and generated client artifacts must not contain secret values.

Client projections may bind local-store and background-sync controls without turning those settings into a second argv schema. Server projections may bind optional Redis reconciliation settings (`redis_url`, bounded polling cadence, and a pubsub invalidation channel) while keeping credential-bearing Redis URLs environment/secret-store only.

## Sync policy

The v1 `[sync]` section is deliberately portable across Rust, Dart/Flutter, TypeScript, and native clients. It declares bounded push/pull intervals, maximum batch size, and a conflict policy (`server-wins`, `client-wins`, `last-write-wins`, or `manual`). Runtime implementations may add operational safeguards but may not silently reinterpret these fields. Operational Redis reconciliation settings are separate from this portable synchronization policy and must not redefine its batch or conflict semantics.

The checked-in example contains only environment-variable names and safe localhost/non-secret defaults; database, auth, Redis, and credential-bearing transport values remain outside Git.
