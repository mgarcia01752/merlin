#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Installed native control metadata uses Rust quantities and preserves lifecycle."""
import gc
import json
import os
from pathlib import Path
import threading
import time
import unittest

import merlin
from merlin import common, downstream, upstream

ROOT = Path(__file__).resolve().parents[3]
PAUSE_ACK_DEADLINE_SECONDS = 10
WORKER_JOIN_TIMEOUT_SECONDS = 10
PROGRESS_POLL_INTERVAL_SECONDS = 0.001


def quantities(value):
    """Read the native result without calculating storage in Python.

    Args:
        value: Rust-produced immutable retained metadata.
    Returns:
        The three public quantities for direct Rust-process comparison.
    """
    return {name: getattr(value, name)
            for name in ('payload_bytes', 'ordered_entries', 'shared_roots')}


def observe(phase, full_loop, transmitter, plain_upstream, shared_upstream):
    """Project current public operations exactly as the independent Rust example.

    Args:
        phase: Lifecycle observation label shared with the reference.
        full_loop: Existing downstream full-loop control.
        transmitter: Independent downstream preview control.
        plain_upstream: Directional upstream control without a common token.
        shared_upstream: Directional upstream control sharing a common token.
    Returns:
        Rust-derived metadata and bounded idle lifecycle observations.
    """
    return {
        'phase': phase,
        'full_loop': {'footprint': quantities(full_loop.retained_footprint()),
                      'snapshot': full_loop.snapshot(),
                      'age_ms': full_loop.last_progress_age_ms()},
        'transmitter': {'footprint': quantities(transmitter.retained_footprint()),
                        'snapshot': transmitter.snapshot()},
        'upstream': {'footprint': quantities(plain_upstream.retained_footprint()),
                     'progress': json.loads(plain_upstream.progress_json()),
                     'is_cancelled': plain_upstream.is_cancelled()},
        'upstream_with_common': {
            'footprint': quantities(shared_upstream.retained_footprint()),
            'progress': json.loads(shared_upstream.progress_json()),
            'is_cancelled': shared_upstream.is_cancelled()},
    }


class ControlFootprintTests(unittest.TestCase):
    def test_direct_rust_process_matches_quantities_and_idle_control_observations(self):
        reference = Path(os.environ['MERLIN_CONTROL_FOOTPRINT_REFERENCE'])
        full_loop = downstream.FullLoopControl()
        transmitter = downstream.TransmitterPreviewControl()
        plain_upstream = upstream.RunControl()
        stop = common.RunControl()
        shared_upstream = upstream.RunControl(stop)
        observations = [observe('idle', full_loop, transmitter, plain_upstream, shared_upstream)]
        full_loop.pause()
        plain_upstream.pause()
        shared_upstream.pause()
        observations.append(observe('pause_requested', full_loop, transmitter, plain_upstream, shared_upstream))
        full_loop.resume()
        plain_upstream.resume()
        shared_upstream.resume()
        observations.append(observe('resumed', full_loop, transmitter, plain_upstream, shared_upstream))
        full_loop.cancel()
        transmitter.cancel()
        plain_upstream.cancel()
        stop.cancel()
        observations.append(observe('cancelled', full_loop, transmitter, plain_upstream, shared_upstream))
        self.assertEqual(observations, json.loads(reference.read_bytes()))
        self.assertIs(merlin.FullLoopControl, downstream.FullLoopControl)
        self.assertIs(merlin.TransmitterPreviewControl, downstream.TransmitterPreviewControl)
        self.assertIs(merlin.Simulation, downstream.Simulation)

    def test_metadata_is_native_immutable_owned_and_rejects_extra_arguments(self):
        controls = [downstream.FullLoopControl(), downstream.TransmitterPreviewControl(),
                    upstream.RunControl(), upstream.RunControl(common.RunControl())]
        metadata = [control.retained_footprint() for control in controls]
        expected = [quantities(value) for value in metadata]
        for control, value in zip(controls, metadata):
            self.assertIs(type(value), common.RetainedFootprint)
            with self.assertRaises(TypeError):
                control.retained_footprint(None)
            for field in quantities(value):
                with self.assertRaises(AttributeError):
                    setattr(value, field, 0)
        del control, controls
        gc.collect()
        self.assertEqual([quantities(value) for value in metadata], expected)

    def test_paused_native_worker_keeps_control_metadata_and_cancelled_report(self):
        simulation = upstream.Simulation()
        simulation.configure_json((ROOT / 'examples/cli/upstream/us-50khz.json').read_bytes())
        accepted = quantities(simulation.retained_footprint())
        stop = common.RunControl()
        control = upstream.RunControl(stop)
        baseline = quantities(control.retained_footprint())
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
            deadline = time.monotonic() + PAUSE_ACK_DEADLINE_SECONDS
            while json.loads(control.progress_json())['status'] != 'Paused':
                self.assertLess(time.monotonic(), deadline)
                self.assertTrue(worker.is_alive())
                time.sleep(PROGRESS_POLL_INTERVAL_SECONDS)
            progress = json.loads(control.progress_json())
            self.assertEqual(quantities(control.retained_footprint()), baseline)
            self.assertEqual(json.loads(control.progress_json()), progress)
            with self.assertRaises(upstream.ApiError) as busy:
                simulation.retained_footprint()
            self.assertEqual(busy.exception.code, common.ErrorCode.Busy)
        finally:
            stop.cancel()
            worker.join(WORKER_JOIN_TIMEOUT_SECONDS)
        self.assertFalse(worker.is_alive())
        self.assertEqual(failures, [])
        self.assertEqual(reports[0].status, 'Cancelled')
        self.assertEqual(reports[0].generated_source_bits, 0)
        original = reports[0].to_json()
        self.assertEqual(quantities(control.retained_footprint()), baseline)
        self.assertEqual(quantities(simulation.retained_footprint()), accepted)
        self.assertEqual(reports[0].to_json(), original)


if __name__ == '__main__':
    unittest.main()
