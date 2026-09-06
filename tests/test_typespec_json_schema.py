from __future__ import annotations

import importlib.util
import json
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SPEC = importlib.util.spec_from_file_location(
    "typespec_json_schema_check", ROOT / "scripts/check-typespec-json-schema.py"
)
assert SPEC and SPEC.loader
CHECK = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(CHECK)


class TypeSpecJsonSchemaParity(unittest.TestCase):
    def test_committed_authorities_have_equal_semantics(self) -> None:
        schema = json.loads(
            (ROOT / "validation/public-contracts.v1.json").read_text(encoding="utf-8")
        )
        typespec = (ROOT / "validation/typespec/validation.tsp").read_text(encoding="utf-8")
        CHECK.compare(CHECK.parse_schema(schema), CHECK.parse_typespec(typespec))

    def test_bound_change_is_a_release_blocker(self) -> None:
        schema = json.loads(
            (ROOT / "validation/public-contracts.v1.json").read_text(encoding="utf-8")
        )
        typespec = (ROOT / "validation/typespec/validation.tsp").read_text(encoding="utf-8")
        schema["$defs"]["PageQuery"]["properties"]["limit"]["maximum"] = 99
        with self.assertRaises(SystemExit):
            CHECK.compare(CHECK.parse_schema(schema), CHECK.parse_typespec(typespec))

    def test_private_model_cannot_be_introduced_as_a_public_authority(self) -> None:
        schema = json.loads(
            (ROOT / "validation/public-contracts.v1.json").read_text(encoding="utf-8")
        )
        typespec = (ROOT / "validation/typespec/validation.tsp").read_text(encoding="utf-8")
        typespec += "\nmodel TrustedActor { id: string; }\n"
        with self.assertRaises(SystemExit):
            CHECK.compare(CHECK.parse_schema(schema), CHECK.parse_typespec(typespec))


if __name__ == "__main__":
    unittest.main()
