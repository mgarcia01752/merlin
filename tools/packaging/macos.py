#!/usr/bin/env python3
#
# Copyright 2026 DOCSIS OFDM Channel Sim contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Native macOS command-line package; no Qt application or DMG is implied."""
import subprocess
from common import architecture, version


def build(payload, output, scratch):
    del scratch
    subprocess.run(["pkgbuild", "--root", str(payload), "--identifier",
                    "org.merlin.ofdm.cli", "--version", version(),
                    "--install-location", "/usr/local", "--ownership", "recommended",
                    str(output / f"merlin-{version()}-macos-{architecture()}.pkg")], check=True)
