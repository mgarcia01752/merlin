#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Typed receiver options, fixture settings and measured UI FEC parity."""
from contextlib import redirect_stdout
import io
import json
from pathlib import Path
import re
import runpy
import sys
import unittest

import merlin as m
from test_api import configured

ROOT = Path(__file__).resolve().parents[3]
EXAMPLE = ROOT / "examples/api/python/run_multi_rxmer_mce.py"
UI_EXAMPLE = EXAMPLE.with_name("run_multi_rxmer_cm_chan_est.py")
# Both entry points share the sibling workflow, as when executed by filename.
sys.path.insert(0, str(EXAMPLE.parent))
WORKBENCH = ROOT / "examples/workbench/downstream/multi-rxmer-2/workbench.merlin.json"
# Observed native release UI Profile Performance results for this exact WB.
# These are software parity anchors, not modem interoperability vectors.
UI_FEC_REFERENCE = [(0, 83, 0, 0), (3, 114, 5, 0), (4, 124, 19, 0)]


class ReceiverModeTests(unittest.TestCase):
    def test_cpa_setters_getters_and_atomic_validation(self):
        """Python shares Rust limits and keeps unrelated session settings."""
        sim, _ = configured()
        sim.set_launch_amplifier_gain_db(12.0)
        sim.set_cpa_attenuation_db(6.0)
        self.assertEqual(sim.get_launch_amplifier_gain_db(), 12.0)
        self.assertEqual(sim.get_cpa_attenuation_db(), 6.0)
        self.assertEqual(sim.options.processing["power"]["attenuation_db"], 6.0)
        for invalid in (-1.0, 60.01, float("nan"), float("inf")):
            with self.assertRaises(m.ApiError) as error:
                sim.set_cpa_attenuation_db(invalid)
            self.assertEqual(error.exception.code, m.ErrorCode.InvalidInput)
            with self.assertRaises(m.ApiError):
                sim.set_launch_amplifier_gain_db(invalid)
            self.assertEqual(sim.get_cpa_attenuation_db(), 6.0)
            self.assertEqual(sim.get_launch_amplifier_gain_db(), 12.0)
        for valid in (0.0, 60.0):
            sim.set_cpa_attenuation_db(valid)
            sim.set_launch_amplifier_gain_db(valid)
            self.assertEqual(sim.get_cpa_attenuation_db(), valid)

    def test_receiver_options_are_typed_copied_and_reported(self):
        """Preserve the old default and independent request/report ownership."""
        self.assertEqual(m.EvaluationOptions().receiver_mode,
                         m.ReceiverMode.CM_CHANNEL_ESTIMATED)
        self.assertEqual(m.ReceiverMode.MERLIN_CHANNEL_ESTIMATED,
                         m.ReceiverMode.MerlinChannelEstimated)
        self.assertEqual(m.ReceiverMode.CM_CHANNEL_ESTIMATED,
                         m.ReceiverMode.SUPPLIED_CHANNEL)
        self.assertEqual(m.ReceiverMode.CmChannelEstimated,
                         m.ReceiverMode.SuppliedChannel)
        sim, _ = configured()
        for mode in (m.ReceiverMode.CM_CHANNEL_ESTIMATED,
                     m.ReceiverMode.PILOT_ESTIMATED,
                     m.ReceiverMode.MERLIN_CHANNEL_ESTIMATED):
            options = sim.options
            options.receiver_mode = mode
            sim.set_options(options)
            options.receiver_mode = m.ReceiverMode.CM_CHANNEL_ESTIMATED
            self.assertEqual(sim.options.receiver_mode, mode)
        with self.assertRaises(TypeError):
            m.EvaluationOptions(receiver_mode="pilot_estimated")
        with self.assertRaises(TypeError):
            options.receiver_mode = "pilot_estimated"
        options = sim.options
        options.receiver_mode = m.ReceiverMode.PILOT_ESTIMATED
        sim.set_options(options)
        batch = sim.run()
        report = next(iter(batch)).report
        self.assertEqual(report.options.receiver_mode, m.ReceiverMode.PILOT_ESTIMATED)
        self.assertEqual(batch.to_dict()["entries"][0]["report"]["options"]["receiver_mode"],
                         "pilot_estimated")
        options.receiver_mode = m.ReceiverMode.CM_CHANNEL_ESTIMATED
        sim.set_options(options)
        self.assertEqual(report.options.receiver_mode, m.ReceiverMode.PILOT_ESTIMATED)

    def test_collection_configuration_preserves_wb_settings_except_receiver(self):
        """Keep shared fixture settings while explicitly choosing MCE in Python."""
        example = runpy.run_path(str(EXAMPLE))  # Importing never runs main().
        workbench = json.loads(WORKBENCH.read_text())
        for constant, key in (("CYCLIC_PREFIX_SAMPLES", "cyclic_prefix_samples"),
                              ("ROLL_OFF_SAMPLES", "roll_off_samples"),
                              ("INTERLEAVER_DEPTH", "time_interleaver_depth")):
            self.assertEqual(example[constant], workbench["transmission"][key])
        for constant, key in (("OBSERVATION_DURATION_MS", "duration_ms"),
                              ("NOISE_SEED", "noise_seed"),
                              ("MAX_LDPC_ITERATIONS", "max_iterations"),
                              ("MAX_RUNTIME_MS", "max_runtime_ms")):
            self.assertEqual(example[constant], workbench["receiver"][key])
        self.assertEqual(workbench["receiver"]["receiver_mode"], "supplied_channel")
        self.assertEqual(example["RECEIVER_MODE"], m.ReceiverMode.MERLIN_CHANNEL_ESTIMATED)
        ui_example = runpy.run_path(str(UI_EXAMPLE))
        self.assertEqual(ui_example["RECEIVER_MODE"], m.ReceiverMode.CM_CHANNEL_ESTIMATED)
        self.assertEqual(workbench["sections"]["measurements"]["options"]["mode"], "average")
        self.assertEqual(example["PROFILE_ASSOCIATION_TOLERANCE_SECONDS"],
                         workbench["sections"]["measurements"]["options"]["tolerance_seconds"])
        self.assertIsNone(workbench["sections"]["measurements"]["response"])
        self.assertEqual(workbench["sections"]["source"]["options"]["mode"], "prbs15")
        self.assertEqual(example["PRBS_SEED"], workbench["sections"]["source"]["options"]["seed"])

    def test_collection_actual_fec_matches_native_ui_reference(self):
        """Run every discovered profile and compare real counters, not estimates."""
        example = runpy.run_path(str(UI_EXAMPLE))
        output = io.StringIO()
        with redirect_stdout(output):
            example["main"]()
        # Extract complete profile blocks from stdout, not whole-output validation.
        # Example Profile 3 with Total 114 yields four numeric groups (3,114,4,0);
        # newlines delimit fields and the literal scientific zero excludes nonzero BER.
        rows = re.findall(r"Profile (\d+): RunStatus.Complete\nTotal: (\d+)\n"
                          r"Corrected: (\d+)\nUncorrectable: (\d+)\nBER: 0.000e\+00", output.getvalue())
        self.assertEqual([tuple(map(int, row)) for row in rows], UI_FEC_REFERENCE)


if __name__ == "__main__":
    unittest.main()
