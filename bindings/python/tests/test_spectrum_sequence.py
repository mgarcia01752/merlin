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
from test_full_loop import request

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
        with self.assertRaises(m.ApiError):
            base.run().spectrum_sequence()
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
