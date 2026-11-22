#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Installed native sequence admission retains exact originals and physical power semantics."""
from concurrent.futures import ThreadPoolExecutor
import json
from pathlib import Path
import unittest

import merlin as root
import merlin.downstream as m
import merlin.merlin as legacy

FIXTURES = Path(__file__).resolve().parents[3] / 'examples/measurements/single-channel'


def profile():
    return m.DPD.from_file(FIXTURES / 'modulation_profile.json', m.ProfileId(0))


def measured(db, epoch=1466967):
    value = json.loads((FIXTURES / 'rxmer.json').read_bytes())
    value['pnm_header']['capture_time'] = epoch
    value['values'] = [db] * 7480
    return m.RxMer.from_bytes('original', json.dumps(value).encode())


def sequence(mode, captures):
    return m.MeasurementSequence(profile(), m.TransmissionConfiguration(), captures,
                                 m.MeasurementSequenceOptions(mode, 10))


class MeasurementSequenceTests(unittest.TestCase):
    def test_idle_capacity_literal_clock_boundaries_need_no_admitted_sequence(self):
        # Independent CP=512 anchors: 23 complete symbols at 25 kHz and
        # 44 complete symbols at 50 kHz per millisecond (42.5/22.5 us periods).
        duration = m.ObservationDuration(1)
        maximum_interval = (1 << 64) - 1  # Independent unsigned 64-bit interval bound.
        for spacing, complete_symbols in [(25000, 23), (50000, 44)]:
            settings = m.TransmissionConfiguration().settings
            settings['subcarrier_spacing'] = str(spacing)
            configuration = m.TransmissionConfiguration.from_json(json.dumps(settings).encode())
            before = configuration.to_json()
            for mode in ('hold', 'interpolate'):
                policy = m.MeasurementSequenceOptions(mode, complete_symbols - 1)
                self.assertEqual(policy.capacity(configuration, duration), 2)
                self.assertEqual(policy.interval_symbols, complete_symbols - 1)
                for interval, expected in [(complete_symbols, 1), (maximum_interval, 1), (1, complete_symbols)]:
                    policy = m.MeasurementSequenceOptions(mode, interval)
                    self.assertEqual(policy.capacity(configuration, duration), expected)
                # Preserve the API's one-hour observation domain and its
                # independent existing 256-capture resource saturation.
                self.assertEqual(policy.capacity(configuration, m.ObservationDuration(3600000)), 256)
                invalid = m.MeasurementSequenceOptions(mode, 0)
                with self.assertRaises(m.ApiError) as error:
                    invalid.capacity(configuration, duration)
                self.assertEqual(error.exception.code, m.ErrorCode.InvalidInput)
                self.assertEqual(invalid.interval_symbols, 0)
            for mode, expected in [('single', 1), ('average', 256)]:
                policy = m.MeasurementSequenceOptions(mode, 0)
                self.assertEqual(policy.capacity(configuration, duration), expected)
                self.assertEqual(policy.interval_symbols, 0)
            self.assertEqual(configuration.to_json(), before)

    def test_average_names_every_original_and_retains_owned_copies(self):
        originals = [measured(40), measured(20), measured(30)]
        hashes = [v.provenance['original_sha256'] for v in originals]
        accepted = sequence('average', originals)
        del originals
        snapshot = accepted.at(99)
        self.assertAlmostEqual(snapshot['normalized_power'][0], 0.0037, places=12)
        self.assertEqual([v['index'] for v in snapshot['contributors']], [0, 1, 2])
        self.assertEqual([v['provenance']['original_sha256'] for v in snapshot['contributors']], hashes)
        for source in snapshot['contributors']:
            self.assertAlmostEqual(source['weight'], 1/3, places=12)
            self.assertEqual(source['provenance']['original_epoch'], 1466967)
        view = accepted.preview(99, count=1)
        self.assertIn('24.318', view['view']['points'][0]['readout'])
        self.assertEqual(view['view']['axis_label'], 'Equivalent RxMER (dB)')
        self.assertEqual(accepted.capture(2).values[0].value, 30)
        snapshot['normalized_power'][0] = 99
        self.assertAlmostEqual(accepted.at(99)['normalized_power'][0], 0.0037, places=12)
        self.assertIs(root.MeasurementSequence, m.MeasurementSequence)
        self.assertIs(legacy.MeasurementSequence, m.MeasurementSequence)
        del accepted
        self.assertEqual(len(view['contributors']), 3)

    def test_temporal_clock_literal_powers_last_hold_and_parallel_queries(self):
        captures = [measured(40), measured(20)]
        accepted = sequence('interpolate', captures)
        info = accepted.info()
        self.assertEqual(info['source_count'], 2)
        self.assertEqual(info['last_transition_symbol'], 10)
        self.assertAlmostEqual(info['interval_milliseconds'], 0.425, places=12)
        self.assertEqual(accepted.capacity(m.ObservationDuration(1)), 3)
        with ThreadPoolExecutor(max_workers=2) as pool:
            first, middle = list(pool.map(accepted.at, [0, 5]))
        self.assertAlmostEqual(first['normalized_power'][0], 0.0001, places=12)
        self.assertAlmostEqual(middle['normalized_power'][0], 0.00505, places=12)
        self.assertEqual([v['weight'] for v in middle['contributors']], [0.5, 0.5])
        self.assertEqual(accepted.at(2**64-1)['contributors'][0]['index'], 1)
        held = sequence('hold', captures)
        self.assertAlmostEqual(held.at(9)['normalized_power'][0], 0.0001, places=12)
        self.assertAlmostEqual(held.at(10)['normalized_power'][0], 0.01, places=12)
        self.assertEqual(accepted.at(0), first)

    def test_rejections_preserve_accepted_sequences_and_options(self):
        first, second = measured(40), measured(20)
        accepted = sequence('single', [first])
        before = accepted.at(0)
        for mode, captures in [('single', []), ('average', [first]*257),
                               ('average', [first, first]), ('single', [first, second]),
                               ('hold', [first]), ('single', [measured(40, 1466968)])]:
            with self.assertRaises(m.ApiError):
                sequence(mode, captures)
        for interval in [0, 2**64-1]:
            with self.assertRaises(m.ApiError):
                m.MeasurementSequence(profile(), m.TransmissionConfiguration(),
                    [first, second, measured(30)], m.MeasurementSequenceOptions('hold', interval))
        with self.assertRaises(m.ApiError):
            accepted.capture(1)
        with self.assertRaises(m.ApiError):
            accepted.preview(0, start=7480, count=1)
        with self.assertRaises(m.ApiError):
            m.MeasurementSequenceOptions('unrecognized')
        options = m.MeasurementSequenceOptions('hold', 10, m.AssociationTolerance(90000))
        self.assertEqual(options.mode, 'hold')
        self.assertEqual(options.interval_symbols, 10)
        self.assertEqual(options.tolerance.value, 90000)
        with self.assertRaises(AttributeError):
            options.interval_symbols = 0
        self.assertEqual(accepted.at(0), before)

    def test_response_admission_distinguishes_unity_original_h_and_epoch_window(self):
        geometry, dpd = m.TransmissionConfiguration(), profile()
        self.assertIsNone(geometry.check_response(dpd))
        response = m.ChannelEstimate.from_file(FIXTURES / 'channel_estimation.json')
        before = response.provenance
        with self.assertRaises(m.ApiError) as raised:
            geometry.check_response(dpd, response)
        self.assertEqual(raised.exception.code, m.ErrorCode.AssociationFailed)
        self.assertIsNone(geometry.check_response(dpd, response, m.AssociationTolerance(90000)))
        self.assertEqual(response.provenance, before)
