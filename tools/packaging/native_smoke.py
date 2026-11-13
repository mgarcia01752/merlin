#!/usr/bin/env python3
#
# Copyright 2026 DOCSIS OFDM Channel Sim contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Exercise the same native CLI binary on each host, without installing packages."""
import argparse
import json
import subprocess
import tempfile
from pathlib import Path
from common import ROOT, version


def smoke(binary):
    binary = binary.resolve(strict=True)
    def run(*arguments, success=True):
        result = subprocess.run([str(binary), *map(str, arguments)], capture_output=True,
                                text=True, timeout=120)
        if (result.returncode == 0) != success:
            raise RuntimeError(f"Unexpected CLI status {result.returncode}: {result.stderr}")
        return result
    if run("--version").stdout.strip() != f"ofdm-fec {version()}":
        raise ValueError("Native CLI version does not agree with VERSION")
    run("--help")
    run("--unknown-option", success=False)
    with tempfile.TemporaryDirectory(prefix="merlin-native-") as directory:
        output = Path(directory) / "result.json"
        arguments = ("simulate", "--input", ROOT / "examples/cli/awgn.json", "--output", output)
        run(*arguments)
        with output.open(encoding="utf-8") as source:
            json.load(source)
        original = output.read_bytes()
        run(*arguments, success=False)
        if output.read_bytes() != original:
            raise ValueError("Failed export changed the existing destination")
        run("simulate", "--input", Path(directory) / "missing.json", "--output",
            Path(directory) / "absent.json", success=False)
        if (Path(directory) / "absent.json").exists():
            raise ValueError("Rejected request published an output")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", type=Path, required=True)
    smoke(parser.parse_args().binary)
