#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Installed native configured-channel previews retain authoritative transfer and input H."""
import json
from pathlib import Path
import unittest

import merlin as root
import merlin.downstream as m

FIXTURES = Path(__file__).resolve().parents[3] / 'examples/measurements/single-channel'


def profile():
    return m.DPD.from_file(FIXTURES / 'modulation_profile.json', m.ProfileId(0))


class ChannelConfigurationTests(unittest.TestCase):
    def test_default_unity_is_bounded_original_grid_and_before_receiver(self):
        geometry, dpd, channel = m.TransmissionConfiguration(), profile(), m.ChannelConfiguration()
        preview = channel.preview(geometry, dpd)
        self.assertEqual(preview['reference_plane'], 'configured_transfer_before_receiver')
        self.assertEqual(preview['key'], dpd.key)
        self.assertEqual(len(preview['effective_response']), 7480)
        self.assertTrue(all(value == [1, 0] for value in preview['effective_response'] if value is not None))
        self.assertLessEqual(len(preview['gain']['points']), 1024)
        self.assertEqual(len(preview['additional_points']), 128)
        self.assertIsNone(preview['power']['millivolts_squared_per_unit'])
        self.assertEqual(preview['filter_coefficients'], [])
        self.assertIsNone(preview['filter_diagnostics'])
        settings = channel.settings
        settings['power']['amplifier_gain_db'] = 6
        self.assertEqual(channel.settings['power']['amplifier_gain_db'], 0)
        self.assertIs(root.ChannelConfiguration, m.ChannelConfiguration)
        del channel, dpd
        self.assertEqual(preview['profile_id'], 0)

    def test_literal_tilt_conditioning_endpoint_gains_and_electrical_reference(self):
        channel = m.ChannelConfiguration.from_json(json.dumps({
            'additional_loss': {'mode': 'manual', 'tilt_db': -6},
            'power': {'amplifier_gain_db': 6, 'attenuation_db': 2},
        }).encode())
        result = channel.preview(m.TransmissionConfiguration(), profile())
        self.assertAlmostEqual(result['additional_points'][0]['additional_gain_db'], 0, places=9)
        self.assertAlmostEqual(result['additional_points'][-1]['additional_gain_db'], -6, places=9)
        self.assertAlmostEqual(result['additional_points'][0]['effective_gain_db'], 4, places=9)
        self.assertAlmostEqual(result['additional_points'][-1]['effective_gain_db'], -2, places=9)
        self.assertAlmostEqual(channel.power_reference['net_amplitude'], 1.5848931924611136, places=9)
        calibrated = m.ChannelConfiguration.from_json(b'{"power":{"input_data_level_dbmv_per_6mhz":0,"receiver_noise_dbmv_hz":-100}}')
        self.assertAlmostEqual(calibrated.power_reference['millivolts_squared_per_unit'], 34.13333333333333, places=9)
        self.assertAlmostEqual(calibrated.power_reference['receiver_noise_power'], 0.0006, places=9)

    def test_raw_source_id_and_actual_echo_coefficients_preserve_inputs(self):
        original = json.loads((FIXTURES / 'modulation_profile.json').read_bytes())
        original['profiles'][0]['profile_id'] = 200
        capture = m.ProfileCapture.from_bytes('original', json.dumps(original).encode())
        geometry = m.TransmissionConfiguration()
        channel = m.ChannelConfiguration()
        raw = channel.preview_capture(geometry, capture, 200)
        accepted = channel.preview(geometry, profile())
        self.assertEqual(raw['profile_id'], 200)
        self.assertEqual(raw['effective_response'], accepted['effective_response'])
        echo = m.ChannelConfiguration.from_json(b'{"filter":{"mode":"echoes","paths":[{"delay_samples":0,"gain":[1,0]},{"delay_samples":1,"gain":[0.25,0]}]}}')
        plan = echo.preview(geometry, profile())
        self.assertEqual(plan['filter_coefficients'], [[1, 0], [0.25, 0]])
        self.assertEqual(plan['filter_diagnostics']['taps'], 2)
        response = m.ChannelEstimate.from_file(FIXTURES / 'channel_estimation.json')
        before = response.provenance
        with self.assertRaises(m.ApiError):
            echo.preview(geometry, profile(), response, m.AssociationTolerance(90000))
        self.assertEqual(response.provenance, before)

    def test_bad_fields_parameters_epoch_identity_and_limits_leave_state_unchanged(self):
        for data in [b'{bad', b'{"unknown":1}', b' ' * 65537,
                     b'{"power":{"amplifier_gain_db":61}}', b'{"filter":{"mode":"echoes","paths":[]}}']:
            with self.assertRaises(m.ApiError):
                m.ChannelConfiguration.from_json(data)
        channel, geometry, dpd = m.ChannelConfiguration(), m.TransmissionConfiguration(), profile()
        before = channel.to_json()
        response = m.ChannelEstimate.from_file(FIXTURES / 'channel_estimation.json')
        with self.assertRaises(m.ApiError) as raised:
            channel.preview(geometry, dpd, response)
        self.assertEqual(raised.exception.code, m.ErrorCode.AssociationFailed)
        channel.preview(geometry, dpd, response, m.AssociationTolerance(90000))
        document = json.loads((FIXTURES / 'channel_estimation.json').read_bytes())
        document['channel_id'] = 33
        wrong = m.ChannelEstimate.from_bytes('wrong identity', json.dumps(document).encode())
        with self.assertRaises(m.ApiError):
            channel.preview(geometry, dpd, wrong, m.AssociationTolerance(90000))
        self.assertEqual(channel.to_json(), before)
        self.assertEqual(channel.preview(geometry, dpd)['additional_low_gain_db'], 0)
