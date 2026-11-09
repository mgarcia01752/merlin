#!/usr/bin/env python3
#
# Copyright 2026 DOCSIS OFDM Channel Sim contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Package safety, no-clobber behavior and complete resource ownership."""
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch
from common import checksums, copy_tree, stage, version
from package import package


class PackageTests(unittest.TestCase):
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
            for name in ("LICENSE", "NOTICE", "fec-codec-LICENSE-MIT", "examples/awgn.json", "crates/ofdm/resources/qam-table/NOTICE"):
                self.assertTrue((resources / name).is_file(), name)
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

    def test_release_links_use_exact_tag_and_repository(self):
        from release_notes import render
        contents = render("example/MERLIN", f"v{version()}")
        self.assertNotIn("@TAG@", contents)
        self.assertIn(f"https://github.com/example/MERLIN/blob/v{version()}/docs/user/install/windows.md", contents)
        with self.assertRaises(ValueError):
            render("example/MERLIN", "main")
