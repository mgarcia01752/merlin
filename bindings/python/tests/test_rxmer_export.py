#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Native binary/JSON export guards, ownership and decoder compatibility."""
import json
from pathlib import Path
import tempfile
import unittest

import merlin as m

ROOT = Path(__file__).resolve().parents[3]
FIXTURE = ROOT / "examples/measurements/single-channel/rxmer.json"


class RxmerExportTests(unittest.TestCase):
    def test_shared_conversion_guards_and_lossless_json(self):
        """Verify Rust numeric anchors, null preservation and no-clobber saves."""
        document = json.loads(FIXTURE.read_text())
        # Independent anchors: range endpoints, half-quarter ties and missing data.
        values = [-2.125, 70.125, 0.125, 63.375, None]
        document["values"] = values
        document["data_length"] = len(values)
        capture = m.RxMer.from_bytes("test.json", json.dumps(document).encode())
        with self.assertRaises(m.ApiError) as failure:
            capture.to_pnm_bytes()
        self.assertEqual(failure.exception.export_code, "out_of_range")
        self.assertEqual(failure.exception.subcarrier,
                         document["first_active_subcarrier_index"])
        self.assertEqual(capture.to_pnm_bytes(clamp=True)[28:], bytes([0, 254, 1, 254, 255]))
        summary = capture.pnm_export_summary(clamp=True)
        self.assertEqual(summary["clipped"], 2)
        self.assertEqual(summary["rounded"], 2)
        self.assertEqual(summary["missing"], 1)
        output = json.loads(capture.to_json_bytes())
        self.assertEqual(output["values"], values)
        self.assertFalse(output["merlin_export"]["modem_measurement"])
        restored = m.RxMer.from_bytes("saved.json", capture.to_json_bytes())
        self.assertEqual(json.loads(restored.to_json_bytes())["values"], values)
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "capture.json"
            capture.save_json(path)
            self.assertEqual(json.loads(path.read_text())["values"], values)
            self.assertTrue(Path(str(path) + ".merlin.json").is_file())
            with self.assertRaises(m.ApiError):
                capture.save_json(path)
            binary = Path(directory) / "capture.bin"
            with self.assertRaises(m.ApiError):
                capture.save_pnm(binary)
            self.assertFalse(binary.exists())
            capture.save_pnm(binary, clamp=True)
            reloaded = m.RxMer.from_file(binary)
            self.assertEqual(json.loads(reloaded.to_json_bytes())["values"],
                             [0.0, 63.5, 0.25, 63.5, None])
            self.assertEqual(json.loads(capture.to_json_bytes())["values"], values)
            with self.assertRaises(m.ApiError):
                capture.save_pnm(binary, clamp=True)


if __name__ == "__main__":
    unittest.main()
