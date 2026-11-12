#!/usr/bin/env python3
#
# Copyright 2026 DOCSIS OFDM Channel Sim contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Native Debian CLI package; build on the oldest supported Linux baseline."""
import shutil
import subprocess
from common import architecture, version
from asset_names import asset_name


def build(payload, output, scratch):
    """Build a canonical Ubuntu CLI DEB while preserving merlin-ofdm identity.

    payload is the staged CLI tree; output is an owned artifact directory;
    scratch is caller-owned temporary storage. Native packaging tools must be
    available on this host. Return None; propagate tool and filesystem failures.
    No package is installed or published.
    """
    root = scratch / "deb"
    shutil.copytree(payload, root / "usr")
    control = root / "DEBIAN"
    control.mkdir()
    arch = {"x64": "amd64", "arm64": "arm64"}[architecture()]
    (control / "control").write_text(
        f"Package: merlin-ofdm\nVersion: {version()}\nArchitecture: {arch}\n"
        "Section: science\nPriority: optional\n"
        "Maintainer: DOCSIS OFDM Channel Sim contributors\n"
        "Depends: libc6 (>= 2.35), libgcc-s1\n"
        "Description: MERLIN single-channel DOCSIS OFDM simulation CLI\n"
        " Rust OFDM, BCH/LDPC and measurement-driven channel experiments.\n",
        encoding="utf-8")
    subprocess.run(["dpkg-deb", "--root-owner-group", "--build", str(root),
                    str(output / asset_name("cli", version(), "ubuntu22.04", arch, "deb"))], check=True)
