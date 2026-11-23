#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Installed native profile inspection retains source IDs/codes and scientific units."""
import hashlib
import json
from pathlib import Path
import unittest

import merlin as root
import merlin.downstream as m

FIXTURES = Path(__file__).resolve().parents[3] / 'examples/measurements/single-channel'


def source():
    record = json.loads((FIXTURES / 'modulation_profile.json').read_bytes())
    def scheme(code, order, bits, role):
        return dict(schema_type='range', num_subcarriers=1, modulation_order_code=code,
                    modulation_order=order, bit_loading=bits, carrier_type=role)
    record['num_profiles'] = 2
    record['profiles'] = [
        dict(profile_id=200, schemes=[scheme(4, 'qam_16', 4, 'data'), scheme(20, 'plc', 4, 'plc'),
                                     scheme(1, 'continuous_pilot', 1, 'continuous_pilot'), scheme(255, 'unknown', None, 'unknown')]),
        dict(profile_id=0, schemes=[scheme(0, 'zero_bit_loaded', 0, 'zero_bit_loaded'), scheme(3, 'reserved_3', None, 'reserved'),
                                   scheme(16, 'exclusion', None, 'exclusion'), scheme(14, 'qam_16384', 14, 'data')]),
    ]
    return json.dumps(record).encode()


