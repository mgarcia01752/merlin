#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Saved UI Designer recipes use Rust admission and actual engine results."""
import base64
import hashlib
import json
import os
from pathlib import Path
import time
import unittest

import merlin as m

ROOT = Path(__file__).resolve().parents[3]
WB = ROOT / "examples/workbench/downstream/cl-192mhz-8mhz-notch/cmce.merlin.json"
RECIPE = ROOT / "crates/pnm-input/tests/fixtures/designer-flat.json"


def document():
    """Assemble the same bounded MCE fixture as the Rust parity test, without science."""
    value = json.loads(WB.read_bytes())
    recipe = RECIPE.read_bytes()
    value["sections"]["measurements"]["rxmer"] = [{
        "name": "UI Designer fixture", "byte_length": len(recipe),
        "sha256": hashlib.sha256(recipe).hexdigest(),
        "base64": base64.b64encode(recipe).decode("ascii"),
    }]
    value["receiver"].update(duration_ms=1, receiver_mode="merlin_estimated")
    return value


class DesignerWorkbenchTests(unittest.TestCase):
    def test_idle_load_and_cancel(self):
        """Loading preserves MCE settings; pre-cancellation yields no simulated report."""
        wb = m.Workbench.from_bytes(json.dumps(document()).encode())
        self.assertEqual([p.value for p in wb.profile_ids], [3])
        self.assertEqual(wb.options[0].receiver_mode, m.ReceiverMode.MERLIN_CHANNEL_ESTIMATED)
        control = m.RunControl()
        control.cancel()
        for batch in wb.run_with_control(control):
            self.assertTrue(all(entry.report is None for entry in batch))

    def test_designer_real_fec_and_rust_parity(self):
        """Run the real decoder and optionally compare a freshly executed Rust report."""
        started = time.perf_counter()
        wb = m.Workbench.from_bytes(json.dumps(document()).encode())
        batches = wb.run()
        del wb
        entry = next(iter(batches[0]))
        self.assertEqual(entry.status, m.RunStatus.Complete)
        self.assertGreater(entry.report.fec.total_codewords.value, 0)
        path = os.environ.get("MERLIN_DESIGNER_PARITY")
        if path:
            expected = json.loads(Path(path).read_text())
            actual = batches[0].to_dict()
            self.assertEqual(actual["status"], expected["status"])
            for got, want in zip(actual["entries"], expected["entries"]):
                self.assertEqual(got["status"], want["status"])
                for key in ("fec", "all_completed_fec", "profile"):
                    self.assertEqual(got["report"][key], want["report"][key])
                # Controller-scoped IDs intentionally differ between independent loads.
                for key in ("receiver_mode", "processing", "limits", "noise_seed", "prbs_seed", "observation"):
                    self.assertEqual(got["report"]["options"][key], want["report"]["options"][key])
        print(f"Designer Python FEC test: {time.perf_counter() - started:.3f}s", flush=True)

    def test_imported_baseline_matches_ordinary_workbench_fec(self):
        """An imported baseline with an empty recipe preserves actual decoder results."""
        value = json.loads(WB.read_bytes())
        value["receiver"]["duration_ms"] = 1
        ordinary = m.Workbench.from_bytes(json.dumps(value).encode()).run()[0].to_dict()
        original = value["sections"]["measurements"]["rxmer"][0]
        payload = json.dumps({"merlin_rxmer_designer": 1,
                              "baseline": {"mode": "imported", "original": original},
                              "recipe": {}, "capture_index": 1, "capture_count": 1}).encode()
        value["sections"]["measurements"]["rxmer"] = [{
            "name": "Imported Designer fixture", "byte_length": len(payload),
            "sha256": hashlib.sha256(payload).hexdigest(),
            "base64": base64.b64encode(payload).decode("ascii"),
        }]
        designed = m.Workbench.from_bytes(json.dumps(value).encode()).run()[0].to_dict()
        self.assertEqual(designed["entries"][0]["status"], "Complete")
        for key in ("fec", "all_completed_fec", "profile"):
            self.assertEqual(designed["entries"][0]["report"][key],
                             ordinary["entries"][0]["report"][key])

    def test_invalid_recipe_structured_error(self):
        """Integrity-valid but unsupported Designer versions fail in the Rust loader."""
        value = document()
        recipe = json.loads(RECIPE.read_bytes())
        recipe["merlin_rxmer_designer"] = 99
        payload = json.dumps(recipe).encode()
        stored = value["sections"]["measurements"]["rxmer"][0]
        stored.update(byte_length=len(payload), sha256=hashlib.sha256(payload).hexdigest(),
                      base64=base64.b64encode(payload).decode("ascii"))
        with self.assertRaises(m.ApiError) as caught:
            m.Workbench.from_bytes(json.dumps(value).encode())
        self.assertEqual(caught.exception.code, m.ErrorCode.UnsupportedCapability)
