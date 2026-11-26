#!/usr/bin/env python3
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
"""Generate synthetic CMTS RxMER through native Rust; Python only performs I/O."""
import argparse
import os
from pathlib import Path

from merlin.upstream import SyntheticRxmerCapture

BINARY_NAME = "synthetic-cmts-rxmer.bin"
SIDECAR_NAME = "synthetic-cmts-rxmer.bin.merlin.json"


def main():
    """Write binary and provenance together in a new, exclusive directory.

    Returns:
        None after printing Rust-generated provenance; no simulation is started.
    Raises:
        Native ApiError for invalid requests; I/O errors for existing destinations
        or failed writes. Failures may leave a new incomplete directory and never
        overwrite input or an existing destination.
    """
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("request", type=Path)
    parser.add_argument("output", type=Path)
    arguments = parser.parse_args()
    with arguments.request.open("rb") as source:
        request = source.read(SyntheticRxmerCapture.MAX_REQUEST_BYTES + 1)
    if len(request) > SyntheticRxmerCapture.MAX_REQUEST_BYTES:
        raise ValueError("Synthetic RxMER request exceeds the input byte limit")
    generated = SyntheticRxmerCapture.from_json(request.decode("utf-8"))
    binary = generated.original_bytes()
    provenance = generated.to_json()

    # mkdir and exclusive file modes preserve existing destinations. The sidecar
    # is written last; do not treat a directory left by a failed write as complete.
    arguments.output.mkdir()
    with (arguments.output / BINARY_NAME).open("xb") as output:
        output.write(binary)
        output.flush()
        os.fsync(output.fileno())
    with (arguments.output / SIDECAR_NAME).open("x", encoding="utf-8") as output:
        output.write(provenance)
        output.flush()
        os.fsync(output.fileno())
    print(provenance)


if __name__ == "__main__":
    main()
