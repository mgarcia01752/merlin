#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Actual channel planes, full-band calibration and separately derived amplifier observations."""
import inspect
import json
import math
import os
from pathlib import Path
import unittest

import merlin.downstream as m
from test_full_loop import inputs, options, request
from test_transmitter_preview import inputs as preview_inputs

AMPLIFIER_GAIN_DB = 20.0
CPA_ATTENUATION_DB = 6.0
INPUT_REFERENCE_DBMV = 0.0
EXPECTED_AMPLITUDE = 10.0
EXPECTED_POWER_RATIO = 100.0
VIEW_FIRST_BIN = 350
SPECTRUM_VIEW_BIN_COUNT = 7
PHYSICAL_VIEW_BIN_COUNT = 10
EXPECTED_FFT_SAMPLES = 8192
EXPECTED_DEFAULT_AVERAGES = 32
OVER_LIMIT_AVERAGES = 33
UNAVAILABLE_SNAPSHOT_INDEX = 999
OWNED_POWER_MUTATION_MARKER = 999
DB_ABSOLUTE_TOLERANCE = 1e-9
POWER_RELATIVE_TOLERANCE = 1e-10
PARITY_RELATIVE_TOLERANCE = 1e-10
PARITY_ABSOLUTE_TOLERANCE = 1e-10


def channel(gain=AMPLIFIER_GAIN_DB, cpa=CPA_ATTENUATION_DB):
    """Build immutable settings through Rust; all electrical calculations remain native."""
    return m.ChannelConfiguration.from_json(json.dumps({'power': {
        'amplifier_gain_db': gain, 'attenuation_db': cpa,
        'input_data_level_dbmv_per_6mhz': INPUT_REFERENCE_DBMV,
    }}).encode())


def observed():
    """Execute the same explicitly configured request as the separate Rust reference."""
    transmission, profile, source, measurements, _ = inputs()
    prepared = m.Simulation.prepare_full_loop(transmission, profile, source, measurements,
                                             channel(), options=options())
    return prepared.with_observations(m.FullLoopObservationPolicy()).run()


