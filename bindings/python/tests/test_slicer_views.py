#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Installed static references and actual soft views remain authoritative Rust results."""
import json
import math
import os
from pathlib import Path
import unittest

import merlin.downstream as m
from test_full_loop import request


class SlicerViewTests(unittest.TestCase):
    def test_static_reference_labels_scaling_region_and_finite_limits(self):
        reference = m.Simulation.constellation_reference(2, amplitude=2)
        self.assertEqual([point['iq'] for point in reference['points']], [[2, 0], [-2, 0]])
        self.assertEqual([point['label'] for point in reference['points']], [0, 1])
        region = m.Simulation.decision_region(2, 0)
        self.assertEqual(region['center_iq'], [1, 0])
        self.assertEqual(region['viewport_half_extent'], 5)
        self.assertEqual(region['boundary_tie_rule'], 'lowest_label')
        self.assertEqual(min(point[0] for point in region['vertices_relative']), -1)
        self.assertEqual(max(point[0] for point in region['vertices_relative']), 5)
        page = m.Simulation.constellation_reference(16384, start=4094, count=5)
        self.assertEqual([point['label'] for point in page['points']], list(range(4094, 4099)))
        self.assertEqual(len(m.Simulation.constellation_reference(16384)['points']), 4096)
        for operation in [lambda: m.Simulation.constellation_reference(8),
                          lambda: m.Simulation.constellation_reference(16384, count=4097),
                          lambda: m.Simulation.constellation_reference(16, start=16),
                          lambda: m.Simulation.decision_region(16, 16),
                          lambda: m.Simulation.decision_region(16, 0, amplitude=math.nan),
                          lambda: m.Simulation.decision_region(16, 0, amplitude=math.inf),
                          lambda: m.Simulation.decision_region(16, 0, amplitude=0)]:
            with self.assertRaises(m.ApiError):
                operation()
        reference['points'][0]['iq'][0] = 99
        self.assertEqual(m.Simulation.constellation_reference(2, amplitude=2)['points'][0]['iq'], [2, 0])

    def test_actual_cell_and_decoder_soft_sequences_keep_distinct_original_extents(self):
        base = request()
        with self.assertRaises(m.ApiError):
            base.run().decoder_soft_view()
        result = base.with_observations(m.FullLoopObservationPolicy()).run()
        query = m.FullLoopCellQuery.from_json(b'{"count":3}')
        cells = result.cell_soft_view(query)
        values = [value for cell in result.qam(query)['cells'] for value in cell['llrs']]
        self.assertEqual(cells['source_plane'], 'post_descrambling_cell_llrs')
        self.assertEqual(cells['view']['statistics']['count'], len(values))
        self.assertEqual(cells['view']['full']['total_values'], len(values))
        for key in ['full', 'near_zero']:
            self.assertEqual(len(cells['view'][key]['bins']), 40)
            self.assertEqual(sum(item['count'] for item in cells['view'][key]['bins']), cells['view'][key]['shown_values'])
        decoder = result.decoder_soft_view()
        self.assertEqual(decoder['stage'], 'rx.bits.inverse')
        self.assertEqual(decoder['element_start'], 0)
        self.assertEqual(decoder['total_elements'], 16200)
        self.assertEqual(decoder['view']['statistics'], result.llr_statistics()['statistics'])
        self.assertEqual(decoder['view']['sequence'][-1]['last'], 16199)
        self.assertLessEqual(len(decoder['view']['sequence']), 1024)
        self.assertEqual(decoder['view']['sign_convention'], 'log_p_zero_over_p_one')
        view = decoder['view']
        self.assertEqual(view['full']['shown_values']+view['negative_limit_count']+view['positive_limit_count'], view['statistics']['count'])
        pilot = result.cell_soft_view(m.FullLoopCellQuery('continuous_pilots'))
        self.assertEqual(pilot['view']['statistics']['count'], 0)
        self.assertEqual(pilot['view']['sequence'], [])
        with self.assertRaises(m.ApiError):
            result.decoder_soft_view(2**64-1)
        decoder['view']['sequence'][0]['minimum'] = 999
        self.assertNotEqual(result.decoder_soft_view()['view']['sequence'][0]['minimum'], 999)
        self.assertEqual(result.report()['summary']['comparison']['bit_errors'], 0)

    @unittest.skipUnless(os.environ.get('MERLIN_FULL_LOOP_PARITY'), 'Direct Rust process reference required')
    def test_direct_rust_process_reference_region_and_actual_soft_views_match(self):
        reference = json.loads(Path(os.environ['MERLIN_FULL_LOOP_PARITY']).read_text())['slicer']
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
                self.assertTrue(math.isclose(actual, expected, rel_tol=1e-10, abs_tol=1e-10), (actual, expected))
            else:
                self.assertEqual(actual, expected)

        compare(m.Simulation.constellation_reference(128, amplitude=2, start=3, count=5), reference['reference'])
        compare(m.Simulation.decision_region(128, 0), reference['region'])
        compare(result.cell_soft_view(m.FullLoopCellQuery.from_json(b'{"count":3}')), reference['cells'])
        compare(result.decoder_soft_view(), reference['decoder'])
