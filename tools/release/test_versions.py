#
# Copyright 2026 PNM Viewer contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Version propagation tests using isolated synthetic releases."""
from pathlib import Path
import shutil
import sys
import tempfile
import unittest

sys.path.insert(0, str(Path(__file__).parent))
import versions


class VersionPropagationTests(unittest.TestCase):
    """Verify all native manifest/lockfile consumers and unrelated dependency pins."""

    def test_two_versions_and_dependency_preservation(self):
        """Propagate two releases and reject deliberate metadata drift."""
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            for name in versions.MANIFESTS + versions.LOCKS:
                target = root / name
                target.parent.mkdir(parents=True, exist_ok=True)
                shutil.copy2(versions.ROOT / name, target)
            for version in ('1.2.3', '2.0.7'):
                versions.synchronize(root, version, write=True)
                self.assertEqual(versions.synchronize(root, version), [])
                self.assertEqual((root / 'VERSION').read_text().strip(), version)
                for name in versions.MANIFESTS:
                    self.assertEqual(versions.cargo_field((root / name).read_text(), 'version'), version)
                self.assertIn('=0.10.1', (root / 'crates/fec-codec/Cargo.toml').read_text())
            manifest = root / versions.MANIFESTS[0]
            manifest.write_text(manifest.read_text().replace('2.0.7', '2.0.6'))
            self.assertEqual(versions.synchronize(root, '2.0.7'), [versions.MANIFESTS[0]])

    def test_missing_input_does_not_write_version(self):
        """An incomplete checkout must fail before creating or changing VERSION."""
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            with self.assertRaises(FileNotFoundError):
                versions.synchronize(root, '1.2.3', write=True)
            self.assertFalse((root / 'VERSION').exists())


if __name__ == '__main__':
    unittest.main()
