use opto_sync_interfaces::{
    ConnectivityMode, ConnectivitySnapshot, ConnectivitySource, ConnectivityState, IngestEnvelope,
    LocalProtocolStatus, Operation, ProtocolMutation, PushRequest,
};

#[test]
fn canonical_ingest_fixtures_have_opposite_outcomes() {
    let valid = include_str!("../fixtures/valid/basic-upsert.json");
    let invalid = include_str!("../fixtures/invalid/unknown-record-key.json");
    let envelope: IngestEnvelope = serde_json::from_str(valid).expect("valid canonical fixture");
    assert_eq!(envelope.format_version, 1);
    assert_eq!(envelope.records[0].table, "todos");
    assert!(serde_json::from_str::<IngestEnvelope>(invalid).is_err());
}

#[test]
fn protocol_and_connectivity_wire_names_are_stable() {
    let request = PushRequest {
        protocol_version: 1,
        client_id: "device-a".into(),
        mutations: vec![ProtocolMutation {
            mutation_id: "1".into(),
            operation: Operation::Upsert,
            table: "todos".into(),
            record_id: "todo-1".into(),
            payload: Some(serde_json::json!({"updatedAt": "1"})),
            base_revision: None,
            resurrect: false,
            status: LocalProtocolStatus::Pending,
        }],
    };
    let request = serde_json::to_value(request).expect("serialize protocol request");
    assert_eq!(request["protocolVersion"], 1);
    assert_eq!(request["mutations"][0]["recordId"], "todo-1");

    let snapshot = ConnectivitySnapshot {
        state: ConnectivityState::Internet,
        mode: ConnectivityMode::Automatic,
        source: ConnectivitySource::Probe,
        changed_at: 1,
        verified_at: Some(1),
    };
    let snapshot = serde_json::to_value(snapshot).expect("serialize connectivity snapshot");
    assert_eq!(snapshot["state"], "internet");
    assert_eq!(snapshot["changedAt"], 1);
}

#[test]
fn interface_source_contains_no_runtime_authority() {
    let source = include_str!("../src/lib.rs");
    for forbidden in [
        "Database::connect",
        "TcpStream",
        "reqwest",
        "async_nats",
        "merge_json",
        "tokio::spawn",
    ] {
        assert!(
            !source.contains(forbidden),
            "found runtime body: {forbidden}"
        );
    }
}
