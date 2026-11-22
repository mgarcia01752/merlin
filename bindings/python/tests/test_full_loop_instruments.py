#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Actual installed native numerical queries preserve original reference planes and bounds."""
import json
import math
import os
from pathlib import Path
import unittest

import merlin.downstream as m
from test_full_loop import request


class FullLoopInstrumentTests(unittest.TestCase):
    def test_original_psd_response_and_likelihood_metadata_and_owned_results(self):
        result = request().with_observations(m.FullLoopObservationPolicy()).run()
        spectrum = result.spectrum()
        self.assertEqual(spectrum['stage'], 'tx.overlap_add.samples')
        self.assertEqual(spectrum['snapshots_used'], 4)
        self.assertEqual(spectrum['available_samples'], 34816)
        self.assertEqual(spectrum['used_samples'], 32768)
        power = spectrum['power']
        self.assertEqual(power['reference_plane'], 'tx_post_overlap_before_channel')
        self.assertEqual(power['density_units'], 'normalized_sample_units_squared_per_hz')
        self.assertEqual(power['sample_rate_hz'], 204800000)
        self.assertEqual(power['zero_frequency_hz'], 631100000)
        self.assertAlmostEqual(power['equivalent_noise_bandwidth_hz'], 37500, places=8)
        self.assertEqual(power['averages'], 7)
        self.assertEqual(len(power['buckets']), 2048)
        limited = result.spectrum(start=123, count=7, max_averages=1)
        self.assertEqual(limited['used_samples'], 8192)
        self.assertEqual(limited['power']['buckets'][0]['first_bin'], 123)
        self.assertEqual(limited['power']['buckets'][-1]['last_bin'], 129)
        channel = result.spectrum(stage='channel.output')
        self.assertEqual(channel['power']['reference_plane'], 'channel_output_receiver_input')
        self.assertGreater(channel['snapshots_used'], 1)
        response = result.response()
        self.assertEqual(response['reference_plane'], 'configured_channel_transfer_before_receiver')
        self.assertIsNone(response['channel_symbol_index'])
        self.assertEqual(response['diagnostics']['real'][0], None)
        self.assertEqual(response['diagnostics']['available_bins'], 7480)
        self.assertTrue(all(value == 0 for value in response['diagnostics']['gain_db'] if value is not None))
        self.assertEqual(response['diagnostics']['peak_delay_microseconds'], 0)
        soft = result.llr_statistics()
        self.assertEqual(soft['sign_convention'], 'log_p_zero_over_p_one')
        self.assertEqual(soft['element_start'], 0)
        self.assertEqual(soft['total_elements'], 16200)
        self.assertEqual(soft['statistics']['count'], 16200)
        self.assertEqual(sum(soft['statistics']['histogram']), 16200)
        soft['statistics']['histogram'][0] = -999
        response['diagnostics']['real'][4000] = -999
        self.assertNotEqual(result.llr_statistics()['statistics']['histogram'][0], -999)
        self.assertNotEqual(result.response()['diagnostics']['real'][4000], -999)
        with self.assertRaises(m.ApiError):
            result.response('receiver_estimate')
        actual = request(receiver_mode='pilot_estimated').with_observations(m.FullLoopObservationPolicy()).run()
        estimate = actual.response('receiver_estimate')
        self.assertEqual(estimate['reference_plane'], 'receiver_estimated_channel_transfer')
        self.assertIsNotNone(estimate['channel_symbol_index'])
        self.assertEqual(estimate['diagnostics']['real'][0], None)

    def test_absent_truncated_gapped_invalid_and_bounded_queries_preserve_science(self):
        idle = request().run()
        for operation in [idle.spectrum, idle.llr_statistics]:
            with self.assertRaises(m.ApiError):
                operation()
        truncated = request().with_observations(m.FullLoopObservationPolicy.from_json(
            b'{"max_elements_per_event":4096}')).run()
        with self.assertRaises(m.ApiError):
            truncated.spectrum()
        self.assertEqual(truncated.llr_statistics()['statistics']['count'], 4096)
        self.assertEqual(truncated.llr_statistics()['total_elements'], 16200)
        incomplete = request(receiver_mode='pilot_estimated').with_observations(
            m.FullLoopObservationPolicy.from_json(b'{"max_elements_per_event":4096}')).run()
        with self.assertRaises(m.ApiError):
            incomplete.response('receiver_estimate')
        result = request().with_observations(m.FullLoopObservationPolicy()).run()
        before = result.observation_stats()
        for keywords in [dict(stage='rx.fft.bins'), dict(stage='unknown'), dict(stage='x' * 129),
                         dict(max_averages=0), dict(max_averages=33), dict(start=2**64-1),
                         dict(count=2**64-1), dict(snapshot=2**64-1)]:
            with self.assertRaises(m.ApiError):
                result.spectrum(**keywords)
        for keywords in [dict(source='unknown'), dict(snapshot=1), dict(velocity_factor=0),
                         dict(velocity_factor=1.01), dict(velocity_factor=math.nan),
                         dict(velocity_factor=math.inf)]:
            with self.assertRaises(m.ApiError):
                result.response(**keywords)
        with self.assertRaises(m.ApiError):
            result.llr_statistics(2**64-1)
        with self.assertRaises(OverflowError):
            result.spectrum(start=-1)
        self.assertEqual(result.spectrum(start=123, count=7), result.spectrum(start=123, count=7))
        self.assertEqual(result.observation_stats(), before)
        self.assertEqual(result.report()['summary']['comparison']['bit_errors'], 0)

    @unittest.skipUnless(os.environ.get('MERLIN_FULL_LOOP_PARITY'), 'Direct Rust process reference required')
    def test_direct_rust_process_spectrum_response_and_actual_llr_statistics_match(self):
        reference = json.loads(Path(os.environ['MERLIN_FULL_LOOP_PARITY']).read_text())['instruments']
        result = request().with_observations(m.FullLoopObservationPolicy()).run()

        def compare(actual, expected):
            if isinstance(expected, dict):
                self.assertEqual(actual.keys(), expected.keys())
                for key in expected:
                    compare(actual[key], expected[key])
            elif isinstance(expected, list):
                self.assertEqual(len(actual), len(expected))
                for a, b in zip(actual, expected):
                    compare(a, b)
            elif isinstance(expected, float):
                self.assertTrue(math.isclose(actual, expected, rel_tol=1e-11, abs_tol=1e-11), (actual, expected))
            else:
                self.assertEqual(actual, expected)

        compare(result.spectrum(start=123, count=7), reference['spectrum'])
        compare(result.response(), reference['response'])
        compare(result.llr_statistics(), reference['llr'])
