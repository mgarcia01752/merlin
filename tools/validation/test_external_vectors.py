#!/usr/bin/env python3
#
# Copyright 2026 DOCSIS OFDM Channel Sim contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Synthetic admission-tool tests; these are not external PHY test vectors."""
import hashlib
import json
import tempfile
import unittest
from pathlib import Path
from admit_external_vectors import ROLES, admit


class AdmissionTests(unittest.TestCase):
    def test_bounded_manifest_checksums_paths_and_explicit_unexecuted_status(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            records = []
            for role in sorted(ROLES):
                (root / role).write_bytes(b"test only")
                records.append({"role": role, "path": role, "bytes": 9,
                                "sha256": hashlib.sha256(b"test only").hexdigest()})
            document = {"schema_version": 1, "producer": "synthetic tool test",
                        "producer_version": "1", "reference": "not independent evidence", "files": records}
            manifest = root / "manifest.json"
            def write():
                manifest.write_text(json.dumps(document), encoding="utf-8")
            write()
            self.assertEqual(admit(manifest)["status"], "admitted_unexecuted")
            records[0]["path"] = "../escape"
            write()
            with self.assertRaises(ValueError):
                admit(manifest)
            records[0]["path"] = records[0]["role"]
            records[0]["sha256"] = "0" * 64
            write()
            with self.assertRaises(ValueError):
                admit(manifest)
            records[0]["sha256"] = hashlib.sha256(b"test only").hexdigest()
            records[1]["role"] = records[0]["role"]
            write()
            with self.assertRaises(ValueError):
                admit(manifest)
