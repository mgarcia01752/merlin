#!/usr/bin/env python3
#
# Copyright 2026 DOCSIS OFDM Channel Sim contributors
# SPDX-License-Identifier: Apache-2.0
#
# File: examples/api/python/run_single_capture.py
"""Developer walkthrough: load captures, configure MERLIN, run, inspect results.

Run from the repository root using an environment with the native wheel:
    local/api-python-test/bin/python examples/api/python/run_single_capture.py

The distribution is named merlin-ofdm; its Python import is merlin. Importing
loads the compiled Rust extension into this process. Rust performs decoding,
scientific validation, simulation and FEC; this script supplies configuration
and prints results. It does not launch the CLI or require the desktop UI.

All input files are bundled examples. The script never modifies those files.
Loading/configuring inputs does not start simulation; sim.run() below does.
"""
from pathlib import Path
from merlin import (
    DPD, OCD, Merlin, ProfileId, ProtocolScope, OcdOverrides, BandwidthHz,
    RxMer, ChannelEstimate, EvaluationOptions, ChannelSelection,
    ObservationDuration, AssociationTolerance, RunLimits, RuntimeLimit,
)


# Example configuration. Change these values here when adapting the workflow.
# These are caller choices, not settings inferred from the capture filenames.
REPOSITORY_PARENT_INDEX = 3  # examples/api/python/ -> repository root.
FIXTURE_DIRECTORY = "examples/workbench/single"
PROFILE_FILENAME = "modulation-profile.bin"
RXMER_FILENAME = "rxmer.bin"
CHANNEL_ESTIMATE_FILENAME = "ds-channel-estimate.bin"
OCCUPIED_BANDWIDTH_HZ = 192_000_000  # Explicit fixture configuration: 192 MHz.
OBSERVATION_DURATION_MS = 1  # Source observation, excluding acquisition/drain.
PROFILE_ASSOCIATION_TOLERANCE_SECONDS = 90_000  # Fixture capture-time allowance.
MAX_RUNTIME_MS = 300_000  # Per-experiment wall-clock limit: 300 seconds.
BER_FORMAT = ".3e"  # Scientific notation with four significant digits.


def main():
    """Run one fixture-backed experiment and display Rust-produced outcomes.

    Returns:
        None after printing each experiment's status, FEC counters and BER.
        Input captures and returned report objects remain unchanged.
    Raises:
        Native ApiError for invalid input, configuration or preflight failures.
        Once an experiment starts, partial/failed outcomes are represented by
        its entry status and any available report, so inspect both.
    """
    # 1. Locate the shared capture fixtures relative to this source file.
    # REPOSITORY_PARENT_INDEX reaches the checkout from examples/api/python/. This
    # avoids depending on the caller's working directory when loading files.
    root = (
        Path(__file__).resolve().parents[REPOSITORY_PARENT_INDEX] / FIXTURE_DIRECTORY
    )

    # 2. Import the Downstream Profile Descriptor (DPD).
    # A CM capture may contain multiple profiles, so select PROFILE_A explicitly.
    # Rust detects binary/JSON from content, validates the selected profile and
    # preserves original capture bytes, carrier assignments and provenance.
    profile = DPD.from_file(root / PROFILE_FILENAME, ProfileId.PROFILE_A)

    # 3. Resolve the OFDM Channel Descriptor (OCD) configuration.
    # Reuse the profile's established grid and carrier roles. This fixture does
    # not establish occupied bandwidth, so supply OCCUPIED_BANDWIDTH_HZ in Hz.
    # BandwidthHz is a Rust-validated quantity, not an unlabelled integer.
    # Omitted timing settings use documented application defaults: CP 512
    # samples, roll-off 128 samples and interleaver depth 16. Reports identify
    # defaults as assumptions rather than measured modem configuration.
    ocd = OCD.from_profile(
        ProtocolScope.DOCSIS31, profile,
        OcdOverrides(bandwidth=BandwidthHz(OCCUPIED_BANDWIDTH_HZ)),
    )

    # 4. Create a controller and accept the selected profile.
    # This owns configuration for later runs; it does not run the engine yet.
    # The same Rust controller is available to native Rust consumers.
    sim = Merlin(ocd)
    sim.set_profile(profile)

    # 5. Add RxMER and channel response independently.
    # RxMER supplies measured receive MER by subcarrier, used by Rust's residual
    # error model. The channel estimate supplies the complex frequency response H.
    # They need not be captured simultaneously or added as matched pairs.
    # add_channel_estimate returns an opaque ID belonging to this controller;
    # the explicit policy below selects that response using the returned ID.
    sim.add_rxmer(RxMer.from_file(root / RXMER_FILENAME))
    channel = sim.add_channel_estimate(
        ChannelEstimate.from_file(root / CHANNEL_ESTIMATE_FILENAME)
    )

    # 6. Choose policies and separate source time from host execution time.
    # - Default RxMER selection is Each: one independent experiment per capture.
    #   There is one capture here, so this run requests one experiment. Other
    #   modes are Individual, AverageAll and InterpolateAt; see the API guide.
    # - Explicit channel selection reuses the chosen H, retaining its original
    #   timestamp/provenance. It does not perform nearest-time matching.
    # - OBSERVATION_DURATION_MS requests source observation in milliseconds,
    #   excluding acquisition/completion/drain. It is not wall-clock duration.
    # - PROFILE_ASSOCIATION_TOLERANCE_SECONDS permits this fixture profile and
    #   RxMER to differ in capture time. It does not rewrite their timestamps
    #   or set the independent nearest-channel tolerance. Choose deliberately
    #   for real inputs rather than copying this fixture-specific value.
    # - MAX_RUNTIME_MS bounds host execution per experiment in milliseconds.
    #   A limit can yield a partial report instead of completion.
    sim.set_options(EvaluationOptions(
        channel_selection=ChannelSelection.explicit(channel),
        observation=ObservationDuration(OBSERVATION_DURATION_MS),
        profile_tolerance=AssociationTolerance(PROFILE_ASSOCIATION_TOLERANCE_SECONDS),
        limits=RunLimits(runtime=RuntimeLimit(MAX_RUNTIME_MS))))

    # 7. Start actual simulation explicitly, then inspect every requested entry.
    # Rust preflights the complete batch and runs the existing TX/channel/RX/FEC
    # engine. The native binding releases the Python interpreter lock while
    # computing. This synchronous call returns an owned batch when it finishes.
    # Complete means execution completed; it does not guarantee FEC success.
    for entry in sim.run():
        print(entry.status)
        report = entry.report

        # 8. Read actual observation-window outcomes, not analytical estimates.
        # A not-started entry has no report; an early partial result may not yet
        # have a FEC window. FEC and profile summaries share the same window.
        # .value explicitly converts a read-only Rust quantity to a Python
        # number for display. Corrected counts come from actual decoded outcomes,
        # not from the number of decoder iterations. BER compares recovered
        # payload bits with the independent source; None means unavailable,
        # never an assumed zero. Reports also expose separately scoped whole-run
        # diagnostics and retain their data after the controller is changed.
        if report is not None and report.fec is not None:
            print("Total:", report.fec.total_codewords.value)
            print("Corrected:", report.fec.corrected_codewords.value)
            print("Uncorrectable:", report.fec.uncorrectable_codewords.value)
            print("BER:", format(report.profile.post_fec_ber.value, BER_FORMAT)
                  if report.profile.post_fec_ber is not None else "unavailable")


# Importing this example for inspection does not execute main or start a run.
if __name__ == "__main__":
    main()
