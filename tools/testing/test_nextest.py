#!/usr/bin/env python3
#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Verify the pinned runner bootstrap without network or executable downloads."""
import hashlib
import io
from pathlib import Path
import tarfile
import tempfile
import unittest
from unittest.mock import patch

import nextest


class BootstrapTests(unittest.TestCase):
    """Synthetic archives exercise integrity and redirected-path rejection."""

    def setUp(self):
        """Prepare isolated storage and an inert archive member."""
        self.temporary = tempfile.TemporaryDirectory()
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name)
        content = io.BytesIO()
        with tarfile.open(fileobj=content, mode='w:gz') as archive:
            member = tarfile.TarInfo('cargo-nextest')
            member.size = len(b'not executable test bytes')
            archive.addfile(member, io.BytesIO(b'not executable test bytes'))
        self.archive = content.getvalue()
        for target, value in (('SHA256', hashlib.sha256(self.archive).hexdigest()),):
            context = patch.object(nextest, target, value)
            context.start()
            self.addCleanup(context.stop)
        for method, value in (('system', 'Linux'), ('machine', 'x86_64')):
            context = patch.object(nextest.platform, method, return_value=value)
            context.start()
            self.addCleanup(context.stop)

    def test_download_then_offline_reverification_and_repair(self):
        """Only pinned archive bytes produce the runner; cache needs no network."""
        with patch.object(nextest.urllib.request, 'urlopen', return_value=io.BytesIO(self.archive)) as network:
            executable = nextest.prepare(self.root)
            network.assert_called_once()
        self.assertEqual(executable.read_bytes(), b'not executable test bytes')
        executable.write_bytes(b'changed executable')
        with patch.object(nextest.urllib.request, 'urlopen', side_effect=AssertionError('offline')):
            self.assertEqual(nextest.prepare(self.root).read_bytes(), b'not executable test bytes')
        (executable.parent / 'nextest.tar.gz').write_bytes(b'corrupt archive')
        with self.assertRaisesRegex(ValueError, 'checksum'):
            nextest.prepare(self.root)

    def test_corrupt_download_and_redirected_storage_are_rejected(self):
        """Failed downloads and symlinks cannot install or replace executables."""
        with patch.object(nextest.urllib.request, 'urlopen', return_value=io.BytesIO(b'bad')):
            with self.assertRaisesRegex(ValueError, 'checksum'):
                nextest.prepare(self.root)
        self.assertFalse(list(self.root.rglob('cargo-nextest')))
        # Resolve outside the selected synthetic checkout, still test-owned.
        project = self.root / 'project'
        project.mkdir()
        (project / 'local').symlink_to(self.root)
        with self.assertRaisesRegex(ValueError, 'redirected'):
            nextest.prepare(project)

    def test_unsupported_host_never_downloads(self):
        """Host validation occurs before writes or network access."""
        with patch.object(nextest.platform, 'machine', return_value='aarch64'):
            with self.assertRaisesRegex(ValueError, 'x86_64'):
                nextest.prepare(self.root)
        self.assertEqual(list(self.root.iterdir()), [])

    def test_link_member_is_rejected_without_touching_its_target(self):
        """Archive member links must never be extracted or followed."""
        victim = self.root / 'keep'
        victim.write_bytes(b'original')
        content = io.BytesIO()
        with tarfile.open(fileobj=content, mode='w:gz') as archive:
            member = tarfile.TarInfo('cargo-nextest')
            member.type = tarfile.SYMTYPE
            member.linkname = str(victim)
            archive.addfile(member)
        data = content.getvalue()
        with patch.object(nextest, 'SHA256', hashlib.sha256(data).hexdigest()), \
                patch.object(nextest.urllib.request, 'urlopen', return_value=io.BytesIO(data)):
            with self.assertRaisesRegex(ValueError, 'member'):
                nextest.prepare(self.root)
        self.assertEqual(victim.read_bytes(), b'original')
