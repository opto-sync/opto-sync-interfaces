#!/usr/bin/env python3
"""Compare the public TypeSpec and JSON Schema authorities semantically.

The two documents intentionally remain independently reviewable.  This gate
normalizes their model names, field types, optionality, closure, and bounds so
one source cannot silently drift from the other without failing CI.
"""

from __future__ import annotations

import json
import re
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SCHEMA_PATH = ROOT / "validation/public-contracts.v1.json"
TYPESPEC_PATH = ROOT / "validation/typespec/validation.tsp"
MODEL_RE = re.compile(r"model\s+(?P<name>[A-Za-z][A-Za-z0-9_]*)\s*\{(?P<body>.*?)\}", re.S)
FIELD_RE = re.compile(
    r"(?P<decorators>(?:@[A-Za-z]+\([^)]*\)\s*)*)"
    r"(?P<name>[A-Za-z][A-Za-z0-9_]*)\??\s*:\s*"
    r"(?P<type>[A-Za-z][A-Za-z0-9_]*)"
    r"(?:\s*=\s*(?P<default>[^;]+))?\s*;"
)
def _constraint(decorators: str, name: str) -> int | None:
    match = re.search(rf"@{name}\((?P<value>-?[0-9]+)\)", decorators)
    return int(match.group("value")) if match else None


def parse_typespec(source: str) -> dict[str, dict]:
    models: dict[str, dict] = {}
    for model_match in MODEL_RE.finditer(source):
        name = model_match.group("name")
        fields: dict[str, dict] = {}
        for field_match in FIELD_RE.finditer(model_match.group("body")):
            field_name = field_match.group("name")
            decorators = field_match.group("decorators")
            field_type = field_match.group("type")
            fields[field_name] = {
                "kind": "integer" if field_type.startswith("uint") else "string",
                "required": "?" not in field_match.group(0),
                "minLength": _constraint(decorators, "minLength"),
                "maxLength": _constraint(decorators, "maxLength"),
                "minimum": _constraint(decorators, "minValue"),
                "maximum": _constraint(decorators, "maxValue"),
            }
            default = field_match.group("default")
            if default is not None:
                fields[field_name]["default"] = json.loads(default.strip())
        if not fields:
            raise SystemExit(f"TypeSpec model has no parseable fields: {name}")
        models[name] = {"closed": True, "fields": fields}
    if not models:
        raise SystemExit("TypeSpec source contains no public models")
    return models


def parse_schema(document: dict) -> dict[str, dict]:
    models: dict[str, dict] = {}
    for name, definition in document.get("$defs", {}).items():
        if definition.get("type") != "object":
            continue
        fields: dict[str, dict] = {}
        required = set(definition.get("required", []))
        for field_name, field in definition.get("properties", {}).items():
            kind = field.get("type")
            if kind == "number":
                kind = "integer"
            fields[field_name] = {
                "kind": kind,
                "required": field_name in required,
                "minLength": field.get("minLength"),
                "maxLength": field.get("maxLength"),
                "minimum": field.get("minimum"),
                "maximum": field.get("maximum"),
            }
            if "default" in field:
                fields[field_name]["default"] = field["default"]
        models[name] = {
            "closed": definition.get("additionalProperties") is False,
            "fields": fields,
        }
    return models


def compare(schema_models: dict[str, dict], typespec_models: dict[str, dict]) -> None:
    if set(schema_models) != set(typespec_models):
        raise SystemExit(
            "TypeSpec/JSON Schema model mismatch: "
            f"schema={sorted(schema_models)}, typespec={sorted(typespec_models)}"
        )
    for model_name in sorted(schema_models):
        schema_model = schema_models[model_name]
        typespec_model = typespec_models[model_name]
        if schema_model["closed"] != typespec_model["closed"]:
            raise SystemExit(f"{model_name}: object closure differs")
        if set(schema_model["fields"]) != set(typespec_model["fields"]):
            raise SystemExit(f"{model_name}: field set differs")
        for field_name in sorted(schema_model["fields"]):
            left = schema_model["fields"][field_name]
            right = typespec_model["fields"][field_name]
            if left != right:
                raise SystemExit(
                    f"{model_name}.{field_name}: semantic mismatch "
                    f"schema={left}, typespec={right}"
                )


def main() -> None:
    schema = json.loads(SCHEMA_PATH.read_text(encoding="utf-8"))
    typespec = TYPESPEC_PATH.read_text(encoding="utf-8")
    compare(parse_schema(schema), parse_typespec(typespec))
    print(
        f"TypeSpec and JSON Schema agree on {len(parse_schema(schema))} public validation models"
    )


if __name__ == "__main__":
    main()
