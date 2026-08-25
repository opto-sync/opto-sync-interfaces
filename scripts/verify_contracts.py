#!/usr/bin/env python3
from __future__ import annotations

import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
PROVENANCE = json.loads((ROOT / "provenance.json").read_text(encoding="utf-8"))
SCHEMA = ROOT / "schemas/opto-sync-envelope.schema.json"


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


expected_schema_digest = PROVENANCE["orderedSources"][0]["sha256"]
if sha256(SCHEMA) != expected_schema_digest:
    raise SystemExit("canonical envelope schema no longer matches its recorded source digest")

schema = json.loads(SCHEMA.read_text(encoding="utf-8"))
if schema.get("$id") != "https://opto-sync.dev/schema/opto-sync-envelope.schema.json":
    raise SystemExit("canonical schema identifier changed")
if schema.get("additionalProperties") is not False:
    raise SystemExit("canonical envelope must reject unknown top-level fields")

bindings = {
    "rust": ROOT / "src/lib.rs",
    "typescript": ROOT / "generated/typescript/index.ts",
    "dart": ROOT / "generated/dart/lib/opto_sync_interfaces.dart",
    "kotlin": ROOT / "generated/kotlin/src/main/kotlin/dev/optosync/interfaces/Contracts.kt",
    "swift": ROOT / "generated/swift/Sources/OptoSyncInterfaces/Contracts.swift",
    "java": ROOT / "generated/java/src/main/java/dev/optosync/interfaces/Contracts.java",
    "c": ROOT / "generated/c/include/opto_sync_interfaces.h",
}
for language, path in bindings.items():
    if not path.is_file() or path.stat().st_size == 0:
        raise SystemExit(f"{language} interface representation is missing")

for forbidden in (
    "Database::connect",
    "TcpStream",
    "reqwest",
    "async_nats",
    "merge_json",
    "tokio::spawn",
):
    if forbidden in (ROOT / "src/lib.rs").read_text(encoding="utf-8"):
        raise SystemExit(f"Rust interface source contains runtime implementation: {forbidden}")

print(
    f"verified canonical schema digest and {len(bindings)} type-only language representations"
)
