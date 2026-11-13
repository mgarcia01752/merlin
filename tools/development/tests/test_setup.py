#!/usr/bin/env python3
#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Check development setup without package installation or network access."""
import importlib.util
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import unittest
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[3]


class SetupTests(unittest.TestCase):
    """Exercise safe installer modes in an isolated checkout."""

    def setUp(self):
        """Create a disposable project under MERLIN's ignored test storage."""
        area = ROOT / "docs/development/local"
        area.mkdir(parents=True, exist_ok=True)
        self.workspace = tempfile.TemporaryDirectory(dir=area)
        self.addCleanup(self.workspace.cleanup)
        self.root = Path(self.workspace.name)
        for name in ("install.sh", "scripts/install-development.sh", "rust-toolchain.toml"):
            target = self.root / name
            target.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(ROOT / name, target)
        script = self.root / "scripts/install-development.sh"
        script.write_text(script.read_text().replace("source /etc/os-release", "ID=ubuntu; VERSION_ID=22.04; PRETTY_NAME='Test Ubuntu'"))
        self.bin = self.root / "fake-bin"
        self.bin.mkdir()
        self.command("uname", 'case "$1" in -s) echo Linux;; -m) echo x86_64;; esac')
        self.command("dpkg-query", "printf 'install ok installed'")
        self.command("python3", "exit 1")
        for command in ("sudo", "curl"):
            self.command(command, f'touch "{self.root}/forbidden"; exit 99')
        self.environment = dict(os.environ, PATH=f"{self.bin}:{os.environ['PATH']}")
        self.environment.pop("QT_SDK_DIR", None)

    def command(self, name, body):
        """Write one fake host command, isolated from production PATH."""
        path = self.bin / name
        path.write_text(f"#!/bin/bash\n{body}\n")
        path.chmod(0o755)

    def run_setup(self, *arguments):
        """Run setup outside its project and capture its status/output."""
        return subprocess.run(["bash", str(self.root / "install.sh"), *arguments],
                              cwd=ROOT, env=self.environment, text=True, capture_output=True)

    def test_safe_modes_and_argument_errors(self):
        """Help/plan/check never install, fetch or create local tool roots."""
        self.assertEqual(self.run_setup("--help").returncode, 0)
        for arguments in ((), ("--unknown",), ("--check",),
                          ("--development", "--check", "--dry-run")):
            self.assertEqual(self.run_setup(*arguments).returncode, 2)
        plan = self.run_setup("--development", "--dry-run")
        self.assertEqual(plan.returncode, 0, plan.stderr)
        self.assertIn("cargo fetch --locked", plan.stdout)
        self.assertEqual(self.run_setup("--development", "--check").returncode, 1)
        self.assertFalse((self.root / "local").exists())
        self.assertFalse((self.root / "forbidden").exists())

    def test_unsupported_platform_is_rejected_before_installation(self):
        """An unsupported CPU cannot reach package installation."""
        self.command("uname", 'case "$1" in -s) echo Linux;; -m) echo aarch64;; esac')
        self.assertEqual(self.run_setup("--development", "--dry-run").returncode, 2)
        self.assertFalse((self.root / "forbidden").exists())

    def test_qt_checks_require_dialog_and_test_modules(self):
        """An incomplete SDK or wrong version cannot be reported ready."""
        spec = importlib.util.spec_from_file_location("merlin_qt_sdk", ROOT / "tools/development/qt_sdk.py")
        module = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(module)
        sdk = self.root / "sdk"
        sdk.mkdir()
        with self.assertRaises(ValueError):
            module.check_sdk(sdk)
        for name in ("bin/qmake", "bin/qmltestrunner", "bin/qmlcachegen",
                     "qml/QtQuick/Controls/qmldir", "qml/QtQuick/Dialogs/qmldir"):
            path = sdk / name
            path.parent.mkdir(parents=True, exist_ok=True)
            path.touch()
        with patch.object(module.subprocess, "check_output", return_value="6.8.3\n"):
            module.check_sdk(sdk)
        with patch.object(module.subprocess, "check_output", return_value="6.9.0\n"):
            with self.assertRaises(ValueError):
                module.check_sdk(sdk)

    def test_windows_qt_sdk_checks_native_executables(self):
        """Windows SDK checks use .exe names and reject mismatched Qt versions."""
        spec = importlib.util.spec_from_file_location("merlin_windows_qt_sdk", ROOT / "tools/development/qt_sdk.py")
        module = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(module)
        sdk = self.root / "windows-sdk"
        for name in ("bin/qmake.exe", "bin/qmltestrunner.exe", "bin/qmlcachegen.exe",
                     "qml/QtQuick/Controls/qmldir", "qml/QtQuick/Dialogs/qmldir"):
            path = sdk / name
            path.parent.mkdir(parents=True, exist_ok=True)
            path.touch()
        with patch.object(module.platform, "system", return_value="Windows"), \
                patch.object(module.subprocess, "check_output", return_value="6.8.3\n") as probe:
            module.check_sdk(sdk)
            self.assertTrue(probe.call_args.args[0][0].endswith('qmake.exe'))



if __name__ == "__main__":
    unittest.main()
