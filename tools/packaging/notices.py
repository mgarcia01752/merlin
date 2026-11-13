#!/usr/bin/env python3
#
# Copyright 2026 DOCSIS OFDM Channel Sim contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Collect complete Cargo dependency notices from pinned local source metadata."""
import hashlib
import json
import platform
import shutil
import subprocess
from pathlib import Path

SHARED_NOTICE_SOURCES = {"termination_attrib": ("termination", "0.1.2")}


def collect_notices(root, destination, manifest=None):
    """Collect full notices for the locked Cargo graph selected by manifest.

    Args:
        root: MERLIN checkout owning the Cargo helper.
        destination: Fresh notices directory.
        manifest: Optional desktop manifest; None selects the CLI workspace.
    Returns:
        None after copying licensed sources and writing their inventory.
    Raises:
        ValueError for missing or unsafe notices; Cargo/copy errors propagate.
    """
    cargo = ["cargo", "+1.85.1"] if platform.system() == "Windows" else ["bash", (root / "scripts/cargo-local.sh").as_posix()]
    information = subprocess.run([*cargo, "-vV"], cwd=root, check=True,
                                 capture_output=True, text=True).stdout
    host = next(line.removeprefix("host: ") for line in information.splitlines() if line.startswith("host: "))
    result = subprocess.run([*cargo, "metadata", "--locked", "--offline", "--format-version", "1",
                             "--filter-platform", host, *(["--manifest-path", str(manifest)] if manifest else [])], cwd=root, check=True,
                            capture_output=True, text=True)
    metadata = json.loads(result.stdout)
    included = {node["id"] for node in metadata["resolve"]["nodes"]}
    packages = [package for package in metadata["packages"] if package["id"] in included and package["source"]]
    destination.mkdir(parents=True)
    records = []
    for package in sorted(packages, key=lambda package: (package["name"], package["version"])):
        source_package = package
        directory = Path(package["manifest_path"]).parent
        source_directory = directory
        files = notice_files(directory)
        if not files and package["name"] in SHARED_NOTICE_SOURCES:
            # The proc-macro's crate archive omits its repository-wide MIT file.
            # Preserve the same repository/author's full notice from the pinned peer.
            name, release = SHARED_NOTICE_SOURCES[package["name"]]
            source_package = next(candidate for candidate in packages if candidate["name"] == name and candidate["version"] == release)
            if source_package["repository"] != package["repository"] or source_package["license"] != package["license"]:
                raise ValueError("Shared dependency notice source changed")
            source_directory = Path(source_package["manifest_path"]).parent
            files = notice_files(source_directory)
        reviewed = None
        if not files and package["version"] == "0.7.3" and package["name"] in {
            "cxx-qt", "cxx-qt-build", "cxx-qt-gen", "cxx-qt-lib", "cxx-qt-macro", "qt-build-utils"
        }:
            reviewed = "cxx-qt-0.7.3"
        if not files and (package["name"], package["version"]) == ("codespan-reporting", "0.11.1"):
            reviewed = "codespan-reporting-0.11.1"
        if reviewed:
            source_directory = root / "tools/packaging/resources" / reviewed
            origin = json.loads((source_directory / "provenance.json").read_text())
            vcs = json.loads((directory / ".cargo_vcs_info.json").read_text())
            if vcs["git"]["sha1"] != origin["commit"] or package["repository"].rstrip("/") != origin["repository"]:
                raise ValueError("Reviewed dependency license origin changed")
            for name, digest in origin["files"].items():
                actual = hashlib.sha256((source_directory / name).read_bytes()).hexdigest()
                if actual != digest:
                    raise ValueError(
                        f"Reviewed dependency license checksum changed: {reviewed}/{name} "
                        f"(expected {digest}, got {actual})")
            files = notice_files(source_directory) + [source_directory / "provenance.json"]
        if not files and (package["name"], package["version"]) == ("pnm", "0.3.6"):
            source_directory = directory.parents[1]
            files = notice_files(source_directory)
        if not files:
            raise ValueError(f"Dependency needs a reviewed full license notice: {package['name']}")
        target = destination / f"{package['name']}-{package['version']}"
        target.mkdir()
        for path in files:
            if path.is_symlink() or path.stat().st_size > 1024 * 1024:
                raise ValueError("Invalid dependency notice source")
            relative = path.relative_to(source_directory)
            copied = target / relative
            copied.parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(path, copied)
        if reviewed or (directory / "LICENSES").is_dir():
            authors = set(package.get("authors", []))
            for source in directory.rglob("*"):
                if source.is_file() and source.suffix in (".rs", ".h", ".cpp"):
                    with source.open("rb") as stream:
                        header = stream.read(8192).decode("utf-8", errors="replace")
                    for line in header.splitlines():
                        if "SPDX-FileCopyrightText:" in line:
                            authors.add(line.partition("SPDX-FileCopyrightText:")[2].strip())
            (target / "AUTHOR-NOTICES").write_text("\n".join(sorted(authors)) + "\n", encoding="utf-8")
        records.append({"name": package["name"], "version": package["version"],
                        "license": package["license"], "repository": package["repository"],
                        "notice_source": f"{source_package['name']}-{source_package['version']}",
                        "files": [str(path.relative_to(source_directory)) for path in files]})
    (destination / "manifest.json").write_text(json.dumps(records, indent=2) + "\n", encoding="utf-8")


def notice_files(directory):
    """Find full root notices and REUSE license texts with author attribution.

    Args:
        directory: One pinned Cargo source root.
    Returns:
        Sorted notice paths, preserving LICENSES subdirectory identity.
    Raises:
        ValueError for a redirected license directory.
    """
    selected = [path for path in directory.iterdir() if path.is_file() and
                path.name.upper().startswith(("LICENSE", "LICENCE", "COPYING", "NOTICE", "COPYRIGHT", "UNLICENSE"))]
    licenses = directory / "LICENSES"
    if licenses.is_symlink():
        raise ValueError("License directory cannot be a symlink")
    if licenses.is_dir():
        selected.extend(path for path in licenses.rglob("*") if path.is_file())
        # REUSE's generic texts keep their actual author notices in source/README.
        readme = directory / "README.md"
        if readme.is_file():
            selected.append(readme)
    return sorted(set(selected))
