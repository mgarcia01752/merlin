#!/usr/bin/env python3
#
# Copyright 2026 DOCSIS OFDM Channel Sim contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Owned package staging; no scientific calculations or dependency installation."""
import hashlib
import platform
import re
import shutil
import subprocess
from pathlib import Path
from notices import collect_notices

ROOT = Path(__file__).resolve().parents[2]


def version():
    value = (ROOT / "VERSION").read_text(encoding="utf-8").strip()
    if not re.fullmatch(r"(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)", value):
        raise ValueError("VERSION must contain three numeric components")
    return value


def architecture():
    machine = platform.machine().lower()
    if machine in ("x86_64", "amd64"):
        return "x64"
    if machine in ("aarch64", "arm64"):
        return "arm64"
    raise ValueError(f"Unsupported native architecture: {machine}")


def copy_tree(source, destination):
    """Reject symlinks before copying an explicit public-resource tree."""
    for path in [source, *source.rglob("*")]:
        if path.is_symlink():
            raise ValueError(f"Package resources cannot contain symlinks: {path}")
    shutil.copytree(source, destination)


def stage(binary, destination):
    expected = f"ofdm-fec {version()}"
    actual = subprocess.run([str(binary), "--version"], check=True,
                            capture_output=True, text=True).stdout.strip()
    if actual != expected:
        raise ValueError(f"Binary version mismatch: expected {expected!r}, got {actual!r}")
    executable = destination / "bin" / binary.name
    executable.parent.mkdir(parents=True)
    shutil.copy2(binary, executable)
    executable.chmod(0o755)
    resources = destination / "share" / "merlin"
    resources.mkdir(parents=True)
    for name in ("LICENSE", "NOTICE", "README.md", "VERSION"):
        shutil.copy2(ROOT / name, resources / name)
    # Preserve the adapted codec's complete MIT notice alongside Apache licensing.
    shutil.copy2(ROOT / "crates/fec-codec/LICENSE-MIT", resources / "fec-codec-LICENSE-MIT")
    qam_destination=resources / "crates/ofdm/resources/qam-table"
    qam_destination.mkdir(parents=True)
    for name in ("LICENSE", "NOTICE", "provenance.json"):
        shutil.copy2(ROOT / "crates/ofdm/resources/qam-table" / name, qam_destination / name)
    collect_notices(ROOT, resources / "third-party")
    for relative in ("docs/user", "docs/reference", "docs/images", "examples"):
        source = ROOT / relative
        if source.exists():
            target = resources / relative
            target.parent.mkdir(parents=True, exist_ok=True)
            copy_tree(source, target)
    shutil.copy2(ROOT / "docs/architecture.md", resources / "docs/architecture.md")
    for path in destination.rglob("*"):
        path.chmod(0o755 if path.is_dir() or path == executable else 0o644)
    return destination


def checksums(output):
    assets = sorted(path for path in output.iterdir() if path.is_file())
    contents = "".join(f"{hashlib.sha256(path.read_bytes()).hexdigest()}  {path.name}\n"
                       for path in assets)
    with (output / "SHA256SUMS").open("x", encoding="ascii", newline="\n") as destination:
        destination.write(contents)
