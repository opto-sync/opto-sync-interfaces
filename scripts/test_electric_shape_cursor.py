#!/usr/bin/env python3
"""Execute Electric cursor Draft 2020-12 fixtures (requires jsonschema)."""
import json
from pathlib import Path
from jsonschema import Draft202012Validator

root = Path(__file__).resolve().parents[1]
schema = json.loads((root / "schemas/electric-shape-cursor.v1.schema.json").read_text())
Draft202012Validator.check_schema(schema)
validate = Draft202012Validator(schema)
fixture = json.loads((root / "fixtures/electric-shape-cursor.v1.json").read_text())
for index, case in enumerate(fixture["valid"]):
    errors = list(validate.iter_errors(case))
    assert not errors, f"valid case {index} rejected: {errors}"
for index, case in enumerate(fixture["invalid"]):
    assert list(validate.iter_errors(case)), f"invalid case {index} unexpectedly accepted"
assert len(fixture["valid"]) >= 3 and len(fixture["invalid"]) >= 7
print(f"electric cursor schema: {len(fixture['valid'])} positive, {len(fixture['invalid'])} negative cases passed")
