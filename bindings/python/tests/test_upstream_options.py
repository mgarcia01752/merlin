#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Accepted options snapshots delegate to Rust, including ownership and Busy."""
import inspect
import json
import os
from pathlib import Path
import threading
import time
import unittest

import merlin.upstream as u

ROOT = Path(__file__).resolve().parents[3]

# Independent finite fixtures, matching the separately executed Rust test.
OVERRIDE_GRANTS = 2
OVERRIDE_CHUNK_SAMPLES = 313
OVERRIDE_TIMEOUT_MS = 90_000
OVERRIDE_RETAINED_POINTS = 29
CONTROL_DEADLINE_SECONDS = 10
CONTROL_POLL_SECONDS = 0.001


class UpstreamOptions(unittest.TestCase):
    def test_passive_owned_snapshot_atomic_rejection_and_independent_rust_parity(self):
        references = json.loads(Path(os.environ['MERLIN_OPTIONS_RUST_REFERENCE_JSON']).read_text())
        self.assertEqual(str(inspect.signature(u.Simulation.options)), '(self, /)')
        for index, spacing in enumerate([50, 25]):
            with self.subTest(spacing=spacing):
                simulation = u.Simulation()
                initial = simulation.options()
                self.assertIsInstance(initial, u.ExperimentOptions)
                source = (ROOT / f'examples/cli/upstream/us-{spacing}khz.json').read_bytes()
                simulation.configure_json(source)
                document = simulation.document().to_json()
                configuration = simulation.configuration()
                document_options = simulation.options()
                simulation.set_options(u.ExperimentOptions(
                    grants=OVERRIDE_GRANTS, chunk_samples=OVERRIDE_CHUNK_SAMPLES,
                    timeout_ms=OVERRIDE_TIMEOUT_MS, retained_points=OVERRIDE_RETAINED_POINTS,
                    retained_taps=['TxOutput']))
                footprint = simulation.retained_footprint()
                quantities = (footprint.payload_bytes, footprint.ordered_entries, footprint.shared_roots)
                snapshot = simulation.options()
                after = simulation.retained_footprint()
                self.assertEqual((after.payload_bytes, after.ordered_entries, after.shared_roots), quantities)
                self.assertEqual(simulation.document().to_json(), document)
                with self.assertRaises(u.ApiError):
                    simulation.set_options(u.ExperimentOptions(grants=0))
                native = json.loads(simulation.run().to_json())
                reference = references[index]
                for key in ['configuration', 'options', 'planned', 'status',
                            'generated_source_bits', 'compared_source_bits', 'bit_errors',
                            'ber', 'reference_recovered', 'not_started_grants']:
                    self.assertEqual(native[key], reference[key], key)
                self.assertEqual([grant['words'] for grant in native['grants']],
                                 [grant['words'] for grant in reference['grants']])
                simulation.set_options(document_options)
                del simulation
                restored = u.Simulation(snapshot)
                restored.configure(configuration)
                self.assertEqual(json.loads(restored.run().to_json())['options'], reference['options'])
                self.assertIsInstance(u.Simulation(initial).options(), u.ExperimentOptions)

    def test_busy_during_acknowledged_pause_and_restored_after_cancel(self):
        simulation = u.Simulation()
        simulation.configure_json((ROOT / 'examples/cli/upstream/us-50khz.json').read_bytes())
        before = simulation.document().to_json()
        control = u.RunControl()
        control.pause()
        reports, failures = [], []

        def run():
            try:
                reports.append(simulation.run_with_control(control))
            except BaseException as cause:
                failures.append(cause)

        worker = threading.Thread(target=run)
        worker.start()
        try:
            deadline = time.monotonic() + CONTROL_DEADLINE_SECONDS
            while json.loads(control.progress_json())['status'] != 'Paused':
                self.assertLess(time.monotonic(), deadline)
                time.sleep(CONTROL_POLL_SECONDS)
            with self.assertRaises(u.ApiError) as busy:
                simulation.options()
            self.assertEqual(busy.exception.code, u.ErrorCode.Busy)
        finally:
            control.cancel()
            worker.join(CONTROL_DEADLINE_SECONDS)
        self.assertFalse(worker.is_alive())
        self.assertFalse(failures, failures)
        self.assertEqual(reports[0].status, 'Cancelled')
        snapshot = simulation.options()
        self.assertIsInstance(snapshot, u.ExperimentOptions)
        self.assertEqual(simulation.document().to_json(), before)


if __name__ == '__main__':
    unittest.main()
