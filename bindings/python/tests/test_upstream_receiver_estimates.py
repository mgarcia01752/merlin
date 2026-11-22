#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Actual received-pilot retention through installed Rust bindings; no scientific Python path."""
import inspect
import json
import os
from pathlib import Path
import threading
import time
import unittest

import merlin.upstream as u
from test_upstream_configured_response import configured
import test_upstream_recovery_views as fixture

ROOT = Path(__file__).resolve().parents[3]
RETAINED_FRAME_BUDGET = 3
RETENTION_SCALAR_BUDGET = 262144
ONE_50KHZ_ESTIMATE_SCALARS = 4096
ONE_25KHZ_ESTIMATE_SCALARS = 8192
FFT_50KHZ_SAMPLES = 2048
FFT_25KHZ_SAMPLES = 4096
QUERY_POINT_CEILING = 4096
OVER_QUERY_POINT_CEILING = 4097
OVER_FRAME_CEILING = 9
OVER_SCALAR_CEILING = 262145
MULTI_GRANT_COUNT = 3
ZOOM_POINT_COUNT = 3
NOISE_VARIANCE = 1e-4
NUMERICAL_TOLERANCE = 1e-11
CONTROL_DEADLINE_SECONDS = 10
CONTROL_POLL_SECONDS = 0.001


def enable(simulation, frames=RETAINED_FRAME_BUDGET, scalars=RETENTION_SCALAR_BUDGET):
    """Apply a typed immutable observation policy through the accepted Rust session."""
    simulation.set_receiver_observation_policy(u.ReceiverObservationPolicy(frames, scalars))


