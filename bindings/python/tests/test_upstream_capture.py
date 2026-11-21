# Copyright 2026 MORGANA contributors
# SPDX-License-Identifier: Apache-2.0
# Adapted for MERLIN from 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
# Changes: namespaces, fixture owners and shared typed error categories.
"""Actual native UCD consumers, frozen source vectors and Rust-process parity."""
from __future__ import annotations

import hashlib
import json
import os
from pathlib import Path
import unittest

import merlin.upstream as upstream

ROOT = Path(__file__).resolve().parents[3]


class CaptureTests(unittest.TestCase):
    def test_native_originals_offsets_inventory_and_rust_process_parity(self) -> None:
        """The actual wrapper preserves originals and matches a separate Rust run."""
        expected = json.loads(Path(os.environ["MERLIN_UPSTREAM_RUST_UCD_JSON"]).read_text())
        for index, name in enumerate(("minimal", "scheduled-50khz")):
            source = bytes.fromhex((ROOT / f"examples/measurements/upstream/ucd/{name}.hex").read_text())
            capture = upstream.UcdCapture.from_mac_management(source)
            self.assertEqual(capture.original_bytes(), source)
            self.assertEqual(capture.original_sha256(), hashlib.sha256(source).hexdigest())
            self.assertEqual(json.loads(capture.to_json()), expected[index])
            retained = capture.original_bytes()
            exported = capture.to_json()
            del capture
            self.assertEqual(retained, source)
            self.assertEqual(json.loads(exported)["header"]["upstream_channel_id"], 7)

    def test_native_structured_errors_unsupported_fragments_and_byte_cap(self) -> None:
        """Native errors preserve Rust category/field/offset without fallback."""
        source = bytes.fromhex((ROOT / "examples/measurements/upstream/ucd/minimal.hex").read_text())
        capture = upstream.UcdCapture.from_mac_management(source)
        original = capture.to_json()
        for offset, value, code, field in (
            (14, 1, "InvalidInput", "ucd.llc"),
            (17, 4, "UnsupportedCapability", "ucd.version"),
            (18, 50, "UnsupportedCapability", "ucd.message_type"),
            (19, 0x12, "InvalidInput", "ucd.fragment_sequence"),
            (19, 0x10, "UnsupportedCapability", "ucd.multipart"),
        ):
            damaged = bytearray(source)
            damaged[offset] = value
            with self.assertRaises(upstream.ApiError) as rejected:
                upstream.UcdCapture.from_mac_management(bytes(damaged))
            self.assertEqual((rejected.exception.code, rejected.exception.field, rejected.exception.offset), (getattr(upstream.ErrorCode, code), field, offset))
        with self.assertRaises(upstream.ApiError) as oversized:
            upstream.UcdCapture.from_mac_management(bytes(65550))
        self.assertEqual((oversized.exception.code, oversized.exception.field, oversized.exception.offset), (upstream.ErrorCode.ResourceLimit, "ucd.bytes", 0))
        self.assertEqual(capture.to_json(), original)
