#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Installed bounded targets: real Rust science, ownership and worker lifecycle."""
import gc
import json
import os
from pathlib import Path
import threading
import time
import unittest

import merlin.common as common
import merlin.downstream as ds
import merlin.upstream as us
from test_downstream_retained_footprint import prepare, quantities
from test_retained_footprint import configured

ROOT = Path(__file__).resolve().parents[3]
PROFILE = ROOT / 'examples/measurements/single-channel/modulation_profile.json'
# Literal original source, identical to the separate Rust process fixture.
FILE_BYTES = b'\xef\xbb\xbfbounded\0source\r\n'
# Independent valid PRBS15 recurrence seed for this preview fixture.
PREVIEW_SEED = 7
# Small result projection; independent of retained populations and PSD resolution.
SAMPLE_PREFIX = 3
# Host test safety deadline; unrelated to simulation active runtime.
CONTROL_DEADLINE_SECONDS = 15
# Brief native progress polling cadence, unrelated to a scientific time step.
CONTROL_POLL_SECONDS = 0.001


def fits(test, actual, bound):
    """Compare three Rust quantities without estimating storage in Python.

    Args:
        test: Current unittest assertion owner.
        actual: Actual native root metadata.
        bound: Public native preflight reservation.
    Returns:
        None after independently comparing every dimension.
    """
    for name, value in quantities(actual).items():
        test.assertLessEqual(value, quantities(bound)[name], name)


def stable(value):
    """Exclude only execution identity and host elapsed time from science parity.

    Args:
        value: Original Rust-produced report dictionary/list/scalar.
    Returns:
        Detached comparison projection; original output remains untouched.
    """
    if isinstance(value, list):
        return [stable(child) for child in value]
    if isinstance(value, dict):
        ignored = {'elapsed_ms'}
        if 'generation' in value and 'grant' in value:
            ignored.add('run')
        return {key: stable(child) for key, child in value.items() if key not in ignored}
    return value


def preview_request(file=False):
    """Prepare the original catalog and one passive source through Rust.

    Args:
        file: Select exact finite bytes instead of continuous PRBS15.
    Returns:
        Reusable idle request and independently retained source.
    """
    source = (ds.SourceInput.from_bytes('source', FILE_BYTES) if file else
              ds.SourceInput.prbs15(ds.PrbsSeed(PREVIEW_SEED)))
    capture = ds.ProfileCapture.from_bytes('profile', PROFILE.read_bytes())
    return ds.TransmitterPreviewRequest.from_capture(
        ds.TransmissionConfiguration(), capture, 0, source), source


