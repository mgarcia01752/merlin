#!/usr/bin/env python3
#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Verify repository-owned decoder source without Git access or Cargo caches."""
import hashlib
import json
from pathlib import Path
import re

ROOT = Path(__file__).resolve().parents[2]


def local_source(root=ROOT):
    """Validate the complete imported source inventory and return its crate root.

    Args:
        root: MERLIN checkout containing the tracked third-party source closure.
    Returns:
        Local decoder crate directory after every recorded byte identity matches.
    Raises:
        ValueError for redirected paths, invalid provenance or changed source.
        OSError for missing source files; no downloads or writes occur.
    """
    root = root.resolve()
    source = root / "third-party/pnm"
    provenance = source / "provenance.json"
    if (source.is_symlink() or provenance.is_symlink()
            or not source.resolve().is_relative_to(root)):
        raise ValueError("PNM source must remain inside MERLIN")
    origin = json.loads(provenance.read_text(encoding="utf-8"))
    if not re.fullmatch(r"[0-9a-f]{40}", origin["revision"]) or not origin["files"]:
        raise ValueError("Invalid PNM source provenance")
    manifest = source / "crates/core/Cargo.toml"
    if manifest.is_symlink() or not manifest.resolve().is_relative_to(source.resolve()):
        raise ValueError("PNM manifest must remain inside its source root")
    version = re.search(r'^version = "([^"\n]+)"$', manifest.read_text(), re.MULTILINE)
    if version is None or version[1] != origin["version"]:
        raise ValueError("PNM package version differs from its recorded source")
    for record in origin["files"]:
        path = source / record["path"]
        if path.is_symlink() or not path.resolve().is_relative_to(source.resolve()):
            raise ValueError("PNM inventory path escapes its source root")
        if hashlib.sha256(path.read_bytes()).hexdigest() != record["sha256"]:
            raise ValueError(f"PNM source checksum changed: {record['path']}")
        if "upstream_copy" in record:
            original = source / record["upstream_copy"]
            if (not original.resolve().is_relative_to(source.resolve())
                    or hashlib.sha256(original.read_bytes()).hexdigest() != record["upstream_sha256"]):
                raise ValueError(f"PNM original documentation changed: {record['path']}")
    for name in ("apps/qt/Cargo.toml", "crates/pnm-input/Cargo.toml"):
        consumer = root / name
        dependency = re.search(r'^pnm = (\{[^\n]+\})$', consumer.read_text(), re.MULTILINE)
        fields = dict(re.findall(r'(path|version) = "([^"\n]+)"', dependency[1])) if dependency else {}
        if (set(fields) != {"path", "version"}
                or "git" in dependency[1]
                or (consumer.parent / fields["path"]).resolve() != manifest.parent.resolve()
                or fields["version"] != "=" + origin["version"]):
            raise ValueError(f"Consumer must use the local pinned PNM source: {name}")
    return manifest.parent


if __name__ == "__main__":
    try:
        location = local_source()
    except (OSError, ValueError, KeyError) as error:
        raise SystemExit(f"Local PNM source verification failed: {error}") from None
    print(f"Local PNM source ready: {location}")
