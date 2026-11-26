#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Installed native parity for passive transmission/source admission."""
import hashlib
import json
import os
from pathlib import Path
import unittest

import merlin as m

ROOT = Path(__file__).resolve().parents[3]


class NativeAdmissionTests(unittest.TestCase):
    def test_source_file_info_limits_partial_targets_and_owned_snapshots(self):
        """Independent original/work ceilings and partial-byte accounting remain Rust-owned."""
        limits = m.SourceInput.file_limits()
        self.assertEqual(limits, {'max_original_bytes': 16777216, 'max_total_bits': 134217728})
        limits['max_original_bytes'] = 0
        self.assertEqual(m.SourceInput.file_limits()['max_original_bytes'], 16777216)
        self.assertIsNone(m.SourceInput.prbs15(m.PrbsSeed(1)).file_info())
        self.assertEqual(m.SourceInput.from_bytes('empty', b'').file_info(),
                         {'original_bytes': 0, 'original_bits': 0, 'target_bits': 0,
                          'fill_bits': 0, 'fill': None})

        # One actual octet plus a separate three-bit append; seven is the independent seed.
        original = bytearray([0x80])
        source = m.SourceInput.from_unnamed_bytes(original, m.FileFillPolicy(11, m.PrbsSeed(7)))
        expected = {'original_bytes': 1, 'original_bits': 8, 'target_bits': 11,
                    'fill_bits': 3, 'fill': {'target_bits': 11, 'seed': 7}}
        info = source.file_info()
        self.assertEqual(info, expected)
        info['fill']['seed'] = 0
        info['target_bits'] = 999
        original[0] = 0
        self.assertEqual(source.file_info(), expected)
        self.assertEqual(source.original_bytes, b'\x80')
        self.assertEqual(source.original_sha256, hashlib.sha256(b'\x80').hexdigest())
        self.assertEqual(source.name, '')
        equal = m.SourceInput.from_bytes('equal', b'\x80', m.FileFillPolicy(8, m.PrbsSeed(1)))
        self.assertEqual(equal.file_info()['fill_bits'], 0)
        self.assertIsNotNone(equal.file_info()['fill'])
        self.assertIsNone(m.SourceInput.from_unnamed_bytes(b'\x80').file_info()['fill'])

    @unittest.skipUnless(os.environ.get('MERLIN_SOURCE_FILE_INFO_REFERENCE'), 'Direct Rust reference required')
    def test_source_file_info_matches_the_direct_rust_process(self):
        """The current installed native artifact matches a bounded independent Rust process."""
        reference = json.loads(Path(os.environ['MERLIN_SOURCE_FILE_INFO_REFERENCE']).read_text())
        sources = [m.SourceInput.prbs15(m.PrbsSeed(1)),
                   m.SourceInput.from_bytes('empty', b''),
                   m.SourceInput.from_unnamed_bytes(b'\x80'),
                   m.SourceInput.from_unnamed_bytes(b'\x80', m.FileFillPolicy(11, m.PrbsSeed(7))),
                   m.SourceInput.from_bytes('equal', b'\x80', m.FileFillPolicy(8, m.PrbsSeed(1)))]
        self.assertEqual(m.SourceInput.file_limits(), reference['limits'])
        self.assertEqual([source.file_info() for source in sources], reference['info'])
        extended = sources[2].with_fill(m.FileFillPolicy(11, m.PrbsSeed(7)))
        reset = extended.with_fill()
        self.assertEqual([extended.file_info(), reset.file_info()], reference['edits'])

    def test_source_fill_replacement_preserves_original_and_rejects_atomic_changes(self):
        """Immutable append editing preserves original bytes/hash and independent source lifetimes."""
        original = m.SourceInput.from_unnamed_bytes(b'\x80')
        extended = original.with_fill(m.FileFillPolicy(11, m.PrbsSeed(7)))
        self.assertEqual(original.target_bits, 8)
        self.assertEqual(extended.target_bits, 11)
        self.assertEqual(extended.original_bytes, original.original_bytes)
        self.assertEqual(extended.original_sha256, original.original_sha256)
        self.assertEqual(extended.name, '')
        reset = extended.with_fill()
        self.assertEqual(reset.target_bits, 8)
        self.assertIsNone(reset.file_info()['fill'])
        with self.assertRaises(m.ApiError):
            extended.with_fill(m.FileFillPolicy(7, m.PrbsSeed(1)))
        self.assertEqual(extended.target_bits, 11)
        with self.assertRaises(m.ApiError) as rejected:
            m.SourceInput.prbs15(m.PrbsSeed(1)).with_fill()
        self.assertEqual(rejected.exception.code, m.ErrorCode.InvalidInput)
        del original
        self.assertEqual(extended.original_bytes, b'\x80')

    def test_spectrum_average_capacity_is_bounded_idle_and_immutable(self):
        # Independent N/hop and 32768-sample retained-prefix anchors for both spacings.
        for spacing, cases, below in [
            ('25000', [(8192, 1), (12287, 1), (12288, 2), (32768, 7), (135168, 32)], 8191),
            ('50000', [(4096, 1), (6143, 1), (6144, 2), (32768, 15), (67584, 32)], 4095),
        ]:
            settings = m.TransmissionConfiguration().settings
            settings['subcarrier_spacing'] = spacing
            configuration = m.TransmissionConfiguration.from_json(json.dumps(settings).encode())
            before = configuration.to_json()
            for budget, expected in cases:
                self.assertEqual(configuration.spectrum_average_capacity(budget), expected)
            # Independent unsigned 64-bit host-size boundary; positive budgets never widen work.
            self.assertEqual(configuration.spectrum_average_capacity((1 << 64) - 1), 32)
            for budget in (0, 1, below):
                with self.assertRaises(m.ApiError) as failure:
                    configuration.spectrum_average_capacity(budget)
                self.assertEqual(failure.exception.code, m.ErrorCode.InvalidInput)
                self.assertEqual(failure.exception.engine_code, 'InvalidLabInput')
            for budget in (-1, 1 << 64):
                with self.assertRaises(OverflowError):
                    configuration.spectrum_average_capacity(budget)
            self.assertEqual(configuration.to_json(), before)

    @unittest.skipUnless(os.environ.get('MERLIN_SPECTRUM_BUDGET_REFERENCE'), 'Direct Rust process reference required')
    def test_spectrum_average_capacity_matches_the_direct_rust_process(self):
        records = json.loads(Path(os.environ['MERLIN_SPECTRUM_BUDGET_REFERENCE']).read_text())
        self.assertEqual([record['spacing'] for record in records], ['25000', '50000'])
        for record in records:
            settings = m.TransmissionConfiguration().settings
            settings['subcarrier_spacing'] = record['spacing']
            configuration = m.TransmissionConfiguration.from_json(json.dumps(settings).encode())
            for case in record['cases']:
                if 'capacity' in case:
                    self.assertEqual(configuration.spectrum_average_capacity(case['max_samples']), case['capacity'])
                else:
                    with self.assertRaises(m.ApiError) as failure:
                        configuration.spectrum_average_capacity(case['max_samples'])
                    self.assertEqual(case['error_code'], 'InvalidInput')
                    self.assertEqual(failure.exception.code, m.ErrorCode.InvalidInput)
                    self.assertEqual(failure.exception.engine_code, case['engine_code'])

    def test_engine_code_is_structured_optional_and_preserves_existing_error_fields(self):
        # Zero is the existing rejected active-runtime boundary; its original code is stable.
        with self.assertRaises(m.ApiError) as failure:
            m.FullLoopOptions.from_json(b'{"max_runtime_ms":0}')
        error = failure.exception
        self.assertEqual(error.engine_code, 'InvalidLabInput')
        self.assertEqual(error.code, m.ErrorCode.InvalidInput)
        self.assertEqual(error.message, 'Invalid channel runtime or retained-word budget.')
        self.assertIsNone(error.legacy_code)
        self.assertIsNone(error.field)
        self.assertIsNone(error.offset)
        # API-only rejection has no retained engine cause, despite an identical generic category.
        with self.assertRaises(m.ApiError) as adapter:
            m.FullLoopOptions.from_json(b'{')
        self.assertEqual(adapter.exception.code, m.ErrorCode.InvalidInput)
        self.assertIsNone(adapter.exception.engine_code)
        self.assertIsNone(m.ApiError('InvalidLabInput is only display text here.').engine_code)

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
