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
from time import perf_counter
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
PROFILE_IDS = (0, 3, 4)  # All profiles verified in this particular capture.
RXMER_FILE_PATTERN = "ds_ofdm_rxmer_per_subcar_*.bin"
OCCUPIED_BANDWIDTH_HZ = 192_000_000  # Explicit fixture configuration: 192 MHz.
OBSERVATION_DURATION_MS = 1  # Per profile; increase for a longer experiment.
PROFILE_ASSOCIATION_TOLERANCE_SECONDS = 7_200  # Fixture capture-time allowance.
MAX_RUNTIME_MS = 300_000  # Per-experiment wall-clock limit: 300 seconds.


def main():
    """Run independent collection-backed profiles and display Rust outcomes.

    Returns:
        None after printing each profile's status, FEC counters, BER and wall time.
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

    # 2. Import every selected captured profile before starting any simulation.
    # PROFILE_IDS lists this fixture's real captured IDs, not QAM orders or
    # wire IDs. Rust decodes and validates each; a missing selection is an error.
    # This example does not guess IDs or silently ignore failed imports.
    profiles = [DPD.from_file(root / PROFILE_FILENAME, ProfileId(profile_id))
                for profile_id in PROFILE_IDS]

    # 3. Load each original RxMER binary in the collection independently.
    # Files are sorted for reproducible insertion order only. Rust decodes their
    # real header timestamps and preserves original bytes/provenance; timestamps
    # are never inferred from filenames. No scientific calculations run here.
    # There is no matching channel estimate in this folder. Select synthetic
    # unity explicitly below rather than borrowing another channel's response.
    captures = sorted(root.glob(RXMER_FILE_PATTERN))
    if not captures:
        raise FileNotFoundError(f"No RxMER captures in {root}")
    observations = [RxMer.from_file(capture) for capture in captures]
    print(f"Loaded {len(captures)} RxMER captures; simulation has not started.")

    # 4. Choose policies and separate source time from host execution time.
    # - AverageAll requests one experiment per profile using all captures.
    #   Rust averages residual power, preserves missing values and validates exact grids.
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
    options = EvaluationOptions(
        rxmer_selection=RxMerSelection.average_all(),
        channel_selection=ChannelSelection.unity(),
        observation=ObservationDuration(OBSERVATION_DURATION_MS),
        profile_tolerance=AssociationTolerance(PROFILE_ASSOCIATION_TOLERANCE_SECONDS),
        limits=RunLimits(runtime=RuntimeLimit(MAX_RUNTIME_MS)))

    # 5. Create a fresh independent Rust controller for each profile.
    # Derive its OCD from that profile's grid and protected roles. Supply the
    # known fixture bandwidth explicitly; omitted timing retains documented
    # application defaults (CP 512, roll-off 128, interleaver depth 16).
    # Each controller restarts the common default PRBS/noise seeds and owns its
    # configuration. The immutable capture objects are reused without editing.
    # The API retains the captured ID; engine bootstrap uses wire Profile A.
    print(f"Profiles: {PROFILE_IDS}; observation: {OBSERVATION_DURATION_MS} ms "
          "per profile; RxMER: AverageAll; channel: unity")
    total_started = perf_counter()
    for profile in profiles:
        ocd = OCD.from_profile(
            ProtocolScope.DOCSIS31, profile,
            OcdOverrides(bandwidth=BandwidthHz(OCCUPIED_BANDWIDTH_HZ)),
        )
        sim = Merlin(ocd)
        sim.set_profile(profile)
        for observation in observations:
            sim.add_rxmer(observation)
        sim.set_options(options)

        # 6. Only run() starts the existing TX/channel/RX/FEC engine.
        # The binding releases Python's interpreter lock while Rust computes.
        # Timing measures host execution, not the simulated observation duration.
        print(f"Profile {profile.profile_id.value}: starting", flush=True)
        started = perf_counter()
        batch = sim.run()
        elapsed = perf_counter() - started
        for entry in batch:
            print(f"Profile {profile.profile_id.value}: {entry.status}")
            report = entry.report

            # 7. Print actual, observation-window FEC outcomes for this profile.
            # Completion does not guarantee FEC success. A not-started result
            # has no report; a partial run may lack an observation FEC window.
            # None is unavailable, never zero. Counters come from Rust decoder
            # outcomes; BER compares decoded payload with the original source.
            # Reports own their data independently of this controller's lifetime.
            if report is not None and report.fec is not None:
                print("Total:", report.fec.total_codewords.value)
                print("Corrected:", report.fec.corrected_codewords.value)
                print("Uncorrectable:", report.fec.uncorrectable_codewords.value)
                print("BER:", report.profile.post_fec_ber.value
                      if report.profile.post_fec_ber is not None else None)
            else:
                print("Observation-window FEC report unavailable.")
        print(f"Profile wall time: {elapsed:.3f} seconds")
    print(f"Total experiment wall time: {perf_counter() - total_started:.3f} seconds")


# Importing this example for inspection does not execute main or start a run.
if __name__ == "__main__":
    main()
