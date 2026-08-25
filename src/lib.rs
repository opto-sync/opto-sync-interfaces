#![forbid(unsafe_code)]
//! Transport-neutral Opto Sync declarations.
//!
//! This crate intentionally contains types only. Merge algorithms, validation,
//! persistence, transport, retry scheduling, and telemetry export belong to
//! `opto-sync-lib`, `syncer.rs`, or an application adapter.

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Operation {
    Upsert,
    Delete,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct IngestRecord {
    pub table: String,
    pub record_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_revision: Option<String>,
    pub payload: Map<String, Value>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct IngestEnvelope {
    pub format_version: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    pub records: Vec<IngestRecord>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LocalProtocolStatus {
    Pending,
    Confirmed,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProtocolMutation {
    pub mutation_id: String,
    pub operation: Operation,
    pub table: String,
    pub record_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_revision: Option<String>,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub resurrect: bool,
    pub status: LocalProtocolStatus,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PushRequest {
    pub protocol_version: u8,
    pub client_id: String,
    pub mutations: Vec<ProtocolMutation>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ResultStatus {
    Applied,
    Duplicate,
    Rejected,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MutationResult {
    pub mutation_id: String,
    pub status: ResultStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_status: Option<ResultStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkpoint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PushResponse {
    pub protocol_version: u8,
    pub client_id: String,
    pub last_mutation_id: String,
    pub checkpoint: String,
    pub results: Vec<MutationResult>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeSource {
    pub client_id: String,
    pub mutation_id: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Change {
    pub checkpoint: String,
    pub table: String,
    pub record_id: String,
    pub operation: Operation,
    pub record: Option<Value>,
    pub revision: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<ChangeSource>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PullResponse {
    pub protocol_version: u8,
    pub checkpoint: String,
    pub has_more: bool,
    pub changes: Vec<Change>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SnapshotRecord {
    pub table: String,
    pub record_id: String,
    pub record: Value,
    pub revision: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SnapshotResponse {
    pub protocol_version: u8,
    pub checkpoint: String,
    pub records: Vec<SnapshotRecord>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueueSnapshot {
    pub client_id: String,
    pub next_mutation_id: String,
    pub checkpoint: String,
    pub mutations: Vec<ProtocolMutation>,
    pub max_pending_mutations: u32,
    pub max_queued_payload_bytes: u32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ConnectivityState {
    Unknown,
    Offline,
    Link,
    Internet,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ConnectivityMode {
    Automatic,
    Offline,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ConnectivitySource {
    Initial,
    Manual,
    BrowserEvent,
    Probe,
    ForcedOffline,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectivitySnapshot {
    pub state: ConnectivityState,
    pub mode: ConnectivityMode,
    pub source: ConnectivitySource,
    pub changed_at: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified_at: Option<u64>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RetryState {
    pub consecutive_failures: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_retry_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConflictDisposition {
    LocalWon,
    RemoteWon,
    Merged,
    Rejected,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConflictOutcome {
    pub table: String,
    pub record_id: String,
    pub local_revision: String,
    pub remote_revision: String,
    pub disposition: ConflictDisposition,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TelemetryRuntime {
    TypeScript,
    Dart,
    Rust,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TelemetryKind {
    StateChanged,
    CycleCompleted,
    CycleFailed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TelemetryStatus {
    Stopped,
    Idle,
    Syncing,
    Offline,
    Backoff,
    Error,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TelemetryEvent {
    pub schema: String,
    pub runtime: TelemetryRuntime,
    pub kind: TelemetryKind,
    pub status: TelemetryStatus,
    pub consecutive_failures: u32,
    pub timestamp: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_retry_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub span_id: Option<String>,
}
