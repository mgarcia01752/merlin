#!/usr/bin/env python3
#
# Copyright 2026 DOCSIS OFDM Channel Sim contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Render release instruction links from the actual workflow repository/tag."""
import argparse
import re
from pathlib import Path
from common import ROOT, version


def render(repository, tag):
    if not re.fullmatch(r"[A-Za-z0-9_.-]+/[A-Za-z0-9_.-]+", repository) or tag != f"v{version()}":
        raise ValueError("Release links require an actual repository and matching VERSION tag")
    template = (ROOT / "docs/user/install/release-notes.md").read_text(encoding="utf-8")
    return template.replace("@REPOSITORY@", repository).replace("@TAG@", tag)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repository", required=True)
    parser.add_argument("--tag", required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    with args.output.open("x", encoding="utf-8") as output:
        output.write(render(args.repository, args.tag))
