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

    def exercise(self, *arguments, workers='4', failure='0', delay='0'):
        """Run the actual launcher against isolated command doubles.

        Args:
            arguments: Launcher phase and optional nextest selection arguments.
            workers: Test-worker environment override.
            failure: Exit status returned by the first fake Cargo command.
            delay: Seconds spent by each fake Cargo command for elapsed checks.
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
            cargo.write_text('#!/bin/sh\nprintf "%s\\n" "$*" >> "$TRACE"\nsleep "$FAKE_DELAY"\nexit "$FAILURE"\n')
            cargo.chmod(0o755)
            trace = root / 'trace'
            environment = dict(os.environ, PATH=f"{root / 'bin'}:{os.environ['PATH']}",
                               FAKE_ROOT=str(root), TRACE=str(trace), FAILURE=failure,
                               MERLIN_TEST_THREADS=workers, FAKE_DELAY=delay)
            result = subprocess.run(['bash', str(root / 'scripts/test-local.sh'), *arguments],
                                    env=environment, text=True, capture_output=True)
            return result, trace.read_text().splitlines() if trace.exists() else []

    def test_elapsed_summary_includes_child_work_and_preserves_failure(self):
        """End-to-end timing includes actual waits and retains the child's status."""
        result, commands = self.exercise('example', delay='1.1')
        self.assertEqual(result.returncode, 0, result.stderr)
        line = result.stderr.strip().splitlines()[-1]
        self.assertTrue(line.startswith('Unit test total time (example): '), line)
        duration = line.split(': ', 1)[1].split()[:3]
        self.assertEqual([value[-1] for value in duration], ['h', 'm', 's'])
        seconds = sum(int(value[:-1]) * factor for value, factor in zip(duration, (3600, 60, 1)))
        self.assertGreaterEqual(seconds, 1)
        self.assertIn('(exit 0)', line)
        result, commands = self.exercise('all', failure='17')
        self.assertEqual(result.returncode, 17)
        self.assertIn('Unit test total time (all): ', result.stderr)
        self.assertIn('(exit 17)', result.stderr)
        for mode in ('--help', 'list'):
            result, commands = self.exercise(mode)
            self.assertEqual(result.returncode, 0, result.stderr)
            self.assertNotIn('Unit test total time', result.stderr)

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
