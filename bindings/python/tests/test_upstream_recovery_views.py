#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Actual upstream recovery views, bounded copies and independent Rust-process parity."""
import inspect
import json
import os
from pathlib import Path
import threading
import time
import unittest

import merlin.upstream as u

PREFIX_SAMPLES = 192
ROLLOFF_SAMPLES = 64
FRAME_SYMBOLS = 6
BANDWIDTH_HZ = 20_000_000
PROFILE_ID = 224
DATA_IUC = 5
PROFILE_RANDOMIZER_SEED = 0x7FFFFF
SOURCE_PRBS15_SEED = 1
FIRST_ACTIVE_CARRIER = 128
ACTIVE_MINISLOTS = 8
DATA_LOADING = 4
MINISLOT_50KHZ_CARRIERS = 8
MINISLOT_25KHZ_CARRIERS = 16
FIRST_50KHZ_PILOT_PATTERN = 1
FIRST_25KHZ_PILOT_PATTERN = 8
GRANT_ID = 7
GRANT_EPOCH_SAMPLE = 73
FIRST_FRAME_ID = 3
GRANT_FRAMES = 2
EXPECTED_FIRST_PAGE_WORDS = 1
EXPECTED_MAX_PAGE_WORDS = 32
OVER_MAX_PAGE_WORDS = 33
EXPECTED_MAX_DECISION_BITS = 262144
OVER_MAX_DECISION_BITS = 262145
EXPECTED_SHORT_WORDS = 3
EXPECTED_SHORT_SOURCE_BITS = 1976
EXPECTED_SHORT_INFORMATION_BITS = 840
EXPECTED_SHORT_MOTHER_BITS = 1120
SEQUENTIAL_GRANTS = 2
EXPECTED_NEXT_GRANT_SOURCE_START = 1976
EXPECTED_FIRST_WORD_COPY_BITS = 2800
CONTROL_GRANTS = 128
CONTROL_CHUNK_SAMPLES = 313
CONTROL_DEADLINE_SECONDS = 10
CONTROL_POLL_SECONDS = 0.001


def configured(spacing=50, options=None, channel=None):
    """Admit labelled original fixture settings; Rust owns every scientific operation."""
    width, pattern = {
        50: (MINISLOT_50KHZ_CARRIERS, FIRST_50KHZ_PILOT_PATTERN),
        25: (MINISLOT_25KHZ_CARRIERS, FIRST_25KHZ_PILOT_PATTERN),
    }[spacing]
    waveform = u.Waveform(spacing, PREFIX_SAMPLES, ROLLOFF_SAMPLES, FRAME_SYMBOLS, BANDWIDTH_HZ)
    slots = [u.Minislot(FIRST_ACTIVE_CARRIER + index * width, DATA_LOADING, pattern)
             for index in range(ACTIVE_MINISLOTS)]
    profile = u.Profile(PROFILE_ID, DATA_IUC, PROFILE_RANDOMIZER_SEED, slots)
    grant = u.SyntheticGrant(GRANT_ID, PROFILE_ID, GRANT_EPOCH_SAMPLE,
                             FIRST_FRAME_ID, GRANT_FRAMES, list(range(ACTIVE_MINISLOTS)))
    us = u.UsConfiguration(waveform, [profile], grant, [0, 1])
    config = u.ExperimentConfig(us, SOURCE_PRBS15_SEED, u.ReceiverConfig(), channel=channel)
    simulation = u.Simulation(options)
    simulation.configure(config)
    return simulation


