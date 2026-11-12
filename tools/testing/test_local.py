#!/usr/bin/env python3
#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Exercise local phase scheduling without Cargo, compilation or downloads."""
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[2]


class LocalRunnerTests(unittest.TestCase):
    """Verify bounded workers, phase ordering and propagated failures."""

    def exercise(self, *arguments, workers='4', failure='0'):
        """Run the actual launcher against isolated command doubles.

        Args:
            arguments: Launcher phase and optional nextest selection arguments.
            workers: Test-worker environment override.
            failure: Exit status returned by the first fake Cargo command.
        Returns:
            Process result and dispatched command lines; no actual tests run.
        """
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / 'scripts').mkdir()
            (root / 'bin').mkdir()
            shutil.copy2(ROOT / 'scripts/test-local.sh', root / 'scripts/test-local.sh')
            python = root / 'bin/python3'
            python.write_text('#!/bin/sh\ncase "$1" in */quick_tests.py) echo "not(test(=omitted_sweep))" ;; *) printf "%s/bin/cargo-nextest\\n" "$FAKE_ROOT" ;; esac\n')
            python.chmod(0o755)
            cargo = root / 'scripts/cargo-local.sh'
            cargo.write_text('#!/bin/sh\nprintf "%s\\n" "$*" >> "$TRACE"\nexit "$FAILURE"\n')
            cargo.chmod(0o755)
            trace = root / 'trace'
            environment = dict(os.environ, PATH=f"{root / 'bin'}:{os.environ['PATH']}",
                               FAKE_ROOT=str(root), TRACE=str(trace), FAILURE=failure,
                               MERLIN_TEST_THREADS=workers)
            result = subprocess.run(['bash', str(root / 'scripts/test-local.sh'), *arguments],
                                    env=environment, text=True, capture_output=True)
            return result, trace.read_text().splitlines() if trace.exists() else []

    def test_all_phases_keep_doctests_examples_and_analysis(self):
        """Parallel scheduling preserves each formerly required test phase."""
        result, commands = self.exercise('all', workers='4')
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(len(commands), 5)
        self.assertIn('nextest run --profile local --workspace --locked --test-threads 4', commands[0])
        self.assertEqual(commands[1], 'test --workspace --doc --locked')
        self.assertIn('--example prepare_rxmer_variants', commands[2])
        self.assertIn('--no-default-features', commands[3])
        self.assertEqual(commands[4], 'test -p docsis-ofdm-fec --no-default-features --doc --locked')

    def test_scoped_selection_and_failure_do_not_widen_or_hide_results(self):
        """Selections are forwarded, and failed workspace tests stop later phases."""
        result, commands = self.exercise('workspace', '-E', 'binary(uniform_profile_orders)', workers='2')
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIn('--test-threads 2 -E binary(uniform_profile_orders)', commands[0])
        result, commands = self.exercise('all', failure='17')
        self.assertEqual(result.returncode, 17)
        self.assertEqual(len(commands), 1)

    def test_quick_selection_reaches_all_nextest_phases_only(self):
        """Filter sweep cases without dropping docs, examples or analysis."""
        result, commands = self.exercise('all', '--test-quick')
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(len(commands), 5)
        for index in (0, 2, 3):
            self.assertIn('-E not(test(=omitted_sweep))', commands[index])
        self.assertEqual(commands[1], 'test --workspace --doc --locked')
        self.assertEqual(commands[4], 'test -p docsis-ofdm-fec --no-default-features --doc --locked')
        result, commands = self.exercise('all', '--test-quick', '-E', 'all()')
        self.assertEqual(result.returncode, 2)
        self.assertEqual(commands, [])
        result, commands = self.exercise('all', '--test-quick', failure='17')
        self.assertEqual(result.returncode, 17)
        self.assertEqual(len(commands), 1)

    def test_invalid_workers_and_modes_stop_before_bootstrap(self):
        """Reject accidental oversubscription and unsupported command modes."""
        for workers in ('0', '-1', '33', 'many'):
            result, commands = self.exercise('all', workers=workers)
            self.assertEqual(result.returncode, 2)
            self.assertEqual(commands, [])
        result, commands = self.exercise('unknown')
        self.assertEqual(result.returncode, 2)
        self.assertEqual(commands, [])
