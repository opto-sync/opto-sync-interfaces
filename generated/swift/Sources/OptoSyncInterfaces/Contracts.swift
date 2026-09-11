public enum JSONValue: Sendable, Equatable {
    case null
    case boolean(Bool)
    case integer(Int64)
    case number(Double)
    case string(String)
    indirect case array([JSONValue])
    indirect case object([String: JSONValue])
}

public enum Operation: String, Sendable { case upsert, delete }
public enum LocalProtocolStatus: String, Sendable { case pending, confirmed }
public enum ResultStatus: String, Sendable { case applied, duplicate, rejected }
public enum ConnectivityState: String, Sendable { case unknown, offline, link, internet }
public enum ConnectivityMode: String, Sendable { case automatic, offline }
public enum ConnectivitySource: String, Sendable {
    case initial, manual, probe
    case browserEvent = "browser-event"
    case forcedOffline = "forced-offline"
}

public struct IngestRecord: Sendable, Equatable {
    public let table: String
    public let recordId: String
    public let operation: Operation?
    public let baseRevision: String?
    public let payload: [String: JSONValue]
}

public struct IngestEnvelope: Sendable, Equatable {
    public let formatVersion: UInt8
    public let source: String?
    public let records: [IngestRecord]
}

public struct ProtocolMutation: Sendable, Equatable {
    public let mutationId: String
    public let operation: Operation
    public let table: String
    public let recordId: String
    public let payload: JSONValue?
    public let baseRevision: String?
    public let resurrect: Bool
    public let status: LocalProtocolStatus
}

public struct PushRequest: Sendable, Equatable {
    public let protocolVersion: UInt8
    public let clientId: String
    public let mutations: [ProtocolMutation]
}

public struct ConnectivitySnapshot: Sendable, Equatable {
    public let state: ConnectivityState
    public let mode: ConnectivityMode
    public let source: ConnectivitySource
    public let changedAt: UInt64
    public let verifiedAt: UInt64?
}
