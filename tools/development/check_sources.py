#!/usr/bin/env python3
#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Check the Cargo-cached PNM source against the Qt manifest's exact Git pin."""
from pathlib import Path
import re
import subprocess

ROOT = Path(__file__).resolve().parents[2]


def expected_pin():
    """Read the controlled one-line pnm dependency declaration.

    Return repository URL, full revision and package version. Raise ValueError
    when the manifest does not declare all three explicit pins.
    """
    # Full dependency line: pnm = { git = "URL", rev = "HASH", version = "=0.3.6" }.
    # Group 1 contains the inline table; no substring match of another package.
    match = re.search(r'^pnm = (\{[^\n]+\})$',
                      (ROOT / "apps/qt/Cargo.toml").read_text(), re.MULTILINE)
    if not match:
        raise ValueError("Missing pinned pnm dependency")
    # Each quoted key/value, e.g. rev = "abc" -> ('rev', 'abc'); no escape syntax
    # is accepted because this is a controlled Git URL/revision/version declaration.
    values = dict(re.findall(r'(git|rev|version) = "([^"\n]+)"', match.group(1)))
    if set(values) != {"git", "rev", "version"} or not re.fullmatch(r'[0-9a-f]{40}', values["rev"]):
        raise ValueError("PNM source must have a full Git revision and exact version")
    if not values["version"].startswith("="):
        raise ValueError("PNM package version must be exact")
    return values["git"], values["rev"], values["version"][1:]


def cached_source():
    """Find the exact checked-out Git revision in MERLIN's Qt Cargo cache.

    Return the source root or None. This checks files and Git metadata only;
    it does not download sources, compile, create directories or modify Git state.
    """
    _, revision, version = expected_pin()
    for manifest in (ROOT / "local/merlin-cargo/git/checkouts").glob("*/*/crates/core/Cargo.toml"):
        checkout = manifest.parents[2]
        result = subprocess.run(["git", "-C", str(checkout), "rev-parse", "HEAD"],
                                text=True, capture_output=True, check=False)
        # Whole package-version line, e.g. version = "0.3.6" -> 0.3.6.
        recorded = re.search(r'^version = "([^"\n]+)"$', manifest.read_text(), re.MULTILINE)
        if result.returncode == 0 and result.stdout.strip() == revision and recorded and recorded.group(1) == version:
            return checkout
    return None


if __name__ == "__main__":
    location = cached_source()
    if location is None:
        print("Pinned PNM source is not cached; run ./install.sh --development")
        raise SystemExit(1)
    print(f"Pinned PNM source ready: {location}")
