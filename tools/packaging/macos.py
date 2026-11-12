#!/usr/bin/env python3
#
# Copyright 2026 DOCSIS OFDM Channel Sim contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Native macOS command-line package; no Qt application or DMG is implied."""
import subprocess
from common import architecture, version
from asset_names import asset_name


def build(payload, output, scratch):
    """Build a canonically named native CLI PKG with its existing installer ID.

    payload is the staged CLI tree; output is an owned artifact directory;
    scratch is caller-owned temporary storage. Native packaging tools must be
    available on this host. Return None; propagate tool and filesystem failures.
    No package is installed or published.
    """
    del scratch
    subprocess.run(["pkgbuild", "--root", str(payload), "--identifier",
                    "org.merlin.ofdm.cli", "--version", version(),
                    "--install-location", "/usr/local", "--ownership", "recommended",
                    str(output / asset_name("cli", version(), "macos", architecture(), "pkg"))], check=True)
