package dev.optosync.interfaces

enum class Operation { UPSERT, DELETE }

enum class LocalProtocolStatus { PENDING, CONFIRMED }

enum class ResultStatus { APPLIED, DUPLICATE, REJECTED }

enum class ConnectivityState { UNKNOWN, OFFLINE, LINK, INTERNET }

enum class ConnectivityMode { AUTOMATIC, OFFLINE }

enum class ConnectivitySource { INITIAL, MANUAL, BROWSER_EVENT, PROBE, FORCED_OFFLINE }

data class IngestRecord(
    val table: String,
    val recordId: String,
    val operation: Operation?,
    val baseRevision: String?,
    val payload: Map<String, Any?>,
)

data class IngestEnvelope(
    val formatVersion: Int,
    val source: String?,
    val records: List<IngestRecord>,
)

data class ProtocolMutation(
    val mutationId: String,
    val operation: Operation,
    val table: String,
    val recordId: String,
    val payload: Any?,
    val baseRevision: String?,
    val resurrect: Boolean,
    val status: LocalProtocolStatus,
)

data class PushRequest(
    val protocolVersion: Int,
    val clientId: String,
    val mutations: List<ProtocolMutation>,
)

data class ConnectivitySnapshot(
    val state: ConnectivityState,
    val mode: ConnectivityMode,
    val source: ConnectivitySource,
    val changedAt: Long,
    val verifiedAt: Long?,
)
