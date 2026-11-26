#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Ordered frame-stage PSD preserves original time rather than claiming a continuous waveform."""
import inspect
import json
import math
import os
from pathlib import Path
import unittest

import merlin.downstream as m
from test_full_loop import request, inputs, options

STAGES = ['tx.ifft.samples', 'tx.cp.samples', 'tx.window.samples', 'rx.cp.samples']

# Distinct fixture policies; numerical equality does not couple their purposes.
RETAINED_FRAMES_PER_STAGE = 3
WELCH_AVERAGES_PER_QUERY = 3
EXPECTED_FFT_SAMPLES = 8192
EXPECTED_TWO_FRAME_SAMPLES = 16384
FIRST_VIEW_BIN = 123
VIEW_BIN_COUNT = 7

# Independent accepted public-signature anchor; not imported from the implementation.
EXPECTED_DEFAULT_AVERAGES = 32
OVER_LIMIT_AVERAGES = 33
UNAVAILABLE_SNAPSHOT_INDEX = 999
OVERSIZED_STAGE_IDENTIFIER_BYTES = 129
OWNED_RESULT_MUTATION_MARKER = 999
PARITY_RELATIVE_TOLERANCE = 1e-10
PARITY_ABSOLUTE_TOLERANCE = 1e-10


def policy():
    settings = m.FullLoopObservationPolicy().settings
    for entry in settings['stages']:
        if entry['stage'] in STAGES:
            entry['max_events'] = RETAINED_FRAMES_PER_STAGE
    return m.FullLoopObservationPolicy.from_json(json.dumps(settings).encode())


