#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Actual Rust-owned role normalization, labels, stage pairing and received response display."""
import inspect
import json
import os
from pathlib import Path
import unittest

import merlin.upstream as u
import test_upstream_recovery_views as fixture

BOOSTED_50KHZ_PATTERN = 5
BOOSTED_25KHZ_PATTERN = 12
RETAINED_POINT_BUDGET = 8192
ESTIMATE_FRAME_BUDGET = 1
ESTIMATE_SCALAR_BUDGET = 8192
QUERY_POINT_CEILING = 4096
REFERENCE_POINT_CEILING = 4096
PAIR_POINT_CEILING = 4096
OVER_QUERY_POINT_LIMIT = 4097
OVER_REFERENCE_POINT_LIMIT = 4097
OVER_PAIR_POINT_LIMIT = 4097
DISPLAY_PAIR_LIMIT = 3
EXPECTED_16QAM_LABEL_ZERO_COMPONENT = 0.9486832980505138
EXPECTED_BOOSTED_PILOT_GAIN = 1.7320508075688772
EXPECTED_TX_PILOTS = 16
EXPECTED_50KHZ_MATCHES = 112
EXPECTED_25KHZ_MATCHES = 240
EXPECTED_AMPLITUDE_DB = 6.989700043360188
EXPECTED_PHASE_RADIANS = 2.0344439357957027
NOISE_VARIANCE = 1e-4
NUMERICAL_ABSOLUTE_TOLERANCE = 1e-10
PROCESS_RELATIVE_TOLERANCE = 1e-12


def configured(spacing=50, channel=None):
    """Construct labelled synthetic input through existing validated public Rust models."""
    width, pattern = {50: (fixture.MINISLOT_50KHZ_CARRIERS, BOOSTED_50KHZ_PATTERN),
                      25: (fixture.MINISLOT_25KHZ_CARRIERS, BOOSTED_25KHZ_PATTERN)}[spacing]
    waveform = u.Waveform(spacing, fixture.PREFIX_SAMPLES, fixture.ROLLOFF_SAMPLES,
                          fixture.FRAME_SYMBOLS, fixture.BANDWIDTH_HZ)
    profile = u.Profile(fixture.PROFILE_ID, fixture.DATA_IUC, fixture.PROFILE_RANDOMIZER_SEED,
                        [u.Minislot(fixture.FIRST_ACTIVE_CARRIER + index * width, fixture.DATA_LOADING, pattern)
                         for index in range(fixture.ACTIVE_MINISLOTS)])
    grant = u.SyntheticGrant(fixture.GRANT_ID, fixture.PROFILE_ID, fixture.GRANT_EPOCH_SAMPLE,
                             fixture.FIRST_FRAME_ID, fixture.GRANT_FRAMES, list(range(fixture.ACTIVE_MINISLOTS)))
    config = u.ExperimentConfig(u.UsConfiguration(waveform, [profile], grant, [0, 1]),
                               fixture.SOURCE_PRBS15_SEED, u.ReceiverConfig(), channel=channel)
    simulation = u.Simulation(u.ExperimentOptions(retained_points=RETAINED_POINT_BUDGET,
        retained_taps=['TxGrid', 'TxPreEq', 'RxFft', 'RxEqualized']))
    simulation.configure(config)
    simulation.set_receiver_observation_policy(u.ReceiverObservationPolicy(ESTIMATE_FRAME_BUDGET,
                                                                           ESTIMATE_SCALAR_BUDGET))
    return simulation


