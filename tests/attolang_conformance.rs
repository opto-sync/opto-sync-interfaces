//! Declaration-level AttoLang contract checks; not compiled guest evidence.
use serde_json::Value;

#[test]
fn attolang_candidate_has_falsifiable_negative_controls() {
    let document: Value = serde_json::from_str(include_str!(
        "../conformance/attolang-scoped-checkpoint.properties.json"
    ))
    .expect("AttoLang conformance declaration is valid JSON");
    assert_eq!(document["schema"], "opto-sync.conformance-properties.v1");
    assert_eq!(document["language"], "atto");
    assert_eq!(document["source_extension"], ".atto");
    assert_eq!(document["evidence_class"], "declaration-only");
    let properties = document["properties"].as_array().expect("property list");
    assert!(properties.len() >= 5);
    let ids: std::collections::HashSet<_> = properties
        .iter()
        .map(|entry| entry["id"].as_str().expect("property id"))
        .collect();
    assert_eq!(ids.len(), properties.len(), "duplicate property id");
    for required in [
        "exact-resume",
        "monotonic-scoped-change",
        "generation-fencing",
        "no-guest-authentication",
        "durable-ack-identity",
    ] {
        assert!(ids.contains(required), "missing property: {required}");
    }
    let controls: std::collections::HashSet<_> = document["required_negative_controls"]
        .as_array()
        .expect("negative controls")
        .iter()
        .map(|item| item.as_str().expect("control"))
        .collect();
    for required in [
        "stale-generation",
        "cross-scope-replay",
        "cursor-regression",
        "uncommitted-snapshot",
        "ack-before-local-fsync",
    ] {
        assert!(
            controls.contains(required),
            "missing negative control: {required}"
        );
    }
    let excluded = document["excluded_claims"]
        .as_array()
        .expect("excluded claims");
    assert!(excluded.iter().any(|value| value == "atto-guest-execution"));
    let manifest: Value = serde_json::from_str(include_str!("../conformance/manifest.json"))
        .expect("conformance manifest valid");
    assert!(manifest["properties"]
        .as_array()
        .expect("declared conformance sources")
        .iter()
        .any(|value| value == "conformance/attolang-scoped-checkpoint.properties.json"));
    assert_eq!(manifest["role"], "declaration-only");
}
