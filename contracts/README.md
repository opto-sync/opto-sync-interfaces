# Contract authority

This directory contains authored Opto Sync interface contracts. It is the declaration boundary, not an implementation repository.

## Authority model

- Human-authored TypeSpec and JSON Schema Draft 2020-12 are independent peer authorities.
- Neither format is generated from the other and neither silently wins a disagreement.
- `ORESoftware/typespec-json-schema-validator` is the fail-closed admission/parity mechanism.
- Generated JSON Schema, OpenAPI, Protobuf, SQL, Contract IR, language bindings, and native headers are evidence/projections only.
- Public wire names are preserved exactly. Shared payload names use `snake_case`; authored HTTP extension headers use the reserved lowercase `x-ores-*` namespace.

The current config peer pair is:

- `opto-sync-config/opto-sync-config.tsp`
- `opto-sync-config/opto-sync-config.schema.json`

Valid and invalid instances live beside the authored pair so validators prove both acceptance and rejection behavior.

## Change classification

Every contract change must be classified before promotion:

1. **additive-compatible** — optional field, new operation with independent negotiation, or other change old clients can safely ignore;
2. **deprecation-compatible** — old spelling/value remains accepted for the declared compatibility window;
3. **mixed-version-conditional** — compatibility depends on explicit version negotiation or feature capability;
4. **breaking** — removal, narrowing, changed nullability, semantic reinterpretation, or wire-name change.

Incompatible versions must be refused explicitly; silent truncation/coercion is not compatibility.

## Runtime boundary

Contracts describe externally observable data and operation semantics. Merge algorithms, storage, retry schedulers, transports, migration execution, and telemetry implementations belong in implementation repositories. Application startup must not mutate server schema as a side effect of contract loading.
