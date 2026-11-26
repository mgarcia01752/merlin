#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Installed actual Rust soft observations; no Python demapper, mask or decoder."""
import inspect
import json
import os
from pathlib import Path
import unittest

import merlin.upstream as u
from test_upstream_configured_response import configured

ROOT = Path(__file__).resolve().parents[3]
RETAINED_CELL_BUDGET = 1024
RETAINED_WORD_BUDGET = 16
WORD_INPUT_BUDGET = 262144
DEFAULT_CELL_PAGE_COUNT = 32
DEFAULT_CELL_QUERY_LLRS = 4096
DEFAULT_WORD_QUERY_LLRS = 32768
OVER_RETAINED_CELLS = 1025
OVER_RETAINED_WORDS = 33
OVER_RETAINED_WORD_LLRS = 262145
OVER_CELL_PAGE_COUNT = 129
OVER_CELL_QUERY_LLRS = 4097
OVER_WORD_QUERY_LLRS = 32769
LIMITED_CELL_COUNT = 3
LIMITED_WORD_COUNT = 1
EXACT_FIRST_WORD_LLRS = 1120
MULTI_GRANT_COUNT = 3
EXPECTED_DEMAPPER_LLR_LIMIT = 50.0
NOISE_VARIANCE = 1e-4
NUMERICAL_ABSOLUTE_TOLERANCE = 1e-10
PROCESS_RELATIVE_TOLERANCE = 1e-12


def enable(simulation, cells=RETAINED_CELL_BUDGET, words=RETAINED_WORD_BUDGET, llrs=WORD_INPUT_BUDGET):
    """Compose independently validated Rust policies; no scientific state is constructed here."""
    simulation.set_receiver_observation_policy(u.ReceiverObservationPolicy().with_soft_metrics(
        u.SoftObservationPolicy(cells, words, llrs)))


