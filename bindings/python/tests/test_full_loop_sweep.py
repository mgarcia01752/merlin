#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Complete raw catalogs and atomic bounded admission before independently controlled runs."""
import copy
import hashlib
import json
import math
import os
from pathlib import Path
import unittest

import merlin
import merlin.downstream as m
from test_full_loop import FIXTURES, inputs, options

SELECTED_SOURCE_ID = 7
OTHER_CAPTURED_SOURCE_ID = 2
INSPECTION_ONLY_SOURCE_ID = 200
ABSENT_SOURCE_ID = 5
EXPECTED_ASSIGNMENT_COUNT = 14960
EXPECTED_SEQUENTIAL_DURATION_MS = 2
EXPECTED_SEQUENTIAL_RUNTIME_MS = 120000
EXPECTED_RETAINED_WORD_DETAILS = 128
OVER_PROFILE_BUDGET_COUNT = 33
UNAVAILABLE_PLAN_INDEX = 2
OWNED_INFO_MUTATION_MARKER = 999
PARITY_RELATIVE_TOLERANCE = 1e-10
PARITY_ABSOLUTE_TOLERANCE = 1e-10


def catalog(ids):
    """Clone one original supported loading with explicit source IDs and unchanged physical roles."""
    document = json.loads((FIXTURES / 'modulation_profile.json').read_bytes())
    template = next(profile for profile in document['profiles'] if profile['profile_id'] == 0)
    profiles = []
    for profile_id in ids:
        profile = copy.deepcopy(template)
        profile['profile_id'] = profile_id
        profiles.append(profile)
    document['profiles'] = profiles
    document['num_profiles'] = len(profiles)
    data = json.dumps(document).encode()
    return m.ProfileCapture.from_bytes('sweep-catalog', data), data


def prepared():
    """Return an accepted request with the same explicit retained-stage policy as Rust."""
    return m.Simulation.prepare_full_loop(*inputs(), options=options()).with_observations(m.FullLoopObservationPolicy())


