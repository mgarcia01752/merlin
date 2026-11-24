#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Installed downstream root metadata; all storage calculations remain Rust-owned."""
import gc
import json
import os
from pathlib import Path
import unittest

import merlin
import merlin.common as common
import merlin.downstream as downstream

ROOT = Path(__file__).resolve().parents[3]
PROJECTS = ROOT / 'examples/workbench'


def quantities(value):
    """Read the immutable native quantities without estimating Python storage.

    Args:
        value: Rust-produced RetainedFootprint for one actual existing owner.
    Returns:
        The three native quantities for independent Rust-process comparison.
    """
    return {name: getattr(value, name)
            for name in ('payload_bytes', 'ordered_entries', 'shared_roots')}


def prepare(project='single', observed=False):
    """Admit original saved bytes through the same public Rust project owner.

    Args:
        project: Tracked saved-project fixture directory.
        observed: Enable the existing validated default observation policy.
    Returns:
        Idle plan and one request without advancing a source or simulation.
    """
    plan = downstream.Workbench.prepare_full_loop(
        (PROJECTS / project / 'workbench.merlin.json').read_bytes())
    request = plan.request(0)
    if observed:
        request = request.with_observations(downstream.FullLoopObservationPolicy())
    return plan, request


class DownstreamRetainedFootprintTests(unittest.TestCase):
    def test_legacy_aliases_metadata_and_detached_archive_lifetime(self):
        self.assertIs(merlin.Simulation, downstream.Simulation)
        for name in ('FullLoopRequest', 'FullLoopSweepPlan', 'FullLoopResult',
                     'FullLoopReceiverArchive'):
            self.assertIs(getattr(merlin, name), getattr(downstream, name))
        plan, request = prepare(observed=True)
        settings = request.settings
        before = quantities(request.retained_footprint())
        output = request.run()
        report = output.report()
        provenance = output.input_summary()
        archive = output.receiver_archive()
        historical = archive.info
        metadata = []
        for owner in (plan, request, output, archive):
            value = owner.retained_footprint()
            self.assertIs(type(value), common.RetainedFootprint)
            self.assertGreater(value.payload_bytes, 0)
            self.assertGreater(value.shared_roots, 0)
            metadata.append(value)
            with self.assertRaises(AttributeError):
                value.payload_bytes = 0
        self.assertEqual(quantities(request.retained_footprint()), before)
        self.assertEqual(request.settings, settings)
        self.assertEqual(output.report(), report)
        self.assertEqual(output.input_summary(), provenance)
        self.assertEqual(archive.info, historical)
        self.assertGreater(historical['payload_cells'], 0)
        self.assertGreater(historical['decoder_llrs'], 0)
        snapshots = [quantities(value) for value in metadata]
        del plan, request, output, owner
        gc.collect()
        self.assertEqual(archive.info, historical)
        self.assertEqual(quantities(archive.retained_footprint()), snapshots[-1])
        del archive
        gc.collect()
        self.assertEqual([quantities(value) for value in metadata], snapshots)

    def test_rejected_candidate_preserves_plan_and_original_request(self):
        plan, request = prepare('multi-rxmer')
        before = quantities(plan.retained_footprint()), quantities(request.retained_footprint())
        with self.assertRaises(downstream.ApiError):
            plan.request(len(plan.info['selected_profile_ids']))
        with self.assertRaises(downstream.ApiError):
            request.with_runtime_limit(0)
        self.assertEqual((quantities(plan.retained_footprint()),
                          quantities(request.retained_footprint())), before)

    @unittest.skipUnless(os.environ.get('MERLIN_DOWNSTREAM_RETAINED_REFERENCE'),
                         'Separate current Rust-process metadata reference required')
    def test_complete_partial_private_cache_and_archive_rust_parity(self):
        reference = json.loads(Path(os.environ['MERLIN_DOWNSTREAM_RETAINED_REFERENCE']).read_text())
        cases = iter(reference['cases'])
        for project in ('single', 'multi-rxmer'):
            for observed in (False, True):
                for cancelled in (False, True):
                    expected = next(cases)
                    self.assertEqual((expected['project'], expected['observed'], expected['cancelled']),
                                     (project, observed, cancelled))
                    plan, request = prepare(project, observed)
                    self.assertEqual(quantities(plan.retained_footprint()), expected['plan'])
                    self.assertEqual(quantities(request.retained_footprint()), expected['request'])
                    control = downstream.FullLoopControl()
                    if cancelled:
                        control.cancel()
                    output = request.run(control)
                    archive = output.receiver_archive()
                    self.assertEqual(quantities(output.retained_footprint()), expected['result'])
                    self.assertEqual(quantities(archive.retained_footprint()), expected['archive'])
                    self.assertEqual(archive.info, expected['archive_info'])
                    self.assertEqual(output.report()['status'], expected['status'])
                    self.assertEqual(output.input_summary(), expected['inputs'])
                    self.assertEqual(output.fec_recovery(), expected['recovery'])
        self.assertIsNone(next(cases, None))


if __name__ == '__main__':
    unittest.main()
