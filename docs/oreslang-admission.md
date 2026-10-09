# Oreslang interface contract admission (candidate, not yet supported)

**Status:** draft / blocked until a compiled Oreslang adapter produces evidence.
**Peer-authority roots:** `contracts/`, `schema/`, `schemas/`; preserve `provenance.json` and the governance policy in `governance/`.

## Admission requirements

1. Use the repo's **independently authored** TypeSpec + JSON Schema Draft 2020-12 sources without rewriting either from generated output. Record the **exact commit SHA**, source closure, and immutable artifact SHA-256 identifiers.
2. Run ORESoftware/typespec-json-schema-validator (TJSV) for current-input parity and canonical Contract IR before evaluating any Oreslang implementation.
3. Generate Oreslang declarations only from an admitted contract projection; reconcile operation names, optional vs required, missing vs explicit null, unions/enums, integer bounds, serialization, protocol errors, and array/tuple semantics against existing Rust/TS/Dart peers.
4. Execute the *same positive and negative* fixtures through the actual Oreslang JVM/GraalVM implementation, recording input digests, compiler/toolchain identity, ingress and egress statuses, and TJSV runtime evidence. Merely parsing this proposal or generating an interface is not a pass.
5. Treat `javascript-browser` and `wasm-browser` as separate **future** targets: no DOM, fetch, storage, or host interop capabilities may be assumed merely because JS/WASM transpilation is planned.
6. Update `governance/`, language/runtime participant matrices and CI atomically **only after** a real Oreslang adapter and executable conformance workflow exist. Until then, admission is fail-closed; do not claim a supported or publishable Oreslang SDK.

## Required tests before moving out of draft

- Exact-source TJSV parity / Contract IR current-input verification.
- Oreslang parser and typecheck for actual declarations, plus positive and negative JSON payload fixtures.
- Cross-language canonical JSON/wire roundtrips; reject extra fields where forbidden, missing required fields, invalid enum tags and out-of-range integers.
- Determinism: repeated generation byte-identical; source or fixture tampering makes admission fail.
- Deny unsupported capabilities in browser-targeted code; compile and smoke-test JS/WASM after backends exist.

This document is an integration gate, **not** a third schema authority or a substitute for a runnable Oreslang adapter.
