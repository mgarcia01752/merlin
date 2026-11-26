#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Installed edited-project provenance and independent Rust execution parity."""
import copy
import gc
import hashlib
import importlib
import json
import os
from pathlib import Path
import sys
import unittest

import merlin
from merlin import common, downstream, upstream

ROOT = Path(__file__).resolve().parents[3]
CONFIGURATION_50 = ROOT / "examples/cli/upstream/us-50khz.json"
MEASURED_50 = ROOT / "examples/measurements/upstream/measured/capture-driven.json"
RUST_REFERENCE_ENV = "MERLIN_PROJECT_EDITS_RUST_REFERENCE_JSON"

# Independent wire anchor: schema 3 introduces separate direction projects.
PROJECT_SCHEMA_VERSION = 3
# The two fixed Rust fixtures exercise the existing 25/50 kHz configurations.
REFERENCE_CASE_COUNT = 2
REFERENCE_SPACINGS_KHZ = {25, 50}
# All integers through this boundary are exact; the fixture exceeds that range.
MAXIMUM_CONTIGUOUS_BINARY64_INTEGER = 1 << 53
RUNTIME_OPTION_FIELDS = (
    "grants", "maximum_samples", "chunk_samples", "timeout_ms",
    "retained_points", "retained_taps",
)
REPORT_FIELDS = (
    "format", "schema_version", "noise_generator", "configuration", "options",
    "planned", "status", "generated_source_bits", "compared_source_bits",
    "unverified_source_bits", "bit_errors", "ber", "reference_recovered",
    "failure", "not_started_grants", "source_configuration_json",
    "source_configuration_original", "source_configuration_sha256",
    "measurement_provenance_json",
)
SOURCE_COUNTER_FIELDS = (
    "generated_source_bits", "compared_source_bits", "unverified_source_bits",
    "bit_errors",
)
IDENTITY_FIELDS = ("run", "generation", "grant")


def _assert_installed_native(case):
    """Require installed native consumers and preserve existing import identities.

    Args:
        case: Active unittest assertions; no source package is inserted into sys.path.
    """
    prefix = Path(sys.prefix).resolve()
    native = Path(merlin._native.__file__).resolve()
    package = Path(merlin.__file__).resolve()
    case.assertTrue(native.is_relative_to(prefix), str(native))
    case.assertTrue(package.is_relative_to(prefix), str(package))
    case.assertTrue(str(native).endswith((".so", ".pyd")), str(native))
    case.assertFalse(package.is_relative_to(ROOT / "bindings/python/python"))
    case.assertIs(common.Project, merlin._native.Project)
    case.assertEqual(common.Project.__module__, "merlin.common")
    case.assertIs(common.ErrorCode, merlin.ErrorCode)
    case.assertIs(upstream.ErrorCode, common.ErrorCode)
    case.assertIs(merlin.Simulation, downstream.Simulation)
    case.assertIs(merlin.Merlin, downstream.Simulation)
    legacy = importlib.import_module("merlin.merlin")
    case.assertIs(legacy.Simulation, downstream.Simulation)
    case.assertIsNot(upstream.Simulation, downstream.Simulation)


def _assert_integer_identities(case, value):
    """Check native identifier types without converting or comparing process IDs.

    Args:
        case: Active unittest assertions.
        value: One finite Rust-produced JSON value; strings remain opaque.
    """
    if isinstance(value, dict):
        if all(field in value for field in IDENTITY_FIELDS):
            for field in IDENTITY_FIELDS:
                case.assertIs(type(value[field]), int, field)
        for field in ("grant_id", "epoch_sample", "first_frame"):
            if field in value:
                case.assertIs(type(value[field]), int, field)
        for child in value.values():
            _assert_integer_identities(case, child)
    elif isinstance(value, list):
        for child in value:
            _assert_integer_identities(case, child)


