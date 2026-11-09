#!/usr/bin/env python3
#
# Copyright 2026 DOCSIS OFDM Channel Sim contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Verify native artifact checksums and collect assets without silent name collisions."""
import argparse
import hashlib
import re
import shutil
from pathlib import Path
from common import checksums


def collect(source, destination, expected_artifacts=None):
    manifests=sorted(source.glob("*/SHA256SUMS"))
    if expected_artifacts is not None and {path.parent.name for path in manifests} != set(expected_artifacts):
        raise ValueError("Expected native platform artifacts are missing or unexpected")
    candidates = []
    names = set()
    for manifest in manifests:
        declared = set()
        for line in manifest.read_text(encoding="ascii").splitlines():
            digest, name = line.split("  ", 1)
            if not re.fullmatch(r"[A-Za-z0-9][A-Za-z0-9._-]*", name) or Path(name).name != name or name in names or name in declared:
                raise ValueError("Duplicate or unsafe native asset name")
            if not name.endswith((".deb", ".pkg", ".msi", ".tar.gz", ".zip")):
                raise ValueError("Unexpected native release asset type")
            path = manifest.parent / name
            if path.is_symlink() or not path.is_file() or hashlib.sha256(path.read_bytes()).hexdigest() != digest:
                raise ValueError(f"Native asset checksum mismatch: {name}")
            candidates.append(path)
            names.add(name)
            declared.add(name)
        if {path.name for path in manifest.parent.iterdir()} != declared | {"SHA256SUMS"}:
            raise ValueError("Native artifact has undeclared files")
    if not candidates:
        raise ValueError("No validated native assets")
    destination.mkdir(parents=True, exist_ok=False)
    try:
        for path in candidates:
            shutil.copy2(path, destination / path.name)
        checksums(destination)
    except BaseException:
        shutil.rmtree(destination)
        raise


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--source", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--expected-artifacts", nargs="+")
    args = parser.parse_args()
    collect(args.source, args.output, args.expected_artifacts)
