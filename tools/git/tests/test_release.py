#!/usr/bin/env python3
#
# Copyright 2026 PNM Viewer contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Exercise release transactions against disposable local bare Git remotes."""
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / 'tools/release'))
from versions import LOCKS, MANIFESTS, synchronize


class ReleaseTests(unittest.TestCase):
    """Verify real Git objects, failed validation, atomic uploads and recovery."""

    def setUp(self):
        """Create a normalized synthetic checkout and local-only origin."""
        temporary = tempfile.TemporaryDirectory(prefix='pnm release tests ')
        self.addCleanup(temporary.cleanup)
        self.base = Path(temporary.name)
        self.root = self.base / 'work'
        self.root.mkdir()
        self.origin = self.base / 'origin.git'
        self.trace = self.base / 'validation-trace'
        self.environment = dict(os.environ, GIT_CONFIG_GLOBAL=os.devnull,
                                GIT_CONFIG_NOSYSTEM='1', RELEASE_TEST_TRACE=str(self.trace),
                                RELEASE_TEST_MODE='pass')
        for name in (*MANIFESTS, *LOCKS, 'tools/release/versions.py',
                     'tools/git/git-push.sh', 'tools/git/push-release.py'):
            target = self.root / name
            target.parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(ROOT / name, target)
        synchronize(self.root, '0.1.0', write=True)
        (self.root / '.gitignore').write_text('__pycache__/\n*.pyc\n')
        (self.root / 'README.md').write_text('Synthetic release fixture\n')
        (self.root / 'scripts').mkdir()
        (self.root / 'scripts/check-before-push.sh').write_text('''#!/bin/bash
set -euo pipefail
cat VERSION >> "$RELEASE_TEST_TRACE"
python3 tools/release/versions.py
case "$RELEASE_TEST_MODE" in
  fail) exit 17 ;;
  dirty) echo unexpected >> README.md ;;
  version-change) echo '# unexpected dependency edit' >> crates/ofdm/Cargo.toml ;;
esac
''')
        self.git('init', '--initial-branch=master')
        self.git('config', 'user.name', 'Release Test')
        self.git('config', 'user.email', 'release@example.invalid')
        self.git('add', '.')
        self.git('commit', '-m', 'Synthetic baseline')
        self.git('init', '--bare', '--initial-branch=master', str(self.origin))
        self.git('remote', 'add', 'origin', str(self.origin))
        self.git('push', 'origin', 'master')
        self.start = self.git('rev-parse', 'HEAD').stdout.strip()

    def git(self, *arguments, check=True, remote=False):
        """Run real Git only in the disposable checkout or its local origin."""
        return subprocess.run(['git', '-C', str(self.origin if remote else self.root), *arguments],
                              env=self.environment, text=True, capture_output=True, check=check)

    def run_helper(self, *arguments):
        """Invoke the public shell helper from outside its checkout."""
        return subprocess.run(['bash', str(self.root / 'tools/git/git-push.sh'), *arguments],
                              cwd=self.base, env=self.environment, text=True,
                              capture_output=True, timeout=30)

    def assert_origin_unchanged(self):
        """Require no remote release commit or tag after a rejected operation."""
        self.assertEqual(self.git('rev-parse', 'master', remote=True).stdout.strip(), self.start)
        self.assertEqual(self.git('tag', '--list', remote=True).stdout.strip(), '')

    def test_patch_release_and_published_noop(self):
        """Validate the bumped tree and publish matching annotated objects once."""
        result = self.run_helper('--tag-patch')
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(self.trace.read_text(), '0.1.1\n')
        self.assertEqual((self.root / 'VERSION').read_text(), '0.1.1\n')
        self.assertEqual(synchronize(self.root, '0.1.1'), [])
        commit = self.git('rev-parse', 'HEAD').stdout.strip()
        self.assertNotEqual(commit, self.start)
        self.assertEqual(self.git('status', '--porcelain').stdout, '')
        self.assertEqual(self.git('cat-file', '-t', 'v0.1.1').stdout.strip(), 'tag')
        self.assertEqual(self.git('rev-parse', 'v0.1.1^{commit}', remote=True).stdout.strip(), commit)
        self.assertEqual(self.git('rev-parse', 'master', remote=True).stdout.strip(), commit)
        result = self.run_helper('--tag-patch')
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIn('Nothing to release', result.stdout)
        self.assertEqual(self.trace.read_text(), '0.1.1\n')

    def publish_baseline(self):
        """Publish the synthetic baseline so force tests start at a stable tag."""
        self.git('tag', '-a', 'v0.1.0', '-m', 'Published baseline')
        self.git('push', 'origin', 'refs/tags/v0.1.0')

    def test_force_published_release_bumps_and_validates_again(self):
        """Allow consecutive clean releases without rewriting existing tags."""
        result = self.run_helper('--tag-patch')
        self.assertEqual(result.returncode, 0, result.stderr)
        previous = self.git('rev-parse', 'v0.1.1', remote=True).stdout.strip()
        result = self.run_helper('--force', '--tag-patch')
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(self.trace.read_text(), '0.1.1\n0.1.2\n')
        self.assertEqual((self.root / 'VERSION').read_text(), '0.1.2\n')
        self.assertEqual(self.git('rev-parse', 'v0.1.1', remote=True).stdout.strip(), previous)
        self.assertEqual(self.git('rev-parse', 'v0.1.2^{commit}', remote=True).stdout.strip(),
                         self.git('rev-parse', 'HEAD').stdout.strip())
        self.assertEqual(self.git('status', '--porcelain').stdout, '')

    def test_force_dry_run_does_not_mutate_published_checkout(self):
        """Preview an intentional fresh patch without checks or ref changes."""
        self.publish_baseline()
        result = self.run_helper('--tag-patch', '--force', '--dry-run')
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIn('0.1.0 -> 0.1.1', result.stdout)
        self.assertEqual(self.git('rev-parse', 'HEAD').stdout.strip(), self.start)
        self.assertEqual(self.git('status', '--porcelain').stdout, '')
        self.assertEqual(self.git('tag', '--list', remote=True).stdout.strip(), 'v0.1.0')
        self.assertFalse(self.trace.exists())

    def test_force_still_stops_on_failed_validation(self):
        """A forced release must validate before creating or pushing a tag."""
        self.publish_baseline()
        self.environment['RELEASE_TEST_MODE'] = 'fail'
        result = self.run_helper('--tag-patch', '--force')
        self.assertNotEqual(result.returncode, 0)
        self.assertEqual(self.trace.read_text(), '0.1.1\n')
        self.assertEqual(self.git('rev-parse', 'master', remote=True).stdout.strip(), self.start)
        self.assertEqual(self.git('tag', '--list', remote=True).stdout.strip(), 'v0.1.0')
        self.assertEqual(self.git('rev-parse', 'HEAD').stdout.strip(), self.start)

    def test_force_still_rejects_dirty_tree_and_tag_collision(self):
        """Force only overrides the published-HEAD no-op, not other guards."""
        self.publish_baseline()
        unrelated = self.root / 'user-work'
        unrelated.write_text('Uncommitted work')
        result = self.run_helper('--tag-patch', '--force')
        self.assertNotEqual(result.returncode, 0)
        unrelated.unlink()
        self.git('tag', 'v0.1.1')
        result = self.run_helper('--tag-patch', '--force')
        self.assertNotEqual(result.returncode, 0)
        self.assertIn('already exists', result.stderr)
        self.assertFalse(self.trace.exists())
        self.assertEqual(self.git('status', '--porcelain').stdout, '')
        self.assertEqual(self.git('tag', '--list', remote=True).stdout.strip(), 'v0.1.0')

    def test_force_does_not_skip_unpublished_release_recovery(self):
        """Preserve an unpushed release tag and require explicit --tag recovery."""
        self.git('tag', '-a', 'v0.1.0', '-m', 'Pending baseline')
        result = self.run_helper('--tag-patch', '--force')
        self.assertNotEqual(result.returncode, 0)
        self.assertIn('already tagged', result.stderr)
        self.assertFalse(self.trace.exists())
        self.assert_origin_unchanged()

    def test_minor_and_major_modes_reset_lower_components(self):
        """Exercise each public bump mode through real manifests and Git refs."""
        for option, expected in [('--tag-minor', '0.2.0'), ('--tag-major', '1.0.0')]:
            if option == '--tag-major':
                (self.root / 'README.md').write_text('Next development change\n')
                self.git('add', 'README.md')
                self.git('commit', '-m', 'Another development change')
            result = self.run_helper(option)
            self.assertEqual(result.returncode, 0, result.stderr)
            self.assertEqual((self.root / 'VERSION').read_text().strip(), expected)
            self.assertEqual(synchronize(self.root, expected), [])
            self.assertEqual(self.git('cat-file', '-t', f'v{expected}', remote=True).stdout.strip(), 'tag')

    def test_dry_run_leaves_files_refs_and_checks_untouched(self):
        """A release preview reads origin but creates no candidate or tag."""
        result = self.run_helper('--tag-patch', '--dry-run')
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIn('0.1.0 -> 0.1.1', result.stdout)
        self.assertEqual(self.git('status', '--porcelain').stdout, '')
        self.assertEqual(self.git('rev-parse', 'HEAD').stdout.strip(), self.start)
        self.assertFalse(self.trace.exists())
        self.assertEqual(self.git('tag', '--list').stdout, '')
        self.assert_origin_unchanged()

    def test_failed_checks_preserve_candidate_without_commit_tag_or_push(self):
        """Validation failure retains version edits for review and stops upload."""
        self.environment['RELEASE_TEST_MODE'] = 'fail'
        result = self.run_helper('--tag-patch')
        self.assertNotEqual(result.returncode, 0)
        self.assertEqual(self.trace.read_text(), '0.1.1\n')
        self.assertEqual((self.root / 'VERSION').read_text().strip(), '0.1.1')
        self.assertEqual(self.git('rev-parse', 'HEAD').stdout.strip(), self.start)
        self.assertEqual(self.git('tag', '--list').stdout, '')
        self.assert_origin_unchanged()

    def test_check_side_effects_stop_before_commit(self):
        """Reject unrelated changes and edits within expected version files."""
        self.environment['RELEASE_TEST_MODE'] = 'version-change'
        result = self.run_helper('--tag-patch')
        self.assertNotEqual(result.returncode, 0)
        self.assertIn('changed during validation', result.stderr)
        self.assertEqual(self.git('rev-parse', 'HEAD').stdout.strip(), self.start)
        self.assert_origin_unchanged()

    def test_unrelated_validation_changes_stop_before_commit(self):
        """Even a successful check cannot add unrelated files to a release."""
        self.environment['RELEASE_TEST_MODE'] = 'dirty'
        result = self.run_helper('--tag-patch')
        self.assertNotEqual(result.returncode, 0)
        self.assertIn('working tree changed', result.stderr)
        self.assertEqual(self.git('rev-parse', 'HEAD').stdout.strip(), self.start)
        self.assert_origin_unchanged()

    def test_atomic_failure_preserves_refs_and_retry_does_not_bump(self):
        """A rejected tag cannot upload only master; --tag recovers the same release."""
        hook = self.origin / 'hooks/update'
        hook.write_text('#!/bin/sh\ncase "$1" in refs/tags/*) exit 1 ;; esac\nexit 0\n')
        hook.chmod(0o755)
        result = self.run_helper('--tag-patch')
        self.assertNotEqual(result.returncode, 0)
        self.assertIn('--tag v0.1.1', result.stderr)
        self.assert_origin_unchanged()
        pending = self.git('rev-parse', 'HEAD').stdout.strip()
        self.assertEqual(self.git('rev-parse', 'v0.1.1^{commit}').stdout.strip(), pending)
        result = self.run_helper('--tag-patch')
        self.assertNotEqual(result.returncode, 0)
        self.assertIn('already tagged', result.stderr)
        hook.unlink()
        result = self.run_helper('--tag', 'v0.1.1')
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(self.git('rev-parse', 'master', remote=True).stdout.strip(), pending)
        self.assertEqual(self.git('rev-parse', 'v0.1.1^{commit}', remote=True).stdout.strip(), pending)
        self.assertEqual((self.root / 'VERSION').read_text().strip(), '0.1.1')
        self.assertEqual(self.trace.read_text(), '0.1.1\n0.1.1\n')

    def test_existing_first_tag_publishes_without_bump(self):
        """Support the initial v0.1.0 release and make a repeated push a no-op."""
        self.git('tag', '-a', 'v0.1.0', '-m', 'Initial release')
        result = self.run_helper('--tag', 'v0.1.0')
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(self.git('rev-parse', 'HEAD').stdout.strip(), self.start)
        self.assertEqual(self.trace.read_text(), '0.1.0\n')
        self.assertEqual(self.git('rev-parse', 'v0.1.0^{commit}', remote=True).stdout.strip(), self.start)
        result = self.run_helper('--tag', 'v0.1.0')
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(self.trace.read_text(), '0.1.0\n')

    def test_unrelated_tags_are_not_followed(self):
        """Explicit refspecs and no-follow-tags override ambient push settings."""
        self.git('tag', '-a', 'v9.9.9', '-m', 'Unrelated tag')
        self.git('config', 'push.followTags', 'true')
        result = self.run_helper('--tag-patch')
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(self.git('tag', '--list', remote=True).stdout.strip(), 'v0.1.1')

    def test_behind_origin_stops_before_version_changes(self):
        """An upstream commit must not be silently bypassed by release preparation."""
        other = self.base / 'other checkout'
        self.git('clone', str(self.origin), str(other))
        (other / 'README.md').write_text('New upstream change\n')
        self.git('-C', str(other), 'add', 'README.md')
        self.git('-C', str(other), '-c', 'user.name=Release Test',
                 '-c', 'user.email=release@example.invalid', 'commit', '-m', 'Upstream change')
        self.git('-C', str(other), 'push', 'origin', 'master')
        remote_commit = self.git('rev-parse', 'master', remote=True).stdout.strip()
        result = self.run_helper('--tag-patch')
        self.assertNotEqual(result.returncode, 0)
        self.assertIn('behind or diverged', result.stderr)
        self.assertFalse(self.trace.exists())
        self.assertEqual(self.git('status', '--porcelain').stdout, '')
        self.assertEqual(self.git('rev-parse', 'master', remote=True).stdout.strip(), remote_commit)

    def test_commit_failure_retains_staged_candidate(self):
        """A failing commit hook leaves reviewable version changes without a tag."""
        hook = self.root / '.git/hooks/pre-commit'
        hook.write_text('#!/bin/sh\nexit 1\n')
        hook.chmod(0o755)
        result = self.run_helper('--tag-patch')
        self.assertNotEqual(result.returncode, 0)
        self.assertEqual(self.git('rev-parse', 'HEAD').stdout.strip(), self.start)
        self.assertEqual(self.git('tag', '--list').stdout, '')
        self.assertEqual((self.root / 'VERSION').read_text().strip(), '0.1.1')
        self.assertTrue(self.git('diff', '--cached', '--name-only').stdout)
        self.assert_origin_unchanged()

    def test_local_and_remote_tag_collisions_are_immutable(self):
        """An occupied target version fails before changing any metadata."""
        self.git('tag', 'v0.1.1')
        result = self.run_helper('--tag-patch')
        self.assertNotEqual(result.returncode, 0)
        self.assertIn('already exists', result.stderr)
        self.git('push', 'origin', 'refs/tags/v0.1.1')
        self.git('tag', '-d', 'v0.1.1')
        result = self.run_helper('--tag-patch')
        self.assertNotEqual(result.returncode, 0)
        self.assertIn('already exists', result.stderr)
        self.assertEqual(self.git('status', '--porcelain').stdout, '')
        self.assertFalse(self.trace.exists())

    def test_existing_tag_mismatch_is_rejected(self):
        """Never replace an origin tag even if its target commit is identical."""
        self.git('tag', '-a', 'v0.1.0', '-m', 'Origin annotation')
        self.git('push', 'origin', 'refs/tags/v0.1.0')
        original = self.git('rev-parse', 'v0.1.0', remote=True).stdout.strip()
        self.git('tag', '-d', 'v0.1.0')
        self.git('tag', '-a', 'v0.1.0', '-m', 'Different local annotation')
        result = self.run_helper('--tag', 'v0.1.0')
        self.assertNotEqual(result.returncode, 0)
        self.assertIn('Remote tag differs', result.stderr)
        self.assertEqual(self.git('rev-parse', 'v0.1.0', remote=True).stdout.strip(), original)
        self.assertFalse(self.trace.exists())

    def test_dirty_checkout_is_rejected(self):
        """Do not include tracked, staged or untracked work in version preparation."""
        (self.root / 'untracked').write_text('User work\n')
        result = self.run_helper('--tag-patch')
        self.assertNotEqual(result.returncode, 0)
        self.assertEqual((self.root / 'VERSION').read_text().strip(), '0.1.0')
        self.assertFalse(self.trace.exists())
        self.assert_origin_unchanged()

    def test_non_master_release_is_rejected(self):
        """Tag workflow ancestry requires master, even though branch-only allows main."""
        self.git('switch', '-c', 'main')
        result = self.run_helper('--tag-patch')
        self.assertNotEqual(result.returncode, 0)
        self.assertFalse(self.trace.exists())
        self.assert_origin_unchanged()

    def test_version_drift_is_rejected_before_checks(self):
        """Consistently committed version metadata is required before bumping."""
        (self.root / 'VERSION').write_text('0.1.2\n')
        self.git('add', 'VERSION')
        self.git('commit', '-m', 'Synthetic drift')
        result = self.run_helper('--tag-patch')
        self.assertNotEqual(result.returncode, 0)
        self.assertIn('consistent VERSION', result.stderr)
        self.assertFalse(self.trace.exists())
        self.assert_origin_unchanged()

    def test_mirror_remote_is_rejected(self):
        """Prevent remote mirror configuration from widening the pushed ref set."""
        self.git('config', 'remote.origin.mirror', 'true')
        result = self.run_helper('--tag-patch')
        self.assertNotEqual(result.returncode, 0)
        self.assertIn('mirror', result.stderr)
        self.assertFalse(self.trace.exists())
        self.assert_origin_unchanged()

    def test_invalid_modes_and_missing_tags_do_not_modify_checkout(self):
        """Reject conflicting/invalid modes and nonmatching or missing tag identities."""
        for arguments in [('--tag-patch', '--tag-minor'), ('--tag',), ('--dry-run',),
                          ('--tag', 'bad'), ('--tag', 'v0.2.0'), ('--tag', 'v0.1.0'),
                          ('--force',), ('--master', '--force'), ('--tag', 'v0.1.0', '--force')]:
            with self.subTest(arguments=arguments):
                result = self.run_helper(*arguments)
                self.assertNotEqual(result.returncode, 0)
                self.assertEqual(self.git('status', '--porcelain').stdout, '')
                self.assertFalse(self.trace.exists())
        self.assert_origin_unchanged()


if __name__ == '__main__':
    unittest.main()
