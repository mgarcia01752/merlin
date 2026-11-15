#!/usr/bin/env python3
#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Check retained timing and failure propagation without compiling examples."""
import json
from pathlib import Path
import subprocess
import tempfile
import unittest
from unittest.mock import patch

import api_examples


class ExampleTimingTests(unittest.TestCase):
    """Timing must survive failed example execution without weakening the gate."""

    def test_success_and_failure_record_elapsed_time(self):
        """Persist independent status and elapsed evidence for both outcomes."""
        for fails in (False, True):
            with self.subTest(fails=fails), tempfile.TemporaryDirectory() as directory:
                log = Path(directory) / "example.log"
                failure = subprocess.CalledProcessError(9, ["fake"]) if fails else None
                with patch.object(api_examples.time, "perf_counter", side_effect=[10.0, 12.5]), \
                        patch.object(api_examples.subprocess, "run", side_effect=failure):
                    if fails:
                        with self.assertRaises(subprocess.CalledProcessError):
                            api_examples.run_example(["fake"], log)
                    else:
                        api_examples.run_example(["fake"], log)
                record = json.loads(log.with_suffix(".json").read_text())
                self.assertEqual(record["elapsed_seconds"], 2.5)
                self.assertEqual(record["status"], "failed" if fails else "passed")


if __name__ == "__main__":
    unittest.main()
