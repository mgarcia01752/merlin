#
# Copyright 2026 PNM Viewer contributors
# SPDX-License-Identifier: Apache-2.0
#
import importlib.util
from pathlib import Path
import shutil
import subprocess
import tempfile
import sys
import unittest

SCRIPT = Path(__file__).with_name("bump-version.py")
sys.path.insert(0, str(SCRIPT.parent))
from versions import MANIFESTS, LOCKS, synchronize

SPEC = importlib.util.spec_from_file_location("bump_version", SCRIPT)
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)


class BumpVersionTests(unittest.TestCase):
    def test_reset_rules(self):
        """Check each bump resets only the lower components.

        Returns None; assertion failures report a behavior regression.
        """
        for part, expected in [("major", "2.0.0"), ("minor", "1.3.0"), ("patch", "1.2.10")]:
            with self.subTest(part=part):
                self.assertEqual(MODULE.bumped("1.2.9", part), expected)

    def test_invalid_versions(self):
        """Check malformed versions raise ValueError.

        Returns None; assertion failures report a behavior regression.
        """
        for value in ["1.2", "1.2.3.4", "01.2.3", "1.-2.3", "1.2.3-dev", "a.b.c", ""]:
            with self.subTest(value=value), self.assertRaises(ValueError):
                MODULE.bumped(value, "patch")

    def test_cli_from_other_directory(self):
        """Check preview, writes and failure preservation outside the repo.

        Returns None; assertion failures report a behavior regression.
        """
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            script = root / "tools/release/bump-version.py"
            script.parent.mkdir(parents=True)
            shutil.copy2(SCRIPT, script)
            shutil.copy2(SCRIPT.with_name("versions.py"), script.with_name("versions.py"))
            repository = SCRIPT.parents[2]
            for name in MANIFESTS + LOCKS:
                target = root / name
                target.parent.mkdir(parents=True, exist_ok=True)
                shutil.copy2(repository / name, target)
            # Keep this synthetic starting release independent of the checkout version.
            synchronize(root, "0.1.0", write=True)
            version = root / "VERSION"
            command = [sys.executable, str(script), "--bump", "minor"]
            before = {path: path.read_bytes() for path in root.rglob('*') if path.is_file()}
            preview = subprocess.run(command + ["--dry-run"], cwd=directory,
                                     check=True, capture_output=True, text=True)
            self.assertIn("0.1.0 -> 0.2.0 (dry run)", preview.stdout)
            for name in MANIFESTS + LOCKS:
                self.assertIn(name, preview.stdout)
            for path, content in before.items():
                self.assertEqual(path.read_bytes(), content)
            self.assertEqual(version.read_text(), "0.1.0\n")
            subprocess.run(command, cwd=directory, check=True, capture_output=True)
            self.assertEqual(version.read_text(), "0.2.0\n")
            version.write_text("1.2.3.4\n")
            result = subprocess.run(command, cwd=directory, capture_output=True)
            self.assertNotEqual(result.returncode, 0)
            self.assertEqual(version.read_text(), "1.2.3.4\n")

            # A preview must detect an incomplete checkout just as a write does.
            version.write_text("0.2.0\n")
            (root / MANIFESTS[0]).unlink()
            for arguments in (command, command + ["--dry-run"]):
                result = subprocess.run(arguments, cwd=directory, capture_output=True)
                self.assertNotEqual(result.returncode, 0)
                self.assertEqual(version.read_text(), "0.2.0\n")


if __name__ == "__main__":
    unittest.main()