class UpstreamReceiverEstimates(unittest.TestCase):
    def assert_iq(self, actual, real, imaginary):
        self.assertAlmostEqual(actual['re'], real, delta=NUMERICAL_TOLERANCE)
        self.assertAlmostEqual(actual['im'], imaginary, delta=NUMERICAL_TOLERANCE)

    def test_policy_validation_defaults_and_atomic_configuration_mode_checks(self):
        self.assertEqual(u.ReceiverObservationPolicy().info(),
                         {'maximum_frames': 0, 'maximum_scalar_slots': 0})
        signature = inspect.signature(u.BatchReport.receiver_estimate)
        self.assertEqual(signature.parameters['maximum_points'].default, QUERY_POINT_CEILING)
        for frames, scalars in [(0, 1), (1, 0), (OVER_FRAME_CEILING, RETENTION_SCALAR_BUDGET),
                               (1, OVER_SCALAR_CEILING)]:
            with self.assertRaises(u.ApiError):
                u.ReceiverObservationPolicy(frames, scalars)
        simulation = configured()
        before = simulation.configured_response()
        enable(simulation, scalars=ONE_50KHZ_ESTIMATE_SCALARS)
        policy = simulation.receiver_observation_policy().info()
        with self.assertRaises(u.ApiError):
            enable(simulation, scalars=1)
        self.assertEqual(simulation.receiver_observation_policy().info(), policy)
        document_bytes = (ROOT / 'examples/cli/upstream/us-25khz.json').read_bytes()
        document = u.ConfigurationDocument.from_json(document_bytes)
        for load in [lambda: simulation.configure(configured(25).configuration()),
                     lambda: simulation.configure_document(document),
                     lambda: simulation.configure_json(document_bytes)]:
            with self.assertRaises(u.ApiError) as rejected:
                load()
            self.assertEqual(rejected.exception.code, u.ErrorCode.ResourceLimit)
            self.assertEqual(simulation.configured_response(), before)
        for invalid in [-1, 1.5]:
            with self.assertRaises((TypeError, OverflowError)):
                u.ReceiverObservationPolicy(invalid, RETENTION_SCALAR_BUDGET)

    def test_actual_complex_effective_response_both_modes_and_original_missing_bins(self):
        # Literal H=2+j and normalized C=j give -1+2j, independently of fitted Rust values.
        for spacing, length in [(50, FFT_50KHZ_SAMPLES), (25, FFT_25KHZ_SAMPLES)]:
            simulation = configured(spacing, channel=u.ChannelConfig(taps=[(2.0, 1.0)]), corrective=True)
            generation = simulation.configured_response()['configuration_generation']
            enable(simulation)
            report = simulation.run()
            self.assertTrue(report.reference_recovered)
            summary = report.receiver_observations()
            self.assertEqual(summary['completed_frames'], fixture.GRANT_FRAMES)
            self.assertEqual(summary['retained_frames'], fixture.GRANT_FRAMES)
            for entry in summary['frames']:
                snapshot = report.receiver_estimate(entry['retained_index'])
                self.assertEqual(snapshot['frame'], entry)
                self.assertEqual(entry['identity']['generation'], generation)
                self.assertEqual(snapshot['reference_plane'], 'received_pilot_effective_c_times_h')
                self.assertEqual(snapshot['fft_samples'], length)
                self.assertEqual(snapshot['points'][0]['frequency_hz'], -51200000.0)
                self.assertIsNone(snapshot['points'][0]['value'])
                point = snapshot['points'][fixture.FIRST_ACTIVE_CARRIER]
                self.assertEqual(point['physical_carrier'], fixture.FIRST_ACTIVE_CARRIER)
                self.assert_iq(point['value'], -1.0, 2.0)

    def test_aggregate_budgets_and_passive_noisy_outcomes_preserve_default_json(self):
        simulation = configured(channel=u.ChannelConfig(noise_variance=NOISE_VARIANCE),
                                options=u.ExperimentOptions(grants=MULTI_GRANT_COUNT))
        original = simulation.run()
        self.assertNotIn('receiver_estimates', json.loads(original.to_json())['observations'])
        self.assertEqual(original.receiver_observations()['retained_frames'], 0)
        original_words = [grant['words'] for grant in json.loads(original.to_json())['grants']]
        enable(simulation)
        observed = simulation.run()
        summary = observed.receiver_observations()
        self.assertEqual(summary['completed_frames'], MULTI_GRANT_COUNT * fixture.GRANT_FRAMES)
        self.assertEqual(summary['retained_frames'], RETAINED_FRAME_BUDGET)
        self.assertEqual(summary['used_scalar_slots'], RETAINED_FRAME_BUDGET * ONE_50KHZ_ESTIMATE_SCALARS)
        self.assertEqual(summary['frames'][-1]['identity']['grant'], fixture.GRANT_ID + 1)
        enable(simulation, scalars=ONE_50KHZ_ESTIMATE_SCALARS)
        exhausted = simulation.run()
        self.assertEqual(exhausted.receiver_observations()['retained_frames'], 1)
        for candidate in [observed, exhausted]:
            self.assertEqual([grant['words'] for grant in json.loads(candidate.to_json())['grants']], original_words)
            self.assertEqual(candidate.compared_source_bits, original.compared_source_bits)
            self.assertEqual(candidate.bit_errors, original.bit_errors)

    def test_owned_queries_limits_zero_channel_and_never_started_inventory(self):
        simulation = configured(channel=u.ChannelConfig(taps=[(0.0, 0.0)]))
        enable(simulation)
        report = simulation.run()
        full = report.receiver_estimate()
        self.assertFalse(report.reference_recovered)
        self.assertIsNone(full['points'][0]['value'])
        self.assert_iq(full['points'][fixture.FIRST_ACTIVE_CARRIER]['value'], 0.0, 0.0)
        zoom = report.receiver_estimate(first_carrier=fixture.FIRST_ACTIVE_CARRIER,
                                       carrier_count=ZOOM_POINT_COUNT, maximum_points=ZOOM_POINT_COUNT)
        self.assertEqual(zoom['points'], full['points'][fixture.FIRST_ACTIVE_CARRIER:
                                                       fixture.FIRST_ACTIVE_CARRIER + ZOOM_POINT_COUNT])
        for arguments in [dict(maximum_points=0), dict(maximum_points=OVER_QUERY_POINT_CEILING),
                          dict(maximum_points=1), dict(first_carrier=FFT_50KHZ_SAMPLES),
                          dict(carrier_count=FFT_25KHZ_SAMPLES), dict(retained_index=fixture.GRANT_FRAMES)]:
            with self.assertRaises(u.ApiError):
                report.receiver_estimate(**arguments)
        changed = report.receiver_observations()
        changed['frames'].clear()
        self.assertTrue(report.receiver_observations()['frames'])
        control = u.RunControl()
        control.cancel()
        cancelled = simulation.run_with_control(control)
        self.assertEqual(cancelled.receiver_observations()['completed_frames'], 0)
        with self.assertRaises(u.ApiError) as unavailable:
            cancelled.receiver_estimate()
        self.assertEqual(unavailable.exception.code, u.ErrorCode.UnsupportedCapability)
        del simulation, report
        self.assert_iq(zoom['points'][0]['value'], 0.0, 0.0)

    def test_busy_policy_access_preserves_accepted_generation_after_cancel(self):
        simulation = configured()
        enable(simulation)
        before = simulation.receiver_observation_policy().info()
        control = u.RunControl()
        control.pause()
        outputs, failures = [], []

        def run():
            try:
                outputs.append(simulation.run_with_control(control))
            except BaseException as cause:
                failures.append(cause)

        worker = threading.Thread(target=run)
        worker.start()
        try:
            deadline = time.monotonic() + CONTROL_DEADLINE_SECONDS
            while json.loads(control.progress_json())['status'] != 'Paused':
                self.assertLess(time.monotonic(), deadline)
                time.sleep(CONTROL_POLL_SECONDS)
            for operation in [simulation.receiver_observation_policy,
                              lambda: simulation.set_receiver_observation_policy(u.ReceiverObservationPolicy())]:
                with self.assertRaises(u.ApiError) as busy:
                    operation()
                self.assertEqual(busy.exception.code, u.ErrorCode.Busy)
        finally:
            control.cancel()
            worker.join(CONTROL_DEADLINE_SECONDS)
        self.assertFalse(worker.is_alive())
        self.assertEqual(failures, [])
        self.assertEqual(outputs[0].status, 'Cancelled')
        self.assertEqual(simulation.receiver_observation_policy().info(), before)

    @unittest.skipUnless(os.environ.get('MERLIN_UPSTREAM_RUST_RECEIVER_ESTIMATES_JSON'),
                         'Direct Rust receiver estimate reference required')
    def test_separate_rust_process_inventory_full_optional_coefficients_and_units_match(self):
        references = json.loads(Path(os.environ['MERLIN_UPSTREAM_RUST_RECEIVER_ESTIMATES_JSON']).read_text())
        for spacing, reference in zip([50, 25], references):
            simulation = u.Simulation()
            simulation.configure_json((ROOT / f'examples/cli/upstream/us-{spacing}khz.json').read_bytes())
            enable(simulation)
            report = simulation.run()
            summary = report.receiver_observations()
            estimates = [report.receiver_estimate(frame['retained_index']) for frame in summary['frames']]
            # Each process allocates its own run/generation IDs; original grant/frame remain exact.
            for actual, expected in zip(summary['frames'], reference['summary']['frames']):
                self.assertGreater(actual['identity']['run'], 0)
                actual['identity']['run'] = expected['identity']['run']
                actual['identity']['generation'] = expected['identity']['generation']
            self.assertEqual(summary, reference['summary'])
            self.assertEqual(len(estimates), len(reference['estimates']))
            for actual, expected in zip(estimates, reference['estimates']):
                actual['frame']['identity'] = expected['frame']['identity']
                actual_points, expected_points = actual.pop('points'), expected['points']
                self.assertEqual(actual, {key: value for key, value in expected.items() if key != 'points'})
                self.assertEqual(len(actual_points), len(expected_points))
                for point, expected_point in zip(actual_points, expected_points):
                    self.assertEqual(point['physical_carrier'], expected_point['physical_carrier'])
                    self.assertEqual(point['frequency_hz'], expected_point['frequency_hz'])
                    if expected_point['value'] is None:
                        self.assertIsNone(point['value'])
                    else:
                        self.assert_iq(point['value'], expected_point['value']['re'], expected_point['value']['im'])
