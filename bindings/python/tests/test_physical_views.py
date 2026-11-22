#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Installed physical views preserve original coordinates, actual values and role provenance."""
import hashlib
import json
import math
import os
from pathlib import Path
import unittest

import merlin.downstream as m
from test_full_loop import FIXTURES, inputs, request


class PhysicalViewTests(unittest.TestCase):
    def test_idle_layout_original_grid_raw_catalog_ids_and_owned_windows(self):
        transmission, profile, *_ = inputs()
        layout = transmission.physical_layout(profile, channel_symbol=1, start=7, count=5)
        self.assertEqual(layout['reference_plane'], 'configured_physical_layout')
        self.assertEqual(layout['layout']['fft_bins'], 8192)
        self.assertEqual(layout['key']['grid']['sample_count'], 7480)
        self.assertEqual(layout['sources'][0]['original_sha256'], hashlib.sha256((FIXTURES / 'modulation_profile.json').read_bytes()).hexdigest())
        self.assertEqual([point['physical_bin'] for point in layout['layout']['carriers']], list(range(7, 12)))
        source = json.loads((FIXTURES / 'modulation_profile.json').read_bytes())
        source['profiles'][0]['profile_id'] = 200
        capture = m.ProfileCapture.from_bytes('catalog', json.dumps(source).encode())
        inspected = transmission.physical_layout_capture(capture, 200)
        self.assertEqual(inspected['profile_id'], 200)
        self.assertEqual(len(inspected['layout']['carriers']), 8192)
        self.assertLessEqual(len(inspected['layout']['role_spans']), 1024)
        for operation in [lambda: transmission.physical_layout(profile, start=8192),
                          lambda: transmission.physical_layout(profile, start=8191, count=2),
                          lambda: transmission.physical_layout(profile, count=8193),
                          lambda: transmission.physical_layout_capture(capture, 255),
                          lambda: m.ProfileId(200)]:
            with self.assertRaises(m.ApiError):
                operation()
        layout['layout']['carriers'][0]['role'] = 'invented'
        self.assertEqual(transmission.physical_layout(profile, start=7, count=1)['layout']['carriers'][0]['role'], 'excluded')

    def test_actual_bin_power_and_typed_transmit_cells_match_retained_originals(self):
        base = request()
        with self.assertRaises(m.ApiError):
            base.run().physical_bins('tx.frame.bins')
        result = base.with_observations(m.FullLoopObservationPolicy()).run()
        for stage in ['tx.frame.bins', 'rx.fft.bins']:
            view = result.physical_bins(stage, start=7, count=13)
            raw = result.stage(stage, start=7, count=13)
            self.assertEqual(view['context'], raw['context'])
            self.assertEqual(view['element_start'], 7)
            self.assertEqual(view['power_units'], 'normalized_bin_power')
            self.assertEqual([point['iq'] for point in view['bins']], raw['data']['values'])
            for point in view['bins']:
                if point['iq'] is None:
                    self.assertIsNone(point['linear_power'])
                else:
                    self.assertEqual(point['linear_power'], sum(value * value for value in point['iq']))
        physical = result.physical_bins('tx.frame.bins')
        self.assertEqual(len(physical['bins']), 8192)
        self.assertEqual(physical['bins'][0]['linear_power'], 0)
        self.assertIsNone(physical['bins'][0]['power_db'])
        for point in physical['bins']:
            if point['iq'] is None:
                self.assertIsNone(point['linear_power'])
                self.assertIsNone(point['power_db'])
        for stage in ['tx.cells.mapped', 'tx.cells.interleaved']:
            view = result.transmit_cells(stage, start=7, count=13)
            raw = result.stage(stage, start=7, count=13)
            self.assertEqual(view['context'], raw['context'])
            self.assertEqual([point['value'] for point in view['cells']], raw['data']['values'])
            self.assertEqual(view['cells'][0]['cell_index'], 7)
            if stage == 'tx.cells.mapped':
                for point in view['cells']:
                    self.assertEqual(point['role'], view['context']['transmit_cell_roles'][point['cell_index']])
        for operation in [lambda: result.physical_bins('tx.ifft.samples'),
                          lambda: result.physical_bins('tx.frame.bins', count=8193),
                          lambda: result.physical_bins('tx.frame.bins', start=8191, count=2),
                          lambda: result.physical_bins('tx.frame.bins', snapshot=999),
                          lambda: result.transmit_cells('tx.frame.bins'),
                          lambda: result.transmit_cells('tx.cells.mapped', count=8193)]:
            with self.assertRaises(m.ApiError):
                operation()
        physical['bins'][0]['linear_power'] = 999
        self.assertEqual(result.physical_bins('tx.frame.bins', count=1)['bins'][0]['linear_power'], 0)
        self.assertEqual(result.report()['summary']['comparison']['bit_errors'], 0)

    @unittest.skipUnless(os.environ.get('MERLIN_FULL_LOOP_PARITY'), 'Direct Rust process reference required')
    def test_direct_rust_process_layout_bins_and_transmit_roles_match(self):
        reference = json.loads(Path(os.environ['MERLIN_FULL_LOOP_PARITY']).read_text())['physical']
        result = request().with_observations(m.FullLoopObservationPolicy()).run()
        transmission, profile, *_ = inputs()
        actual = {
            'layout': transmission.physical_layout(profile, channel_symbol=1, start=350, count=10),
            'tx': result.physical_bins('tx.frame.bins', start=350, count=10),
            'rx': result.physical_bins('rx.fft.bins', start=350, count=10),
            'mapped': result.transmit_cells('tx.cells.mapped', start=7, count=13),
            'interleaved': result.transmit_cells('tx.cells.interleaved', start=7, count=13),
        }

        def compare(value, expected):
            if isinstance(expected, dict):
                self.assertEqual(value.keys(), expected.keys())
                for key in expected:
                    compare(value[key], expected[key])
            elif isinstance(expected, list):
                self.assertEqual(len(value), len(expected))
                for a, b in zip(value, expected):
                    compare(a, b)
            elif isinstance(expected, float):
                self.assertTrue(math.isclose(value, expected, rel_tol=1e-10, abs_tol=1e-10), (value, expected))
            else:
                self.assertEqual(value, expected)

        # Loader provenance is tested independently; the direct process uses a different loader/name.
        self.assertEqual(actual['layout']['sources'][0]['original_sha256'], reference['layout']['sources'][0]['original_sha256'])
        for value in [actual, reference]:
            value['layout'] = {key: item for key, item in value['layout'].items() if key != 'sources'}
        compare(actual, reference)