class FullLoopSweepTests(unittest.TestCase):
    def test_mixed_catalog_preserves_raw_ids_exact_originals_and_checked_run_loading(self):
        capture, original = catalog([INSPECTION_ONLY_SOURCE_ID, SELECTED_SOURCE_ID, 0, OTHER_CAPTURED_SOURCE_ID])
        profile = capture.simulation_profile(m.ProfileId(SELECTED_SOURCE_ID))
        self.assertEqual(capture.profile_ids, (0, OTHER_CAPTURED_SOURCE_ID, SELECTED_SOURCE_ID, INSPECTION_ONLY_SOURCE_ID))
        self.assertEqual(tuple(profile_id.value for profile_id in profile.profile_ids),
                         (0, OTHER_CAPTURED_SOURCE_ID, SELECTED_SOURCE_ID))
        self.assertEqual(profile.profile_id.value, SELECTED_SOURCE_ID)
        self.assertEqual(capture.provenance['original_sha256'], hashlib.sha256(original).hexdigest())
        self.assertEqual(profile.sources[0]['original_sha256'], capture.provenance['original_sha256'])
        with self.assertRaises(m.ApiError):
            m.ProfileId(INSPECTION_ONLY_SOURCE_ID)
        with self.assertRaises(m.ApiError):
            capture.simulation_profile(m.ProfileId(ABSENT_SOURCE_ID))
        with self.assertRaises(m.ApiError):
            m.DPD.from_bytes('legacy', original, m.ProfileId(0))

    def test_selection_preflight_bounds_owned_info_and_independent_complete_cancelled_runs(self):
        capture, _ = catalog([SELECTED_SOURCE_ID, OTHER_CAPTURED_SOURCE_ID, 0])
        base = prepared()
        plan = m.FullLoopSweepPlan(base, capture, 'selected', [SELECTED_SOURCE_ID, 0])
        self.assertIs(m.FullLoopSweepPlan, merlin.FullLoopSweepPlan)
        info = plan.info
        self.assertEqual(info['selected_profile_ids'], [0, SELECTED_SOURCE_ID])
        self.assertEqual(info['total_profile_assignments'], EXPECTED_ASSIGNMENT_COUNT)
        self.assertEqual(info['requested_observation_duration_ms'], EXPECTED_SEQUENTIAL_DURATION_MS)
        self.assertEqual(info['maximum_sequential_runtime_ms'], EXPECTED_SEQUENTIAL_RUNTIME_MS)
        self.assertEqual(info['maximum_retained_word_details'], EXPECTED_RETAINED_WORD_DETAILS)
        info['selected_profile_ids'][0] = OWNED_INFO_MUTATION_MARKER
        self.assertEqual(plan.info['selected_profile_ids'], [0, SELECTED_SOURCE_ID])
        control = m.FullLoopControl()
        first = plan.request(0).run(control)
        self.assertEqual(first.report()['status'], 'complete')
        self.assertEqual(first.report()['summary']['comparison']['bit_errors'], 0)
        with self.assertRaises(m.ApiError):
            plan.request(1).run(control)
        cancelled = m.FullLoopControl()
        cancelled.cancel()
        partial = plan.request(1).run(cancelled)
        self.assertEqual(partial.report()['status'], 'cancelled')
        self.assertEqual(partial.input_summary()['profile_id'], SELECTED_SOURCE_ID)
        self.assertEqual(first.report()['status'], 'complete')
        self.assertGreater(first.observation_stats()['retained_events'], 0)
        for mode, ids in [('selected', []), ('selected', [0, 0]),
                          ('selected', [ABSENT_SOURCE_ID]), ('selected', [0] * OVER_PROFILE_BUDGET_COUNT),
                          ('selected', None), ('current', [0]), ('all_captured', [0]), ('unknown', None)]:
            with self.assertRaises(m.ApiError):
                m.FullLoopSweepPlan(base, capture, mode, ids)
        with self.assertRaises(m.ApiError):
            plan.request(UNAVAILABLE_PLAN_INDEX)
        mixed, _ = catalog([0, INSPECTION_ONLY_SOURCE_ID])
        with self.assertRaises(m.ApiError) as raised:
            m.FullLoopSweepPlan(base, mixed, 'all_captured')
        self.assertEqual(raised.exception.code, m.ErrorCode.UnsupportedCapability)
        selected = m.FullLoopSweepPlan(base, mixed, 'selected', [0])
        self.assertEqual(selected.info['source_profile_ids'], [0, INSPECTION_ONLY_SOURCE_ID])
        self.assertEqual(m.FullLoopSweepPlan(base, capture).request(0).settings, base.settings)
        self.assertEqual(m.FullLoopSweepPlan(base, capture, 'all_captured').info['selected_profile_ids'],
                         [0, OTHER_CAPTURED_SOURCE_ID, SELECTED_SOURCE_ID])

    @unittest.skipUnless(os.environ.get('MERLIN_FULL_LOOP_PARITY'), 'Direct Rust process reference required')
    def test_separate_rust_process_sweep_info_original_profile_ids_and_actual_science_match(self):
        capture, _ = catalog([SELECTED_SOURCE_ID, OTHER_CAPTURED_SOURCE_ID, 0])
        plan = m.FullLoopSweepPlan(prepared(), capture, 'selected', [SELECTED_SOURCE_ID, 0])
        reference = json.loads(Path(os.environ['MERLIN_FULL_LOOP_PARITY']).read_text())['sweep']
        actual = {'info': plan.info, 'results': []}
        for index in range(len(plan.info['selected_profile_ids'])):
            output = plan.request(index).run()
            report = output.report()
            actual['results'].append({'profile_id': output.input_summary()['profile_id'],
                                      'status': report['status'], 'summary': report['summary']})

        def compare(actual_value, expected_value):
            if isinstance(expected_value, dict):
                self.assertEqual(actual_value.keys(), expected_value.keys())
                for key in expected_value:
                    compare(actual_value[key], expected_value[key])
            elif isinstance(expected_value, list):
                self.assertEqual(len(actual_value), len(expected_value))
                for actual_item, expected_item in zip(actual_value, expected_value):
                    compare(actual_item, expected_item)
            elif isinstance(expected_value, float):
                self.assertTrue(math.isclose(actual_value, expected_value,
                                            rel_tol=PARITY_RELATIVE_TOLERANCE,
                                            abs_tol=PARITY_ABSOLUTE_TOLERANCE), (actual_value, expected_value))
            else:
                self.assertEqual(actual_value, expected_value)
        compare(actual, reference)
