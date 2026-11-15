#!/usr/bin/env python3
#
# Copyright 2026 DOCSIS OFDM Channel Sim contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Build a local native Linux/Windows wheel with complete locked dependency notices."""
import argparse
import hashlib
import os
import platform
from pathlib import Path
import shutil
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[2]
BINDING = ROOT / "bindings/python"
sys.path.insert(0, str(ROOT / "tools/packaging"))
from notices import collect_notices


def build_command(interpreter, binding, output, host):
    """Select native maturin flags without changing runtime/scientific behavior.

    Args:
        interpreter: Python used to build the matching ABI extension.
        binding: Binding Cargo manifest path.
        output: Generated wheel directory.
        host: sys.platform value; only Linux and Windows x64 are in this milestone.
    Returns:
        Argument vector for a locked optimized native wheel build.
    Raises:
        ValueError for an unsupported host platform.
    """
    if not (host.startswith("linux") or host == "win32"):
        raise ValueError("Native wheel packaging supports Linux and Windows only")
    compatibility = []
    if host.startswith("linux"):
        compatibility = ["--compatibility", "linux"]
    return [interpreter, "-m", "maturin", "build", "--manifest-path", str(binding),
            "--release", "--locked", *compatibility, "--interpreter", interpreter,
            "--out", str(output)]


def main():
    """Collect reproducible legal assets, then invoke pinned local maturin.

    Arguments come from argparse: --out selects generated wheel storage.
    Returns no scientific result and never installs or publishes anything.
    Raises subprocess/filesystem errors for invalid sources or failed builds.
    Only generated notice staging and wheel/checksum artifacts may be replaced.
    """
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--out", type=Path, default=ROOT / "local/python-wheels")
    args = parser.parse_args()
    if platform.machine().lower() not in ("x86_64", "amd64"):
        parser.error("Native wheel release targets are Linux/Windows x64")
    if not (sys.platform.startswith("linux") or sys.platform == "win32"):
        parser.error("Native wheel release targets are Linux/Windows x64")
    destination = BINDING / "target/wheel-notices"
    if any(parent.is_symlink() for parent in [BINDING, BINDING / "target", destination]):
        raise ValueError("Refuse redirected notice staging")
    if destination.exists():
        if not (destination / "manifest.json").is_file():
            raise ValueError("Refuse unrecognized notice staging directory")
        if any(path.is_symlink() for path in destination.rglob("*")):
            raise ValueError("Refuse redirected generated notices")
        shutil.rmtree(destination)
    collect_notices(ROOT, destination, BINDING / "Cargo.toml")
    for source, name in (
        ("LICENSE", "MERLIN-LICENSE"), ("NOTICE", "MERLIN-NOTICE"),
        ("crates/fec-codec/LICENSE-MIT", "fec-codec-LICENSE-MIT"),
        ("crates/ofdm/resources/qam-table/LICENSE", "qam-table-LICENSE"),
        ("crates/ofdm/resources/qam-table/NOTICE", "qam-table-NOTICE"),
    ):
        shutil.copyfile(ROOT / source, destination / name)
    environment = os.environ.copy()
    if (ROOT / "local/cargo/bin/cargo").is_file():
        environment.setdefault("CARGO_HOME", str(ROOT / "local/cargo"))
        environment.setdefault("RUSTUP_HOME", str(ROOT / "local/rustup"))
        environment["PATH"] = str(ROOT / "local/cargo/bin") + os.pathsep + environment.get("PATH", "")
    args.out.mkdir(parents=True, exist_ok=True)
    subprocess.run(build_command(sys.executable, BINDING / "Cargo.toml",
                                 args.out.resolve(), sys.platform),
                   cwd=ROOT, env=environment, check=True)
    for wheel in args.out.glob("merlin_ofdm-*.whl"):
        digest = hashlib.sha256(wheel.read_bytes()).hexdigest()
        wheel.with_suffix(wheel.suffix + ".sha256").write_text(
            f"{digest}  {wheel.name}\n", encoding="utf-8")


if __name__ == "__main__":
    main()
