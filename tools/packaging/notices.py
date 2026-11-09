#!/usr/bin/env python3
#
# Copyright 2026 DOCSIS OFDM Channel Sim contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Collect complete Cargo dependency notices from pinned local source metadata."""
import json
import shutil
import subprocess
from pathlib import Path

SHARED_NOTICE_SOURCES = {"termination_attrib": ("termination", "0.1.2")}


def collect_notices(root, destination):
    cargo = ["bash", (root / "scripts/cargo-local.sh").as_posix()]
    information = subprocess.run([*cargo, "-vV"], cwd=root, check=True,
                                 capture_output=True, text=True).stdout
    host = next(line.removeprefix("host: ") for line in information.splitlines() if line.startswith("host: "))
    result = subprocess.run([*cargo, "metadata", "--locked", "--offline", "--format-version", "1",
                             "--filter-platform", host], cwd=root, check=True,
                            capture_output=True, text=True)
    metadata = json.loads(result.stdout)
    included = {node["id"] for node in metadata["resolve"]["nodes"]}
    packages = [package for package in metadata["packages"] if package["id"] in included and package["source"]]
    destination.mkdir(parents=True)
    records = []
    for package in sorted(packages, key=lambda package: (package["name"], package["version"])):
        source_package = package
        directory = Path(package["manifest_path"]).parent
        files = notice_files(directory)
        if not files and package["name"] in SHARED_NOTICE_SOURCES:
            # The proc-macro's crate archive omits its repository-wide MIT file.
            # Preserve the same repository/author's full notice from the pinned peer.
            name, release = SHARED_NOTICE_SOURCES[package["name"]]
            source_package = next(candidate for candidate in packages if candidate["name"] == name and candidate["version"] == release)
            if source_package["repository"] != package["repository"] or source_package["license"] != package["license"]:
                raise ValueError("Shared dependency notice source changed")
            files = notice_files(Path(source_package["manifest_path"]).parent)
        if not files:
            raise ValueError(f"Dependency needs a reviewed full license notice: {package['name']}")
        target = destination / f"{package['name']}-{package['version']}"
        target.mkdir()
        for path in files:
            if path.is_symlink() or path.stat().st_size > 1024 * 1024:
                raise ValueError("Invalid dependency notice source")
            shutil.copy2(path, target / path.name)
        records.append({"name": package["name"], "version": package["version"],
                        "license": package["license"], "repository": package["repository"],
                        "notice_source": f"{source_package['name']}-{source_package['version']}",
                        "files": [path.name for path in files]})
    (destination / "manifest.json").write_text(json.dumps(records, indent=2) + "\n", encoding="utf-8")


def notice_files(directory):
    return sorted(path for path in directory.iterdir() if path.is_file() and
                  path.name.upper().startswith(("LICENSE", "LICENCE", "COPYING", "NOTICE", "COPYRIGHT", "UNLICENSE")))
