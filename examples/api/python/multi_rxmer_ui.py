#!/usr/bin/env python3
#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
# File: examples/api/python/multi_rxmer_ui.py
"""Run the collection with the saved multi-rxmer-2 UI workbench settings.

Run from the repository root after installing the native merlin-ofdm wheel:
    local/api-python-test/bin/python examples/api/python/multi_rxmer_ui.py

The saved WB selects PilotEstimated. The sibling multi_rxmer.py selects MCE
by default; this companion explicitly chooses pilot estimation while reusing
its complete documented workflow, captures, seeds, timing, limits and output.

Profiles are discovered through Rust and run independently. Rust handles all
calculations and FEC; no Qt process or CLI subprocess is needed. Importing does
not start simulation or alter captures. This matches the bundled saved WB,
not arbitrary settings edited later in the UI or a general WB JSON importer.
"""
from merlin import ReceiverMode
from multi_rxmer import main as run_collection


# Match examples/workbench/multi-rxmer-2/workbench.merlin.json.
RECEIVER_MODE = ReceiverMode.PILOT_ESTIMATED


def main():
    """Run the shared collection workflow with the saved UI receiver mode.

    Returns:
        None after displaying actual per-profile status, FEC, BER and wall time.
    Raises:
        FileNotFoundError or native ApiError as documented by run_collection.
        Partial/failed experiments remain visible through printed result status.
    """
    run_collection(receiver_mode=RECEIVER_MODE)


if __name__ == "__main__":
    main()
