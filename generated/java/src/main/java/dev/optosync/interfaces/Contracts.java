package dev.optosync.interfaces;

import java.util.List;
import java.util.Map;

public final class Contracts {
  private Contracts() {}

  public enum Operation { UPSERT, DELETE }
  public enum LocalProtocolStatus { PENDING, CONFIRMED }
  public enum ResultStatus { APPLIED, DUPLICATE, REJECTED }
  public enum ConnectivityState { UNKNOWN, OFFLINE, LINK, INTERNET }
  public enum ConnectivityMode { AUTOMATIC, OFFLINE }
  public enum ConnectivitySource { INITIAL, MANUAL, BROWSER_EVENT, PROBE, FORCED_OFFLINE }

  public sealed interface JsonValue
      permits JsonNull, JsonBoolean, JsonInteger, JsonNumber, JsonString, JsonArray, JsonObject {}

  public record JsonNull() implements JsonValue {}
  public record JsonBoolean(boolean value) implements JsonValue {}
  public record JsonInteger(long value) implements JsonValue {}
  public record JsonNumber(double value) implements JsonValue {}
  public record JsonString(String value) implements JsonValue {}
  public record JsonArray(List<JsonValue> value) implements JsonValue {}
  public record JsonObject(Map<String, JsonValue> value) implements JsonValue {}

  public record IngestRecord(
      String table,
      String recordId,
      Operation operation,
      String baseRevision,
      Map<String, JsonValue> payload) {}

  public record IngestEnvelope(
      int formatVersion,
      String source,
      List<IngestRecord> records) {}

  public record ProtocolMutation(
      String mutationId,
      Operation operation,
      String table,
      String recordId,
      JsonValue payload,
      String baseRevision,
      boolean resurrect,
      LocalProtocolStatus status) {}

  public record PushRequest(
      int protocolVersion,
      String clientId,
      List<ProtocolMutation> mutations) {}

  public record ConnectivitySnapshot(
      ConnectivityState state,
      ConnectivityMode mode,
      ConnectivitySource source,
      long changedAt,
      Long verifiedAt) {}
}
