#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Installed native metadata and direct Rust parity; Python performs no sizing."""
import gc
import json
import os
from pathlib import Path
import threading
import time
import unittest

import merlin
import merlin.common as common
import merlin.downstream as downstream
import merlin.upstream as upstream

ROOT = Path(__file__).resolve().parents[3]
# Match the small explicit observation fixture in the independent Rust example.
ESTIMATE_FRAMES = 2
ESTIMATE_SCALARS = 16384
SOFT_CELLS = 8
SOFT_WORDS = 2
SOFT_WORD_LLRS = 32768
CONTROL_DEADLINE_SECONDS = 10
CONTROL_POLL_SECONDS = 0.001


def quantities(value):
    """Read the native immutable result without estimating any storage in Python.

    Args:
        value: Actual Rust-produced RetainedFootprint.
    Returns:
        Named fields for direct comparison with independent Rust-process output.
    """
    return {name: getattr(value, name)
            for name in ('payload_bytes', 'ordered_entries', 'shared_roots')}


def configured(spacing=50, observed=False):
    """Admit the same original document and observation policy as the Rust example.

    Args:
        spacing: Existing tracked 50- or 25-kHz fixture.
        observed: Enable bounded actual estimate and soft stores.
    Returns:
        Idle native upstream session with no Python scientific calculations.
    """
    simulation = upstream.Simulation()
    simulation.configure_json((ROOT / f'examples/cli/upstream/us-{spacing}khz.json').read_bytes())
    if observed:
        policy = upstream.ReceiverObservationPolicy(ESTIMATE_FRAMES, ESTIMATE_SCALARS)
        policy = policy.with_soft_metrics(
            upstream.SoftObservationPolicy(SOFT_CELLS, SOFT_WORDS, SOFT_WORD_LLRS))
        simulation.set_receiver_observation_policy(policy)
    return simulation


class RetainedFootprintTests(unittest.TestCase):
    def test_common_model_is_native_immutable_and_preserves_downstream_alias(self):
        self.assertIs(merlin.Simulation, downstream.Simulation)
        value = upstream.Simulation().retained_footprint()
        self.assertIs(type(value), common.RetainedFootprint)
        self.assertGreater(value.payload_bytes, 0)
        for name in quantities(value):
            with self.assertRaises(AttributeError):
                setattr(value, name, 0)
        with self.assertRaises(TypeError):
            common.RetainedFootprint()

    def test_queries_preserve_science_provenance_and_detached_metadata_lifetime(self):
        simulation = configured(observed=True)
        source = simulation.document().original_bytes()
        before = quantities(simulation.retained_footprint())
        report = simulation.run()
        original = report.to_json()
        value = report.retained_footprint()
        metadata = quantities(value)
        self.assertGreater(metadata['payload_bytes'], 0)
        self.assertGreater(metadata['ordered_entries'], 0)
        self.assertGreater(metadata['shared_roots'], 0)
        self.assertTrue(report.reference_recovered)
        self.assertEqual(simulation.document().original_bytes(), source)
        self.assertEqual(quantities(simulation.retained_footprint()), before)
        self.assertEqual(quantities(report.retained_footprint()), metadata)
        self.assertEqual(report.to_json(), original)
        simulation.configure_json((ROOT / 'examples/cli/upstream/us-25khz.json').read_bytes())
        del simulation, report
        gc.collect()
        self.assertEqual(quantities(value), metadata)

    def test_rejected_configuration_preserves_accepted_footprint(self):
        simulation = configured()
        before = quantities(simulation.retained_footprint())
        with self.assertRaises(upstream.ApiError):
            simulation.configure_json(b'{}')
        self.assertEqual(quantities(simulation.retained_footprint()), before)

    def test_paused_running_session_returns_busy_and_cancel_restores_metadata(self):
        simulation = configured(observed=True)
        before = quantities(simulation.retained_footprint())
        control = upstream.RunControl()
        control.pause()
        reports, failures = [], []

        def run():
            try:
                reports.append(simulation.run_with_control(control))
            except BaseException as failure:
                failures.append(failure)

        worker = threading.Thread(target=run)
        worker.start()
        try:
            deadline = time.monotonic() + CONTROL_DEADLINE_SECONDS
            while json.loads(control.progress_json())['status'] != 'Paused':
                self.assertLess(time.monotonic(), deadline)
                self.assertTrue(worker.is_alive())
                time.sleep(CONTROL_POLL_SECONDS)
            with self.assertRaises(upstream.ApiError) as rejected:
                simulation.retained_footprint()
            self.assertEqual(rejected.exception.code, common.ErrorCode.Busy)
        finally:
            control.cancel()
            worker.join(CONTROL_DEADLINE_SECONDS)
        self.assertFalse(worker.is_alive())
        self.assertEqual(failures, [])
        self.assertEqual(len(reports), 1)
        self.assertEqual(reports[0].status, 'Cancelled')
        self.assertGreater(reports[0].retained_footprint().payload_bytes, 0)
        self.assertEqual(quantities(simulation.retained_footprint()), before)

    @unittest.skipUnless(os.environ.get('MERLIN_RETAINED_FOOTPRINT_REFERENCE'),
                         'Separate current Rust-process metadata reference required')
    def test_installed_native_complete_partial_and_private_store_parity(self):
        reference = json.loads(Path(os.environ['MERLIN_RETAINED_FOOTPRINT_REFERENCE']).read_text())
        self.assertEqual(quantities(upstream.Simulation().retained_footprint()), reference['idle'])
        cases = iter(reference['cases'])
        for spacing in [50, 25]:
            for observed in [False, True]:
                for cancelled in [False, True]:
                    expected = next(cases)
                    simulation = configured(spacing, observed)
                    self.assertEqual(quantities(simulation.retained_footprint()), expected['session'])
                    control = upstream.RunControl()
                    if cancelled:
                        control.cancel()
                    report = simulation.run_with_control(control)
                    self.assertEqual(quantities(report.retained_footprint()), expected['report'])
                    self.assertEqual(report.status, expected['status'])
                    self.assertEqual(report.reference_recovered, expected['recovered'])
                    self.assertEqual(expected['observed'], observed)
                    self.assertEqual(expected['cancelled'], cancelled)
        self.assertIsNone(next(cases, None))


if __name__ == '__main__':
    unittest.main()
