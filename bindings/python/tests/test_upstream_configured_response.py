#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Configured C/H diagnostics and periodic impulses; no Python scientific implementation."""
import inspect
import json
import math
import os
from pathlib import Path
import threading
import time
import unittest

import merlin.upstream as u
import test_upstream_recovery_views as fixture
from test_upstream_experiment import configuration as baseline_configuration

CORRECTIVE_BANK_GENERATION = 11
RAW_CORRECTIVE_IMAGINARY_GAIN = 2.0
INITIAL_REPLAY_BANK_ID = 5
SNAPSHOT_EVENT_ID = 17
UPDATE_GRANT_SYMBOL = 3
PHYSICAL_DELAY_SAMPLES = 2
PHYSICAL_PHASE_RADIANS = math.pi / 2
SECOND_PHYSICAL_TAP_AMPLITUDE = 0.5
EXPECTED_MAXIMUM_POINTS = 4096
OVER_MAXIMUM_POINTS = 4097
FFT_50KHZ_SAMPLES = 2048
FFT_25KHZ_SAMPLES = 4096
EXPECTED_DC_AMPLITUDE_DB = 3.5218251811136247
EXPECTED_SECOND_IMPULSE_RELATIVE_DB = -6.020599913279624
NUMERICAL_ABSOLUTE_TOLERANCE = 1e-11
PROCESS_RELATIVE_TOLERANCE = 1e-12
NONZERO_FREQUENCY_OFFSET_HZ = 1000.0
CONTROL_GRANTS = 128
CONTROL_CHUNK_SAMPLES = 127
CONTROL_DEADLINE_SECONDS = 10
CONTROL_POLL_SECONDS = 0.001


def configured(spacing=50, channel=None, corrective=False, replay=False, options=None):
    """Construct labelled typed fixtures; Rust owns C normalization and every response calculation."""
    width, pattern, length = {
        50: (fixture.MINISLOT_50KHZ_CARRIERS, fixture.FIRST_50KHZ_PILOT_PATTERN, FFT_50KHZ_SAMPLES),
        25: (fixture.MINISLOT_25KHZ_CARRIERS, fixture.FIRST_25KHZ_PILOT_PATTERN, FFT_25KHZ_SAMPLES),
    }[spacing]
    waveform = u.Waveform(spacing, fixture.PREFIX_SAMPLES, fixture.ROLLOFF_SAMPLES,
                          fixture.FRAME_SYMBOLS, fixture.BANDWIDTH_HZ)
    slots = [u.Minislot(fixture.FIRST_ACTIVE_CARRIER + index * width, fixture.DATA_LOADING, pattern)
             for index in range(fixture.ACTIVE_MINISLOTS)]
    profile = u.Profile(fixture.PROFILE_ID, fixture.DATA_IUC, fixture.PROFILE_RANDOMIZER_SEED, slots)
    grant = u.SyntheticGrant(fixture.GRANT_ID, fixture.PROFILE_ID, fixture.GRANT_EPOCH_SAMPLE,
                             fixture.FIRST_FRAME_ID, fixture.GRANT_FRAMES, list(range(fixture.ACTIVE_MINISLOTS)))
    us = u.UsConfiguration(waveform, [profile], grant, [0, 1])
    c, playback = None, None
    if corrective or replay:
        raw = [(0.0, RAW_CORRECTIVE_IMAGINARY_GAIN)] * length
        for carrier in [0, 1]:
            raw[carrier] = (0.0, 0.0)
        bank = u.PreEqBank.normalized(us, raw, CORRECTIVE_BANK_GENERATION)
        if replay:
            c = u.PreEqBank.unity(us)
            event = u.PreEqEvent.snapshot(SNAPSHOT_EVENT_ID, UPDATE_GRANT_SYMBOL, bank)
            playback = u.PreEqPlayback(us, INITIAL_REPLAY_BANK_ID, c, [event], [])
        else:
            c = bank
    configuration = u.ExperimentConfig(us, fixture.SOURCE_PRBS15_SEED, u.ReceiverConfig(),
                                       pre_eq=c, channel=channel, pre_eq_updates=playback)
    simulation = u.Simulation(options)
    simulation.configure(configuration)
    return simulation


def two_tap_channel():
    """Admit the literal two-tap, two-sample-delay and quarter-turn phase fixture through Rust."""
    return u.ChannelConfig(taps=[(1.0, 0.0), (SECOND_PHYSICAL_TAP_AMPLITUDE, 0.0)],
                           delay_samples=PHYSICAL_DELAY_SAMPLES, phase_radians=PHYSICAL_PHASE_RADIANS)


