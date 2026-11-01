# Electric / Phoenix.Sync shape resume contract (v1)

The canonical schema is `schemas/electric-shape-cursor.v1.schema.json`.
This is an additive **type-only interoperability contract**, not an implementation of Electric, an Opto Sync pull checkpoint, or a server authorization policy.

A persisted cursor is `{version:1, scopeKey, shapeHandle, offset}`. `scopeKey` is an application-supplied, non-secret identity for the exact tenant and shape subscription. A different authenticated tenant, filter, source table, or selection must never restore another scope's cursor. Do not persist credentials in the cursor.

**Electric offsets and Opto Sync protocol checkpoints are separate domains.** Electric's `shapeHandle + offset` identifies a position within one shape. Opto Sync's `checkpoint` is a server commit-ordered position and cannot acknowledge a push; only a validated immutable push response may do so. Never translate either offset into the other.

The adapter must atomically apply a delivered change batch and persist its Electric cursor. On `must-refetch`, it must invalidate the old cursor and replace only the authoritative shape projection; pending local mutations and their acknowledgements remain untouched. Until an `up-to-date` barrier, no partial rebuild may be rendered as a complete authoritative snapshot. A crash after cursor invalidation must restart a fresh shape snapshot, not resume the stale position.

Electric shape change keys are opaque identities. Do not assume a delete message contains the primary-key column or parse the shape key as an Opto Sync record ID. Authorization is enforced by the application server and database, not the client adapter.

A `scopeKey` does not grant access and is not a substitute for tenant-scoped HTTP authorization. This contract intentionally does not specify how applications derive or hash the key.

## Test

```sh
python3 -m pip install jsonschema
python3 scripts/test_electric_shape_cursor.py
```

This test validates the fixture corpus against JSON Schema Draft 2020-12. It does not prove real Electric or Phoenix execution.
