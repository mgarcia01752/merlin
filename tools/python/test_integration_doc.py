#!/usr/bin/env python3
#
# Copyright 2026 DOCSIS OFDM Channel Sim contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Verify deterministic, versioned installation docs and filesystem boundaries."""
from pathlib import Path
import shutil
import tempfile
import unittest

from update_integration_doc import DOCUMENT, INPUTS, ROOT, render, update


class IntegrationDocumentTests(unittest.TestCase):
    """Exercise only disposable inputs; never invoke network or repository Git."""

    def setUp(self):
        """Copy the minimum production inputs into a disposable directory."""
        temporary = tempfile.TemporaryDirectory()
        self.addCleanup(temporary.cleanup)
        self.root = Path(temporary.name)
        for name in ('VERSION', 'bindings/python/Cargo.toml', *INPUTS):
            target = self.root / name
            target.parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(ROOT / name, target)

    def set_version(self, version):
        """Set a matching synthetic package and source version in the fixture."""
        previous = (self.root / 'VERSION').read_text().strip()
        (self.root / 'VERSION').write_text(version + '\n')
        manifest = self.root / 'bindings/python/Cargo.toml'
        manifest.write_text(manifest.read_text().replace(
            f'version = "{previous}"', f'version = "{version}"', 1))

    def test_check_is_read_only_and_writes_are_idempotent(self):
        """Stale checks never create/replace output; unchanged writes retain mtime."""
        target = self.root / DOCUMENT
        self.assertTrue(update(self.root))
        self.assertFalse(target.exists())
        self.assertTrue(update(self.root, write=True))
        stamp = target.stat().st_mtime_ns
        self.assertFalse(update(self.root, write=True))
        self.assertEqual(target.stat().st_mtime_ns, stamp)
        target.write_text('stale user edit\n')
        self.assertTrue(update(self.root))
        self.assertEqual(target.read_text(), 'stale user edit\n')

    def test_versions_refresh_every_install_pin_without_claiming_publication(self):
        """Patch/minor/major changes update pins and preserve honest asset status."""
        for version in ('1.2.3', '1.3.0', '2.0.0'):
            self.set_version(version)
            self.assertTrue(update(self.root, write=True))
            text = (self.root / DOCUMENT).read_text()
            self.assertIn(f'**v{version}**', text)
            self.assertIn(f'merlin-ofdm=={version}', text)
            self.assertIn(f'merlin_ofdm-{version}-cp310', text)
            self.assertIn('not a claim that the tag already exists', text)
            self.assertIn('<VERIFIED_WHEEL_SHA256>', text)
            self.assertNotIn('{{VERSION}}', text)
            self.assertEqual(text, render(self.root))

    def test_invalid_metadata_and_tokens_preserve_existing_document(self):
        """Reject drift and invalid templates before any output mutation."""
        update(self.root, write=True)
        original = (self.root / DOCUMENT).read_bytes()
        manifest = self.root / 'bindings/python/Cargo.toml'
        prior = manifest.read_text()
        manifest.write_text(prior.replace('version = "', 'version = "9', 1))
        with self.assertRaises(ValueError):
            update(self.root, write=True)
        self.assertEqual((self.root / DOCUMENT).read_bytes(), original)
        manifest.write_text(prior)
        template = self.root / INPUTS[0]
        template.write_text(template.read_text() + '{{UNKNOWN}}')
        with self.assertRaises(ValueError):
            update(self.root, write=True)
        self.assertEqual((self.root / DOCUMENT).read_bytes(), original)

    def test_redirected_output_and_inputs_are_rejected(self):
        """Never follow symlinks when reading generator inputs or replacing output."""
        update(self.root, write=True)
        original = (self.root / DOCUMENT).read_bytes()
        external = self.root / 'outside'
        external.write_text('preserve\n')
        for name in (DOCUMENT, INPUTS[0], 'bindings/python/Cargo.toml'):
            target = self.root / name
            previous = target.read_bytes()
            target.unlink()
            target.symlink_to(external)
            with self.assertRaises(ValueError):
                update(self.root, write=True)
            self.assertEqual(external.read_text(), 'preserve\n')
            target.unlink()
            target.write_bytes(previous)
        self.assertEqual((self.root / DOCUMENT).read_bytes(), original)


if __name__ == '__main__':
    unittest.main()