class BoundedSessionOutputTests(unittest.TestCase):
    def test_downstream_dimensions_reject_before_control_and_preserve_complete_science(self):
        _, request = prepare(observed=True)
        bound = request.result_retained_bound()
        before = request.settings
        for name, value in quantities(bound).items():
            self.assertGreater(value, 0)
            control = ds.FullLoopControl()
            limit = common.RetainedFootprint(**{**quantities(bound), name: value - 1})
            with self.assertRaises(ds.ApiError) as rejected:
                request.run_bounded(limit, control)
            self.assertEqual(rejected.exception.code, common.ErrorCode.ResourceLimit)
            self.assertEqual(control.snapshot()['state'], 'idle')
            self.assertEqual(request.settings, before)
        legacy = request.run()
        output = request.run_bounded(bound)
        fits(self, output.retained_footprint(), bound)
        self.assertEqual(stable(output.report()), stable(legacy.report()))
        self.assertEqual(output.fec_recovery(), legacy.fec_recovery())
        self.assertEqual(output.stage_inventory(), legacy.stage_inventory())
        snapshot = output.input_summary(), output.fec_recovery()
        del request, legacy
        gc.collect()
        self.assertEqual((output.input_summary(), output.fec_recovery()), snapshot)

    def test_downstream_bounded_worker_keeps_gil_busy_and_cancel_contracts(self):
        _, request = prepare(observed=True)
        bound = request.result_retained_bound()
        control = ds.FullLoopControl()
        control.pause()
        results, failures = [], []

        def run():
            try:
                results.append(request.run_bounded(bound, control))
            except BaseException as failure:
                failures.append(failure)

        worker = threading.Thread(target=run)
        worker.start()
        try:
            deadline = time.monotonic() + CONTROL_DEADLINE_SECONDS
            while control.snapshot()['state'] != 'paused':
                self.assertLess(time.monotonic(), deadline)
                self.assertTrue(worker.is_alive())
                time.sleep(CONTROL_POLL_SECONDS)
            with self.assertRaises(ds.ApiError) as rejected:
                request.run_bounded(bound, control)
            self.assertEqual(rejected.exception.code, common.ErrorCode.Busy)
        finally:
            control.cancel()
            worker.join(CONTROL_DEADLINE_SECONDS)
        self.assertFalse(worker.is_alive())
        self.assertEqual(failures, [])
        self.assertEqual(len(results), 1)
        self.assertEqual(results[0].report()['status'], 'cancelled')
        fits(self, results[0].retained_footprint(), bound)

    def test_upstream_dimensions_reject_and_restore_idle_session_lease(self):
        simulation = configured(observed=True)
        bound = simulation.batch_report_retained_bound()
        before = quantities(simulation.retained_footprint())
        original = simulation.document().original_bytes()
        for name, value in quantities(bound).items():
            self.assertGreater(value, 0)
            control = us.RunControl()
            progress = control.progress_json()
            limit = common.RetainedFootprint(**{**quantities(bound), name: value - 1})
            with self.assertRaises(us.ApiError) as rejected:
                simulation.run_with_control_bounded(control, limit)
            self.assertEqual(rejected.exception.code, common.ErrorCode.ResourceLimit)
            self.assertEqual(control.progress_json(), progress)
            self.assertEqual(quantities(simulation.retained_footprint()), before)
            self.assertEqual(simulation.document().original_bytes(), original)
        legacy = simulation.run()
        output = simulation.run_with_control_bounded(us.RunControl(), bound)
        fits(self, output.retained_footprint(), bound)
        self.assertEqual(stable(json.loads(output.to_json())), stable(json.loads(legacy.to_json())))
        report = output.to_json()
        del simulation, legacy
        gc.collect()
        self.assertEqual(output.to_json(), report)

    def test_upstream_bounded_worker_keeps_gil_busy_cancel_and_reusable_session(self):
        simulation = configured(observed=True)
        bound = simulation.batch_report_retained_bound()
        before = quantities(simulation.retained_footprint())
        control = us.RunControl()
        control.pause()
        results, failures = [], []

        def run():
            try:
                results.append(simulation.run_with_control_bounded(control, bound))
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
            with self.assertRaises(us.ApiError) as rejected:
                simulation.batch_report_retained_bound()
            self.assertEqual(rejected.exception.code, common.ErrorCode.Busy)
        finally:
            control.cancel()
            worker.join(CONTROL_DEADLINE_SECONDS)
        self.assertFalse(worker.is_alive())
        self.assertEqual(failures, [])
        self.assertEqual(len(results), 1)
        self.assertEqual(results[0].status, 'Cancelled')
        fits(self, results[0].retained_footprint(), bound)
        self.assertEqual(quantities(simulation.retained_footprint()), before)

    def test_preview_exact_bound_rejection_precancel_and_original_source(self):
        request, source = preview_request(file=True)
        bound = request.preview_retained_bound()
        self.assertEqual(bound.ordered_entries, 0)
        for name in ('payload_bytes', 'shared_roots'):
            control = ds.TransmitterPreviewControl()
            limit = common.RetainedFootprint(**{**quantities(bound), name: getattr(bound, name) - 1})
            with self.assertRaises(ds.ApiError) as rejected:
                request.generate_bounded(limit, control)
            self.assertEqual(rejected.exception.code, common.ErrorCode.ResourceLimit)
            self.assertEqual(control.snapshot()['state'], 'idle')
        cancelled = ds.TransmitterPreviewControl()
        cancelled.cancel()
        with self.assertRaises(ds.ApiError):
            request.generate_bounded(bound, cancelled)
        self.assertEqual(cancelled.snapshot()['state'], 'cancelled')
        legacy = request.generate()
        output = request.generate_bounded(bound)
        self.assertEqual(quantities(output.retained_footprint()), quantities(bound))
        self.assertEqual(output.info(), legacy.info())
        self.assertEqual(output.sample_view(), legacy.sample_view())
        self.assertEqual(output.spectrum(), legacy.spectrum())
        self.assertEqual(source.original_bytes, FILE_BYTES)
        self.assertTrue(output.info()['source_exhausted'])

    @unittest.skipUnless(os.environ.get('MERLIN_BOUNDED_SESSION_REFERENCE'),
                         'Separate current Rust-process bound/science reference required')
    def test_all_bounded_products_match_separate_rust_process(self):
        reference = json.loads(Path(os.environ['MERLIN_BOUNDED_SESSION_REFERENCE']).read_text())
        self.assertEqual(tuple(map(len, (reference['downstream'], reference['upstream'],
                                         reference['transmitter']))), (8, 8, 2))
        for case in reference['downstream']:
            _, request = prepare(case['project'], case['observed'])
            bound = request.result_retained_bound()
            control = ds.FullLoopControl()
            if case['cancelled']:
                control.cancel()
            result = request.run_bounded(bound, control)
            self.assertEqual(quantities(bound), case['bound'])
            self.assertEqual(quantities(result.retained_footprint()), case['actual'])
            self.assertEqual(stable(result.report()), stable(case['report']))
            self.assertEqual(result.input_summary(), case['inputs'])
            self.assertEqual(result.fec_recovery(), case['recovery'])
            self.assertEqual(result.stage_inventory(), case['inventory'])
            self.assertEqual(result.observation_stats(), case['stats'])
        for case in reference['upstream']:
            simulation = configured(case['spacing'], case['observed'])
            bound = simulation.batch_report_retained_bound()
            control = us.RunControl()
            if case['cancelled']:
                control.cancel()
            report = simulation.run_with_control_bounded(control, bound)
            self.assertEqual(quantities(bound), case['bound'])
            self.assertEqual(quantities(report.retained_footprint()), case['actual'])
            self.assertEqual(stable(json.loads(report.to_json())), stable(case['report']))
        for case in reference['transmitter']:
            request, _ = preview_request(case['file'])
            bound = request.preview_retained_bound()
            preview = request.generate_bounded(bound)
            self.assertEqual(quantities(bound), case['bound'])
            self.assertEqual(quantities(preview.retained_footprint()), case['actual'])
            self.assertEqual(preview.info(), case['info'])
            self.assertEqual(preview.sample_view(0, SAMPLE_PREFIX), case['samples'])
            self.assertEqual(preview.spectrum(), case['spectrum'])


if __name__ == '__main__':
    unittest.main()
