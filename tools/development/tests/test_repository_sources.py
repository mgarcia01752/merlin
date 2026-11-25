#!/usr/bin/env python3
#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Protect the self-contained source closure and immutable decoder evidence."""
import importlib.util
import json
import os
from pathlib import Path
import re
import shutil
import tempfile
import unittest
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[3]
SPEC = importlib.util.spec_from_file_location("merlin_sources", ROOT / "tools/development/check_sources.py")
SOURCES = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(SOURCES)


class RepositorySourceTests(unittest.TestCase):
    """Check real source identities and reject hidden external dependencies."""

    def test_source_verification_does_not_need_a_cargo_or_git_cache(self):
        """An empty cache cannot change whether tracked decoder bytes are ready."""
        with tempfile.TemporaryDirectory() as directory:
            with patch.dict(os.environ, CARGO_HOME=directory, GIT_ALLOW_PROTOCOL="file"):
                self.assertEqual(SOURCES.local_source(), ROOT / "third-party/pnm/crates/core")
            self.assertEqual(list(Path(directory).iterdir()), [])

    def test_all_local_cargo_graphs_reject_git_repository_dependencies(self):
        """Future changes cannot silently reintroduce private Git source access."""
        manifests = [ROOT / "Cargo.toml", ROOT / "apps/qt/Cargo.toml",
                     ROOT / "bindings/python/Cargo.toml", ROOT / "third-party/pnm/Cargo.toml"]
        manifests += list((ROOT / "crates").glob("*/Cargo.toml"))
        manifests += list((ROOT / "third-party/pnm").glob("**/Cargo.toml"))
        for manifest in manifests:
            with self.subTest(manifest=manifest.relative_to(ROOT)):
                text = manifest.read_text()
                self.assertNotRegex(text, r"\bgit\s*=")

                # Dependency and target-source paths must stay inside this checkout.
                for relative in re.findall(r'\bpath\s*=\s*"([^"\n]+)"', text):
                    self.assertFalse(Path(relative).is_absolute())
                    target = (manifest.parent / relative).resolve()
                    self.assertTrue(target.is_relative_to(ROOT), (manifest, relative))
                    self.assertTrue(target.exists(), (manifest, relative))
        for name in ("Cargo.lock", "apps/qt/Cargo.lock", "bindings/python/Cargo.lock",
                     "third-party/pnm/Cargo.lock"):
            with self.subTest(lockfile=name):
                self.assertNotIn('source = "git+', (ROOT / name).read_text())

    def test_changed_or_missing_decoder_source_fails_readiness(self):
        """A copied provenance record never legitimizes altered decoder code."""
        area = ROOT / "docs/development/local"
        area.mkdir(parents=True, exist_ok=True)
        with tempfile.TemporaryDirectory(dir=area) as directory:
            root = Path(directory)
            shutil.copytree(ROOT / "third-party/pnm", root / "third-party/pnm")
            for name in ("apps/qt/Cargo.toml", "crates/pnm-input/Cargo.toml"):
                target = root / name
                target.parent.mkdir(parents=True, exist_ok=True)
                shutil.copyfile(ROOT / name, target)
            origin = json.loads((root / "third-party/pnm/provenance.json").read_text())
            source = root / "third-party/pnm" / origin["files"][0]["path"]
            source.write_bytes(source.read_bytes() + b"changed\n")
            with self.assertRaisesRegex(ValueError, "checksum changed"):
                SOURCES.local_source(root)
            source.unlink()
            with self.assertRaises(OSError):
                SOURCES.local_source(root)


if __name__ == "__main__":
    unittest.main()
