#!/usr/bin/env python3
#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Verify reviewed notice bytes across Git line-ending policies."""
import hashlib
import json
from pathlib import Path
import shutil
import subprocess
import tempfile
from types import SimpleNamespace
import unittest
from unittest.mock import patch

from notices import collect_notices

ROOT = Path(__file__).resolve().parents[2]


class NoticeCheckoutTests(unittest.TestCase):
    """Exercise actual Git checkout conversion and the notice collector."""

    def checkout(self, root, preserve):
        """Stage original resources and materialize a Windows-style checkout.

        Args:
            root: Fresh temporary repository directory.
            preserve: Include the project's byte-preserving attributes when true.
        Returns:
            Checked-out resource directory and locked dependency metadata.
        """
        resources = root / 'tools/packaging/resources'
        resources.parent.mkdir(parents=True)
        shutil.copytree(ROOT / 'tools/packaging/resources', resources)
        if preserve:
            shutil.copy2(ROOT / '.gitattributes', root / '.gitattributes')
        for arguments in (['init', '-q'], ['-c', 'core.autocrlf=false', 'add', '.']):
            subprocess.run(['git', '-C', str(root), *arguments], check=True,
                           capture_output=True)
        shutil.rmtree(resources)
        subprocess.run(['git', '-C', str(root), '-c', 'core.autocrlf=true',
                        'checkout-index', '--all', '--force'], check=True,
                       capture_output=True)
        packages = []
        for directory in sorted(resources.glob('*/provenance.json')):
            reviewed = directory.parent.name
            name, version = reviewed.rsplit('-', 1)
            origin = json.loads(directory.read_text())
            source = root / 'cache' / reviewed
            source.mkdir(parents=True)
            (source / '.cargo_vcs_info.json').write_text(
                json.dumps({'git': {'sha1': origin['commit']}}))
            packages.append({'id': reviewed, 'name': name, 'version': version,
                             'source': 'registry+test', 'manifest_path': str(source / 'Cargo.toml'),
                             'repository': origin['repository'], 'license': 'MIT OR Apache-2.0'})
        return resources, {'packages': packages,
                           'resolve': {'nodes': [{'id': item['id']} for item in packages]}}

    def collect(self, root, metadata, destination):
        """Run real notice validation/copying with bounded synthetic Cargo metadata.

        Args:
            root: Checkout containing the reviewed resources.
            metadata: Locked dependency graph pointing to temporary source roots.
            destination: Fresh output directory.
        Returns:
            None; collector failures propagate unchanged to the assertion.
        """
        responses = [SimpleNamespace(stdout='host: x86_64-pc-windows-msvc\n'),
                     SimpleNamespace(stdout=json.dumps(metadata))]
        with patch('notices.subprocess.run', side_effect=responses):
            collect_notices(root, destination)

    def test_windows_checkout_preserves_exact_reviewed_notices(self):
        """Pinned bytes survive autocrlf and full collection; unpinned bytes fail."""
        for preserve in (False, True):
            with self.subTest(preserve=preserve), tempfile.TemporaryDirectory() as area:
                root = Path(area)
                resources, metadata = self.checkout(root, preserve)
                destination = root / 'notices'
                if not preserve:
                    with self.assertRaisesRegex(ValueError, 'checksum changed:'):
                        self.collect(root, metadata, destination)
                    continue
                self.collect(root, metadata, destination)
                for provenance in resources.glob('*/provenance.json'):
                    for name, digest in json.loads(provenance.read_text())['files'].items():
                        copied = destination / provenance.parent.name / name
                        self.assertEqual(hashlib.sha256(copied.read_bytes()).hexdigest(), digest)

    def test_changed_license_still_fails_with_path_and_hashes(self):
        """Byte preservation never permits altered reviewed license content."""
        with tempfile.TemporaryDirectory() as area:
            root = Path(area)
            resources, metadata = self.checkout(root, True)
            license_path = resources / 'codespan-reporting-0.11.1/LICENSE'
            license_path.write_bytes(license_path.read_bytes() + b'altered\n')
            with self.assertRaisesRegex(ValueError,
                    r'checksum changed: codespan-reporting-0.11.1/LICENSE .*expected .*got '):
                self.collect(root, metadata, root / 'notices')


if __name__ == '__main__':
    unittest.main()
