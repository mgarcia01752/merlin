#!/usr/bin/env python3
#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
# File: examples/api/python/run_multi_rxmer_cmce_workbench.py
"""Run a saved MERLIN WB through its native Rust API, without Qt or the CLI.

Usage from the repository root after installing the native wheel:
    local/api-python-test/bin/python examples/api/python/run_multi_rxmer_cmce_workbench.py
    local/api-python-test/bin/python examples/api/python/run_multi_rxmer_cmce_workbench.py /path/to/workbench.merlin.json

The bundled multi-RxMER WB uses supplied CM channel estimation (CMCE).
A different input WB retains its own receiver mode; this filename does not force it.

Rust loads the WB's embedded original captures and saved scientific settings.
There are no duplicated profile, timing, receiver-mode or seed constants here.
Only presentation formatting and the default demonstration path live in Python.
Unsupported source/playback modes raise structured ApiError instead of silently
changing the simulation. See the user guide for the supported WB contract.
"""
import argparse
from pathlib import Path
from time import perf_counter
from merlin import Workbench


REPOSITORY_PARENT_INDEX = 3  # examples/api/python -> repository root.
DEFAULT_WORKBENCH = (Path(__file__).resolve().parents[REPOSITORY_PARENT_INDEX]
                     / "examples/workbench/downstream/multi-rxmer-2/workbench.merlin.json")
BER_FORMAT = ".3e"  # Four significant digits, including scientific zero.


def main():
    """Load a saved WB, then explicitly simulate it and print owned native results.

    Returns:
        None after printing settings, per-profile status, FEC, BER and host time.
    Raises:
        Native ApiError for input/integrity/capability/preflight failures.
        Partial engine outcomes remain visible in batch/entry status and reports.
    """
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("workbench", nargs="?", type=Path, default=DEFAULT_WORKBENCH,
                        help="Saved self-contained WB JSON; default: multi-rxmer-2")
    args = parser.parse_args()

    # 1. Loading reads embedded originals and validates all selected profiles in Rust.
    # It does not run simulation or need the capture files beside the saved WB.
    workbench = Workbench.from_file(args.workbench.expanduser())
    print("Loaded WB; simulation has not started:", args.workbench)
    ids = workbench.profile_ids
    for profile_id, options in zip(ids, workbench.options):
        print(f"Profile {profile_id.value}: receiver {options.receiver_mode}; "
              f"observation {options.observation.value} ms; "
              f"noise seed {options.noise_seed.value}")

    # 2. This explicit call starts the existing Rust engine and releases the GIL.
    # Use run_with_control(RunControl()) instead when cancellation is needed.
    started = perf_counter()
    batches = workbench.run()
    elapsed = perf_counter() - started

    # 3. Each profile owns a separate batch with controller-scoped IDs. The reports
    # survive disposal of the WB; unavailable counters are never replaced with zero.
    for profile_id, batch in zip(ids, batches):
        for entry in batch:
            print(f"Profile {profile_id.value}: {entry.status}")
            report = entry.report
            if report is None or report.fec is None:
                print("Observation-window FEC report unavailable.")
                continue
            print("Total:", report.fec.total_codewords.value)
            print("Corrected:", report.fec.corrected_codewords.value)
            print("Uncorrectable:", report.fec.uncorrectable_codewords.value)
            ber = report.profile.post_fec_ber
            print("BER:", format(ber.value, BER_FORMAT) if ber is not None else "unavailable")
    print(f"Total experiment wall time: {elapsed:.3f} seconds")


if __name__ == "__main__":
    main()
