#
# Copyright 2026 DOCSIS OFDM Channel Sim contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Compare independently executed native Python calls with recorded Rust jobs."""
import json
import math
import os
from pathlib import Path
import unittest
import merlin as m

ROOT = Path(__file__).resolve().parents[3]


class RustPythonParity(unittest.TestCase):
    def test_every_static_selection_policy_matches_rust(self):
        directory = Path(os.environ["MERLIN_API_PARITY"])
        expected = json.loads((directory / "rust-results.json").read_text())
        fixture = ROOT / "examples/measurements/single-channel"
        dpd = m.DPD.from_file(fixture / "modulation_profile.json", m.ProfileId.PROFILE_A)
        ocd = m.OCD.from_profile(m.ProtocolScope.DOCSIS31, dpd,
                               m.OcdOverrides(bandwidth=m.BandwidthHz(192_000_000)))
        sim = m.Merlin(ocd)
        sim.set_profile(dpd)
        first_capture = m.RxMer.from_file(directory / "first.json")
        epoch = first_capture.provenance["original_epoch"]
        first = sim.add_rxmer(first_capture)
        sim.add_rxmer(m.RxMer.from_file(directory / "second.json"))
        channel = sim.add_channel_estimate(m.ChannelEstimate.from_file(fixture / "channel_estimation.json"))
        retained = []
        for name, policy in (
            ("each", m.RxMerSelection.each()),
            ("individual", m.RxMerSelection.individual(first)),
            ("average", m.RxMerSelection.average_all()),
            ("interpolate", m.RxMerSelection.interpolate_at(m.CaptureTime(epoch + 5))),
        ):
            sim.set_options(m.EvaluationOptions(
                rxmer_selection=policy,
                channel_selection=m.ChannelSelection.explicit(channel),
                observation=m.ObservationDuration(1),
                profile_tolerance=m.AssociationTolerance(0xffffffff),
                limits=m.RunLimits(runtime=m.RuntimeLimit(300_000))))
            batch = sim.run()
            retained.append((batch, batch.to_json()))
            actual = batch.to_dict()
            reference = expected[name]
            self.assertEqual(actual["status"], reference["status"])
            self.assertEqual(len(actual["entries"]), len(reference["entries"]))
            for received, original in zip(actual["entries"], reference["entries"]):
                self.assertEqual(received["status"], original["status"])
                report, baseline = received["report"], original["report"]
                self.assertEqual(report["fec"], baseline["fec"])
                self.assertEqual(report["all_completed_fec"], baseline["all_completed_fec"])
                for key in ("compared_bits", "bit_errors", "profile_id", "scope"):
                    self.assertEqual(report["profile"][key], baseline["profile"][key])
                for key, tolerance in (("post_fec_ber", 1e-12), ("payload_rate", 1e-6)):
                    left, right = report["profile"][key], baseline["profile"][key]
                    if left is None or right is None:
                        self.assertIs(left, right)
                    else:
                        self.assertTrue(math.isclose(left, right, rel_tol=0, abs_tol=tolerance))
                self.assertEqual(report["sources"]["epochs"], baseline["sources"]["epochs"])
                self.assertEqual(report["sources"]["weights"], baseline["sources"]["weights"])
                self.assertEqual(report["sources"]["target_epoch"], baseline["sources"]["target_epoch"])
                for left, right in zip(report["carrier_mer"], baseline["carrier_mer"]):
                    for key in ("subcarrier", "scope", "samples"):
                        self.assertEqual(left[key], right[key])
                    for key, tolerance in (("signal_power", 1e-12), ("error_power", 1e-12), ("mer", 1e-6)):
                        if left[key] is None or right[key] is None:
                            self.assertIs(left[key], right[key])
                        else:
                            self.assertTrue(math.isclose(left[key], right[key], rel_tol=0, abs_tol=tolerance))
                summary, old_summary = report["diagnostics"]["summary"], baseline["diagnostics"]["summary"]
                for key in ("payload_ldpc_failures", "payload_bch_failures", "payload_ldpc_iterations",
                            "bch_corrected_bits", "comparison"):
                    self.assertEqual(summary[key], old_summary[key])
            for entry in batch:
                if entry.report.profile is not None:
                    self.assertEqual(entry.report.fec.scope, entry.report.profile.scope)
                    self.assertGreater(entry.report.profile.compared_bits.value, 0)
        del sim
        for batch, original in retained:
            self.assertEqual(batch.to_json(), original)


if __name__ == "__main__":
    unittest.main()