class UpstreamConfiguredResponse(unittest.TestCase):
    def assert_iq(self, value, real, imaginary):
        self.assertAlmostEqual(value['re'], real, delta=NUMERICAL_ABSOLUTE_TOLERANCE)
        self.assertAlmostEqual(value['im'], imaginary, delta=NUMERICAL_ABSOLUTE_TOLERANCE)

    def assert_snapshot(self, actual, expected):
        """Compare all schema/identity/coordinate fields, with explicit tolerance only for floats."""
        if isinstance(expected, dict):
            self.assertEqual(set(actual), set(expected))
            for key in expected:
                self.assert_snapshot(actual[key], expected[key])
        elif isinstance(expected, list):
            self.assertEqual(len(actual), len(expected))
            for actual_item, expected_item in zip(actual, expected):
                self.assert_snapshot(actual_item, expected_item)
        elif isinstance(expected, float):
            tolerance = max(NUMERICAL_ABSOLUTE_TOLERANCE, abs(expected) * PROCESS_RELATIVE_TOLERANCE)
            self.assertAlmostEqual(actual, expected, delta=tolerance)
        else:
            self.assertEqual(actual, expected)

    def test_literal_two_tap_transfer_phase_periodic_inverse_and_installed_defaults(self):
        for owner in [u.Simulation, u.BatchReport]:
            signature = inspect.signature(owner.configured_response)
            self.assertEqual(signature.parameters['maximum_points'].default, EXPECTED_MAXIMUM_POINTS)
            self.assertEqual(signature.parameters['source'].default, 'physical_channel')
            self.assertIs(signature.parameters['include_impulse'].default, False)
        for spacing, length in [(50, FFT_50KHZ_SAMPLES), (25, FFT_25KHZ_SAMPLES)]:
            snapshot = configured(spacing, channel=two_tap_channel()).configured_response(include_impulse=True)
            self.assertEqual(snapshot['fft_samples'], length)
            self.assert_iq(snapshot['points'][length // 2]['value'], 0.0, 1.5)
            self.assert_iq(snapshot['points'][length // 4]['value'], 0.5, -1.0)
            self.assert_iq(snapshot['points'][3 * length // 4]['value'], -0.5, -1.0)
            self.assertAlmostEqual(snapshot['points'][length // 2]['amplitude_db'],
                                   EXPECTED_DC_AMPLITUDE_DB, delta=NUMERICAL_ABSOLUTE_TOLERANCE)
            impulse = snapshot['impulse']
            self.assertEqual(impulse['period_samples'], length)
            self.assert_iq(impulse['points'][PHYSICAL_DELAY_SAMPLES]['value'], 0.0, 1.0)
            self.assert_iq(impulse['points'][PHYSICAL_DELAY_SAMPLES + 1]['value'], 0.0, 0.5)
            self.assertAlmostEqual(impulse['points'][PHYSICAL_DELAY_SAMPLES + 1]['relative_power_db'],
                                   EXPECTED_SECOND_IMPULSE_RELATIVE_DB, delta=NUMERICAL_ABSOLUTE_TOLERANCE)

    def test_original_c_replay_excluded_zeros_zoom_and_owned_frozen_report_data(self):
        simulation = configured(corrective=True)
        snapshot = simulation.configured_response('cm_pre_eq', include_impulse=True)
        self.assert_iq(snapshot['points'][fixture.FIRST_ACTIVE_CARRIER]['value'], 0.0, 1.0)
        self.assertEqual(snapshot['pre_eq_generation'], CORRECTIVE_BANK_GENERATION)
        for carrier in [0, 1]:
            self.assertIsNone(snapshot['points'][carrier]['amplitude_db'])
            self.assertIsNone(snapshot['points'][carrier]['phase_radians'])
        zoom = simulation.configured_response('cm_pre_eq', first_carrier=fixture.FIRST_ACTIVE_CARRIER,
                                              carrier_count=1, maximum_points=1, include_impulse=True)
        self.assertEqual(zoom['impulse'], snapshot['impulse'])
        zoom['points'].clear()
        self.assertTrue(simulation.configured_response('cm_pre_eq')['points'])
        report = simulation.run()
        frozen = report.configured_response('cm_pre_eq')
        self.assertEqual(frozen['configuration_generation'], snapshot['configuration_generation'])
        self.assertIsNotNone(frozen['run_id'])
        simulation.configure(baseline_configuration())
        self.assertEqual(report.configured_response('cm_pre_eq'), frozen)
        del report, simulation
        self.assert_iq(frozen['points'][fixture.FIRST_ACTIVE_CARRIER]['value'], 0.0, 1.0)
        simulation = configured(replay=True)
        before = simulation.configured_response('cm_pre_eq', first_carrier=fixture.FIRST_ACTIVE_CARRIER,
                                                carrier_count=1)
        after = simulation.configured_response('cm_pre_eq', grant_symbol=UPDATE_GRANT_SYMBOL,
                                               first_carrier=fixture.FIRST_ACTIVE_CARRIER, carrier_count=1)
        self.assert_iq(before['points'][0]['value'], 1.0, 0.0)
        self.assert_iq(after['points'][0]['value'], 0.0, 1.0)
        self.assertEqual(after['pre_eq_generation'], CORRECTIVE_BANK_GENERATION)
        self.assertNotEqual(after['pre_eq_generation'], SNAPSHOT_EVENT_ID)

    def test_missing_bounds_zero_transfer_cfo_and_never_started_identity_states(self):
        with self.assertRaises(u.ApiError) as missing:
            u.Simulation().configured_response()
        self.assertEqual(missing.exception.code, u.ErrorCode.MissingSetting)
        simulation = configured()
        original = simulation.configured_response()
        for arguments in [dict(source='received_estimate'), dict(maximum_points=0),
                          dict(maximum_points=OVER_MAXIMUM_POINTS), dict(maximum_points=1),
                          dict(first_carrier=FFT_50KHZ_SAMPLES), dict(carrier_count=FFT_25KHZ_SAMPLES),
                          dict(grant_symbol=fixture.FRAME_SYMBOLS * fixture.GRANT_FRAMES)]:
            with self.assertRaises(u.ApiError):
                simulation.configured_response(**arguments)
        for arguments in [dict(first_carrier=-1), dict(carrier_count=1.5), dict(unknown=1)]:
            with self.assertRaises((OverflowError, TypeError)):
                simulation.configured_response(**arguments)
        self.assertEqual(simulation.configured_response(), original)
        control = u.RunControl()
        control.cancel()
        report = simulation.run_with_control(control)
        never_started = report.configured_response()
        self.assertIsNone(never_started['run_id'])
        self.assertIsNone(never_started['configuration_generation'])
        zero = configured(channel=u.ChannelConfig(taps=[(0.0, 0.0)])).configured_response(include_impulse=True)
        self.assertTrue(all(point['amplitude_db'] is None and point['phase_radians'] is None
                            for point in zero['points']))
        self.assertEqual(zero['impulse']['peak_amplitude'], 0.0)
        time_varying = configured(channel=u.ChannelConfig(frequency_offset_hz=NONZERO_FREQUENCY_OFFSET_HZ))
        for source in ['physical_channel', 'composite']:
            with self.assertRaises(u.ApiError) as unsupported:
                time_varying.configured_response(source)
            self.assertEqual(unsupported.exception.code, u.ErrorCode.UnsupportedCapability)
        self.assertTrue(time_varying.configured_response('cm_pre_eq')['points'])

    def test_busy_session_guard_preserves_the_same_configured_generation_after_cancel(self):
        simulation = configured(options=u.ExperimentOptions(grants=CONTROL_GRANTS, chunk_samples=CONTROL_CHUNK_SAMPLES))
        before = simulation.configured_response()
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
            with self.assertRaises(u.ApiError) as busy:
                simulation.configured_response()
            self.assertEqual(busy.exception.code, u.ErrorCode.Busy)
        finally:
            control.cancel()
            worker.join(CONTROL_DEADLINE_SECONDS)
        self.assertFalse(worker.is_alive())
        self.assertEqual(failures, [])
        self.assertEqual(outputs[0].status, 'Cancelled')
        self.assertEqual(simulation.configured_response(), before)

    @unittest.skipUnless(os.environ.get('MERLIN_UPSTREAM_RUST_CONFIGURED_RESPONSE_JSON'), 'Direct Rust response reference required')
    def test_separate_rust_process_planes_units_full_periodic_values_and_metadata_match(self):
        references = iter(json.loads(Path(os.environ['MERLIN_UPSTREAM_RUST_CONFIGURED_RESPONSE_JSON']).read_text()))
        for spacing in [50, 25]:
            simulation = configured(spacing, channel=two_tap_channel(), corrective=True)
            for source in ['cm_pre_eq', 'physical_channel', 'composite']:
                expected = next(references)
                actual = simulation.configured_response(source, include_impulse=True)
                self.assertGreater(actual['configuration_generation'], 0)
                actual['configuration_generation'] = expected['configuration_generation']
                self.assert_snapshot(actual, expected)
