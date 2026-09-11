enum OptoSyncOperation { upsert, delete }

enum OptoSyncLocalProtocolStatus { pending, confirmed }

enum OptoSyncResultStatus { applied, duplicate, rejected }

enum OptoSyncConnectivityState { unknown, offline, link, internet }

enum OptoSyncConnectivityMode { automatic, offline }

enum OptoSyncConnectivitySource {
  initial,
  manual,
  browserEvent,
  probe,
  forcedOffline,
}

final class OptoSyncIngestRecord {
  const OptoSyncIngestRecord({
    required this.table,
    required this.recordId,
    required this.payload,
    this.operation,
    this.baseRevision,
  });

  final String table;
  final String recordId;
  final OptoSyncOperation? operation;
  final String? baseRevision;
  final Map<String, Object?> payload;
}

final class OptoSyncIngestEnvelope {
  const OptoSyncIngestEnvelope({
    required this.formatVersion,
    required this.records,
    this.source,
  });

  final int formatVersion;
  final String? source;
  final List<OptoSyncIngestRecord> records;
}

final class OptoSyncProtocolMutation {
  const OptoSyncProtocolMutation({
    required this.mutationId,
    required this.operation,
    required this.table,
    required this.recordId,
    required this.resurrect,
    required this.status,
    this.payload,
    this.baseRevision,
  });

  final String mutationId;
  final OptoSyncOperation operation;
  final String table;
  final String recordId;
  final Object? payload;
  final String? baseRevision;
  final bool resurrect;
  final OptoSyncLocalProtocolStatus status;
}

final class OptoSyncPushRequest {
  const OptoSyncPushRequest({
    required this.protocolVersion,
    required this.clientId,
    required this.mutations,
  });

  final int protocolVersion;
  final String clientId;
  final List<OptoSyncProtocolMutation> mutations;
}

final class OptoSyncConnectivitySnapshot {
  const OptoSyncConnectivitySnapshot({
    required this.state,
    required this.mode,
    required this.source,
    required this.changedAt,
    this.verifiedAt,
  });

  final OptoSyncConnectivityState state;
  final OptoSyncConnectivityMode mode;
  final OptoSyncConnectivitySource source;
  final int changedAt;
  final int? verifiedAt;
}
