#!/usr/bin/env python3
#
# Copyright 2026 DOCSIS OFDM Channel Sim contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Package safety, no-clobber behavior and complete resource ownership."""
import tempfile
import subprocess
import unittest
import hashlib
import json
from pathlib import Path
from unittest.mock import patch
from common import checksums, copy_tree, stage, version
from package import package


class PackageTests(unittest.TestCase):
    def test_capture_checkout_preserves_bytes_with_windows_autocrlf(self):
        """Keep provenance and embedded originals exact under Windows Git settings."""
        source = Path(__file__).resolve().parents[2]
        relative = Path("examples/workbench/downstream/cl-192mhz-8mhz-notch/modulation_profile.json")
        original = (source / relative).read_bytes()
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            checkout = root / "checkout"
            checkout.mkdir()
            (checkout / ".gitattributes").write_bytes((source / ".gitattributes").read_bytes())
            (checkout / relative).parent.mkdir(parents=True)
            (checkout / relative).write_bytes(original)
            subprocess.run(["git", "init", "--quiet"], cwd=checkout, check=True)
            git = ["git", "-c", "core.autocrlf=true"]
            subprocess.run([*git, "add", ".gitattributes", relative.as_posix()],
                           cwd=checkout, check=True, capture_output=True)
            (checkout / relative).unlink()
            subprocess.run([*git, "checkout-index", "--force", relative.as_posix()],
                           cwd=checkout, check=True, capture_output=True)
            self.assertEqual((checkout / relative).read_bytes(), original)

    def test_existing_output_preserved(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            binary = root / "ofdm-fec"
            binary.write_text("placeholder", encoding="ascii")
            output = root / "existing"
            output.mkdir()
            sentinel = output / "keep"
            sentinel.write_text("unchanged", encoding="ascii")
            with self.assertRaises(FileExistsError):
                package(binary, output)
            self.assertEqual(sentinel.read_text(), "unchanged")

    def test_symlink_resources_reject(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            source = root / "source"
            source.mkdir()
            try:
                (source / "link").symlink_to(root)
            except OSError:
                self.skipTest("Host does not permit unprivileged symlinks")
            with self.assertRaises(ValueError):
                copy_tree(source, root / "destination")
            self.assertFalse((root / "destination").exists())

    def test_matching_binary_resources_licenses_and_checksums(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            binary = root / "ofdm-fec"
            binary.write_bytes(b"placeholder binary")
            with patch("common.subprocess.run") as run, patch("common.collect_notices"):
                run.return_value.stdout = f"ofdm-fec {version()}\n"
                payload = stage(binary, root / "payload")
            resources = payload / "share/merlin"
            for name in ("LICENSE", "NOTICE", "fec-codec-LICENSE-MIT", "examples/cli/awgn.json", "crates/ofdm/resources/qam-table/NOTICE",
                         "examples/cli/upstream/us-50khz.json", "examples/cli/upstream/us-25khz.json",
                         "examples/measurements/upstream/measured/capture-driven.json",
                         "docs/user/upstream/cli.md", "docs/user/python/upstream.md"):
                self.assertTrue((resources / name).is_file(), name)
            examples = resources / "examples/workbench"
            manifest = json.loads((examples / "manifest.json").read_text())
            bundle_ids = [bundle["id"] for bundle in manifest["bundles"]]
            self.assertCountEqual(bundle_ids, [
                "single", "multi-rxmer", "multi-rxmer-2", "cl-192mhz-8mhz-notch",
                "upstream-ideal-50khz", "upstream-ideal-25khz",
                "upstream-synthetic-rxmer-50khz", "upstream-synthetic-rxmer-25khz",
            ])
            advertised = [path for bundle in manifest['bundles']
                          for path in [bundle['workbench'], *bundle.get('companions', [])]]
            self.assertCountEqual(advertised, [str(path.relative_to(examples))
                                              for path in examples.rglob('*.merlin.json')])
            original_examples = Path(__file__).resolve().parents[2] / 'examples/workbench'
            for relative in advertised:
                self.assertEqual((examples / relative).read_bytes(),
                                 (original_examples / relative).read_bytes())
            for bundle in manifest["bundles"]:
                workbench = examples / bundle["workbench"]
                self.assertTrue(workbench.is_file())
                saved = json.loads(workbench.read_text())
                self.assertEqual(workbench.relative_to(examples).parts[0], bundle['direction'])
                if bundle['direction'] == 'downstream':
                    self.assertEqual(saved["receiver"]["receiver_mode"], "supplied_channel")
                else:
                    self.assertEqual(saved['direction'], 'upstream')

                    # Independent format anchor: direction-tagged saved projects use schema 3.
                    self.assertEqual(saved['schema_version'], 3)
                    self.assertEqual(saved['contents']['input']['document'].encode(),
                                     (examples / bundle['source_document']).read_bytes())
                self.assertTrue((workbench.parent / "LICENSE").is_file())
                self.assertTrue((workbench.parent / "NOTICE").is_file())
                provenance = json.loads((examples / bundle["provenance"]).read_text())
                for capture in provenance["files"]:
                    payload = (workbench.parent / capture["file"]).read_bytes()
                    self.assertEqual(len(payload), capture["bytes"])
                    self.assertEqual(hashlib.sha256(payload).hexdigest(), capture["sha256"])
            output = root / "assets"
            output.mkdir()
            (output / "example").write_bytes(b"abc")
            checksums(output)
            self.assertEqual((output / "SHA256SUMS").read_text(),
                             "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad  example\n")
            with self.assertRaises(FileExistsError):
                checksums(output)

    def test_wrong_binary_version_rejects(self):
        with tempfile.TemporaryDirectory() as directory:
            with patch("common.subprocess.run") as run:
                run.return_value.stdout = "ofdm-fec 999.0.0\n"
                with self.assertRaises(ValueError):
                    stage(Path(directory) / "ofdm-fec", Path(directory) / "payload")


class ReleaseCollectionTests(unittest.TestCase):
    def test_checksums_collection_tampering_and_existing_output(self):
        from collect_release import collect
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            native = root / "inputs/linux"
            native.mkdir(parents=True)
            (native / "merlin.tar.gz").write_bytes(b"test artifact")
            checksums(native)
            output = root / "release"
            collect(root / "inputs", output)
            self.assertEqual((output / "merlin.tar.gz").read_bytes(), b"test artifact")
            with self.assertRaises(FileExistsError):
                collect(root / "inputs", output)
            (native / "merlin.tar.gz").write_bytes(b"corrupted")
            with self.assertRaises(ValueError):
                collect(root / "inputs", root / "bad")
            self.assertFalse((root / "bad").exists())

    def test_release_requires_both_checksummed_wheel_artifacts(self):
        """Missing platforms and modified wheel bytes prevent release collection."""
        from collect_release import collect
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            expected = ['python-ubuntu', 'python-windows']
            linux = root / 'inputs/python-ubuntu'
            windows = root / 'inputs/python-windows'
            linux.mkdir(parents=True)
            name = 'merlin_ofdm-1.2.3-cp310-cp310-linux_x86_64.whl'
            (linux / name).write_bytes(b'wheel collection fixture')
            checksums(linux)
            with self.assertRaises(ValueError):
                collect(root / 'inputs', root / 'missing', expected)
            windows.mkdir()
            win_name = 'merlin_ofdm-1.2.3-cp310-cp310-win_amd64.whl'
            (windows / win_name).write_bytes(b'Windows collection fixture')
            checksums(windows)
            collect(root / 'inputs', root / 'valid', expected)
            self.assertTrue((root / 'valid' / win_name).is_file())
            (windows / win_name).write_bytes(b'tampered wheel')
            with self.assertRaises(ValueError):
                collect(root / 'inputs', root / 'tampered', expected)

    def test_release_links_use_exact_tag_and_repository(self):
        from release_notes import render
        contents = render("example/MERLIN", f"v{version()}")
        self.assertNotIn("@VERSION@", contents)
        self.assertIn(f"sudo apt install ./merlin-desktop_{version()}_ubuntu22.04_amd64.deb", contents)
        self.assertIn(f"merlin-cli_{version()}_linux_amd64.tar.gz", contents)
        self.assertIn("compatibility copy", contents)
        self.assertNotIn("merlin_0.1.0_", contents)
        self.assertNotIn("@TAG@", contents)
        self.assertNotIn("@REPOSITORY@", contents)
        self.assertIn("/docs/user/install/windows.md", contents)
        self.assertNotIn("/docs/user/install/macos.md", contents)
        self.assertIn(f"https://github.com/example/MERLIN/blob/v{version()}/docs/user/install/linux.md", contents)
        with self.assertRaises(ValueError):
            render("example/MERLIN", "main")
