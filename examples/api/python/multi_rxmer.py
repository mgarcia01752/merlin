#!/usr/bin/env python3
#
# Copyright 2026 DOCSIS OFDM Channel Sim contributors
# SPDX-License-Identifier: Apache-2.0
#
# File: examples/api/python/multi_rxmer.py
"""Developer walkthrough: average a multi-RxMER collection and simulate its FEC.

Run from the repository root using an environment with the native wheel:
    local/api-python-test/bin/python examples/api/python/multi_rxmer.py

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
    RxMer, RxMerSelection, EvaluationOptions, ChannelSelection,
    ObservationDuration, AssociationTolerance, RunLimits, RuntimeLimit,
)


# Example configuration. Change these values here when adapting the workflow.
# These are caller choices, not settings inferred from the capture filenames.
REPOSITORY_PARENT_INDEX = 3  # examples/api/python/ -> repository root.
FIXTURE_DIRECTORY = "examples/workbench/multi-rxmer-2"
PROFILE_FILENAME = "ds_ofdm_modulation_profile_aabbccddeeff_193_1764824373.bin"
RXMER_FILE_PATTERN = "ds_ofdm_rxmer_per_subcar_*.bin"
OCCUPIED_BANDWIDTH_HZ = 192_000_000  # Explicit fixture configuration: 192 MHz.
OBSERVATION_DURATION_MS = 1  # Source observation, excluding acquisition/drain.
PROFILE_ASSOCIATION_TOLERANCE_SECONDS = 7_200  # Fixture capture-time allowance.
MAX_RUNTIME_MS = 300_000  # Per-experiment wall-clock limit: 300 seconds.


def main():
    """Run one collection-backed experiment and display Rust-produced outcomes.

    Returns:
        None after printing each experiment's status, FEC counters and BER.
        Input captures and returned report objects remain unchanged.
    Raises:
        FileNotFoundError if the configured folder contains no RxMER binaries.
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

    # 5. Load each original RxMER binary in the collection independently.
    # Files are sorted for reproducible insertion order only. Rust decodes their
    # real header timestamps and preserves original bytes/provenance; timestamps
    # are never inferred from filenames. No scientific calculations run here.
    # There is no matching channel estimate in this folder. Select synthetic
    # unity explicitly below rather than borrowing another channel's response.
    captures = sorted(root.glob(RXMER_FILE_PATTERN))
    if not captures:
        raise FileNotFoundError(f"No RxMER captures in {root}")
    for capture in captures:
        sim.add_rxmer(RxMer.from_file(capture))
    print(f"Loaded {len(captures)} RxMER captures; simulation has not started.")

    # 6. Choose policies and separate source time from host execution time.
    # - AverageAll requests ONE experiment using all captures. Rust averages
    #   residual power, preserves missing values and validates exact grids.
    #   It does not average dB or simulate the collection's hour of history.
    # - Unity is an explicit synthetic response, not a measured H. The desktop
    #   WB uses pilot-estimated receiver correction; that receiver setting and
    #   desktop WB JSON import are not exposed by the current high-level API.
    #   This example shares the original captures, not every desktop setting.
    # - OBSERVATION_DURATION_MS requests source observation in milliseconds,
    #   excluding acquisition/completion/drain. It is not wall-clock duration.
    # - PROFILE_ASSOCIATION_TOLERANCE_SECONDS permits this fixture profile and
    #   RxMER to differ in capture time. It does not rewrite their timestamps
    #   or set the independent nearest-channel tolerance. Choose deliberately
    #   for real inputs rather than copying this fixture-specific value.
    # - MAX_RUNTIME_MS bounds host execution per experiment in milliseconds.
    #   A limit can yield a partial report instead of completion.
    sim.set_options(EvaluationOptions(
        rxmer_selection=RxMerSelection.average_all(),
        channel_selection=ChannelSelection.unity(),
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
            print("BER:", report.profile.post_fec_ber.value
                  if report.profile.post_fec_ber is not None else None)


# Importing this example for inspection does not execute main or start a run.
if __name__ == "__main__":
    main()