class ProjectConfigurationEdits(unittest.TestCase):
    def test_edited_projects_preserve_values_originals_and_independent_rust_parity(self):
        """Keep current values and exact prior bytes through owned native reload/run."""
        _assert_installed_native(self)
        reference = json.loads(
            Path(os.environ[RUST_REFERENCE_ENV]).read_text(encoding="utf-8")
        )
        cases = reference["cases"]
        self.assertEqual(len(cases), REFERENCE_CASE_COUNT)
        self.assertEqual(len({item["name"] for item in cases}), REFERENCE_CASE_COUNT)
        spacings = set()
        for item in cases:
            with self.subTest(case=item["name"]):
                self.assertIsInstance(item["name"], str)
                self.assertIsInstance(item["project"], str)
                self.assertIsInstance(item["report"], dict)
                self.assertIs(type(item["original_input_bytes"]), int)
                wire = json.loads(item["project"])
                self.assertEqual(wire["schema_version"], PROJECT_SCHEMA_VERSION)
                self.assertEqual(wire["application"], "MERLIN")
                self.assertEqual(wire["direction"], "upstream")
                source = wire["contents"]["input"]
                self.assertEqual(source["kind"], "configuration")
                current_text = source["document"]
                original_text = source["original_document"]
                self.assertIsInstance(current_text, str)
                self.assertIsInstance(original_text, str)
                current = json.loads(current_text)
                self.assertNotEqual(current, json.loads(original_text))
                original = original_text.encode("utf-8")
                original_sha256 = hashlib.sha256(original).hexdigest()
                self.assertEqual(item["original_input_bytes"], len(original))
                selected = next(
                    preset for preset in current["presets"]
                    if preset["id"] == current["selected_preset"]
                )
                spacings.add(selected["channel"]["subcarrier_spacing_khz"])
                grant_id = current["simulation"]["grant_id"]
                self.assertIs(type(grant_id), int)
                self.assertGreater(grant_id, MAXIMUM_CONTIGUOUS_BINARY64_INTEGER)

                project = common.Project.from_bytes(item["project"].encode("utf-8"))
                self.assertEqual(project.direction, "upstream")
                self.assertEqual(project.original_document, item["project"].encode("utf-8"))
                exported = project.to_json()
                restored_wire = json.loads(exported)
                self.assertEqual(restored_wire["contents"]["input"]["document"], current_text)
                self.assertEqual(
                    restored_wire["contents"]["input"]["original_document"],
                    original_text,
                )
                self.assertEqual(restored_wire["contents"]["options"], wire["contents"]["options"])
                self.assertEqual(
                    common.Project.collection_info([project])["original_input_bytes"],
                    item["original_input_bytes"],
                )
                restored = common.Project.from_bytes(exported.encode("utf-8"))
                session = restored.upstream_simulation()
                self.assertIsInstance(session, upstream.Simulation)
                snapshot = session.document()
                self.assertIsInstance(snapshot, upstream.ConfigurationDocument)
                current_export = snapshot.to_json()
                self.assertEqual(json.loads(current_export), current)
                self.assertEqual(current_export, item["report"]["source_configuration_json"])
                self.assertEqual(snapshot.original_bytes(), original)
                self.assertEqual(snapshot.original_sha256(), original_sha256)
                self.assertEqual(
                    snapshot.inventory(),
                    [preset["id"] for preset in current["presets"]],
                )
                del restored, project
                gc.collect()
                self.assertEqual(snapshot.to_json(), current_export)
                self.assertEqual(snapshot.original_bytes(), original)
                self.assertEqual(snapshot.original_sha256(), original_sha256)

                result = session.run()
                del session
                gc.collect()
                report = json.loads(result.to_json())
                for field in REPORT_FIELDS:
                    self.assertEqual(report[field], item["report"][field], field)
                self.assertEqual(
                    [grant["words"] for grant in report["grants"]],
                    [grant["words"] for grant in item["report"]["grants"]],
                )
                self.assertEqual(
                    [grant["identity"]["grant"] for grant in report["grants"]],
                    [grant["identity"]["grant"] for grant in item["report"]["grants"]],
                )
                for field in SOURCE_COUNTER_FIELDS:
                    self.assertIs(type(report[field]), int, field)
                self.assertTrue(report["reference_recovered"])
                self.assertEqual(bytes(report["source_configuration_original"]), original)
                self.assertEqual(report["source_configuration_sha256"], original_sha256)
                resolved_grant = report["configuration"]["us_configuration"]["grant"]
                self.assertEqual(resolved_grant["id"], grant_id)
                self.assertIs(type(resolved_grant["id"]), int)
                for not_started in report["not_started_grants"]:
                    self.assertIs(type(not_started), int)
                _assert_integer_identities(self, current)
                _assert_integer_identities(self, report)
                self.assertEqual(snapshot.to_json(), current_export)
                self.assertEqual(snapshot.original_bytes(), original)
                self.assertEqual(snapshot.original_sha256(), original_sha256)
        self.assertEqual(spacings, REFERENCE_SPACINGS_KHZ)

    def test_old_shape_and_invalid_original_are_compatible_and_structured(self):
        """Preserve old omission and accepted ownership after failed new admissions."""
        _assert_installed_native(self)
        original = CONFIGURATION_50.read_bytes()
        document = original.decode("utf-8")
        settings = json.loads(document)["simulation"]
        wire = {
            "schema_version": PROJECT_SCHEMA_VERSION,
            "application": "MERLIN",
            "direction": "upstream",
            "contents": {
                "input": {"kind": "configuration", "document": document},
                "options": {field: settings[field] for field in RUNTIME_OPTION_FIELDS},
                "receiver_observations": {
                    "maximum_frames": 0,
                    "maximum_scalar_slots": 0,
                },
            },
        }
        admitted_bytes = json.dumps(wire).encode("utf-8")
        accepted = common.Project.from_bytes(admitted_bytes)
        before = accepted.to_json()
        exported = json.loads(before)
        self.assertNotIn("original_document", exported["contents"]["input"])
        self.assertEqual(exported["contents"]["input"]["document"], document)
        self.assertEqual(accepted.original_document, admitted_bytes)
        retained = accepted.upstream_simulation()
        retained_document = retained.document().to_json()
        invalid_originals = (
            ("malformed", "{"),
            ("unknown-format", '{"format":"unknown"}'),
            ("non-string", {"document": document}),
        )
        candidates = []
        for name, invalid in invalid_originals:
            candidate = copy.deepcopy(wire)
            candidate["contents"]["input"]["original_document"] = invalid
            candidates.append((name, candidate))
        measured = copy.deepcopy(wire)
        measured["contents"]["input"] = {
            "kind": "measured",
            "document": MEASURED_50.read_bytes().decode("utf-8"),
            "original_document": document,
        }
        candidates.append(("measured-configuration-only-field", measured))
        for name, candidate in candidates:
            with self.subTest(rejection=name):
                with self.assertRaises((downstream.ApiError, upstream.ApiError)) as caught:
                    common.Project.from_bytes(json.dumps(candidate).encode("utf-8"))
                self.assertEqual(caught.exception.code, common.ErrorCode.InvalidInput)
                self.assertEqual(accepted.to_json(), before)
                self.assertEqual(accepted.original_document, admitted_bytes)
                self.assertEqual(retained.document().to_json(), retained_document)
                self.assertEqual(retained.document().original_bytes(), original)
                reused = accepted.upstream_simulation()
                self.assertEqual(reused.document().to_json(), retained_document)
                self.assertEqual(reused.document().original_bytes(), original)
        reloaded = common.Project.from_bytes(before.encode("utf-8"))
        self.assertEqual(reloaded.to_json(), before)
        self.assertNotIn("original_document", json.loads(reloaded.to_json())["contents"]["input"])


if __name__ == "__main__":
    unittest.main()
