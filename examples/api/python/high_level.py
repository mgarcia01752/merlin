#!/usr/bin/env python3
#
# Copyright 2026 DOCSIS OFDM Channel Sim contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Run the bundled binary fixtures through MERLIN's native high-level Rust API."""
from pathlib import Path
from merlin import (
    DPD, OCD, Merlin, ProfileId, ProtocolScope, OcdOverrides, BandwidthHz,
    RxMer, ChannelEstimate, EvaluationOptions, ChannelSelection,
    ObservationDuration, AssociationTolerance, RunLimits, RuntimeLimit,
)


def main():
    """Load inputs individually, run explicitly, and print actual decoder counts.

    Returns None after displaying Rust-produced results. Raises native ApiError
    for invalid fixtures/settings; no input file is written or modified.
    """
    root = Path(__file__).resolve().parents[3] / "examples/workbench/single"
    profile = DPD.from_file(root / "modulation-profile.bin", ProfileId.PROFILE_A)
    ocd = OCD.from_profile(ProtocolScope.DOCSIS31, profile,
                           OcdOverrides(bandwidth=BandwidthHz(192_000_000)))
    sim = Merlin(ocd)
    sim.set_profile(profile)
    sim.add_rxmer(RxMer.from_file(root / "rxmer.bin"))
    channel = sim.add_channel_estimate(ChannelEstimate.from_file(root / "ds-channel-estimate.bin"))
    sim.set_options(EvaluationOptions(
        channel_selection=ChannelSelection.explicit(channel),
        observation=ObservationDuration(1),
        profile_tolerance=AssociationTolerance(90_000),
        limits=RunLimits(runtime=RuntimeLimit(300_000))))
    for entry in sim.run():
        print(entry.status)
        report = entry.report
        if report is not None and report.fec is not None:
            print("Total:", report.fec.total_codewords.value)
            print("Corrected:", report.fec.corrected_codewords.value)
            print("Uncorrectable:", report.fec.uncorrectable_codewords.value)
            print("BER:", report.profile.post_fec_ber.value
                  if report.profile.post_fec_ber is not None else None)


if __name__ == "__main__":
    main()
