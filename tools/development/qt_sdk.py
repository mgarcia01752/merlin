#!/usr/bin/env python3
#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Provision or check the project-local, pinned Ubuntu x64 Qt SDK."""
import argparse
import os
from pathlib import Path
import platform
import subprocess
import sys
import venv

ROOT = Path(__file__).resolve().parents[2]
QT_VERSION = "6.8.3"
AQT_VERSION = "3.3.0"


def sdk_path():
    """Resolve the explicit SDK override or the pinned project-local SDK.

    Return a Path without creating directories, downloading or changing state.
    """
    return Path(os.environ.get("QT_SDK_DIR", ROOT / "local/qt" / QT_VERSION / "gcc_64"))


def check_sdk(path):
    """Validate the SDK version and UI/test modules without downloads or writes.

    Args:
        path: Existing native Qt SDK directory.
    Returns:
        None when all required files exist and qmake reports the pinned version.
    Raises:
        ValueError or subprocess errors for missing/incompatible SDK components.
    """
    for relative in ("bin/qmake", "bin/qmltestrunner",
                     "qml/QtQuick/Controls/qmldir", "qml/QtQuick/Dialogs/qmldir"):
        if not (path / relative).is_file():
            raise ValueError(f"Qt SDK component missing: {path / relative}")
    if not any((path / location).is_file() for location in ("bin/qmlcachegen", "libexec/qmlcachegen")):
        raise ValueError(f"Qt QML compiler missing: {path}")
    environment = os.environ.copy()
    environment["LD_LIBRARY_PATH"] = str(path / "lib") + os.pathsep + environment.get("LD_LIBRARY_PATH", "")
    version = subprocess.check_output([str(path / "bin/qmake"), "-query", "QT_VERSION"],
                                      text=True, env=environment).strip()
    if version != QT_VERSION:
        raise ValueError(f"Expected Qt {QT_VERSION}, found {version}")


def install_sdk():
    """Install the pinned SDK and installer into ignored project-local storage.

    Reuse a complete explicit SDK read-only. An incomplete external override is
    rejected; installation never writes into another checkout or system SDK.
    Raise ValueError/subprocess errors on unsafe paths, download or SDK failures.
    """
    selected = sdk_path()
    try:
        check_sdk(selected)
        print(f"Qt SDK already ready: {selected}")
        return
    except (ValueError, OSError, subprocess.CalledProcessError):
        if "QT_SDK_DIR" in os.environ:
            raise ValueError("Explicit QT_SDK_DIR is incomplete; select a complete SDK or unset it for local setup")
    for path in (ROOT / "local", ROOT / "local/qt", ROOT / "local/qt-tools"):
        if not path.resolve().is_relative_to(ROOT.resolve()):
            raise ValueError(f"Refusing setup through an external path: {path}")
    tools = ROOT / "local/qt-tools"
    python = tools / "bin/python"
    if not python.is_file():
        venv.EnvBuilder(with_pip=True).create(tools)
    subprocess.run([str(python), "-m", "pip", "install", f"aqtinstall=={AQT_VERSION}"], check=True)
    subprocess.run([str(python), "-m", "aqt", "install-qt", "linux", "desktop", QT_VERSION,
                    "linux_gcc_64", "-O", str(ROOT / "local/qt"), "--archives",
                    "qtbase", "qtdeclarative", "qtshadertools", "qtsvg", "qttools", "icu"], check=True)
    check_sdk(selected)


def main():
    """Run a read-only check/plan or explicit local SDK installation.

    Return zero on success and one on a validation/download failure. Argument
    errors return argparse's status 2; no system packages are installed here.
    """
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("action", choices=("check", "install", "dry-run"))
    args = parser.parse_args()
    if platform.system() != "Linux" or platform.machine() != "x86_64":
        parser.error("Initial SDK setup supports Linux x86_64 only")
    try:
        if args.action == "check":
            check_sdk(sdk_path())
            print(f"Qt {QT_VERSION} ready: {sdk_path()}")
        elif args.action == "dry-run":
            print(f"Qt {QT_VERSION} gcc_64: {sdk_path()}; aqtinstall {AQT_VERSION} under local/qt-tools")
        else:
            install_sdk()
    except (ValueError, OSError, subprocess.CalledProcessError) as error:
        print(f"ERROR: {error}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
