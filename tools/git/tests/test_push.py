#!/usr/bin/env python3
#
# Copyright 2026 PNM Viewer contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Check the push gate using fake Git and validation; never contact a remote."""
from pathlib import Path
import os
import shutil
import subprocess
import tempfile
import unittest

PROJECT = Path(__file__).resolve().parents[3]


class PushGateTests(unittest.TestCase):
    """A failed check or changed checkout must prevent the push invocation."""

    def exercise(self, mode, *arguments):
        """Run a copied helper with controlled Git responses and return its trace."""
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / "tools/git").mkdir(parents=True)
            (root / "scripts").mkdir()
            (root / "bin").mkdir()
            (root / "tools/python").mkdir()
            (root / "tools/python/update_integration_doc.py").write_text(
                'import os, pathlib, sys\n'
                'if os.environ["MODE"] == "doc-fail": sys.exit(23)\n'
                'if os.environ["MODE"] == "doc-stale": pathlib.Path(os.environ["STATE"]).touch()\n'
                'pathlib.Path(os.environ["DOC_TRACE"]).write_text("refreshed")\n')
            shutil.copy2(PROJECT / "tools/git/git-push.sh", root / "tools/git/git-push.sh")
            shutil.copy2(PROJECT / "tools/git/docs_only.py", root / "tools/git/docs_only.py")
            (root / "scripts/check-before-push.sh").write_text(
                '#!/bin/bash\n'
                'echo validate >> "$TRACE"\n'
                'if [[ $# -gt 0 ]]; then printf "%s\\n" "$@" >> "$TRACE"; fi\n'
                'touch "$STATE"\n'
                'if [[ "$MODE" == fail ]]; then exit 17; fi\n', encoding="utf-8")
            git = root / "bin/git"
            git.write_text('''#!/bin/bash
shift 2
case "$1" in
  rev-parse)
    if [[ "$2" == --is-inside-work-tree ]]; then echo true
    elif [[ "$MODE" == changed && -e "$STATE" ]]; then echo changed
    else echo validated; fi ;;
  status)
    if [[ ( "$MODE" == dirty || "$MODE" == doc-stale ) && -e "$STATE" ]]; then echo ' M source'; fi ;;
  branch) echo master ;;
  remote) echo git@example.invalid:repo ;;
  ls-remote)
    if [[ "$MODE" == remote-fail ]]; then exit 19; fi
    if [[ "$MODE" == docs || "$MODE" == mixed ]]; then echo 'base refs/heads/master'; fi ;;
  fetch|merge-base) exit 0 ;;
  rev-list) echo outgoing ;;
  diff-tree)
    if [[ "$MODE" == docs ]]; then printf 'README.md\\0'
    else printf 'source.rs\\0'; fi ;;
  push) echo "push $*" >> "$TRACE" ;;
  *) exit 99 ;;
esac
''', encoding="utf-8")
            git.chmod(0o755)
            trace = root / "trace"
            result = subprocess.run(["bash", str(root / "tools/git/git-push.sh"), *arguments],
                                    capture_output=True, text=True,
                                    env=dict(os.environ, PATH=f"{root / 'bin'}:{os.environ['PATH']}",
                                             MODE=mode, TRACE=str(trace), STATE=str(root / "state"),
                                             DOC_TRACE=str(root / "doc-trace")))
            if mode not in ("doc-fail",):
                self.assertTrue((root / "doc-trace").exists())
            return result, trace.read_text() if trace.exists() else ""

    def test_failed_checks_block_push(self):
        """Propagate validation failure without invoking git push."""
        result, trace = self.exercise("fail")
        self.assertEqual(result.returncode, 17)
        self.assertEqual(trace, "validate\n")
        self.assertIn("Push command total time (branch): ", result.stderr)
        self.assertIn("(exit 17)", result.stderr)

    def test_explicit_skip_test_is_forwarded_without_bypassing_gate(self):
        """Branch pushes forward the option and retain failure handling."""
        for mode in ('pass', 'fail'):
            result, trace = self.exercise(mode, '--skip-test')
            self.assertTrue(trace.startswith('validate\n--skip-test\n'))
            if mode == 'pass':
                self.assertEqual(result.returncode, 0, result.stderr)
                self.assertIn('push ', trace)
            else:
                self.assertNotEqual(result.returncode, 0)
                self.assertNotIn('push ', trace)

    def test_quick_selection_keeps_validation_and_failure_guards(self):
        """Branch quick mode must reach the gate and never bypass a failure."""
        for mode in ('pass', 'fail'):
            result, trace = self.exercise(mode, '--test-quick')
            self.assertTrue(trace.startswith('validate\n--test-quick\n'))
            self.assertEqual(result.returncode == 0, mode == 'pass')
            self.assertEqual('push ' in trace, mode == 'pass')

    def test_changed_checkout_blocks_push(self):
        """Reject commit or file changes after otherwise successful checks."""
        for mode in ("changed", "dirty"):
            result, trace = self.exercise(mode)
            self.assertNotEqual(result.returncode, 0)
            self.assertNotIn("push", trace)

    def test_success_pushes_only_validated_commit(self):
        """Push the tested object explicitly to the selected branch."""
        result, trace = self.exercise("pass")
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertTrue(trace.startswith("validate\n"))
        self.assertIn("validated:refs/heads/master", trace)

    def test_remote_failure_stops_before_validation(self):
        """An inaccessible remote is not mistaken for an empty repository."""
        result, trace = self.exercise("remote-fail")
        self.assertEqual(result.returncode, 19)
        self.assertEqual(trace, "")

    def test_documentation_range_skips_validation(self):
        result, trace = self.exercise("docs")
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertNotIn("validate\n", trace)
        self.assertIn("validated:refs/heads/master", trace)
        self.assertIn("Documentation-only", result.stdout)

    def test_mixed_range_runs_validation(self):
        result, trace = self.exercise("mixed")
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertTrue(trace.startswith("validate\n"))

    def test_document_refresh_and_generator_failures_block_push(self):
        """Never push stale guide changes or bypass generator failures."""
        for mode in ('doc-stale', 'doc-fail'):
            result, trace = self.exercise(mode, '--skip-test')
            self.assertNotEqual(result.returncode, 0)
            self.assertEqual(trace, '')
            if mode == 'doc-stale':
                self.assertIn('review and commit', result.stderr)
