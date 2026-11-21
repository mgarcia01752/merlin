#!/usr/bin/env python3
#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Protect installed native acceptance identity and retained command failures."""
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest

from native_api import passed_cases, run


class NativeApiRunnerTests(unittest.TestCase):
    def test_complete_named_passes_exclude_failed_skipped_and_duplicate_runs(self):
        """No count-only, skipped or partial run becomes native acceptance."""
        body = ('test_actual (test_api.NativeTests)\nActual work. ... ok\n\n'
                '----------------------------------------------------------------------\nRan 1 tests in 0.01s\n\nOK\n')
        self.assertEqual(passed_cases(body), [{'test_file': 'bindings/python/tests/test_api.py',
                                             'qualified_test_case': 'NativeTests.test_actual'}])
        for changed in (body.replace('Ran 1', 'Ran 2'), body.replace('OK\n', 'FAILED\n'),
                        body.replace('OK\n', 'OK (skipped=1)\n'), body.replace('test_actual (test_api.NativeTests)', 'unknown'),
                        body.replace('Ran 1', 'Ran 2').replace('----------------------------------------------------------------------',
                            'test_actual (test_api.NativeTests)\nDuplicate. ... ok\n----------------------------------------------------------------------')):
            with self.assertRaises(ValueError):
                passed_cases(changed)

    def test_actual_child_failure_keeps_both_streams_and_exit_identity(self):
        """Run an independent tiny process; never mock a native/scientific pass."""
        with tempfile.TemporaryDirectory() as temporary:
            directory = Path(temporary)
            report = []
            command = [sys.executable, '-c', 'import sys; print("out"); print("err", file=sys.stderr); sys.exit(7)']
            with self.assertRaises(subprocess.CalledProcessError):
                run(command, directory, os.environ.copy(), 'failure', report, cwd=directory)
            self.assertEqual(report[0]['exit_code'], 7)
            self.assertGreaterEqual(report[0]['elapsed_seconds'], 0)
            self.assertEqual((directory / 'failure.stdout').read_text().strip(), 'out')
            self.assertEqual((directory / 'failure.stderr').read_text().strip(), 'err')
