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
from asset_names import asset_name
from notices import notice_files
from desktop_payload import copy_examples


def scratch_directory():
    """Create isolated test storage, including its ignored fresh-checkout parent.

    Returns:
        TemporaryDirectory owned and cleaned by the caller's context manager.
    Raises:
        OSError if checkout-local scratch storage cannot be created.
    """
    area = ROOT / 'docs/development/local'
    area.mkdir(parents=True, exist_ok=True)
    return tempfile.TemporaryDirectory(dir=area)


class DesktopTests(unittest.TestCase):
    def test_desktop_examples_exclude_cli_and_keep_runnable_workbenches(self):
        """Desktop and CLI payloads must expose the correct example inventory."""
        with scratch_directory() as directory:
            destination = Path(directory) / 'examples'
            copy_examples(destination)
            self.assertEqual({path.name for path in destination.iterdir()},
                             {'README.md', 'workbench', 'measurements'})
            self.assertFalse((destination / 'cli').exists())
            for name in ('single', 'multi-rxmer', 'multi-rxmer-2'):
                self.assertTrue((destination / 'workbench' / name / 'workbench.merlin.json').is_file())
            self.assertTrue((destination / 'measurements/single-channel/rxmer.json').is_file())
            with self.assertRaises(FileExistsError):
                copy_examples(destination)

    def test_existing_output_is_never_replaced(self):
        with scratch_directory() as directory:
            output = Path(directory)
            sentinel = output / 'keep'
            sentinel.write_text('keep')
            with self.assertRaises(FileExistsError):
                package(Path('missing'), Path('missing'), output)
            self.assertEqual(sentinel.read_text(), 'keep')

    def test_workflow_and_update_metadata_share_baseline_asset_name(self):
        workflow = (ROOT / '.github/workflows/ci.yml').read_text()
        self.assertIn('runs-on: ubuntu-22.04', workflow)
        self.assertIn('desktop_smoke.py --installed', workflow)
        self.assertIn('sudo apt-get remove -y merlin', workflow)
        self.assertIn('env -u QT_SDK_DIR ./install.sh --development', workflow)
        source = (ROOT / 'apps/qt/src/platform/updates/release.rs').read_text()
        self.assertIn('ubuntu22.04_amd64.deb', source)
        self.assertIn('merlin-desktop_', source)
        self.assertEqual(asset_name('desktop', '0.1.5', 'ubuntu22.04', 'amd64', 'deb'),
                         'merlin-desktop_0.1.5_ubuntu22.04_amd64.deb')
        self.assertIn('desktop_alias(artifact, version())', (ROOT / 'tools/packaging/desktop.py').read_text())
        self.assertNotIn('dist/desktop/*.deb', workflow)
        self.assertIn('dist/desktop/merlin-desktop_*.deb', workflow)
        release = (ROOT / '.github/workflows/release.yml').read_text()
        self.assertIn('uses: ./.github/workflows/ci.yml', release)
        self.assertIn('release: true', release)
        self.assertIn('dependency-security:', workflow)
        self.assertIn('desktop:', workflow)
        self.assertIn('merlin-assets-desktop-ubuntu22-amd64', release)

    def test_install_guide_uses_generic_local_apt_path(self):
        """Avoid stale fixed versions and repository-style package lookups."""
        guide = (ROOT / 'docs/user/install/ubuntu-desktop.md').read_text()
        self.assertIn('sudo apt install ./PACKAGE.deb', guide)
        self.assertIn('exact', guide)
        self.assertIn('Unable to locate package', guide)
        self.assertNotIn('merlin_0.1.0_ubuntu22.04_amd64.deb', guide)



class LicenseTests(unittest.TestCase):
    def test_reuse_license_directory_keeps_text_and_attribution(self):
        with scratch_directory() as directory:
            root = Path(directory)
            (root / 'LICENSES').mkdir()
            (root / 'LICENSES/MIT.txt').write_text('Full MIT license text')
            (root / 'README.md').write_text('Copyright author')
            self.assertEqual({str(path.relative_to(root)) for path in notice_files(root)},
                             {'LICENSES/MIT.txt', 'README.md'})
