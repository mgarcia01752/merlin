#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Installed native parity for passive transmission/source admission."""
import hashlib
import json
from pathlib import Path
import unittest

import merlin as m

ROOT = Path(__file__).resolve().parents[3]


class NativeAdmissionTests(unittest.TestCase):
    def test_unnamed_file_preserves_original_bytes_fill_and_named_rejection(self):
        # Independent eight-byte/64-bit original and finite80-bit fill target.
        original = b'\0exact\r\n'
        source = m.SourceInput.from_unnamed_bytes(original)
        self.assertEqual(source.kind, 'file')
        self.assertEqual(source.name, '')
        self.assertEqual(source.original_bytes, original)
        self.assertEqual(source.original_sha256, '541070fc2b7aa44069811cafe1c6810b607e696dc92792089c0fd88789bb8797')
        self.assertEqual(source.target_bits, 64)
        self.assertIsNone(source.prbs_seed)
        filled = m.SourceInput.from_unnamed_bytes(original, m.FileFillPolicy(80, m.PrbsSeed(7)))
        self.assertEqual(filled.original_bytes, original)
        self.assertEqual(filled.original_sha256, source.original_sha256)
        self.assertEqual(filled.target_bits, 80)
        with self.assertRaises(m.ApiError):
            m.SourceInput.from_unnamed_bytes(original, m.FileFillPolicy(8, m.PrbsSeed(7)))
        with self.assertRaises(m.ApiError):
            m.SourceInput.from_unnamed_bytes(b'\0' * (16 * 1024 * 1024 + 1))
        with self.assertRaises(m.ApiError) as named:
            m.SourceInput.from_bytes('', original)
        self.assertEqual(named.exception.code, m.ErrorCode.ResourceLimit)
        empty = m.SourceInput.from_unnamed_bytes(b'')
        self.assertEqual((empty.name, empty.original_bytes, empty.target_bits), ('', b'', 0))
        self.assertEqual(source.original_bytes, original)

    def test_minimum_observation_has_exact_period_rounding_and_wide_thresholds(self):
        # Independent periods: CP=512 gives 17/400 ms at 25 kHz and 9/400 ms
        # at 50 kHz. The 1024-sample CP gives 45 us at 25 kHz. Index 399
        # completes exactly 400 periods at 17/9 ms, without an extra millisecond.
        for last, spacing, prefix, expected in [
            (0, '25000', 512, 1),
            (399, '25000', 512, 17),
            (399, '50000', 512, 9),
            (1000, '25000', 512, 43),
            (400, '25000', 512, 18),
            (2000, '25000', 512, 86),
            (1000, '50000', 512, 23),
            (1000, '25000', 1024, 46),
            (100000, '25000', 512, 4251),
        ]:
            settings = m.TransmissionConfiguration().settings
            settings['subcarrier_spacing'] = spacing
            settings['cyclic_prefix_samples'] = prefix
            configuration = m.TransmissionConfiguration.from_json(json.dumps(settings).encode())
            before = configuration.to_json()
            self.assertEqual(configuration.minimum_observation_milliseconds_after(last), expected)
            self.assertGreater(configuration.timing(m.ObservationDuration(expected))['complete_symbols'], last)
            if expected > 1:
                self.assertLessEqual(configuration.timing(m.ObservationDuration(expected - 1))['complete_symbols'], last)
            self.assertEqual(configuration.to_json(), before)
        configuration = m.TransmissionConfiguration()
        maximum_index = (1 << 64) - 1  # Independent unsigned source-index boundary.
        # ceil(2^64 * 17/400) ms is a reportable threshold, not an admitted run.
        threshold = configuration.minimum_observation_milliseconds_after(maximum_index)
        self.assertEqual(threshold, 783986623132655944)
        with self.assertRaises(m.ApiError):
            m.ObservationDuration(threshold)
        for invalid in (-1, 1 << 64):
            with self.assertRaises(OverflowError):
                configuration.minimum_observation_milliseconds_after(invalid)

    def test_transmission_literal_timing_and_owned_settings(self):
        accepted = m.TransmissionConfiguration()
        before = accepted.to_json()
        self.assertEqual(accepted.timing(m.ObservationDuration(1)), {
            'sample_rate_hz': 204800000, 'fft_samples': 8192,
            'symbol_period_samples': 8704, 'windowed_symbol_samples': 8832,
            'observation_samples': 204800, 'complete_symbols': 23,
            'unused_samples': 4608,
        })
        changed = accepted.settings
        changed['subcarrier_spacing'] = '50000'
        alternate = m.TransmissionConfiguration.from_json(json.dumps(changed).encode())
        self.assertEqual(alternate.timing(m.ObservationDuration(1))['complete_symbols'], 44)
        self.assertEqual(accepted.to_json(), before)
        with self.assertRaises(AttributeError):
            accepted.settings = changed

    def test_transmission_rejection_and_original_profile_compatibility(self):
        accepted = m.TransmissionConfiguration()
        for data, code in ((b'{bad', m.ErrorCode.InvalidInput),
                           (b'{}', m.ErrorCode.InvalidInput),
                           (b' ' * 4097, m.ErrorCode.ResourceLimit)):
            with self.assertRaises(m.ApiError) as context:
                m.TransmissionConfiguration.from_json(data)
            self.assertEqual(context.exception.code, code)
        profile = m.DPD.from_file(ROOT / 'examples/measurements/single-channel/modulation_profile.bin', m.ProfileId(0))
        self.assertIsNone(accepted.check_profile(profile))
        mismatch = accepted.settings
        mismatch['subcarrier_spacing'] = '50000'
        other = m.TransmissionConfiguration.from_json(json.dumps(mismatch).encode())
        with self.assertRaises(m.ApiError) as context:
            other.check_profile(profile)
        self.assertEqual(context.exception.code, m.ErrorCode.IncompatibleGrid)
        self.assertIsNone(accepted.check_profile(profile))

    def test_exact_byte_file_source_fill_bounds_and_lifetime(self):
        original = bytes([0xef, 0xbb, 0xbf, 0, 13, 10, 0xff, 1])
        fill = m.FileFillPolicy(80, m.PrbsSeed(7))
        accepted = m.SourceInput.from_bytes('unaltered.dat', original, fill)
        self.assertEqual(accepted.kind, 'file')
        self.assertEqual(accepted.name, 'unaltered.dat')
        self.assertEqual(accepted.original_bytes, original)
        self.assertEqual(accepted.original_sha256, hashlib.sha256(original).hexdigest())
        self.assertEqual(accepted.target_bits, 80)
        self.assertIsNone(accepted.prbs_seed)
        self.assertEqual(fill.seed.value, 7)
        self.assertEqual(fill.target_bits, 80)
        with self.assertRaises(m.ApiError):
            m.SourceInput.from_bytes('short', original, m.FileFillPolicy(63, m.PrbsSeed(7)))
        with self.assertRaises(m.ApiError):
            m.FileFillPolicy(134217729, m.PrbsSeed(7))
        del fill
        self.assertEqual(accepted.original_bytes, original)
        self.assertEqual(accepted.target_bits, 80)

    def test_empty_eof_and_continuous_seed_remain_distinct(self):
        empty = m.SourceInput.from_bytes('empty', b'')
        self.assertEqual(empty.original_bytes, b'')
        self.assertEqual(empty.target_bits, 0)
        continuous = m.SourceInput.prbs15(m.PrbsSeed(32767))
        self.assertEqual(continuous.kind, 'prbs15')
        self.assertEqual(continuous.prbs_seed.value, 32767)
        self.assertIsNone(continuous.original_bytes)
        self.assertIsNone(continuous.target_bits)
        self.assertIsNone(continuous.name)


if __name__ == '__main__':
    unittest.main()
