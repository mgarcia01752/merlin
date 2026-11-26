# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
"""Actual installed generator parity; Python never performs RxMER arithmetic."""
import hashlib
import json
import os
from pathlib import Path
import unittest

import merlin
from merlin.upstream import ApiError, ErrorCode, RxmerCapture, SyntheticRxmerCapture

ROOT = Path(__file__).resolve().parents[3]
REQUESTS = ROOT / "examples/measurements/upstream/synthetic-rxmer"
BINARY_NAME = "synthetic-cmts-rxmer.bin"
SIDECAR_NAME = "synthetic-cmts-rxmer.bin.merlin.json"


class SyntheticRxmerTests(unittest.TestCase):
    def test_installed_native_matches_independent_rust_outputs_and_owns_results(self):
        """Both installed constructors match the separately executed Rust example."""
        self.assertTrue(str(merlin._native.__file__).endswith((".so", ".pyd")))
        self.assertEqual(SyntheticRxmerCapture.MAX_REQUEST_BYTES, 262144)
        expected = Path(os.environ["MERLIN_SYNTHETIC_RXMER_RUST_OUTPUT"])
        for name in ("us-50khz", "us-25khz"):
            request = (REQUESTS / (name + ".json")).read_text()
            generated = SyntheticRxmerCapture.from_json(request)
            binary, text = generated.original_bytes(), generated.to_json()
            self.assertEqual(binary, (expected / name / BINARY_NAME).read_bytes())
            self.assertEqual(json.loads(text), json.loads((expected / name / SIDECAR_NAME).read_text()))
            del generated
            observation = json.loads(text)
            self.assertEqual(observation["origin"], "synthetic")
            self.assertFalse(observation["measurement_performed"])
            self.assertEqual(observation["binary_sha256"], hashlib.sha256(binary).hexdigest())
            self.assertEqual(RxmerCapture.from_pnm(binary).readings_db(), observation["readings_db"])

    def test_literal_scaling_missing_values_and_conversion_policy(self):
        """Independent quarter-dB anchors and clamp summaries come from Rust."""
        request = json.loads((REQUESTS / "us-50khz.json").read_text())
        request["values"] = {"kind": "samples", "readings_db": [0, 23.75, 63.5, None]}
        generated = SyntheticRxmerCapture.from_json(json.dumps(request))
        self.assertEqual(RxmerCapture.from_pnm(generated.original_bytes()).raw_readings(), bytes([0, 95, 254, 255]))
        request["values"] = {"kind": "samples", "readings_db": [-1, 23.625, 64, None]}
        with self.assertRaises(ApiError) as rejected:
            SyntheticRxmerCapture.from_json(json.dumps(request))
        self.assertEqual(rejected.exception.code, ErrorCode.InvalidInput)
        self.assertEqual(rejected.exception.field, "synthetic_rxmer.readings_db")
        request["range_policy"] = "clamp"
        generated = SyntheticRxmerCapture.from_json(json.dumps(request))
        self.assertEqual(RxmerCapture.from_pnm(generated.original_bytes()).raw_readings(), bytes([0, 95, 254, 255]))
        summary = json.loads(generated.to_json())["conversion"]
        self.assertEqual((summary["clipped"], summary["rounded"], summary["missing"]), (2, 1, 1))

    def test_invalid_metadata_and_resource_errors_remain_structured(self):
        """Malformed requests fail through the shared API without fake byte offsets."""
        request = json.loads((REQUESTS / "us-50khz.json").read_text())
        for key, value, code in (("number_of_averages", 0, ErrorCode.InvalidInput),
                                 ("subcarrier_spacing_khz", 100, ErrorCode.UnsupportedCapability),
                                 ("ccap_id", "s" * 256, ErrorCode.ResourceLimit)):
            candidate = dict(request)
            candidate[key] = value
            with self.assertRaises(ApiError) as rejected:
                SyntheticRxmerCapture.from_json(json.dumps(candidate))
            self.assertEqual(rejected.exception.code, code)
            self.assertIsNone(rejected.exception.offset)
