#!/usr/bin/env python3
#
# Copyright 2026 DOCSIS OFDM Channel Sim contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Run the same bounded upstream configuration through the installed native API."""
from pathlib import Path

from merlin.upstream import ExperimentOptions, Simulation

ROOT = Path(__file__).resolve().parents[3]


def main() -> None:
    """Load without work, then print actual Rust-derived execution outcomes.

    Raises:
        Native ApiError for invalid/unsupported input or execution failures;
        RuntimeError if the reference run does not recover complete source data.
    """
    simulation = Simulation(ExperimentOptions())
    simulation.configure_json((ROOT / "examples/cli/upstream/us-50khz.json").read_bytes())
    print("Profiles discovered by Rust:", simulation.preview().profile_ids)
    report = simulation.run()
    print("Status:", report.status)
    print("Compared source bits:", report.compared_source_bits)
    print("Bit errors:", report.bit_errors)
    if not report.reference_recovered:
        raise RuntimeError("reference experiment did not fully recover source")


if __name__ == "__main__":
    main()
