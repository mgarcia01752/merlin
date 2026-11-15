#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Offline installer contract checks; never install system packages."""
import hashlib
import importlib.util
import json
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

SPEC = importlib.util.spec_from_file_location("installer", Path(__file__).with_name("install.py"))
installer = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(installer)


class InstallerTests(unittest.TestCase):
    """Cover mutation guards, release integrity and explicit source dispatch."""

    def test_tag_validation(self):
        """Accept stable tags and reject arbitrary paths or moving references.

        No subprocesses or network calls are required.
        """
        self.assertEqual(installer.release_identity("0.3.1"), ("v0.3.1", "0.3.1"))
        for invalid in ("master", "../v0.3.1", "v01.2.3", "v1.2.3;true"):
            with self.assertRaises(ValueError):
                installer.release_identity(invalid)

    def test_unrelated_destination(self):
        """Protect a populated non-environment folder from modification.

        Temporary input remains intact after rejection.
        """
        with tempfile.TemporaryDirectory() as directory:
            target = Path(directory)
            (target / "keep").write_text("original")
            with self.assertRaises(ValueError):
                installer.check_destination(target)
            self.assertEqual((target / "keep").read_text(), "original")

    def test_check_has_no_install(self):
        """Dry runs must never download or elevate, even with the apt flag.

        Mock compatible host prerequisites and assert mutation boundaries.
        """
        with patch.object(installer, "host_platform", return_value="linux_x86_64"), \
             patch.object(installer, "check_destination", return_value=False), \
             patch.object(installer, "find_python", return_value=(["python"], {"venv": True, "ensurepip": True})), \
             patch.object(installer, "verified_wheel") as download, \
             patch.object(installer, "install_prerequisites") as apt:
            installer.main(["--tag", "v0.3.1", "--check", "--install-prerequisites"])
            download.assert_not_called()
            apt.assert_not_called()

    def test_verified_and_corrupt_wheels(self):
        """Accept exact published checksums and reject altered wheel bytes.

        Downloads are replaced by deterministic temporary fixtures.
        """
        name = "merlin_ofdm-0.3.1-cp310-cp310-linux_x86_64.whl"
        original = b"test wheel bytes"
        for payload in (original, b"corrupt"):
            def download(url, destination, limit):
                """Write the requested fixture without network access.

                The checksum always describes the original payload.
                """
                if url.endswith("v0.3.1"):
                    destination.write_text(json.dumps({"tag_name": "v0.3.1", "assets": [{"name": name}, {"name": "SHA256SUMS"}]}))
                elif url.endswith("SHA256SUMS"):
                    destination.write_text(hashlib.sha256(original).hexdigest() + "  " + name)
                else:
                    destination.write_bytes(payload)
            with tempfile.TemporaryDirectory() as directory, patch.object(installer, "download", side_effect=download):
                if payload == original:
                    self.assertEqual(installer.verified_wheel("v0.3.1", "0.3.1", "linux_x86_64", Path(directory)).read_bytes(), original)
                else:
                    with self.assertRaisesRegex(ValueError, "mismatch"):
                        installer.verified_wheel("v0.3.1", "0.3.1", "linux_x86_64", Path(directory))


if __name__ == "__main__":
    unittest.main()
