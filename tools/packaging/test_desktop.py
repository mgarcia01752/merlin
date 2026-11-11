#!/usr/bin/env python3
#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Desktop package identity, no-clobber and workflow delivery contracts."""
import tempfile
from pathlib import Path
import unittest
from desktop import package
from common import ROOT
from notices import notice_files


class DesktopTests(unittest.TestCase):
    def test_existing_output_is_never_replaced(self):
        with tempfile.TemporaryDirectory(dir=ROOT / 'docs/development/local') as directory:
            output = Path(directory)
            sentinel = output / 'keep'
            sentinel.write_text('keep')
            with self.assertRaises(FileExistsError):
                package(Path('missing'), Path('missing'), output)
            self.assertEqual(sentinel.read_text(), 'keep')

    def test_workflow_and_update_metadata_share_baseline_asset_name(self):
        workflow = (ROOT / '.github/workflows/ubuntu-desktop.yml').read_text()
        self.assertIn('runs-on: ubuntu-22.04', workflow)
        self.assertIn('desktop_smoke.py --installed', workflow)
        self.assertIn('sudo apt-get remove -y merlin', workflow)
        source = (ROOT / 'apps/qt/src/platform/updates/release.rs').read_text()
        self.assertIn('ubuntu22.04_amd64.deb', source)
        self.assertIn('ubuntu22.04_amd64.deb', (ROOT / 'tools/packaging/desktop.py').read_text())
        release = (ROOT / '.github/workflows/release.yml').read_text()
        self.assertIn('dependency-security, desktop', release)
        self.assertIn('merlin-assets-desktop-ubuntu22-amd64', release)


class LicenseTests(unittest.TestCase):
    def test_reuse_license_directory_keeps_text_and_attribution(self):
        with tempfile.TemporaryDirectory(dir=ROOT / 'docs/development/local') as directory:
            root = Path(directory)
            (root / 'LICENSES').mkdir()
            (root / 'LICENSES/MIT.txt').write_text('Full MIT license text')
            (root / 'README.md').write_text('Copyright author')
            self.assertEqual({str(path.relative_to(root)) for path in notice_files(root)},
                             {'LICENSES/MIT.txt', 'README.md'})
