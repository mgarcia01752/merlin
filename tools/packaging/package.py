#!/usr/bin/env python3
#
# Copyright 2026 DOCSIS OFDM Channel Sim contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Build native CLI deliverables from an already tested, matching-version binary."""
import argparse
import importlib
import platform
import shutil
import tempfile
from pathlib import Path
from common import ROOT, architecture, checksums, stage, version


def package(binary, output, native=False):
    if binary.is_symlink():
        raise ValueError("Package binary cannot be a symlink")
    binary = binary.resolve(strict=True)
    output = output.absolute()
    if output.exists():
        raise FileExistsError(f"Output directory already exists: {output}")
    system = platform.system().lower()
    expected_name = "ofdm-fec.exe" if system == "windows" else "ofdm-fec"
    if binary.name != expected_name or binary.is_symlink():
        raise ValueError(f"Expected a native {expected_name} executable")
    output.parent.mkdir(parents=True, exist_ok=True)
    # mkdir is exclusive: refuse racing/pre-existing destinations before publication.
    output.mkdir()
    try:
        with tempfile.TemporaryDirectory(prefix="merlin-package-", dir=output.parent) as directory:
            scratch = Path(directory)
            os_name = "macos" if system == "darwin" else system
            basename = f"merlin-{version()}-{os_name}-{architecture()}"
            payload = stage(binary, scratch / basename)
            archive = "zip" if system == "windows" else "gztar"
            shutil.make_archive(str(output / basename), archive, root_dir=scratch, base_dir=basename)
            if native:
                module = {"linux": "linux", "darwin": "macos", "windows": "windows"}.get(system)
                if module is None:
                    raise ValueError(f"Unsupported native installer platform: {system}")
                importlib.import_module(module).build(payload, output, scratch)
            checksums(output)
    except BaseException:
        shutil.rmtree(output)
        raise


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", type=Path, default=ROOT / "target/release/ofdm-fec")
    parser.add_argument("--output", type=Path, required=True, help="new directory; never replaced")
    parser.add_argument("--native", action="store_true", help="also build DEB, PKG or MSI on this host")
    args = parser.parse_args()
    package(args.binary, args.output, args.native)


if __name__ == "__main__":
    main()