class ChannelSpectrumTests(unittest.TestCase):
    def assert_nested(self, actual, expected):
        if isinstance(expected, dict):
            self.assertEqual(actual.keys(), expected.keys())
            for key in expected:
                self.assert_nested(actual[key], expected[key])
        elif isinstance(expected, list):
            self.assertEqual(len(actual), len(expected))
            for actual_item, expected_item in zip(actual, expected):
                self.assert_nested(actual_item, expected_item)
        elif isinstance(expected, float):
            self.assertTrue(math.isclose(actual, expected,
                                        rel_tol=PARITY_RELATIVE_TOLERANCE,
                                        abs_tol=PARITY_ABSOLUTE_TOLERANCE), (actual, expected))
        else:
            self.assertEqual(actual, expected)

    def test_actual_tap_units_zoom_gain_missing_bounds_and_owned_results(self):
        for method in [m.FullLoopResult.calibrated_spectrum, m.FullLoopResult.channel_spectrum]:
            self.assertEqual(inspect.signature(method).parameters['max_averages'].default,
                             EXPECTED_DEFAULT_AVERAGES)
        disabled = request().run()
        with self.assertRaises(m.ApiError):
            disabled.channel_spectrum()
        with self.assertRaises(m.ApiError):
            disabled.amplifier_bins()
        result = observed()
        before_stats, before_spectrum = result.observation_stats(), result.spectrum()
        before = result.channel_spectrum(start=VIEW_FIRST_BIN, count=SPECTRUM_VIEW_BIN_COUNT)
        after = result.channel_spectrum('after_amplifier', start=VIEW_FIRST_BIN,
                                       count=SPECTRUM_VIEW_BIN_COUNT)
        actual = result.calibrated_spectrum(start=VIEW_FIRST_BIN, count=SPECTRUM_VIEW_BIN_COUNT)
        self.assertEqual(before['power']['bucket_units'], 'dbmv_per_hz')
        self.assertEqual(after['derived_amplitude'], EXPECTED_AMPLITUDE)
        self.assertEqual(before['first_sample'], after['first_sample'])
        self.assertEqual(before['context'], after['context'])
        self.assertEqual(before['power']['channel_power'], actual['power']['channel_power'])
        reading = before['power']['channel_power']
        self.assertEqual(reading, result.channel_spectrum()['power']['channel_power'])
        amplified = after['power']['channel_power']
        self.assertTrue(math.isclose(amplified['normalized_power'] / reading['normalized_power'],
                                    EXPECTED_POWER_RATIO, rel_tol=POWER_RELATIVE_TOLERANCE))
        self.assertAlmostEqual(amplified['dbmv'] - reading['dbmv'], AMPLIFIER_GAIN_DB,
                               delta=DB_ABSOLUTE_TOLERANCE)
        received = result.channel_spectrum('receiver_input')
        self.assertEqual(received, result.calibrated_spectrum('channel.output'))
        for boundary in ['before', 'after']:
            view = result.amplifier_bins(boundary, start=VIEW_FIRST_BIN, count=PHYSICAL_VIEW_BIN_COUNT)
            raw = result.physical_bins('tx.frame.bins', start=VIEW_FIRST_BIN, count=PHYSICAL_VIEW_BIN_COUNT)
            self.assertEqual(view['observation']['layout'], raw['layout'])
            amplitude = EXPECTED_AMPLITUDE if boundary == 'after' else 1.0
            for actual_bin, original_bin in zip(view['observation']['bins'], raw['bins']):
                expected_iq = None if original_bin['iq'] is None else [part * amplitude for part in original_bin['iq']]
                self.assertEqual(actual_bin['iq'], expected_iq)
        for operation in [lambda: result.channel_spectrum('unknown'),
                          lambda: result.channel_spectrum(max_averages=OVER_LIMIT_AVERAGES),
                          lambda: result.channel_spectrum(start=EXPECTED_FFT_SAMPLES),
                          lambda: result.channel_spectrum(snapshot=UNAVAILABLE_SNAPSHOT_INDEX),
                          lambda: result.amplifier_bins('receiver_input'),
                          lambda: result.calibrated_spectrum('rx.fft.bins')]:
            with self.assertRaises(m.ApiError):
                operation()
        before['power']['channel_power']['dbmv'] = OWNED_POWER_MUTATION_MARKER
        self.assertNotEqual(result.channel_spectrum()['power']['channel_power']['dbmv'],
                            OWNED_POWER_MUTATION_MARKER)
        self.assertEqual(result.observation_stats(), before_stats)
        self.assertEqual(result.spectrum(), before_spectrum)
        self.assertEqual(result.report()['summary']['comparison']['bit_errors'], 0)

    def test_idle_calibration_keeps_relative_mode_and_does_not_apply_configured_amplifier(self):
        result = m.TransmitterPreviewRequest(*preview_inputs()).generate()
        before = result.spectrum()
        full = result.calibrated_spectrum(channel())
        zoom = result.calibrated_spectrum(channel(), start=VIEW_FIRST_BIN, count=SPECTRUM_VIEW_BIN_COUNT)
        self.assertEqual(full['channel_power'], zoom['channel_power'])
        self.assertEqual(full, result.calibrated_spectrum(channel(gain=0.0, cpa=0.0)))
        relative = result.calibrated_spectrum(m.ChannelConfiguration())
        self.assertIsNone(relative['channel_power']['dbmv'])
        self.assertIsNone(relative['millivolts_squared_per_unit'])
        self.assertEqual(relative['buckets'], before['buckets'])
        self.assertEqual(result.spectrum(), before)
        with self.assertRaises(m.ApiError):
            result.calibrated_spectrum(channel(), start=EXPECTED_FFT_SAMPLES)

    @unittest.skipUnless(os.environ.get('MERLIN_FULL_LOOP_PARITY') and os.environ.get('MERLIN_TRANSMITTER_PARITY'),
                         'Direct Rust process references required')
    def test_direct_rust_process_channel_planes_and_idle_electrical_units_match(self):
        result = observed()
        reference = json.loads(Path(os.environ['MERLIN_FULL_LOOP_PARITY']).read_text())['channel_spectra']
        actual = {
            'before': result.channel_spectrum(start=VIEW_FIRST_BIN, count=SPECTRUM_VIEW_BIN_COUNT),
            'after': result.channel_spectrum('after_amplifier', start=VIEW_FIRST_BIN, count=SPECTRUM_VIEW_BIN_COUNT),
            'receiver': result.channel_spectrum('receiver_input', start=VIEW_FIRST_BIN, count=SPECTRUM_VIEW_BIN_COUNT),
            'calibrated': result.calibrated_spectrum(start=VIEW_FIRST_BIN, count=SPECTRUM_VIEW_BIN_COUNT),
            'bins_before': result.amplifier_bins('before', start=VIEW_FIRST_BIN, count=PHYSICAL_VIEW_BIN_COUNT),
            'bins_after': result.amplifier_bins('after', start=VIEW_FIRST_BIN, count=PHYSICAL_VIEW_BIN_COUNT),
        }
        self.assert_nested(actual, reference)
        preview = m.TransmitterPreviewRequest(*preview_inputs()).generate()
        expected = json.loads(Path(os.environ['MERLIN_TRANSMITTER_PARITY']).read_text())['calibrated']
        self.assert_nested(preview.calibrated_spectrum(channel(cpa=0.0), start=VIEW_FIRST_BIN,
                                                      count=SPECTRUM_VIEW_BIN_COUNT), expected)
