#!/usr/bin/env python3
#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
# File: examples/api/python/run_designed_rxmer.py
"""Run every compatible captured profile against the custom designed RxMER.

Run from the checkout after installing the native extension:
    local/api-python-test/bin/python examples/api/python/run_designed_rxmer.py

This is synthetic RxMER Designer output, not a modem measurement. Its adjacent
.bin.merlin.json sidecar preserves the recipe and export provenance. The native
loader reads the binary; it does not reapply the recipe or import that sidecar.
All decoding, validation, signal processing and FEC remain in Rust.
"""
from pathlib import Path
from time import perf_counter
from merlin import (
    DPD, OCD, Merlin, ProtocolScope, OcdOverrides, BandwidthHz,
    RxMer, RxMerSelection, EvaluationOptions, ChannelSelection, ReceiverMode,
    ObservationDuration, AssociationTolerance, RunLimits, RuntimeLimit,
    NoiseSeed, PrbsSeed, IterationLimit,
    CyclicPrefixSamples, RollOffSamples, InterleaverDepth,
)


REPOSITORY_PARENT_INDEX = 3
RXMER_PATH = "examples/measurements/designed-rxmer/merlin_rxmer_designed.bin"
PROFILE_PATH = "examples/workbench/single/modulation-profile.bin"
OCCUPIED_BANDWIDTH_HZ = 192_000_000
OBSERVATION_DURATION_MS = 1  # Source time, excluding acquisition/drain.
PROFILE_ASSOCIATION_TOLERANCE_SECONDS = 90_000  # Explicit fixture epoch allowance.
MAX_RUNTIME_MS = 300_000  # Independent host budget per profile.
RECEIVER_MODE = ReceiverMode.SUPPLIED_CHANNEL
NOISE_SEED = 41
PRBS_SEED = 0x7FFF
MAX_LDPC_ITERATIONS = 50
CYCLIC_PREFIX_SAMPLES = 512
ROLL_OFF_SAMPLES = 128
INTERLEAVER_DEPTH = 16
BER_FORMAT = ".3e"


def main():
    """Load the designed original, then explicitly run each captured profile.

    Returns:
        None after displaying actual status, FEC, BER and host execution time.
        Inputs and designer provenance remain unchanged.
    Raises:
        Native ApiError for input/configuration/preflight failures. Partial runs
        preserve their status and available reports; unavailable counters stay absent.
    """
    # 1. Resolve original paths relative to this script, without altering inputs.
    root = Path(__file__).resolve().parents[REPOSITORY_PARENT_INDEX]
    profile_path = root / PROFILE_PATH
    observation = RxMer.from_file(root / RXMER_PATH)

    # 2. Discover real profiles through Rust. The single bundle's grid matches
    # this designed capture; the multi-rxmer-2 channel has a different grid.
    # Discovery/loading does not execute simulation. Keep original epochs intact.
    profile_ids = DPD.profile_ids_from_file(profile_path)
    profiles = [DPD.from_file(profile_path, profile_id) for profile_id in profile_ids]
    print("Synthetic designed RxMER; not a modem measurement.")
    print("Loaded custom RxMER; simulation has not started.")
    print("Profiles:", [profile_id.value for profile_id in profile_ids])
    print("Receiver:", RECEIVER_MODE, "physical channel: unity")
    started = perf_counter()

    # 3. Give every profile an independent controller with identical seeds and
    # finite source/runtime bounds. Rust validates association and exact grids.
    for profile in profiles:
        ocd = OCD.from_profile(
            ProtocolScope.DOCSIS31, profile,
            OcdOverrides(bandwidth=BandwidthHz(OCCUPIED_BANDWIDTH_HZ),
                         cyclic_prefix=CyclicPrefixSamples(CYCLIC_PREFIX_SAMPLES),
                         roll_off=RollOffSamples(ROLL_OFF_SAMPLES),
                         interleaver_depth=InterleaverDepth(INTERLEAVER_DEPTH)),
        )
        sim = Merlin(ocd)
        sim.set_profile(profile)
        original_id = sim.add_rxmer(observation)
        # 4. Select this original explicitly. The designed tilt/ripple/notch are
        # already encoded in its RxMER values: do not apply the recipe again.
        # Unity means no additional complex channel impairment; supplied-channel
        # correction uses that flat reference. It does not remove RxMER noise.
        sim.set_options(EvaluationOptions(
            rxmer_selection=RxMerSelection.individual(original_id),
            channel_selection=ChannelSelection.unity(),
            receiver_mode=RECEIVER_MODE,
            observation=ObservationDuration(OBSERVATION_DURATION_MS),
            profile_tolerance=AssociationTolerance(PROFILE_ASSOCIATION_TOLERANCE_SECONDS),
            noise_seed=NoiseSeed(NOISE_SEED), prbs_seed=PrbsSeed(PRBS_SEED),
            limits=RunLimits(runtime=RuntimeLimit(MAX_RUNTIME_MS),
                             iterations=IterationLimit(MAX_LDPC_ITERATIONS))))

        # 5. Only this call starts simulation and releases the interpreter lock.
        # Reports own actual outcomes, independently of the controller's lifetime.
        for entry in sim.run():
            print(f"Profile {profile.profile_id.value}: {entry.status}")
            report = entry.report
            if report is None or report.fec is None:
                print("Observation-window FEC report unavailable.")
                continue
            print("Total:", report.fec.total_codewords.value)
            print("Corrected:", report.fec.corrected_codewords.value)
            print("Uncorrectable:", report.fec.uncorrectable_codewords.value)
            ber = report.profile.post_fec_ber
            print("BER:", format(ber.value, BER_FORMAT) if ber is not None else "unavailable")
    print(f"Total experiment wall time: {perf_counter() - started:.3f} seconds")


# Importing the walkthrough never starts a simulation.
if __name__ == "__main__":
    main()
