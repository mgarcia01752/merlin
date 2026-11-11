#!/usr/bin/env python3
#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Exercise the real local gate with isolated command doubles, without builds."""
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[3]


class ValidationOptionTests(unittest.TestCase):
    """Verify retained checks and omitted expensive commands independently."""

    def test_skip_test_keeps_static_tooling_and_build_checks(self):
        """Run the actual gate and inspect every dispatched validation command."""
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / 'scripts').mkdir()
            (root / 'tools/git').mkdir(parents=True)
            (root / 'bin').mkdir()
            shutil.copy2(ROOT / 'scripts/check-before-push.sh', root / 'scripts/check-before-push.sh')
            for name in ('tools/bump-version.sh', 'tools/git/git-save.sh', 'tools/git/git-push.sh'):
                (root / name).write_text('#!/bin/bash\n')
            stub = '#!/bin/bash\nprintf "%s\\n" "$*" >> "$GATE_TRACE"\n'
            for name in ('scripts/cargo-local.sh', 'bin/python3', 'bin/shellcheck'):
                path = root / name
                path.write_text(stub)
                path.chmod(0o755)
            trace = root / 'trace'
            environment = dict(os.environ, PATH=f"{root / 'bin'}:{os.environ['PATH']}",
                               GATE_TRACE=str(trace))
            result = subprocess.run(['bash', str(root / 'scripts/check-before-push.sh'), '--skip-test'],
                                    env=environment, capture_output=True, text=True)
            self.assertEqual(result.returncode, 0, result.stderr)
            commands = trace.read_text().splitlines()
            self.assertIn('tools/release/versions.py', commands)
            self.assertIn("-m unittest discover -s tools/git/tests -p test_*.py", commands)
            self.assertIn('fmt --all -- --check', commands)
            self.assertIn('clippy --workspace --all-targets --locked -- -D warnings', commands)
            self.assertIn('build -p ofdm-fec-cli --release --locked', commands)
            self.assertFalse(any(command.startswith(('test ', 'run ')) for command in commands))
            self.assertIn('were NOT run', result.stdout)
            trace.unlink()
            for arguments in (['--unknown'], ['--skip-test', '--unknown']):
                result = subprocess.run(['bash', str(root / 'scripts/check-before-push.sh'), *arguments],
                                        env=environment, capture_output=True, text=True)
                self.assertEqual(result.returncode, 2)
                self.assertFalse(trace.exists())