class UpstreamQamViews(unittest.TestCase):
    def assert_snapshot(self, actual, expected):
        """Compare every field; process-local IDs differ while original physical coordinates remain exact."""
        if isinstance(expected, dict):
            self.assertEqual(set(actual), set(expected))
            for key in expected:
                if key == 'identity':
                    self.assertEqual(actual[key]['grant'], expected[key]['grant'])
                    self.assertGreater(actual[key]['run'], 0)
                    self.assertGreater(actual[key]['generation'], 0)
                else:
                    self.assert_snapshot(actual[key], expected[key])
        elif isinstance(expected, list):
            self.assertEqual(len(actual), len(expected))
            for value, reference in zip(actual, expected):
                self.assert_snapshot(value, reference)
        elif isinstance(expected, float):
            self.assertAlmostEqual(actual, expected,
                delta=max(NUMERICAL_ABSOLUTE_TOLERANCE, abs(expected) * PROCESS_RELATIVE_TOLERANCE))
        else:
            self.assertEqual(actual, expected)

    def test_installed_defaults_actual_labels_recorded_gain_and_received_phase_both_modes(self):
        self.assertEqual(inspect.signature(u.BatchReport.qam_constellation).parameters['maximum_points'].default,
                         QUERY_POINT_CEILING)
        self.assertEqual(inspect.signature(u.BatchReport.qam_constellation).parameters['maximum_references'].default,
                         REFERENCE_POINT_CEILING)
        self.assertEqual(inspect.signature(u.BatchReport.qam_pairing).parameters['maximum_pairs'].default,
                         PAIR_POINT_CEILING)
        for spacing in [50, 25]:
            report = configured(spacing, u.ChannelConfig(taps=[(-1.0, 2.0)])).run()
            self.assertTrue(report.reference_recovered)
            view = report.qam_constellation()
            self.assertEqual(view['normalization'], 'receiver_unit_reference')
            self.assertAlmostEqual(view['references'][0]['value']['re'],
                                   EXPECTED_16QAM_LABEL_ZERO_COMPONENT, delta=NUMERICAL_ABSOLUTE_TOLERANCE)
            for point in view['points']:
                self.assertEqual(point['unit_value'], point['observation']['value'])
            pilots = report.qam_constellation('TxGrid', loading=0)
            self.assertEqual(pilots['payload_loading'], 0)
            self.assertEqual(pilots['reference_bits'], 1)
            self.assertEqual(len(pilots['references']), 2)
            for point in pilots['points']:
                self.assertEqual(point['observation']['role'], 'KnownPilot')
                self.assertEqual(point['observation']['loading'], 0)
                self.assertAlmostEqual(point['normalization_divisor'], EXPECTED_BOOSTED_PILOT_GAIN,
                                       delta=NUMERICAL_ABSOLUTE_TOLERANCE)
            active = report.receiver_estimate(first_carrier=fixture.FIRST_ACTIVE_CARRIER, carrier_count=1)['points'][0]
            self.assertAlmostEqual(active['amplitude_db'], EXPECTED_AMPLITUDE_DB, delta=NUMERICAL_ABSOLUTE_TOLERANCE)
            self.assertAlmostEqual(active['phase_radians'], EXPECTED_PHASE_RADIANS, delta=NUMERICAL_ABSOLUTE_TOLERANCE)
            missing = report.receiver_estimate(first_carrier=0, carrier_count=1)['points'][0]
            self.assertIsNone(missing['amplitude_db'])
            self.assertIsNone(missing['phase_radians'])

    def test_pairing_original_coordinates_whole_counts_caps_and_owned_data(self):
        for spacing, expected in [(50, EXPECTED_50KHZ_MATCHES), (25, EXPECTED_25KHZ_MATCHES)]:
            simulation = configured(spacing)
            report = simulation.run()
            full = report.qam_pairing()
            self.assertEqual(full['matched_total'], expected)
            self.assertEqual(full['unpaired_left'], EXPECTED_TX_PILOTS)
            for pair in full['pairs']:
                for key in ['identity', 'frame', 'symbol', 'carrier', 'loading']:
                    self.assertEqual(pair['left'][key], pair['right'][key])
            capped = report.qam_pairing(maximum_pairs=DISPLAY_PAIR_LIMIT)
            self.assertEqual(capped['pairs'], full['pairs'][:DISPLAY_PAIR_LIMIT])
            self.assertEqual(capped['matched_total'], full['matched_total'])
            self.assertTrue(capped['capped'])
            pilots = report.qam_pairing(loading=0)
            self.assertEqual(pilots['pairs'], [])
            self.assertEqual(pilots['unpaired_left'], EXPECTED_TX_PILOTS)
            changed = report.qam_pairing()
            changed['pairs'].clear()
            self.assertEqual(report.qam_pairing(), full)
            del simulation, report
            self.assertEqual(len(capped['pairs']), DISPLAY_PAIR_LIMIT)

    def test_bounds_unsupported_raw_planes_empty_failure_and_passive_noisy_outcomes(self):
        simulation = configured(channel=u.ChannelConfig(noise_variance=NOISE_VARIANCE))
        report = simulation.run()
        for tap in ['TxPreEq', 'RxFft']:
            self.assertTrue(report.qam(tap).values)
            with self.assertRaises(u.ApiError) as unsupported:
                report.qam_constellation(tap)
            self.assertEqual(unsupported.exception.code, u.ErrorCode.UnsupportedCapability)
        for arguments in [dict(maximum_points=0), dict(maximum_points=OVER_QUERY_POINT_LIMIT),
                          dict(maximum_references=OVER_REFERENCE_POINT_LIMIT), dict(maximum_references=1),
                          dict(loading=14), dict(tap_name='unknown')]:
            with self.assertRaises(u.ApiError):
                report.qam_constellation(**arguments)
        for arguments in [dict(maximum_pairs=0), dict(maximum_pairs=OVER_PAIR_POINT_LIMIT), dict(left='unknown'),
                          dict(left='TxOutput')]:
            with self.assertRaises(u.ApiError):
                report.qam_pairing(**arguments)
        report.qam_constellation()
        report.qam_pairing()
        again = simulation.run()
        self.assertEqual(json.loads(report.to_json())['grants'][0]['words'], json.loads(again.to_json())['grants'][0]['words'])
        self.assertEqual(report.bit_errors, again.bit_errors)
        for trace in json.loads(report.to_json())['observations']['traces'].values():
            self.assertNotIn('role_gains', trace)
        failed = configured(channel=u.ChannelConfig(taps=[(0.0, 0.0)])).run()
        self.assertFalse(failed.reference_recovered)
        self.assertEqual(failed.qam_pairing()['matched_total'], 0)
        zero = failed.receiver_estimate(first_carrier=fixture.FIRST_ACTIVE_CARRIER, carrier_count=1)['points'][0]
        self.assertEqual(zero['value'], {'re': 0.0, 'im': 0.0})
        self.assertIsNone(zero['amplitude_db'])
        self.assertIsNone(zero['phase_radians'])
        control = u.RunControl()
        control.cancel()
        partial = simulation.run_with_control(control)
        self.assertEqual(partial.qam_pairing()['pairs'], [])
        with self.assertRaises(u.ApiError):
            partial.qam_constellation()

    @unittest.skipUnless(os.environ.get('MERLIN_UPSTREAM_RUST_QAM_VIEWS_JSON'),
                         'Direct Rust QAM-view reference required')
    def test_separate_rust_process_full_observations_labels_pairs_and_received_display_match(self):
        references = json.loads(Path(os.environ['MERLIN_UPSTREAM_RUST_QAM_VIEWS_JSON']).read_text())
        self.assertEqual(len(references), 2)
        for spacing, reference in zip([50, 25], references):
            report = configured(spacing, u.ChannelConfig(taps=[(-1.0, 2.0)])).run()
            actual = {'constellation_rx': report.qam_constellation(),
                      'constellation_tx_pilots': report.qam_constellation('TxGrid', loading=0),
                      'pairing': report.qam_pairing(),
                      'received_response': report.receiver_estimate(first_carrier=fixture.FIRST_ACTIVE_CARRIER, carrier_count=1),
                      'missing_response': report.receiver_estimate(first_carrier=0, carrier_count=1)}
            self.assert_snapshot(actual, reference)
