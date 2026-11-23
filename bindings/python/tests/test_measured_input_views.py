#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Installed native synthetic-unity views and domain reports retain exact Rust semantics."""
import json
import os
from pathlib import Path
import unittest

import merlin.downstream as m
from test_profile_capture import source


class MeasuredInputViewTests(unittest.TestCase):
    def test_unity_keeps_raw_source_ids_projection_values_and_independent_owned_views(self):
        original = source()
        capture = m.ProfileCapture.from_bytes('original-source', original)
        before = capture.metadata
        for metric, reading in [('gain_db', '0.000 … 0.000 Gain (dB)'),
                                ('phase_degrees', '0.000 … 0.000 Phase (degrees)'),
                                ('real', '1.000 … 1.000 Real'),
                                ('imaginary', '0.000 … 0.000 Imaginary')]:
            report = capture.unity_response_preview(200, metric)
            self.assertEqual(report['profile_id'], 200)
            self.assertEqual(report['reference_plane'], 'configured_synthetic_unity_response')
            self.assertEqual(report['projection'], metric)
            self.assertEqual(report['grid']['sample_count'], 4)
            self.assertTrue(all(point['y'] == 0.5 and not point['gap'] and point['readout'].endswith(reading)
                                for point in report['view']['points']))
            self.assertTrue(report['view']['points'][0]['readout'].startswith('640.000–640.000 MHz'))
            self.assertNotIn('provenance', report)
            report['view']['points'].clear()
            self.assertEqual(len(capture.unity_response_preview(200, metric)['view']['points']), 4)
        retained = capture.unity_response_preview(0, 'real')
        self.assertTrue(all(not point['gap'] for point in retained['view']['points']))
        self.assertEqual(capture.metadata, before)
        del capture
        self.assertEqual(retained['grid']['sample_count'], 4)

    def test_unity_rejects_invalid_conversions_identity_and_viewports(self):
        capture = m.ProfileCapture.from_bytes('source', source())
        for value in [-1, 256]:
            with self.assertRaises(OverflowError):
                capture.unity_response_preview(value)
        for arguments in [dict(start=-1), dict(count=-1), dict(start=2**64)]:
            with self.assertRaises(OverflowError):
                capture.unity_response_preview(200, **arguments)
        with self.assertRaises(m.ApiError) as absent:
            capture.unity_response_preview(255)
        self.assertEqual(absent.exception.code, m.ErrorCode.InvalidIdentifier)
        with self.assertRaises(m.ApiError):
            capture.unity_response_preview(200, 'unknown')
        with self.assertRaises(m.ApiError):
            capture.unity_response_preview(200, start=4, count=1)
        zoom = capture.unity_response_preview(200, 'real', 1, 2)['view']
        self.assertEqual((zoom['start'], zoom['count'], zoom['total']), (1, 2, 4))
        self.assertEqual(capture.unity_response_preview(200, start=1000)['view']['start'], 0)

    def test_limits_preserve_all_existing_domains_and_return_independent_dictionaries(self):
        expected = dict(minimum_mer_db=-30.0, maximum_mer_db=120.0, maximum_captures=256,
                        maximum_values=4000000, maximum_value_edits=32,
                        maximum_random_standard_deviation_db=20.0,
                        maximum_absolute_tilt_db=60.0, maximum_ripple_peak_to_peak_db=60.0,
                        maximum_notch_depth_db=60.0, minimum_echo_distance_metres=0.001,
                        maximum_echo_distance_metres=10000.0)
        report = m.RxMerDesign.limits()
        self.assertEqual(report, expected)
        report['maximum_values'] = -1
        self.assertEqual(m.RxMerDesign.limits(), expected)

    @unittest.skipUnless(os.environ.get('MERLIN_MEASURED_INPUT_VIEW_REFERENCE'), 'Direct Rust process reference required')
    def test_all_unity_fields_and_limits_match_separate_public_rust_process(self):
        root = Path(__file__).resolve().parents[3]
        capture = m.ProfileCapture.from_bytes('source', (root / 'examples/measurements/single-channel/modulation_profile.json').read_bytes())
        actual = dict(unity={metric: capture.unity_response_preview(0, metric)
                             for metric in ['gain_db', 'phase_degrees', 'real', 'imaginary']},
                      limits=m.RxMerDesign.limits())
        expected = json.loads(Path(os.environ['MERLIN_MEASURED_INPUT_VIEW_REFERENCE']).read_text())
        self.assertEqual(actual, expected)
