#!/usr/bin/env python3
#
# Copyright 2026 DOCSIS OFDM Channel Sim contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Admit independent vector provenance/hashes; this does not execute a PHY comparison."""
import argparse
import hashlib
import json
import re
from pathlib import Path

ROLES = {"waveform_samples", "transmission_settings", "expected_payload", "expected_control"}
MAX_MANIFEST_BYTES = 1024 * 1024
MAX_VECTOR_FILE_BYTES = 64 * 1024 * 1024


def admit(path):
    path = path.resolve(strict=True)
    if path.stat().st_size > MAX_MANIFEST_BYTES:
        raise ValueError("External vector manifest exceeds1MiB")
    document = json.loads(path.read_bytes())
    if set(document) != {"schema_version", "producer", "producer_version", "reference", "files"}:
        raise ValueError("Unexpected external vector manifest fields")
    if type(document["schema_version"]) is not int or document["schema_version"] != 1:
        raise ValueError("Unsupported external vector manifest revision")
    for field in ("producer", "producer_version", "reference"):
        if not isinstance(document[field], str) or not 1 <= len(document[field]) <= 4096:
            raise ValueError(f"Missing/bounded external provenance: {field}")
    if not isinstance(document["files"], list) or len(document["files"]) != len(ROLES):
        raise ValueError("Exactly four independently produced vector files are required")
    roles = set()
    names = set()
    for record in document["files"]:
        if set(record) != {"role", "path", "sha256", "bytes"} or record["role"] not in ROLES or record["role"] in roles:
            raise ValueError("Missing, duplicate or unexpected vector role")
        if not isinstance(record["path"], str) or not record["path"] or "\\" in record["path"]:
            raise ValueError("Vector paths must be portable relative POSIX paths")
        relative = Path(record["path"])
        if relative.is_absolute() or ".." in relative.parts or relative in names:
            raise ValueError("Unsafe/duplicate vector path")
        source = path.parent / relative
        if any(parent.is_symlink() for parent in [source, *source.parents] if parent != path.parent):
            raise ValueError("Vector paths cannot use symlinks")
        source.resolve(strict=True).relative_to(path.parent)
        if not source.is_file() or type(record["bytes"]) is not int or not 0 < record["bytes"] <= MAX_VECTOR_FILE_BYTES or source.stat().st_size != record["bytes"]:
            raise ValueError("Vector size/type mismatch or resource limit")
        if not isinstance(record["sha256"], str) or not re.fullmatch("[0-9a-f]{64}", record["sha256"]):
            raise ValueError("Invalid vector SHA256")
        digest = hashlib.sha256()
        with source.open("rb") as stream:
            for chunk in iter(lambda: stream.read(1024 * 1024), b""):
                digest.update(chunk)
        if digest.hexdigest() != record["sha256"]:
            raise ValueError("External vector checksum mismatch")
        roles.add(record["role"])
        names.add(relative)
    return {"schema_version": 1, "status": "admitted_unexecuted", "provenance": document,
            "notice": "Hashes and declarations checked only; independent origin and PHY interoperability remain unverified."}


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--manifest", type=Path, required=True)
    parser.add_argument("--report", type=Path, required=True)
    args = parser.parse_args()
    report = admit(args.manifest)
    with args.report.open("x", encoding="utf-8") as output:
        json.dump(report, output, indent=2)
        output.write("\n")
