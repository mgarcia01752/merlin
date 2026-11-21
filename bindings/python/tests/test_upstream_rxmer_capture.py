# Copyright 2026 MORGANA contributors
# SPDX-License-Identifier: Apache-2.0
# Adapted for MERLIN from 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
# Changes: namespaces, fixture owners and shared typed error categories.
"""Actual upstream RxMER native consumers; Rust owns decoding and dB conversion."""
from __future__ import annotations

import hashlib
import json
import os
from pathlib import Path
import unittest

import merlin.upstream as upstream

ROOT = Path(__file__).resolve().parents[3]


class RxmerCaptureTests(unittest.TestCase):
    def test_native_actual_rust_process_parity_metadata_readings_and_lifetimes(self) -> None:
        """Both authored files preserve originals and match a separate Rust process."""
        expected = json.loads(Path(os.environ["MERLIN_UPSTREAM_RUST_RXMER_JSON"]).read_text())
        for index, name in enumerate(("ccap-50khz", "ccap-25khz")):
            source = bytes.fromhex((ROOT / f"examples/measurements/upstream/rxmer/{name}.hex").read_text())
            capture = upstream.RxmerCapture.from_pnm(source)
            self.assertEqual(capture.original_bytes(), source)
            self.assertEqual(capture.original_sha256(), hashlib.sha256(source).hexdigest())
            observation = json.loads(capture.to_json())
            self.assertEqual(observation, expected[index])
            self.assertEqual(capture.raw_readings(), source[297:])
            self.assertEqual(capture.readings_db(), expected[index]["readings_db"])
            self.assertEqual(capture.ccap_id_text(), expected[index]["ccap_id_text"])
            self.assertEqual(capture.pre_eq_enabled(), index == 0)
            original, readings, text = capture.original_bytes(), capture.readings_db(), capture.to_json()
            del capture
            self.assertEqual(original, source)
            self.assertEqual(readings, expected[index]["readings_db"])
            self.assertEqual(json.loads(text), expected[index])
        self.assertEqual(expected[0]["readings_db"], [0, 23.75, 31.75, 32, 40, 63.25, 63.5, None])

    def test_native_unknown_metadata_and_structured_rejections(self) -> None:
        """Raw ambiguous fields survive; malformed data has shared exact offsets."""
        source = bytes.fromhex((ROOT / "examples/measurements/upstream/rxmer/ccap-50khz.hex").read_text())
        candidate = bytearray(source)
        candidate[5] = 255
        candidate[285] = 2
        candidate[14] = 255
        candidate[269] = 165
        capture = upstream.RxmerCapture.from_pnm(bytes(candidate))
        self.assertIsNone(capture.ccap_id_text())
        self.assertIsNone(capture.pre_eq_enabled())
        self.assertEqual(capture.original_bytes(), bytes(candidate))
        self.assertEqual(json.loads(capture.to_json())["header"]["ccap_id_raw"][-1], 165)
        for offset, byte, code, field, position in (
            (0, 0, "UnsupportedCapability", "rxmer.file_type", 0),
            (4, 1, "UnsupportedCapability", "rxmer.major_version", 4),
            (292, 100, "UnsupportedCapability", "rxmer.subcarrier_spacing", 292),
            (296, 7, "InvalidInput", "rxmer.data_length", 293),
            (290, 255, "InvalidInput", "rxmer.subcarrier_grid", 290),
        ):
            damaged = bytearray(source)
            damaged[offset] = byte
            with self.assertRaises(upstream.ApiError) as rejected:
                upstream.RxmerCapture.from_pnm(bytes(damaged))
            self.assertEqual((rejected.exception.code, rejected.exception.field, rejected.exception.offset), (getattr(upstream.ErrorCode, code), field, position))
        with self.assertRaises(upstream.ApiError) as oversized:
            upstream.RxmerCapture.from_pnm(bytes(4394))
        self.assertEqual((oversized.exception.code, oversized.exception.field, oversized.exception.offset), (upstream.ErrorCode.ResourceLimit, "rxmer.bytes", 0))
