#!/usr/bin/env python3
#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Release filename, updater compatibility and native package contracts."""
import hashlib
from pathlib import Path
import shutil
import subprocess
import tempfile
import unittest
from unittest.mock import patch

from asset_names import asset_name, desktop_alias, legacy_desktop_name
from common import checksums, version
import linux
import package as portable


class AssetNameTests(unittest.TestCase):
    """Canonical names preserve existing package identities and updater access."""

    def test_component_version_platform_architecture_order(self):
        """Every prepared format uses the same fields and canonical CPU names."""
        for role, target, arch, extension, expected in [
            ('desktop', 'ubuntu22.04', 'amd64', 'deb', 'merlin-desktop_0.1.5_ubuntu22.04_amd64.deb'),
            ('cli', 'ubuntu22.04', 'x64', 'deb', 'merlin-cli_0.1.5_ubuntu22.04_amd64.deb'),
            ('cli', 'linux', 'x64', 'tar.gz', 'merlin-cli_0.1.5_linux_amd64.tar.gz'),
            ('cli', 'windows', 'x64', 'msi', 'merlin-cli_0.1.5_windows_amd64.msi'),
            ('cli', 'windows', 'x64', 'zip', 'merlin-cli_0.1.5_windows_amd64.zip'),
            ('cli', 'macos', 'arm64', 'pkg', 'merlin-cli_0.1.5_macos_arm64.pkg'),
            ('cli', 'macos', 'x64', 'tar.gz', 'merlin-cli_0.1.5_macos_amd64.tar.gz'),
        ]:
            self.assertEqual(asset_name(role, '0.1.5', target, arch, extension), expected)
        for field, value in [(0, '../cli'), (1, '01.1.5'), (1, '0.1.5-rc1'),
                             (2, 'ubuntu24.04'), (3, 'x86'), (4, '../deb')]:
            arguments = ['cli', '0.1.5', 'ubuntu22.04', 'amd64', 'deb']
            arguments[field] = value
            with self.assertRaises(ValueError):
                asset_name(*arguments)

    def test_legacy_desktop_copy_is_identical_checksummed_and_never_clobbered(self):
        """Old updaters receive the exact canonical bytes through a distinct name."""
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            canonical = root / asset_name('desktop', '0.1.5', 'ubuntu22.04', 'amd64', 'deb')
            canonical.write_bytes(b'identical package bytes')
            alias = desktop_alias(canonical, '0.1.5')
            self.assertEqual(alias.name, 'merlin_0.1.5_ubuntu22.04_amd64.deb')
            self.assertEqual(alias.read_bytes(), canonical.read_bytes())
            self.assertNotEqual(alias.stat().st_ino, canonical.stat().st_ino)
            checksums(root)
            digest = hashlib.sha256(canonical.read_bytes()).hexdigest()
            self.assertEqual((root / 'SHA256SUMS').read_text().count(digest), 2)
            with self.assertRaises(FileExistsError):
                desktop_alias(canonical, '0.1.5')
            self.assertEqual(alias.read_bytes(), b'identical package bytes')
            self.assertEqual(legacy_desktop_name('0.1.5'), alias.name)

    def test_linux_portable_archive_uses_canonical_name_and_checksum(self):
        """Build a tiny archive and enforce both payload directory and asset name."""
        import tarfile
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            binary = root / 'ofdm-fec'
            binary.write_bytes(b'not executed')
            output = root / 'assets'
            def stage_fixture(source, destination):
                destination.mkdir()
                (destination / 'fixture').write_bytes(source.read_bytes())
                return destination
            with patch('package.platform.system', return_value='Linux'), \
                    patch('package.architecture', return_value='x64'), \
                    patch('package.stage', side_effect=stage_fixture):
                portable.package(binary, output)
            name = asset_name('cli', version(), 'linux', 'amd64', 'tar.gz')
            self.assertTrue((output / name).is_file())
            with tarfile.open(output / name) as archive:
                self.assertIn(name.removesuffix('.tar.gz') + '/fixture', archive.getnames())
            self.assertIn(name, (output / 'SHA256SUMS').read_text())

    @unittest.skipUnless(shutil.which('dpkg-deb'), 'Native Debian tools unavailable')
    def test_cli_deb_filename_changes_without_renaming_installed_package(self):
        """Build and inspect a tiny native DEB without installing or executing it."""
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            payload = root / 'payload'
            (payload / 'bin').mkdir(parents=True)
            (payload / 'bin/ofdm-fec').write_text('inert packaging fixture')
            output, scratch = root / 'output', root / 'scratch'
            output.mkdir()
            scratch.mkdir()
            with patch('linux.architecture', return_value='x64'):
                linux.build(payload, output, scratch)
            name = asset_name('cli', version(), 'ubuntu22.04', 'amd64', 'deb')
            self.assertEqual([path.name for path in output.iterdir()], [name])
            fields = subprocess.check_output(['dpkg-deb', '--field', str(output / name),
                                              'Package', 'Version', 'Architecture'], text=True)
            self.assertIn('Package: merlin-ofdm\n', fields)
            self.assertIn(f'Version: {version()}\n', fields)
            self.assertIn('Architecture: amd64\n', fields)
