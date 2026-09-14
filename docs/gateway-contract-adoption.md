# Opto Sync gateway contract adoption

`opto-sync-gateway.rs` implements the realtime protocol boundary defined by this interfaces repository. It must not fork the protocol model from `syncer.rs` or client projections.

Maintain independently authored TypeSpec and JSON Schema Draft 2020-12 peers for protocol-v1 hello, cursor, mutation, acknowledgement, heartbeat, resync and stable error envelopes. Shared field names are snake_case. TJSV compares the authored peers without regenerating one authority from the other.

Tenant/principal/device identity is transport-authenticated scope and must not be accepted as payload authority. Cursor semantics bind sequence/token/version and retention behavior. Acknowledgement semantics must explicitly mean durable persistence; transport receipt or in-memory admission is insufficient.

Cross-runtime fixtures must cover valid hello/resume, expired cursor, duplicate hello, malformed frames, oversized identifiers, duplicate mutation identity, wrong JSON types, credential/role smuggling, heartbeat, reconnect, persistence unavailable, and retryable versus terminal errors.

TypeScript, Dart and Rust clients should consume generated read-only projections plus shared fixture vectors. A contract change that alters ACK durability, cursor meaning, identity authority, retention behavior or stable failure codes requires an explicit version/migration plan.

Future binary/WebTransport/gRPC transports may project the same semantics, but transport additions may not create a second sync/freshness authority or weaken the durable checkpoint model.