class UpstreamSoftObservations(unittest.TestCase):
    def assert_snapshot(self, actual, expected):
        """Compare every actual field; only process-local IDs and finite float tolerance differ."""
        if isinstance(expected, dict):
            self.assertEqual(set(actual), set(expected))
            for key in expected:
                if key == 'identity':
                    self.assertEqual(actual[key]['grant'], expected[key]['grant'])
                    self.assertGreater(actual[key]['run'], 0)
                    self.assertGreater(actual[key]['generation'], 0)
                else:
                    self.assert_snapshot(actual[key], expected[key])
        elif isinstance(expected, list):
            self.assertEqual(len(actual), len(expected))
            for value, reference in zip(actual, expected):
                self.assert_snapshot(value, reference)
        elif isinstance(expected, float):
            self.assertAlmostEqual(actual, expected,
                delta=max(NUMERICAL_ABSOLUTE_TOLERANCE, abs(expected) * PROCESS_RELATIVE_TOLERANCE))
        else:
            self.assertEqual(actual, expected)

    def test_policies_composition_installed_defaults_and_invalid_budgets(self):
        original = u.ReceiverObservationPolicy()
        disabled = original.info()
        soft = u.SoftObservationPolicy(RETAINED_CELL_BUDGET, RETAINED_WORD_BUDGET, WORD_INPUT_BUDGET)
        composed = original.with_soft_metrics(soft)
        self.assertNotIn('soft_metrics', original.info())
        self.assertEqual(composed.info()['soft_metrics'], soft.info())
        self.assertEqual(composed.with_soft_metrics(u.SoftObservationPolicy()).info(), disabled)
        self.assertEqual(inspect.signature(u.BatchReport.soft_cells).parameters['maximum_cells'].default,
                         DEFAULT_CELL_PAGE_COUNT)
        self.assertEqual(inspect.signature(u.BatchReport.soft_cells).parameters['maximum_llrs'].default,
                         DEFAULT_CELL_QUERY_LLRS)
        self.assertEqual(inspect.signature(u.BatchReport.soft_word).parameters['maximum_llrs'].default,
                         DEFAULT_WORD_QUERY_LLRS)
        for arguments in [(OVER_RETAINED_CELLS, 0, 0), (0, OVER_RETAINED_WORDS, WORD_INPUT_BUDGET),
                          (0, 1, OVER_RETAINED_WORD_LLRS), (0, 0, 1), (0, 1, 0)]:
            with self.assertRaises(u.ApiError):
                u.SoftObservationPolicy(*arguments)
        for value in [-1, 0.5]:
            with self.assertRaises((TypeError, OverflowError)):
                u.SoftObservationPolicy(value, 0, 0)

    def test_actual_cells_and_completed_word_inputs_keep_original_positions_flags_and_planes(self):
        simulation = configured()
        enable(simulation)
        report = simulation.run()
        self.assertTrue(report.reference_recovered)
        self.assertEqual(report.receiver_observations()['retained_frames'], 0)
        summary = report.soft_observations()
        self.assertEqual(summary['completed_words'], len(json.loads(report.to_json())['grants'][0]['words']))
        page = report.soft_cells()
        self.assertEqual(page['bit_order'], 'msb_first')
        self.assertEqual(page['demapper_llr_limit'], EXPECTED_DEMAPPER_LLR_LIMIT)
        self.assertEqual(page['llr_convention'], 'natural_log_p0_over_p1;positive_favors_zero')
        self.assertEqual(page['cells'][0]['coded_offset'], 0)
        for cell in page['cells']:
            self.assertEqual(len(cell['demapper_llrs']), cell['loading'])
            self.assertEqual(len(cell['descrambled_llrs']), cell['loading'])
            self.assertNotEqual(cell['role'], 'KnownPilot')
        word = report.soft_word()
        outcome = report.fec_words()['words'][0]
        self.assertEqual(len(word['input_llrs']), word['transmitted_bits'])
        self.assertEqual(word['mother_codeword_bits'] - word['shortened_bits'], word['transmitted_bits'])
        self.assertEqual(word['code_family'], outcome['code_family'])
        self.assertEqual(word['iterations'], outcome['word']['decoded']['iterations'])
        self.assertEqual(word['parity_valid'], outcome['word']['decoded']['parity_valid'])
        self.assertEqual(word['word']['identity'], outcome['identity'])

    def test_global_budgets_passive_noisy_outcomes_and_owned_queries(self):
        simulation = configured(channel=u.ChannelConfig(noise_variance=NOISE_VARIANCE),
                                options=u.ExperimentOptions(grants=MULTI_GRANT_COUNT))
        original = simulation.run()
        self.assertNotIn('receiver_estimates', json.loads(original.to_json())['observations'])
        self.assertEqual(original.soft_observations()['completed_words'], 0)
        enable(simulation)
        observed = simulation.run()
        self.assertGreater(observed.soft_observations()['omitted_cells'], 0)
        self.assertEqual(observed.soft_observations()['retained_cells'], RETAINED_CELL_BUDGET)
        enable(simulation, cells=LIMITED_CELL_COUNT, words=LIMITED_WORD_COUNT, llrs=EXACT_FIRST_WORD_LLRS)
        limited = simulation.run()
        summary = limited.soft_observations()
        self.assertEqual(summary['retained_cells'], LIMITED_CELL_COUNT)
        self.assertEqual(summary['retained_words'], LIMITED_WORD_COUNT)
        self.assertEqual(summary['retained_word_llrs'], EXACT_FIRST_WORD_LLRS)
        self.assertGreater(summary['omitted_words'], 0)
        for candidate in [observed, limited]:
            self.assertEqual(candidate.bit_errors, original.bit_errors)
            self.assertEqual(candidate.compared_source_bits, original.compared_source_bits)
            actual_words = [grant['words'] for grant in json.loads(candidate.to_json())['grants']]
            self.assertEqual(actual_words, [grant['words'] for grant in json.loads(original.to_json())['grants']])
        page = limited.soft_cells()
        word = limited.soft_word()
        changed = limited.soft_cells()
        changed['cells'].clear()
        self.assertEqual(limited.soft_cells(), page)
        del simulation, original, observed, limited
        self.assertEqual(len(word['input_llrs']), EXACT_FIRST_WORD_LLRS)

    def test_query_limits_actual_erasure_metrics_and_never_started_words(self):
        simulation = configured(channel=u.ChannelConfig(taps=[(0.0, 0.0)]))
        enable(simulation)
        report = simulation.run()
        self.assertFalse(report.reference_recovered)
        for cell in report.soft_cells()['cells']:
            self.assertIsNone(cell['corrected'])
            self.assertTrue(all(value == 0.0 for value in cell['demapper_llrs']))
        for arguments in [dict(maximum_cells=0), dict(maximum_cells=OVER_CELL_PAGE_COUNT),
                          dict(maximum_llrs=OVER_CELL_QUERY_LLRS), dict(maximum_llrs=1),
                          dict(first_index=OVER_RETAINED_CELLS)]:
            with self.assertRaises(u.ApiError):
                report.soft_cells(**arguments)
        for arguments in [dict(maximum_llrs=0), dict(maximum_llrs=OVER_WORD_QUERY_LLRS),
                          dict(maximum_llrs=1), dict(retained_index=OVER_RETAINED_WORDS)]:
            with self.assertRaises(u.ApiError):
                report.soft_word(**arguments)
        control = u.RunControl()
        control.cancel()
        partial = simulation.run_with_control(control)
        self.assertEqual(partial.status, 'Cancelled')
        self.assertEqual(partial.soft_observations()['completed_words'], 0)
        self.assertEqual(partial.soft_cells()['cells'], [])
        with self.assertRaises(u.ApiError) as unavailable:
            partial.soft_word()
        self.assertEqual(unavailable.exception.code, u.ErrorCode.UnsupportedCapability)

    @unittest.skipUnless(os.environ.get('MERLIN_UPSTREAM_RUST_SOFT_OBSERVATIONS_JSON'),
                         'Direct Rust soft-observation reference required')
    def test_separate_rust_process_cells_full_word_vectors_flags_and_metadata_match(self):
        references = json.loads(Path(os.environ['MERLIN_UPSTREAM_RUST_SOFT_OBSERVATIONS_JSON']).read_text())
        self.assertEqual(len(references), 2)
        for spacing, reference in zip([50, 25], references):
            simulation = u.Simulation()
            simulation.configure_json((ROOT / f'examples/cli/upstream/us-{spacing}khz.json').read_bytes())
            enable(simulation)
            report = simulation.run()
            summary = report.soft_observations()
            actual = {'summary': summary, 'cells': report.soft_cells(),
                      'words': [report.soft_word(entry['retained_index']) for entry in summary['words']]}
            self.assert_snapshot(actual, reference)