class SpectrumSequenceTests(unittest.TestCase):
    def test_actual_frame_origins_averages_bounds_and_owned_results(self):
        signature = inspect.signature(m.FullLoopResult.spectrum_sequence)
        self.assertEqual(signature.parameters['max_averages'].default, EXPECTED_DEFAULT_AVERAGES)
        base = request()
        disabled = base.run()
        with self.assertRaises(m.ApiError):
            disabled.spectrum_sequence()
        with self.assertRaises(m.ApiError):
            disabled.calibrated_spectrum_sequence()
        result = base.with_observations(policy()).run()
        before = result.observation_stats()
        for stage in STAGES:
            view = result.spectrum_sequence(stage, max_averages=WELCH_AVERAGES_PER_QUERY, start=FIRST_VIEW_BIN, count=VIEW_BIN_COUNT)
            self.assertEqual(view['sampling_basis'], 'ordered_frame_stage_samples')
            self.assertEqual(view['available_samples'], EXPECTED_TWO_FRAME_SAMPLES)
            self.assertEqual(view['used_samples'], EXPECTED_TWO_FRAME_SAMPLES)
            self.assertEqual(view['power']['averages'], WELCH_AVERAGES_PER_QUERY)
            self.assertEqual(len(view['segments']), 2)
            self.assertEqual(view['stop'], 'work_limit')
            self.assertEqual(view['physically_contiguous'], stage == 'tx.cp.samples')
            self.assertEqual(sum(part['analyzed_samples'] for part in view['segments']), view['used_samples'])
            for part in view['segments']:
                raw = result.stage(stage, snapshot=part['retained_index'], count=1)
                self.assertEqual(part['original_sample_start'], raw['context']['sample_start_index'])
                self.assertEqual(part['total_elements'], raw['total_elements'])
                self.assertEqual(part['channel_symbol_index'], raw['context']['channel_symbol_index'])
            self.assertEqual(result.spectrum(stage, max_averages=WELCH_AVERAGES_PER_QUERY)['power']['averages'], 1)
        for operation in [lambda: result.spectrum_sequence('tx.overlap_add.samples'),
                          lambda: result.spectrum_sequence('rx.fft.bins'),
                          lambda: result.spectrum_sequence(max_averages=0),
                          lambda: result.spectrum_sequence(max_averages=OVER_LIMIT_AVERAGES),
                          lambda: result.spectrum_sequence(start=EXPECTED_FFT_SAMPLES),
                          lambda: result.spectrum_sequence(snapshot=UNAVAILABLE_SNAPSHOT_INDEX),
                          lambda: result.spectrum_sequence('x'*OVERSIZED_STAGE_IDENTIFIER_BYTES)]:
            with self.assertRaises(m.ApiError):
                operation()
        view = result.spectrum_sequence(max_averages=WELCH_AVERAGES_PER_QUERY)
        view['segments'][0]['original_sample_start'] = OWNED_RESULT_MUTATION_MARKER
        self.assertNotEqual(result.spectrum_sequence(max_averages=WELCH_AVERAGES_PER_QUERY)['segments'][0]['original_sample_start'], OWNED_RESULT_MUTATION_MARKER)
        self.assertEqual(result.observation_stats(), before)
        self.assertEqual(result.report()['summary']['comparison']['bit_errors'], 0)
        truncated = policy().settings
        truncated['max_elements_per_event'] = EXPECTED_FFT_SAMPLES
        partial = base.with_observations(m.FullLoopObservationPolicy.from_json(json.dumps(truncated).encode())).run()
        view = partial.spectrum_sequence('tx.cp.samples', max_averages=WELCH_AVERAGES_PER_QUERY)
        self.assertEqual(view['stop'], 'incomplete_frame')
        self.assertEqual(len(view['segments']), 1)
        self.assertEqual(view['available_samples'], EXPECTED_FFT_SAMPLES)
        self.assertEqual(view['power']['averages'], 1)
        self.assertLess(view['segments'][0]['retained_elements'], view['segments'][0]['total_elements'])
        calibrated = partial.calibrated_spectrum_sequence('tx.cp.samples', max_averages=WELCH_AVERAGES_PER_QUERY)
        self.assertEqual(calibrated['sequence'], view)
        self.assertEqual(calibrated['power']['averages'], 1)


    @unittest.skipUnless(os.environ.get('MERLIN_FULL_LOOP_PARITY'), 'Direct Rust process reference required')
    def test_direct_rust_process_ordered_frame_spectra_and_original_segments_match(self):
        reference = json.loads(Path(os.environ['MERLIN_FULL_LOOP_PARITY']).read_text())['sequence_spectra']
        result = request().with_observations(policy()).run()

        def compare(actual, expected):
            if isinstance(expected, dict):
                self.assertEqual(actual.keys(), expected.keys())
                for key in expected:
                    compare(actual[key], expected[key])
            elif isinstance(expected, list):
                self.assertEqual(len(actual), len(expected))
                for actual_item, expected_item in zip(actual, expected):
                    compare(actual_item, expected_item)
            elif isinstance(expected, float):
                self.assertTrue(math.isclose(actual, expected, rel_tol=PARITY_RELATIVE_TOLERANCE, abs_tol=PARITY_ABSOLUTE_TOLERANCE), (actual, expected))
            else:
                self.assertEqual(actual, expected)

        compare([result.spectrum_sequence(stage, max_averages=WELCH_AVERAGES_PER_QUERY, start=FIRST_VIEW_BIN, count=VIEW_BIN_COUNT) for stage in STAGES], reference)


class CalibratedSpectrumSequenceTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        # Immutable actual results are reused by inspections; no live state is shared.
        cls.results = {}
        for electrical in (False, True):
            accepted = list(inputs())
            level = None
            if electrical:
                level = 0.0
            channel = {'power': {'input_data_level_dbmv_per_6mhz': level}}
            accepted[-1] = m.ChannelConfiguration.from_json(json.dumps(channel).encode())
            cls.results[electrical] = m.Simulation.prepare_full_loop(
                *accepted, options=options()).with_observations(policy()).run()

    def test_calibrated_sequence_keeps_sampling_units_errors_and_owned_results(self):
        signature = inspect.signature(m.FullLoopResult.calibrated_spectrum_sequence)
        self.assertEqual(signature.parameters['max_averages'].default, EXPECTED_DEFAULT_AVERAGES)
        # Independent zero-dBmV/6-MHz scale at 204.8 MS/s; no application constant imported.
        expected_calibration = 204_800_000.0 / 6_000_000.0
        for electrical, result in self.results.items():
            scale = None
            if electrical:
                scale = expected_calibration
            before, stats = result.report(), result.observation_stats()
            for stage in STAGES:
                kwargs = dict(max_averages=WELCH_AVERAGES_PER_QUERY, start=FIRST_VIEW_BIN, count=VIEW_BIN_COUNT)
                view = result.calibrated_spectrum_sequence(stage, **kwargs)
                self.assertEqual(view['sequence'], result.spectrum_sequence(stage, **kwargs))
                self.assertEqual(view['sequence']['sampling_basis'], 'ordered_frame_stage_samples')
                self.assertEqual(view['sequence']['used_samples'], EXPECTED_TWO_FRAME_SAMPLES)
                self.assertEqual(view['power']['averages'], WELCH_AVERAGES_PER_QUERY)
                self.assertEqual(view['power']['millivolts_squared_per_unit'], scale)
                self.assertEqual(view['power']['channel_power']['dbmv'] is not None, electrical)
                full = result.calibrated_spectrum_sequence(stage, max_averages=WELCH_AVERAGES_PER_QUERY)
                self.assertEqual(view['power']['channel_power'], full['power']['channel_power'])
                view['sequence']['segments'][0]['original_sample_start'] = OWNED_RESULT_MUTATION_MARKER
                self.assertNotEqual(result.calibrated_spectrum_sequence(stage, **kwargs)['sequence']['segments'][0]['original_sample_start'], OWNED_RESULT_MUTATION_MARKER)
            for kwargs in [dict(stage='tx.overlap_add.samples'), dict(stage='rx.fft.bins'),
                           dict(max_averages=0), dict(max_averages=OVER_LIMIT_AVERAGES),
                           dict(start=EXPECTED_FFT_SAMPLES), dict(snapshot=UNAVAILABLE_SNAPSHOT_INDEX),
                           dict(stage='x'*OVERSIZED_STAGE_IDENTIFIER_BYTES)]:
                with self.assertRaises(m.ApiError) as original:
                    result.spectrum_sequence(**kwargs)
                with self.assertRaises(m.ApiError) as calibrated:
                    result.calibrated_spectrum_sequence(**kwargs)
                self.assertEqual(calibrated.exception.code, original.exception.code)
                self.assertEqual(calibrated.exception.engine_code, original.exception.engine_code)
            with self.assertRaises(OverflowError):
                result.calibrated_spectrum_sequence(snapshot=-1)
            self.assertEqual(result.report(), before)
            self.assertEqual(result.observation_stats(), stats)

    @unittest.skipUnless(os.environ.get('MERLIN_CALIBRATED_SEQUENCE_REFERENCE'), 'Direct Rust process reference required')
    def test_calibrated_sequences_match_the_direct_rust_process(self):
        records = json.loads(Path(os.environ['MERLIN_CALIBRATED_SEQUENCE_REFERENCE']).read_text())
        self.assertEqual([row['electrical'] for row in records], [False, True])

        def compare(actual, expected):
            if isinstance(expected, dict):
                self.assertEqual(actual.keys(), expected.keys())
                for key in expected:
                    compare(actual[key], expected[key])
            elif isinstance(expected, list):
                self.assertEqual(len(actual), len(expected))
                for left, right in zip(actual, expected):
                    compare(left, right)
            elif isinstance(expected, float):
                self.assertTrue(math.isclose(actual, expected, rel_tol=PARITY_RELATIVE_TOLERANCE, abs_tol=PARITY_ABSOLUTE_TOLERANCE), (actual, expected))
            else:
                self.assertEqual(actual, expected)

        for row in records:
            result = self.results[row['electrical']]
            compare([result.calibrated_spectrum_sequence(stage, max_averages=WELCH_AVERAGES_PER_QUERY,
                start=FIRST_VIEW_BIN, count=VIEW_BIN_COUNT) for stage in STAGES], row['spectra'])
            self.assertEqual(result.report()['summary']['comparison'], row['comparison'])
