#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Installed receiver selections keep actual scientific populations separate from display caps."""
import json
import math
import os
from pathlib import Path
import unittest

import merlin
import merlin.downstream as m
from test_full_loop import request


def query(**values):
    return m.FullLoopCellQuery.from_json(json.dumps(values).encode())


class FullLoopCellTests(unittest.TestCase):
    def test_actual_pairings_all_selected_metrics_and_owned_query_parameters(self):
        self.assertIs(merlin.FullLoopCellQuery, m.FullLoopCellQuery)
        result = request().run()
        selected = query(count=5)
        settings = selected.settings
        settings['count'] = 999
        self.assertEqual(selected.settings['count'], 5)
        view = result.qam(selected)
        original = result.report()['constellation']['pairs']
        self.assertGreater(view['selection']['selected_cells'], 2048)
        self.assertEqual(view['selection']['retained_cells'], len(original))
        self.assertEqual(len(view['cells']), 5)
        for cell, pair in zip(view['cells'], original):
            for key in ['physical_bin', 'frequency_hz', 'channel_symbol_index', 'source_symbol_index',
                        'profile_id', 'modulation_order', 'before_equalization', 'after_equalization', 'llrs']:
                self.assertEqual(cell[key], pair[key])
            self.assertEqual(cell['reference_iq'], pair.get('reference_iq'))
        later = result.qam(query(start=5, count=3))
        self.assertEqual(later['raw_nearest_mer'], view['raw_nearest_mer'])
        self.assertEqual(later['raw_tx_reference_mer'], view['raw_tx_reference_mer'])
        self.assertEqual(view['raw_nearest_mer']['sample_count'], view['selection']['selected_cells'])
        self.assertEqual(later['cells'][0]['physical_bin'], original[5]['physical_bin'])
        view['cells'][0]['after_equalization'] = [99, 99]
        self.assertNotEqual(result.qam(query(count=1))['cells'][0]['after_equalization'], [99, 99])

    def test_all_actual_roles_decisions_and_cell_soft_values_keep_reference_coordinates(self):
        result = request().run()
        for stream in ['payload', 'ncp', 'plc_data', 'plc_preamble', 'continuous_pilots', 'scattered_pilots']:
            with self.subTest(stream=stream):
                view = result.qam(query(stream=stream, count=3))
                self.assertEqual(len(view['cells']), 3)
                decisions = result.cell_decisions(query(stream=stream, count=3))
                self.assertEqual(len(decisions['rows']), 3)
                for cell, row in zip(view['cells'], decisions['rows']):
                    self.assertEqual(row['cell'], cell)
                    amplitude = cell['reference_amplitude']
                    self.assertEqual(row['decision']['normalized_iq'], [value/amplitude for value in cell['after_equalization']])
                    if stream.endswith('_pilots'):
                        self.assertEqual(amplitude, 2)
                        self.assertEqual(cell['modulation_order'], 2)
                    if stream not in ['payload', 'ncp']:
                        self.assertEqual(cell['llrs'], [])
                if stream in ['payload', 'ncp']:
                    self.assertGreater(decisions['llr_statistics']['count'], 0)
                else:
                    self.assertEqual(decisions['llr_statistics']['count'], 0)
        self.assertEqual(result.report()['summary']['comparison']['bit_errors'], 0)

    def test_absent_filtered_invalid_and_bounded_views_never_create_history(self):
        for data in [b'{', b'{"unknown":1}', b'{"stream":"unknown"}', b'{"modulation_order":8}',
                     b'{"first_bin":8192}', b'{"first_symbol":3,"last_symbol":2}',
                     b'{"count":2049}', b' ' * 4097]:
            with self.assertRaises(m.ApiError):
                m.FullLoopCellQuery.from_json(data)
        for stream in ['unknown', 'x' * 129]:
            with self.assertRaises(m.ApiError):
                m.FullLoopCellQuery(stream)
        self.assertEqual(m.FullLoopCellQuery('ncp').settings['stream'], 'ncp')
        absent = request(constellation=None, protected_constellation=None).run()
        self.assertEqual(absent.qam()['selection']['retained_cells'], 0)
        self.assertEqual(absent.qam()['cells'], [])
        self.assertEqual(absent.cell_decisions()['rows'], [])
        result = request().run()
        empty = result.qam(query(profile_id=255))
        self.assertGreater(empty['selection']['retained_cells'], 0)
        self.assertEqual(empty['selection']['selected_cells'], 0)
        self.assertEqual(empty['cells'], [])
        self.assertEqual(empty['raw_nearest_mer']['calculation_status'], 'no_samples')
        self.assertIsNone(empty['raw_tx_reference_mer'])
        with self.assertRaises(m.ApiError):
            result.cell_decisions(query(count=129))
        with self.assertRaises(m.ApiError):
            result.qam(query(start=2**64-1))

    @unittest.skipUnless(os.environ.get('MERLIN_FULL_LOOP_PARITY'), 'Direct Rust process reference required')
    def test_direct_rust_process_selected_cells_metrics_and_decisions_match(self):
        expected = json.loads(Path(os.environ['MERLIN_FULL_LOOP_PARITY']).read_text())['cells']
        result = request().run()

        def compare(actual, reference):
            if isinstance(reference, dict):
                self.assertEqual(actual.keys(), reference.keys())
                for key in reference:
                    compare(actual[key], reference[key])
            elif isinstance(reference, list):
                self.assertEqual(len(actual), len(reference))
                for a, b in zip(actual, reference):
                    compare(a, b)
            elif isinstance(reference, float):
                self.assertTrue(math.isclose(actual, reference, rel_tol=1e-10, abs_tol=1e-10), (actual, reference))
            else:
                self.assertEqual(actual, reference)

        compare(result.qam(query(count=5)), expected['qam'])
        compare(result.cell_decisions(query(count=3)), expected['decisions'])
