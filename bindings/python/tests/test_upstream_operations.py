#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Targeted native conversions absent from the retained upstream source tests."""
import json
import unittest

import merlin.upstream as upstream
from test_upstream_experiment import configuration, us_configuration


class OperationConversions(unittest.TestCase):
    def test_owned_configuration_preview_and_actual_report_field_projections(self):
        """Inspect native copies and exact Rust-derived field/reference-plane data."""
        simulation = upstream.Simulation()
        self.assertIsNone(simulation.configuration())
        simulation.configure(configuration())
        accepted = simulation.configuration()
        preview = simulation.preview()
        self.assertEqual(preview.source_bits, json.loads(preview.to_json())['source_bits'])
        report = simulation.run()
        native = json.loads(report.to_json())
        self.assertEqual(report.generated_source_bits, native['generated_source_bits'])
        self.assertEqual(report.generated_source_bits, preview.source_bits)
        spectrum = report.spectrum('TxOutput', 50)
        snapshot = json.loads(spectrum.to_json())
        self.assertEqual(spectrum.first_sample, snapshot['first_sample'])
        self.assertEqual(spectrum.frequency_hz, snapshot['frequency_hz'])
        self.assertEqual(spectrum.frequency_hz[0], -51_200_000.0)
        self.assertEqual(spectrum.frequency_hz[1024], 0.0)
        del simulation
        another = upstream.Simulation()
        another.configure(accepted)
        self.assertEqual(another.preview().source_bits, preview.source_bits)

    def test_normalized_bank_generation_and_initialize_event_through_rust(self):
        """Retain phase-only normalization and distinguish initialization from multiplication."""
        config = us_configuration()
        raw = [(0.0, 0.0)] * 2 + [(0.0, 2.0)] * 2046
        bank = upstream.PreEqBank.normalized(config, raw, 2)
        self.assertEqual(bank.generation, 2)
        self.assertEqual(bank.coefficients()[:3], [(0.0, 0.0), (0.0, 0.0), (0.0, 1.0)])
        event = upstream.PreEqEvent.initialize(3, 6, 2, 10, [(128, (-1.0, 0.0))])
        playback = upstream.PreEqPlayback(config, 2, bank, [event], [upstream.ProbeReference(10, 2)])
        self.assertEqual(playback.coefficients(0)[128], (0.0, 1.0))
        self.assertEqual(playback.coefficients(6)[128], (-1.0, 0.0))
        self.assertEqual(playback.coefficients(6)[129], (0.0, 1.0))
        with self.assertRaises(upstream.ApiError) as invalid:
            upstream.PreEqBank.normalized(config, raw, 0)
        self.assertEqual(invalid.exception.code, upstream.ErrorCode.InvalidInput)
        self.assertEqual(bank.generation, 2)
