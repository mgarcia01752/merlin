#!/usr/bin/env python3
#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
# File: examples/api/python/run_multi_rxmer_cm_chan_est.py
"""Run the collection with the saved multi-rxmer-2 UI workbench settings.

Run from the repository root after installing the native merlin-ofdm wheel:
    local/api-python-test/bin/python examples/api/python/run_multi_rxmer_cm_chan_est.py

The saved WB selects CmChannelEstimated. The sibling run_multi_rxmer_mce.py selects MCE
by default; this companion chooses supplied-channel correction while reusing
its complete documented workflow, captures, seeds, timing, limits and output.

Profiles are discovered through Rust and run independently. Rust handles all
calculations and FEC; no Qt process or CLI subprocess is needed. Importing does
not start simulation or alter captures. This matches the bundled saved WB,
not arbitrary settings edited later in the UI or a general WB JSON importer.
"""
from merlin import ReceiverMode
from run_multi_rxmer_mce import main as run_collection


# Match examples/workbench/downstream/multi-rxmer-2/workbench.merlin.json.
RECEIVER_MODE = ReceiverMode.CM_CHANNEL_ESTIMATED


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
