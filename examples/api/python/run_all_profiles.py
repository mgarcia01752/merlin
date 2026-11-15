#!/usr/bin/env python3
#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
# File: examples/api/python/run_all_profiles.py
"""Load one profile collection and run every retained profile explicitly."""
from pathlib import Path
from merlin import (
    Merlin, ProtocolScope, RxMer, EvaluationOptions, ChannelSelection,
    AssociationTolerance, ObservationDuration, RunLimits, RuntimeLimit,
)

REPOSITORY_PARENT_INDEX = 3
FIXTURE_DIRECTORY = "examples/workbench/single"
OBSERVATION_DURATION_MS = 1
PROFILE_TOLERANCE_SECONDS = 90_000
MAX_RUNTIME_MS = 300_000


def main():
    """Run all loaded profiles and print actual per-profile FEC outcomes."""
    root = Path(__file__).resolve().parents[REPOSITORY_PARENT_INDEX] / FIXTURE_DIRECTORY
    # Load once; the controller retains the profiles and resolves channel settings.
    sim = Merlin.from_profile_file(root / "modulation-profile.bin", ProtocolScope.DOCSIS31)
    print("Loaded profiles:", [profile_id.value for profile_id in sim.profile_ids])
    # Reuse one original RxMER capture for each profile in the collection.
    sim.add_rxmer(RxMer.from_file(root / "rxmer.bin"))
    sim.set_options(EvaluationOptions(
        channel_selection=ChannelSelection.unity(),
        observation=ObservationDuration(OBSERVATION_DURATION_MS),
        profile_tolerance=AssociationTolerance(PROFILE_TOLERANCE_SECONDS),
        limits=RunLimits(runtime=RuntimeLimit(MAX_RUNTIME_MS))))
    # Only this call starts simulation; each profile receives fresh engine state.
    for entry in sim.run_all_profiles():
        print("Profile:", entry.profile_id.value, "Status:", entry.status)
        report = entry.report
        if report is not None and report.fec is not None:
            print("Uncorrectable:", report.fec.uncorrectable_codewords.value)
            ber = report.profile.post_fec_ber if report.profile is not None else None
            print("BER:", f"{ber.value:.3e}" if ber is not None else "unavailable")


if __name__ == "__main__":
    main()