class ProfileCaptureTests(unittest.TestCase):
    def test_source_catalog_ocd_geometry_preserves_byte_ids_and_run_id_limits(self):
        # Independent byte-ID boundary 255 and absent source 199; both spacings
        # retain four original carriers from physical carrier 356.
        for spacing in (25000, 50000):
            document = json.loads(source())
            document['profiles'][0]['profile_id'] = 255
            document['subcarrier_spacing'] = spacing
            original = json.dumps(document).encode()
            capture = m.ProfileCapture.from_bytes('source-boundary.json', original)
            settings = m.TransmissionConfiguration().settings
            settings['subcarrier_spacing'] = str(spacing)
            configuration = m.TransmissionConfiguration.from_json(json.dumps(settings).encode())
            before = configuration.to_json()
            self.assertIsNone(configuration.check_profile_capture(capture, 0))
            self.assertIsNone(configuration.check_profile_capture(capture, 255))
            with self.assertRaises(m.ApiError) as missing:
                configuration.check_profile_capture(capture, 199)
            self.assertEqual(missing.exception.code, m.ErrorCode.InvalidIdentifier)
            with self.assertRaises(m.ApiError):
                m.ProfileId(255)
            for invalid in (-1, 256):  # Just beyond either side of a byte ID.
                with self.assertRaises(OverflowError):
                    configuration.check_profile_capture(capture, invalid)
            if spacing == 25000:
                settings['subcarrier_spacing'] = '50000'
            else:
                settings['subcarrier_spacing'] = '25000'
            mismatch = m.TransmissionConfiguration.from_json(json.dumps(settings).encode())
            with self.assertRaises(m.ApiError) as grid:
                mismatch.check_profile_capture(capture, 255)
            self.assertEqual(grid.exception.code, m.ErrorCode.IncompatibleGrid)
            self.assertEqual(configuration.to_json(), before)
            self.assertEqual(capture.profile_ids, (0, 255))
            self.assertEqual(capture.provenance['original_sha256'], hashlib.sha256(original).hexdigest())

        capture = m.ProfileCapture.from_file(FIXTURES / 'modulation_profile.json')
        profile = capture.simulation_profile(m.ProfileId(0))
        settings = m.TransmissionConfiguration().settings
        # The approximately 187 MHz source span cannot fit the valid 24 MHz OCD.
        settings['occupied_bandwidth_hz'] = 24000000
        configuration = m.TransmissionConfiguration.from_json(json.dumps(settings).encode())
        with self.assertRaises(m.ApiError) as selected:
            configuration.check_profile_capture(capture, 0)
        with self.assertRaises(m.ApiError) as existing:
            configuration.check_profile(profile)
        self.assertEqual(selected.exception.code, m.ErrorCode.IncompatibleGrid)
        self.assertEqual(selected.exception.code, existing.exception.code)
        self.assertEqual(str(selected.exception), str(existing.exception))

    def test_binary_json_inventory_schemes_science_and_original_hashes_match(self):
        binary_bytes = (FIXTURES / 'modulation_profile.bin').read_bytes()
        json_bytes = (FIXTURES / 'modulation_profile.json').read_bytes()
        binary = m.ProfileCapture.from_bytes('wrong.json', binary_bytes)
        decoded = m.ProfileCapture.from_bytes('wrong.bin', json_bytes)
        self.assertEqual(binary.profile_ids, (0, 1, 2, 3))
        self.assertEqual(decoded.profile_ids, binary.profile_ids)
        for identity in binary.profile_ids:
            self.assertEqual(binary.summary(identity), decoded.summary(identity))
            self.assertEqual(binary.schemes(identity), decoded.schemes(identity))
        self.assertEqual(binary.metadata['original_kind'], 'BinaryPnm')
        self.assertEqual(decoded.metadata['original_kind'], 'DecodedJson')
        self.assertEqual(binary.provenance['original_sha256'], hashlib.sha256(binary_bytes).hexdigest())
        self.assertEqual(decoded.provenance['original_sha256'], hashlib.sha256(json_bytes).hexdigest())
        self.assertIs(root.ProfileCapture, m.ProfileCapture)

    def test_complete_source_ids_and_literal_role_capacity_shannon_and_coordinates(self):
        capture = m.ProfileCapture.from_bytes('source', source())
        self.assertEqual(capture.profile_ids, (0, 200))
        self.assertEqual(capture.metadata['source_profile_order'], [200, 0])
        page = capture.schemes(200)
        plc = page['schemes'][1]
        self.assertEqual(plc['modulation_order_code'], 20)
        self.assertEqual(plc['first_physical_carrier'], 357)
        self.assertEqual(plc['first_frequency_hz'], 640025000)
        self.assertEqual(plc['constellation'], 'qam_16')
        self.assertEqual(plc['preamble']['constellation'], 'bpsk')
        self.assertEqual(plc['preamble']['shannon_minimum_mer_db'], 0)
        unknown = page['schemes'][3]
        self.assertEqual(unknown['modulation_order_code'], 255)
        self.assertIsNone(unknown['bit_loading'])
        self.assertIsNone(unknown['ideal_shannon_snr_db'])
        self.assertEqual(capture.summary(200)['uncoded_data_capacity_bps'], 100000)
        self.assertEqual(capture.summary(0)['uncoded_data_capacity_bps'], 350000)
        self.assertFalse(capture.summary(200)['capacity_complete'])
        view = capture.inspect(m.ProfileViewRequest(profiles=[200], metric='shannon_db', max_buckets=1))
        self.assertEqual((view['first_frequency_hz'], view['last_frequency_hz']), (640000000, 640075000))
        point = view['series'][0]['buckets'][0]
        self.assertEqual(point['minimum'], 0)
        self.assertAlmostEqual(point['maximum'], 11.760912590556813, places=12)
        self.assertTrue(point['gap'])
        with self.assertRaises(m.ApiError):
            m.ProfileId(200)
        with self.assertRaises(m.ApiError):
            m.DPD.from_bytes('existing contract', source(), m.ProfileId(0))

    def test_request_page_identity_and_output_bounds_reject_without_mutation(self):
        capture = m.ProfileCapture.from_bytes('source', source())
        for kwargs in [dict(profiles=list(range(257))), dict(roles=['data']*8),
                       dict(roles=['unrecognized']), dict(metric='measured_mer'),
                       dict(max_buckets=0), dict(max_buckets=2049)]:
            with self.assertRaises(m.ApiError):
                m.ProfileViewRequest(**kwargs)
        for kwargs in [dict(profiles=[1]), dict(start=4, count=1), dict(max_buckets=1)]:
            with self.assertRaises(m.ApiError):
                capture.inspect(m.ProfileViewRequest(**kwargs))
        self.assertEqual(capture.inspect(m.ProfileViewRequest(profiles=[]))['series'], [])
        self.assertIsNone(capture.inspect(m.ProfileViewRequest(profiles=[200], roles=[]))['series'][0]['buckets'][0]['maximum'])
        for arguments in [(1, 0, 1), (0, 5, 1), (0, 0, 257)]:
            with self.assertRaises(m.ApiError):
                capture.schemes(*arguments)
        metadata = capture.metadata
        metadata['header']['capture_time'] = 0
        self.assertEqual(capture.metadata['header']['capture_time'], 1466967)
        request = m.ProfileViewRequest(profiles=[200], max_buckets=1)
        view = capture.inspect(request)
        view['series'][0]['buckets'][0]['maximum'] = 99
        self.assertEqual(capture.inspect(request)['series'][0]['buckets'][0]['maximum'], 4)
        del capture
        self.assertEqual(view['sources'][0]['original_name'], 'source')

    def test_existing_dpd_view_and_summary_share_the_same_scientific_owner(self):
        path = FIXTURES / 'modulation_profile.json'
        capture = m.ProfileCapture.from_file(path)
        dpd = m.DPD.from_file(path, m.ProfileId(0))
        self.assertEqual(dpd.summary(), capture.summary(0))
        request = m.ProfileViewRequest(profiles=[0], start=20, count=4, max_buckets=4)
        actual, expected = dpd.view(request), capture.inspect(request)
        for field in ('key', 'sources', 'metric', 'start', 'count', 'first_frequency_hz',
                      'last_frequency_hz', 'selected_role_counts', 'series'):
            self.assertEqual(actual[field], expected[field])
        self.assertEqual(actual['profiles'], [capture.summary(0)])
        self.assertLessEqual(sum(len(s['buckets']) for s in capture.inspect(m.ProfileViewRequest(max_buckets=16))['series']), 16)
        with self.assertRaises(m.ApiError):
            dpd.view(m.ProfileViewRequest(profiles=[1]))

    def test_source_frame_is_owned_preserves_raw_identity_and_roles(self):
        original = source()
        capture = m.ProfileCapture.from_bytes('raw source', original)
        frame = capture.frame(200)
        self.assertEqual(frame['profile_id'], 200)
        self.assertEqual(frame['capture_time'], 1466967)
        self.assertEqual([v['carrier_type'] for v in frame['assignments']],
                         ['data', 'plc', 'continuous_pilot', 'unknown'])
        self.assertIsNone(frame['assignments'][-1]['bit_loading'])
        frame['assignments'][0]['bit_loading'] = 0
        self.assertEqual(capture.frame(200)['assignments'][0]['bit_loading'], 4)
        self.assertEqual(capture.metadata['source_profile_order'], [200, 0])
        with self.assertRaises(m.ApiError) as absent:
            capture.frame(199)
        self.assertEqual(absent.exception.code, m.ErrorCode.InvalidIdentifier)
        for invalid in (-1, 256):  # Independent byte-ID boundaries.
            with self.assertRaises(OverflowError):
                capture.frame(invalid)
        self.assertEqual(capture.provenance['original_sha256'], hashlib.sha256(original).hexdigest())
        retained = capture.frame(200)
        del capture
        self.assertEqual(retained['profile_id'], 200)