class UpstreamRecoveryViews(unittest.TestCase):
    def test_actual_original_shortening_source_offsets_owned_decisions_and_exact_defaults(self):
        signature = inspect.signature(u.BatchReport.fec_words)
        self.assertEqual(signature.parameters['maximum_words'].default, EXPECTED_FIRST_PAGE_WORDS)
        self.assertEqual(signature.parameters['max_decision_bits'].default, EXPECTED_MAX_DECISION_BITS)
        simulation = configured()
        report = simulation.run()
        verification = report.verification()
        summary = report.fec_summary()
        page = report.fec_words()
        self.assertEqual(verification['source_seed'], SOURCE_PRBS15_SEED)
        self.assertEqual(verification['compared_source_bits'], EXPECTED_SHORT_SOURCE_BITS)
        self.assertEqual(verification['bit_errors'], 0)
        self.assertEqual(verification['ber'], 0.0)
        self.assertTrue(verification['reference_recovered'])
        self.assertEqual(summary['completed_words'], EXPECTED_SHORT_WORDS)
        self.assertEqual(summary['pending_words'], 0)
        word = page['words'][0]
        self.assertEqual(word['code_family'], 'upstream_short')
        self.assertEqual(word['mother_information_bits'], EXPECTED_SHORT_INFORMATION_BITS)
        self.assertEqual(word['mother_codeword_bits'], EXPECTED_SHORT_MOTHER_BITS)
        self.assertEqual(word['source_start_bit'], 0)
        self.assertEqual(page['identity']['grant'], GRANT_ID)
        self.assertEqual(page['next_word'], EXPECTED_FIRST_PAGE_WORDS)
        original_word = json.loads(report.to_json())['grants'][0]['words'][0]
        self.assertEqual(word['word'], original_word)
        word['word']['payload'].clear()
        self.assertEqual(report.fec_words()['words'][0]['word'], original_word)
        retained = report.fec_words()
        simulation.run()
        del report, simulation
        self.assertEqual(retained['words'][0]['word'], original_word)
        report = configured(options=u.ExperimentOptions(grants=SEQUENTIAL_GRANTS)).run()
        page = report.fec_words(grant_index=1, max_decision_bits=EXPECTED_FIRST_WORD_COPY_BITS)
        self.assertEqual(page['words'][0]['source_start_bit'], EXPECTED_NEXT_GRANT_SOURCE_START)
        self.assertEqual(page['copied_decision_bits'], EXPECTED_FIRST_WORD_COPY_BITS)

    def test_bounds_continuations_end_positions_and_scalar_totals_are_independent_of_pages(self):
        for spacing in [50, 25]:
            report = configured(spacing).run()
            summary, verification = report.fec_summary(), report.verification()
            page = report.fec_words(maximum_words=EXPECTED_MAX_PAGE_WORDS)
            self.assertEqual(len(page['words']), summary['completed_words'])
            self.assertIsNone(page['next_word'])
            end = report.fec_words(first_word=summary['completed_words'])
            self.assertEqual(end['words'], [])
            self.assertIsNone(end['next_word'])
            self.assertEqual(end['copied_decision_bits'], 0)
            for arguments in [dict(grant_index=1), dict(first_word=summary['completed_words'] + 1),
                              dict(maximum_words=0), dict(maximum_words=OVER_MAX_PAGE_WORDS),
                              dict(max_decision_bits=0), dict(max_decision_bits=1),
                              dict(max_decision_bits=OVER_MAX_DECISION_BITS)]:
                with self.assertRaises(u.ApiError):
                    report.fec_words(**arguments)
            for arguments in [dict(grant_index=-1), dict(maximum_words=1.5), dict(unknown=1)]:
                with self.assertRaises((OverflowError, TypeError)):
                    report.fec_words(**arguments)
            self.assertEqual(report.fec_summary(), summary)
            self.assertEqual(report.verification(), verification)

    def test_precancel_and_actual_destructive_channel_preserve_unavailable_and_failed_outcomes(self):
        simulation = configured()
        control = u.RunControl()
        control.cancel()
        report = simulation.run_with_control(control)
        verification, summary = report.verification(), report.fec_summary()
        self.assertEqual(verification['status'], 'Cancelled')
        self.assertEqual(verification['compared_source_bits'], 0)
        self.assertIsNone(verification['ber'])
        self.assertFalse(verification['reference_recovered'])
        self.assertIsNone(summary['maximum_iterations'])
        self.assertEqual(summary['pending_words'], summary['planned_words'])
        with self.assertRaises(u.ApiError):
            report.fec_words()
        report = configured(channel=u.ChannelConfig(taps=[(0.0, 0.0)])).run()
        verification, summary = report.verification(), report.fec_summary()
        self.assertEqual(verification['status'], 'Completed')
        self.assertGreater(verification['bit_errors'], 0)
        self.assertFalse(verification['reference_recovered'])
        self.assertGreater(summary['nonconverged_words'], 0)
        self.assertGreater(summary['parity_invalid_words'], 0)
        self.assertEqual(report.fec_words()['words'][0]['word'],
                         json.loads(report.to_json())['grants'][0]['words'][0])

    def test_actual_cancellation_retains_completed_words_and_source_coverage(self):
        simulation = configured(options=u.ExperimentOptions(grants=CONTROL_GRANTS,
                                                            chunk_samples=CONTROL_CHUNK_SAMPLES))
        control = u.RunControl()
        outputs, failures = [], []

        def run():
            try:
                outputs.append(simulation.run_with_control(control))
            except BaseException as cause:
                failures.append(cause)

        worker = threading.Thread(target=run)
        worker.start()
        try:
            deadline = time.monotonic() + CONTROL_DEADLINE_SECONDS
            while json.loads(control.progress_json())['compared_bits'] == 0:
                self.assertLess(time.monotonic(), deadline)
                time.sleep(CONTROL_POLL_SECONDS)
        finally:
            control.cancel()
            worker.join(CONTROL_DEADLINE_SECONDS)
        self.assertFalse(worker.is_alive())
        self.assertEqual(failures, [])
        report = outputs[0]
        verification, summary = report.verification(), report.fec_summary()
        self.assertEqual(verification['status'], 'Cancelled')
        self.assertGreater(verification['compared_source_bits'], 0)
        self.assertLess(verification['compared_source_bits'], verification['planned_source_bits'])
        self.assertFalse(verification['reference_recovered'])
        self.assertGreater(summary['completed_words'], 0)
        self.assertGreater(summary['pending_words'], 0)
        self.assertTrue(report.fec_words()['words'])

    @unittest.skipUnless(os.environ.get('MERLIN_UPSTREAM_RUST_RECOVERY_JSON'), 'Direct Rust recovery reference required')
    def test_separate_rust_process_actual_views_match_both_numerologies(self):
        references = json.loads(Path(os.environ['MERLIN_UPSTREAM_RUST_RECOVERY_JSON']).read_text())
        for spacing, expected in zip([50, 25], references):
            report = configured(spacing).run()
            self.assertEqual(report.verification(), expected['verification'])
            self.assertEqual(report.fec_summary(), expected['fec_summary'])
            page = report.fec_words()

            # Only process-local identity components differ; original grant identity stays exact.
            for actual_identity, expected_identity in [
                (page['identity'], expected['fec_words']['identity']),
                (page['words'][0]['identity'], expected['fec_words']['words'][0]['identity']),
            ]:
                self.assertEqual(actual_identity['grant'], expected_identity['grant'])
                self.assertGreater(actual_identity['run'], 0)
                self.assertGreater(actual_identity['generation'], 0)
                for key in ['run', 'generation']:
                    actual_identity[key] = expected_identity[key]
            self.assertEqual(page, expected['fec_words'])
