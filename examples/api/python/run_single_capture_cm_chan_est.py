#!/usr/bin/env python3
#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
# File: examples/api/python/run_single_capture_cm_chan_est.py
"""Apply one CM channel estimate to the channel and use it for receiver correction.

The same captured response is selected once and reused by Rust on both sides.
This is supplied-reference correction, not blind pilot or MCE estimation.
RxMER still supplies residual noise; correction does not remove that impairment.
"""
from pathlib import Path
from merlin import (
    Simulation, ProfileId, EvaluationOptions, RxMerSelection, ChannelSelection,
    ReceiverMode, ObservationDuration, AssociationTolerance, RunLimits,
    RuntimeLimit, RunStatus,
)

REPOSITORY_PARENT_INDEX = 3
FIXTURE_DIRECTORY = "examples/workbench/single"
PROFILE_FILENAME = "modulation-profile.bin"
RXMER_FILENAME = "rxmer.bin"
CHANNEL_ESTIMATE_FILENAME = "ds-channel-estimate.bin"
OBSERVATION_DURATION_MS = 1
PROFILE_ASSOCIATION_TOLERANCE_SECONDS = 90_000
MAX_RUNTIME_MS = 300_000
BER_FORMAT = ".3e"


def main():
    """Run the bounded fixture and print independently owned Rust FEC results.

    Returns:
        None; input captures are read-only and output is printed to the terminal.
    Raises:
        ApiError for invalid files/settings; RuntimeError for incomplete results.
    """

    # Locate the shared examples independently of the caller's working directory.
    root = Path(__file__).resolve().parents[REPOSITORY_PARENT_INDEX] / FIXTURE_DIRECTORY

    # Create an idle simulation, load the profile collection and select Profile A.
    sim = Simulation()
    sim.load_profiles(root / PROFILE_FILENAME)
    sim.select_profile(ProfileId.PROFILE_A)

    # Load each measurement once; returned identifiers belong to this simulation.
    rxmer = sim.add_rxmer_file(root / RXMER_FILENAME)
    channel = sim.add_channel_estimate_file(root / CHANNEL_ESTIMATE_FILENAME)

    # Configure the channel impairment and receiver correction independently.
    sim.set_options(EvaluationOptions(
        rxmer_selection=RxMerSelection.individual(rxmer),

        # Apply the captured complex response to the simulated channel.
        channel_selection=ChannelSelection.explicit(channel),

        # Correct using that same supplied response; no MCE training is requested.
        receiver_mode=ReceiverMode.CM_CHANNEL_ESTIMATED,
        observation=ObservationDuration(OBSERVATION_DURATION_MS),

        # This fixture allowance does not rewrite the original capture timestamps.
        profile_tolerance=AssociationTolerance(PROFILE_ASSOCIATION_TOLERANCE_SECONDS),
        limits=RunLimits(runtime=RuntimeLimit(MAX_RUNTIME_MS)),
    ))

    # Only this call starts the Rust engine; the binding releases the interpreter lock.
    for entry in sim.run():
        report = entry.report
        if entry.status != RunStatus.Complete or report is None or report.fec is None:
            raise RuntimeError("Channel-estimate example did not produce complete FEC results")

        # Read observation-window counters, not whole-run acquisition/drain counters.
        print(entry.status)
        print("Total:", report.fec.total_codewords.value)
        print("Corrected:", report.fec.corrected_codewords.value)
        print("Uncorrectable:", report.fec.uncorrectable_codewords.value)
        print("BER:", format(report.profile.post_fec_ber.value, BER_FORMAT)
              if report.profile.post_fec_ber is not None else "unavailable")


if __name__ == "__main__":
    main()
