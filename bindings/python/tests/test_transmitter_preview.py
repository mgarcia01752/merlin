#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Installed native idle TX API, immutable results and real cross-thread cancellation."""
import json
import os
from pathlib import Path
import threading
import time
import unittest

import merlin
import merlin.downstream as m

PROFILE = Path(__file__).resolve().parents[3] / 'examples/measurements/single-channel/modulation_profile.json'


def inputs():
    return (m.TransmissionConfiguration(), m.DPD.from_file(PROFILE, m.ProfileId(0)),
            m.SourceInput.prbs15(m.PrbsSeed(32767)))


class TransmitterPreviewTests(unittest.TestCase):
    def test_actual_generation_progress_count_normalization_and_owned_queries(self):
        transmission, profile, source = inputs()
        request = m.TransmitterPreviewRequest(transmission, profile, source)
        control = m.TransmitterPreviewControl()
        self.assertEqual(control.snapshot()['state'], 'idle')
        result = request.generate(control)
        progress, info, spectrum = control.snapshot(), result.info(), result.spectrum()
        self.assertEqual(progress['state'], 'completed')
        self.assertEqual(progress['completed_symbols'], 31)
        self.assertEqual(progress['total_symbols'], 31)
        self.assertEqual(info['sample_count'], 135168)
        self.assertEqual(info['first_sample'], 130560)
        self.assertEqual(info['warmup_symbols'], 15)
        self.assertFalse(info['source_exhausted'])
        self.assertEqual(info['reference_plane'], 'tx_post_overlap_before_channel')
        self.assertEqual(spectrum['averages'], 32)
        self.assertEqual(spectrum['fft_samples'], 8192)
        self.assertAlmostEqual(spectrum['equivalent_noise_bandwidth_hz'], 37500, places=7)
        self.assertEqual(len(spectrum['buckets']), 2048)
        self.assertEqual(len(result.sample_view()['samples']), 4096)
        self.assertEqual(result.sample_view(135160)['first_sample'], 265720)
        before = result.sample_view(0, 64)
        repeated = request.generate()
        self.assertEqual(before, repeated.sample_view(0, 64))
        before['samples'][0][0] = 12345
        self.assertNotEqual(before, result.sample_view(0, 64))
        self.assertIs(merlin.TransmitterPreview, m.TransmitterPreview)
        del result, request, profile, source, transmission
        self.assertEqual(repeated.info()['sources'][0]['original_sha256'], info['sources'][0]['original_sha256'])
        self.assertEqual(len(repeated.sample_view(135160)['samples']), 8)
        with self.assertRaises(m.ApiError) as raised:
            # A finished one-shot control cannot be used again.
            m.TransmitterPreviewRequest(*inputs()).generate(control)
        self.assertEqual(raised.exception.code, m.ErrorCode.Busy)

    def test_finite_source_eof_hash_queries_and_input_retention(self):
        transmission, profile, _ = inputs()
        original = b'exact source\r\n'
        source = m.SourceInput.from_bytes('source', original)
        result = m.TransmitterPreviewRequest(transmission, profile, source).generate()
        self.assertTrue(result.info()['source_exhausted'])
        self.assertEqual(result.info()['generated_source_bits'], 112)
        self.assertEqual(result.info()['source_sha256'], source.original_sha256)
        self.assertEqual(source.original_bytes, original)
        self.assertEqual(len(result.spectrum(100, 20)['buckets']), 20)
        for start, count in [(0, 4097), (135168, 0), (135167, 2)]:
            with self.assertRaises(m.ApiError):
                result.sample_view(start, count)
        for start, count in [(8192, 0), (8191, 2)]:
            with self.assertRaises(m.ApiError):
                result.spectrum(start, count)
        with self.assertRaises(OverflowError):
            result.sample_view(-1, 1)

    def test_native_worker_releases_gil_for_busy_progress_and_cancellation(self):
        request = m.TransmitterPreviewRequest(*inputs())
        control = m.TransmitterPreviewControl()
        errors = []

        def generate():
            try:
                request.generate(control)
            except m.ApiError as cause:
                errors.append(cause)

        worker = threading.Thread(target=generate)
        worker.start()
        deadline = time.monotonic() + 10
        while control.snapshot()['state'] == 'idle':
            self.assertLess(time.monotonic(), deadline)
            time.sleep(0.001)
        self.assertEqual(control.snapshot()['state'], 'running')
        with self.assertRaises(m.ApiError) as raised:
            request.generate(control)
        self.assertEqual(raised.exception.code, m.ErrorCode.Busy)
        control.cancel()
        worker.join(timeout=15)
        self.assertFalse(worker.is_alive())
        self.assertEqual(len(errors), 1)
        self.assertEqual(control.snapshot()['state'], 'cancelled')
        self.assertLessEqual(control.snapshot()['completed_symbols'], control.snapshot()['total_symbols'])
        precancel = m.TransmitterPreviewControl()
        precancel.cancel()
        with self.assertRaises(m.ApiError):
            request.generate(precancel)
        self.assertEqual(precancel.snapshot()['completed_symbols'], 0)
        self.assertEqual(precancel.snapshot()['state'], 'cancelled')

    def test_idle_timing_weights_and_raw_identity_preserve_existing_support(self):
        transmission, profile, source = inputs()
        timing = transmission.symbol_timing()
        self.assertEqual(timing['useful_us'], 40)
        self.assertEqual(timing['symbol_us'], 42.5)
        self.assertEqual(timing['guard_us'], 1.875)
        self.assertIsNone(timing['extra_path_feet'])
        self.assertLessEqual(len(timing['points']), 130)
        self.assertAlmostEqual(transmission.symbol_timing(0.82)['reflection_spacing_feet'], 756.1202496309055, places=6)
        weights = transmission.window()
        self.assertEqual(weights['stride_samples'], 8704)
        self.assertEqual(len(weights['coefficients']), 8832)
        for index in range(128):
            self.assertAlmostEqual(weights['coefficients'][index] + weights['coefficients'][8704 + index], 1, places=12)
        for velocity in [0, -1, 1.01, float('nan'), float('inf')]:
            with self.assertRaises(m.ApiError):
                transmission.symbol_timing(velocity)
        document = json.loads(PROFILE.read_bytes())
        document['profiles'][0]['profile_id'] = 200
        capture = m.ProfileCapture.from_bytes('raw', json.dumps(document).encode())
        for identity in [99, 200]:
            with self.assertRaises(m.ApiError):
                m.TransmitterPreviewRequest.from_capture(transmission, capture, identity, source)
        capture = m.ProfileCapture.from_bytes('source', PROFILE.read_bytes())
        m.TransmitterPreviewRequest.from_capture(transmission, capture, 0, source)

    @unittest.skipUnless(os.environ.get('MERLIN_TRANSMITTER_PARITY'), 'Direct Rust process reference required')
    def test_direct_rust_process_samples_psd_timing_and_progress_parity(self):
        reference = json.loads(Path(os.environ['MERLIN_TRANSMITTER_PARITY']).read_text())
        transmission, profile, source = inputs()
        control = m.TransmitterPreviewControl()
        output = m.TransmitterPreviewRequest(transmission, profile, source).generate(control)
        self.assertEqual(output.info()['key'], reference['info']['key'])
        self.assertEqual(output.info()['layout'], reference['info']['layout'])
        for name in ['sample_count', 'first_sample', 'generated_source_bits', 'warmup_symbols', 'source_exhausted']:
            self.assertEqual(output.info()[name], reference['info'][name])
        samples = output.sample_view(0, 8)
        self.assertEqual(samples['first_sample'], reference['samples']['first_sample'])
        for actual, expected in zip(samples['samples'], reference['samples']['samples']):
            for value, anchor in zip(actual, expected):
                self.assertAlmostEqual(value, anchor, places=11)
        spectrum = output.spectrum(100, 16)
        self.assertEqual(spectrum['density_units'], reference['spectrum']['density_units'])
        for actual, expected in zip(spectrum['buckets'], reference['spectrum']['buckets']):
            self.assertEqual(actual['first_bin'], expected['first_bin'])
            self.assertEqual(actual['last_bin'], expected['last_bin'])
            for name in ['minimum_db', 'maximum_db']:
                self.assertAlmostEqual(actual[name], expected[name], places=9)
        self.assertEqual(control.snapshot(), reference['progress'])
        self.assertEqual(transmission.symbol_timing(0.82), reference['timing'])
