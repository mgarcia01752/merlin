#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Installed native owned actual stage retention with explicit resource/coordinate semantics."""
import json
import os
from pathlib import Path
import unittest

import merlin.downstream as m
from test_full_loop import request


class FullLoopObservationTests(unittest.TestCase):
    def test_real_capture_keeps_science_source_pilots_grids_phases_and_owned_results(self):
        original = request()
        policy = m.FullLoopObservationPolicy()
        selected = original.with_observations(policy)
        self.assertIsNone(original.observation_policy)
        self.assertEqual(len(selected.observation_policy.settings['stages']), 22)
        baseline = original.run()
        result = selected.run()
        self.assertEqual(result.report()['summary']['comparison'], baseline.report()['summary']['comparison'])
        self.assertEqual(result.report()['retained_words'], baseline.report()['retained_words'])
        self.assertEqual(baseline.stage_inventory(), [])
        self.assertIsNone(baseline.observation_policy)
        self.assertEqual(result.observation_policy.settings, policy.settings)
        self.assertGreater(result.observation_stats()['retained_events'], 0)
        source = result.stage('source.payload', count=32)
        self.assertEqual(source['total_elements'], 14232)
        self.assertEqual(source['context']['phase'], 'observation')
        self.assertEqual(source['data']['kind'], 'bits')
        self.assertEqual(source['data']['values'], [int(bit) for bit in '11111111111111101010101010101001'])
        physical = result.stage('tx.frame.bins', count=16)
        self.assertEqual(physical['total_elements'], 8192)
        self.assertEqual(physical['data']['values'], [[0, 0]] * 16)
        self.assertEqual(physical['context']['physical_grid']['zero_frequency_hz'], 631100000)
        pilots = result.stage('tx.pilots.continuous')
        self.assertEqual(pilots['data']['kind'], 'pilots')
        self.assertGreater(len(pilots['data']['values']), 0)
        for pilot in pilots['data']['values'][:8]:
            actual = result.stage('tx.frame.bins', start=pilot['bin'], count=1)
            self.assertEqual(actual['context']['channel_symbol_index'], pilots['context']['channel_symbol_index'])
            self.assertEqual(actual['data']['values'][0], pilot['iq'])
            self.assertAlmostEqual(sum(value * value for value in pilot['iq']), 4, places=12)
        self.assertEqual(result.stage('tx.plc.message')['context']['phase'], 'warmup')
        self.assertEqual(result.stage('tx.overlap_add.samples')['context']['waveform_grid']['sample_rate_hz'], 204800000)
        wave = result.stage('tx.overlap_add.samples', start=7, count=5)
        self.assertEqual(wave['element_start'], 7)
        self.assertEqual(len(wave['data']['values']), 5)
        source['data']['values'][0] = 99
        self.assertEqual(result.stage('source.payload', count=1)['data']['values'], [1])
        del original, selected, policy, baseline
        self.assertEqual(result.stage('source.payload', count=1)['total_elements'], 14232)

    def test_retention_byte_exhaustion_is_explicit_and_science_continues(self):
        original = request()
        tiny = m.FullLoopObservationPolicy.from_json(b'{"max_retained_bytes":1}')
        result = original.with_observations(tiny).run()
        self.assertEqual(result.report()['status'], 'complete')
        stats = result.observation_stats()
        self.assertEqual(stats['retained_bytes'], 0)
        self.assertEqual(stats['retained_events'], 0)
        self.assertGreater(stats['dropped_events'], 0)
        self.assertTrue(stats['byte_budget_exhausted'])
        self.assertEqual(result.stage_inventory(), [])
        self.assertEqual(result.report()['summary']['comparison']['bit_errors'], 0)
        self.assertGreater(result.report()['summary']['comparison']['bits_tested'], 0)
        limited = m.FullLoopObservationPolicy.from_json(b'{"stages":[{"stage":"tx.overlap_add.samples","max_events":1,"observation_only":true}],"max_elements_per_event":16,"max_total_elements":16,"max_events":1}')
        result = original.with_observations(limited).run()
        self.assertEqual(result.stage_inventory()[0]['snapshots'], 1)
        self.assertEqual(result.observation_stats()['retained_elements'], 16)

    def test_policy_missing_and_query_limits_reject_without_changing_originals(self):
        for data in [b'{', b'{"unknown":1}', b' ' * 65537, b'{"stages":[]}',
                     b'{"max_events":0}', b'{"max_elements_per_event":16385}',
                     b'{"max_retained_bytes":100663297}',
                     b'{"stages":[{"stage":"legacy.noise.output","max_events":1,"observation_only":true}]}']:
            with self.assertRaises(m.ApiError):
                m.FullLoopObservationPolicy.from_json(data)
        policy = m.FullLoopObservationPolicy()
        duplicate = policy.settings
        duplicate['stages'].append(duplicate['stages'][0])
        with self.assertRaises(m.ApiError):
            m.FullLoopObservationPolicy.from_json(json.dumps(duplicate).encode())
        result = request().with_observations(policy).run()
        before = result.stage('tx.overlap_add.samples', start=7, count=5)
        for keywords in [{'count': 4097}, {'start': 2**64-1}, {'start': 8191, 'count': 1000}, {'snapshot': 2**64-1}]:
            with self.assertRaises(m.ApiError):
                result.stage('tx.overlap_add.samples', **keywords)
        for stage in ['unknown.stage', 'x' * 129, 'legacy.noise.output']:
            with self.assertRaises(m.ApiError):
                result.stage(stage)
        with self.assertRaises(OverflowError):
            result.stage('tx.overlap_add.samples', start=-1)
        self.assertEqual(before, result.stage('tx.overlap_add.samples', start=7, count=5))
        stopped = m.FullLoopControl()
        stopped.cancel()
        result = request().with_observations(policy).run(stopped)
        self.assertEqual(result.report()['status'], 'cancelled')
        self.assertEqual(result.stage_inventory(), [])

    @unittest.skipUnless(os.environ.get('MERLIN_FULL_LOOP_PARITY'), 'Direct Rust process reference required')
    def test_direct_rust_process_source_pilot_and_actual_waveform_windows_match(self):
        reference = json.loads(Path(os.environ['MERLIN_FULL_LOOP_PARITY']).read_text())['stages']
        result = request().with_observations(m.FullLoopObservationPolicy()).run()
        source = result.stage('source.payload', count=32)
        self.assertEqual(source['data'], reference['source']['data'])
        self.assertEqual(source['context'], reference['source']['context'])
        self.assertEqual(result.stage('tx.frame.bins', count=16)['data'], reference['physical']['data'])
        actual = result.stage('tx.pilots.continuous', count=5)
        self.assertEqual(actual['data'], reference['pilots']['data'])
        actual = result.stage('tx.overlap_add.samples', start=7, count=4)
        self.assertEqual(actual['element_start'], reference['waveform']['element_start'])
        self.assertEqual(actual['total_elements'], reference['waveform']['total_elements'])
        for point, expected in zip(actual['data']['values'], reference['waveform']['data']['values']):
            for value, anchor in zip(point, expected):
                self.assertAlmostEqual(value, anchor, places=11)
